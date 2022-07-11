//Jyotsna's attempt to use the thiserror crate
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<T>,
}

#[derive(Error, Debug)]
pub enum Error<T> {
    #[error("error in reqwest: {source:?}")]
    Reqwest {
        #[from]
        source: reqwest::Error,
    },

    #[error("error in serde: {source:?}")]
    Serde {
        #[from]
        source: serde_json::Error,
    },

    #[error("error in IO: {source:?}")]
    Io {
        #[from]
        source: std::io::Error,
    },

    #[error("error in response: status code {{error.status:?}}")]
    ResponseError(ResponseContent<T>),
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

pub mod entries_api;
pub mod index_api;
pub mod pubkey_api;
pub mod server_api;
pub mod timestamp_api;
pub mod tlog_api;

pub mod configuration;
