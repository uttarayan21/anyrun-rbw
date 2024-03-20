//! Load the commands module from rbw binary

use std::str::FromStr;

use error_stack::Report;
use rbw::cipherstring;
use serde::Serialize;
use crate::Error;
use crate::Result;
// pub fn decrypt_cypher(entry: &rbw::db::Entry) -> Result<DecryptedCipher> {
//     let folder = entry.folder.as_ref().map(|f| rbw::actions::decrypt(f, None).unwrap());
// }

pub fn decrypt(
    cipherstring: &str,
    org_id: Option<&str>,
) -> Result<String> {
    // let mut sock = connect();
    todo!()
}






#[derive(Debug, Clone, Serialize)]
#[cfg_attr(test, derive(Eq, PartialEq))]
struct DecryptedCipher {
    id: String,
    folder: Option<String>,
    name: String,
    data: DecryptedData,
    fields: Vec<DecryptedField>,
    notes: Option<String>,
    history: Vec<DecryptedHistoryEntry>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
#[cfg_attr(test, derive(Eq, PartialEq))]
enum DecryptedData {
    Login {
        username: Option<String>,
        password: Option<String>,
        totp: Option<String>,
        uris: Option<Vec<DecryptedUri>>,
    },
    Card {
        cardholder_name: Option<String>,
        number: Option<String>,
        brand: Option<String>,
        exp_month: Option<String>,
        exp_year: Option<String>,
        code: Option<String>,
    },
    Identity {
        title: Option<String>,
        first_name: Option<String>,
        middle_name: Option<String>,
        last_name: Option<String>,
        address1: Option<String>,
        address2: Option<String>,
        address3: Option<String>,
        city: Option<String>,
        state: Option<String>,
        postal_code: Option<String>,
        country: Option<String>,
        phone: Option<String>,
        email: Option<String>,
        ssn: Option<String>,
        license_number: Option<String>,
        passport_number: Option<String>,
        username: Option<String>,
    },
    SecureNote,
}

#[derive(Debug, Clone, Serialize)]
#[cfg_attr(test, derive(Eq, PartialEq))]
struct DecryptedField {
    name: Option<String>,
    value: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[cfg_attr(test, derive(Eq, PartialEq))]
struct DecryptedHistoryEntry {
    last_used_date: String,
    password: String,
}

#[derive(Debug, Clone, Serialize)]
#[cfg_attr(test, derive(Eq, PartialEq))]
struct DecryptedUri {
    uri: String,
    match_type: Option<rbw::api::UriMatchType>,
}

enum ListField {
    Name,
    Id,
    User,
    Folder,
}

impl FromStr for ListField {
    type Err = Report<Error>;

    fn from_str(s: &str) -> crate::Result<Self> {
        Ok(match s {
            "name" => Self::Name,
            "id" => Self::Id,
            "user" => Self::User,
            "folder" => Self::Folder,
            _ => return Err(Report::new(Error).attach_printable(format!("Unknown field {s}"))),
        })
    }
}
