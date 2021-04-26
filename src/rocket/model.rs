use std::error::Error;
use std::fmt;


#[derive(Serialize, Deserialize, Debug)]
pub struct Email {
    address: String,
    verified: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Preferences {
    preferences: serde_json::Value
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
    #[serde(rename(deserialize = "_id"))]
    _id: String,
    name: String,
    emails: Vec<Email>,
    status: String,
    status_connection: String,
    username: String,
    utc_offset: i64,
    active: bool,
    roles: Vec<String>,
    settings: Preferences,
    avatar_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Auth {
    pub auth_token: String,
    pub user_id: String,
    pub me: User,
}

// #[derive(Serialize, Deserialize)]
// pub struct Room {
//     _id: String,
//     t: String,
//     name: String,
//     u: User,
//     topic: String,
//     muted: [String],
//     jitsi_timeout: String,
// }


pub type Result<T> = std::result::Result<T, RocketChatErrorType>;

#[derive(Debug)]
pub enum RocketChatErrorType {
    Io(std::io::Error),
    Serde(serde_json::Error),
    Curl(curl::Error),
    //
    Regular(ErrorKind),
    Custom(String),
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ErrorKind {
    General,
    NotAuthorized,
}

impl ErrorKind {
    fn as_str(&self) -> &str {
        match *self {
            ErrorKind::General => "general error",
            ErrorKind::NotAuthorized => "not authorized",
        }
    }
}

impl Error for RocketChatErrorType {
    fn description(&self) -> &str {
        match *self {
            RocketChatErrorType::Io(ref err) => err.description(),
            RocketChatErrorType::Serde(ref err) => err.description(),
            RocketChatErrorType::Curl(ref err) => err.description(),
            RocketChatErrorType::Regular(ref err) => err.as_str(),
            RocketChatErrorType::Custom(ref err) => err,
        }
    }
}

impl fmt::Display for RocketChatErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RocketChatErrorType::Io(ref err) => err.fmt(f),
            RocketChatErrorType::Serde(ref err) => err.fmt(f),
            RocketChatErrorType::Curl(ref err) => err.fmt(f),
            RocketChatErrorType::Regular(ref err) => write!(f, "A regular error occurred {:?}", err),
            RocketChatErrorType::Custom(ref err) => write!(f, "A custom error occurred {:?}", err),
        }
    }
}


impl From<std::io::Error> for RocketChatErrorType {
    fn from(err: std::io::Error) -> RocketChatErrorType {
        RocketChatErrorType::Io(err)
    }
}

impl From<serde_json::Error> for RocketChatErrorType {
    fn from(err: serde_json::Error) -> RocketChatErrorType {
        RocketChatErrorType::Serde(err)
    }
}

impl From<curl::Error> for RocketChatErrorType {
    fn from(err: curl::Error) -> RocketChatErrorType {
        RocketChatErrorType::Curl(err)
    }
}
