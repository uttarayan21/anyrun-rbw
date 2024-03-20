mod decrypted;
mod sock;
mod actions;

use abi_stable::std_types::{ROption, RString, RVec};
use error_stack::{Report, ResultExt};

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

pub struct State {
    db: rbw::db::Db,
}

impl State {
    pub fn load() -> Result<Self> {
        let config = rbw::config::Config::load()
            .change_context(Error)
            .attach_printable("Failed to load rbw config")?;
        config
            .email
            .as_ref()
            .map_or_else(
                || Err(Report::new(Error)),
                |email| rbw::db::Db::load(&config.server_name(), email).change_context(Error),
            )
            .map(|db| State { db })
    }
    pub fn find_entries(&self, query: &str) -> Result<Vec<String>> {
        // self.db.entries.iter();
        Ok(vec![])
    }

    pub fn decrypted_entries(&self) -> () {

    }
}

