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
pub fn fail_on_err0<T>(func: impl FnOnce() -> Result<T>) -> T {
    match func() {
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

pub struct State {
    entries: Vec<Entry>,
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

impl State {
    pub fn load() -> Result<Self> {
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
            Ok(Self { entries })
        } else {
            Err(Error.into())
        }
    }
    pub fn find_entries(&self, query: &str) -> Vec<Entry> {
        self.entries
            .iter()
            .filter(|entry| entry.name.contains(query))
            .cloned()
            .collect()
    }
}

#[init]
fn init(_: RString) -> State {
    fail_on_err0(State::load)
}

#[info]
fn info() -> PluginInfo {
    PluginInfo {
        name: "Bitwarden".into(),
        icon: "bitwarden".into(),
    }
}

#[get_matches]
fn get_matches(input: RString, state: &State) -> RVec<Match> {
    state
        .find_entries(&input)
        .into_iter()
        .map(|entry| Match {
            id: ROption::RNone,
            title: format!("{}/{}", entry.name, entry.user).into(),
            description: Some(format!("{}", entry.folder).into()).into(),
            use_pango: true,
            icon: ROption::RNone,
        })
        .collect()
}

#[handler]
fn handler(selection: Match, state: &State) -> HandleResult {
    HandleResult::Copy(
        state
            .entries
            .iter()
            .find(|entry| format!("{}/{}", entry.name, entry.user) == selection.title)
            .map(|entry| entry.id.as_bytes().to_vec())
            .unwrap_or_default()
            .into(),
    )
}
