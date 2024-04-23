use ::tap::*;
use abi_stable::std_types::{ROption, RString, RVec};
use anyrun_plugin::*;
use error_stack::{Report, ResultExt};
use std::process::Command;

type Result<T, E = Report<Error>> = core::result::Result<T, E>;
#[derive(Debug)]
pub struct Error;
impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "Error: anyrun-rbw error")
    }
}
impl std::error::Error for Error {}
pub fn fail_on_err<Args, T>(args: Args, func: impl FnOnce(Args) -> Result<T>) -> T {
    match func(args) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{:?}", e);
            if let Some(ret) = e.downcast_ref::<i32>() {
                std::process::exit(*ret);
            } else {
                std::process::exit(1);
            }
        }
    }
}
// pub fn fail_on_err0<T>(func: impl FnOnce() -> Result<T>) -> T {
//     match func() {
//         Ok(t) => t,
//         Err(e) => {
//             eprintln!("{:?}", e);
//             if let Some(ret) = e.downcast_ref::<i32>() {
//                 std::process::exit(*ret);
//             } else {
//                 std::process::exit(1);
//             }
//         }
//     }
// }

pub struct State {
    entries: Vec<Entry>,
    prefix: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Entry {
    id: String,
    name: String,
    user: String,
    folder: String,
}

impl core::str::FromStr for Entry {
    type Err = Report<Error>;
    fn from_str(s: &str) -> Result<Self> {
        let mut parts = s.split('\t');
        let id = parts.next().ok_or(Error)?;
        let name = parts.next().ok_or(Error)?;
        let user = parts.next().ok_or(Error)?;
        let folder = parts.next().ok_or(Error)?;
        Ok(Self {
            id: id.to_string(),
            name: name.to_string(),
            user: user.to_string(),
            folder: folder.to_string(),
        })
    }
}

impl Entry {
    pub fn get(&self) -> Result<String> {
        let out = Command::new("rbw")
            .arg("get")
            .arg(&self.id)
            .output()
            .change_context(Error)?;
        String::from_utf8(out.stdout)
            .change_context(Error)
            .attach_printable("Failed change context from output")
            .pipe(|p| {
                p.and_then(|pass| {
                    pass.is_empty()
                        .then(|| Err(Error.into()))
                        .unwrap_or(Ok(pass))
                })
            })
    }
}

impl State {
    pub fn load(config: Config) -> Result<Self> {
        let out = Command::new("rbw")
            .arg("list")
            .arg("--fields")
            .arg("id,name,user,folder")
            .output()
            .change_context(Error)
            .attach_printable("Failed to run the command for rbw")?;
        if out.status.success() {
            let entries = String::from_utf8(out.stdout)
                .change_context(Error)
                .attach_printable("Failed to parse the output of rbw")?;
            let entries = entries
                .lines()
                .map(|line| line.parse::<Entry>())
                .collect::<Result<Vec<_>>>()?;
            Ok(Self {
                entries,
                prefix: config.prefix,
            })
        } else {
            Err(Error.into())
        }
    }

    pub fn find_entries(&self, query: &str) -> Vec<Entry> {
        if self
            .prefix
            .as_deref()
            .map(|prefix| !query.starts_with(prefix))
            .unwrap_or(true)
            || query.is_empty()
        {
            return vec![];
        }
        let query = query.pipe(|q| {
            if let Some(prefix) = self.prefix.as_deref() {
                q.strip_prefix(prefix).unwrap_or(q).trim()
            } else {
                q
            }
        });
        use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
        let matcher = SkimMatcherV2::default();
        self.entries
            .iter()
            // .filter(|entry| entry.name.contains(query))
            .filter(|entry| {
                matcher
                    .fuzzy_match(
                        &format!("{}/{}/{}", entry.folder, entry.name, entry.user),
                        query,
                    )
                    .is_some()
            })
            .cloned()
            .collect()
    }
}

#[derive(serde::Deserialize)]
pub struct Config {
    prefix: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            prefix: Some(":p".into()),
        }
    }
}

#[init]
fn init(config_dir: RString) -> State {
    let config = (|| -> Result<Config> {
        let config = std::fs::read(config_dir.as_str()).change_context(Error)?;
        let config: Config = ron::de::from_bytes(&config).change_context(Error)?;
        Ok(config)
    })()
    .unwrap_or_default();
    fail_on_err(config, State::load)
}

#[info]
fn info() -> PluginInfo {
    PluginInfo {
        name: "Bitwarden".into(),
        icon: "bitwarden".into(),
    }
}

#[get_matches]
pub fn get_matches(input: RString, state: &State) -> RVec<Match> {
    state
        .find_entries(&input)
        .into_iter()
        .map(|entry| {
            let mut digest = crc64fast::Digest::new();
            digest.write(entry.id.as_bytes());
            Match {
                id: Some(digest.sum64()).into(),
                title: [entry.name, entry.user].join("/").into(),
                description: Some(entry.folder.into()).into(),
                use_pango: true,
                icon: ROption::RNone,
            }
        })
        .collect()
}

#[handler]
pub fn handler(selection: Match, state: &State) -> HandleResult {
    HandleResult::Copy(
        state
            .entries
            .iter()
            .find(|e| crc64(&e.id) == selection.id.unwrap_or_default())
            .and_then(|entry| entry.get().ok())
            .map(|e| e.as_bytes().to_vec())
            .unwrap_or_default()
            .into(),
    )
}

fn crc64(input: impl AsRef<[u8]>) -> u64 {
    let mut digest = crc64fast::Digest::new();
    digest.write(input.as_ref());
    digest.sum64()
}
