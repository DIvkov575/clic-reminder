#[derive(thiserror::Error, Debug)]
pub enum ClicError {
    #[error("User input invalid, please ensure input id len < 4")]
    InvalidInput,
    #[error("Too many consecutive id creation retry attempts")]
    TooManyIDRetries,
    #[error("please ensure $HOME environment variable is set")]
    MissingHomeDir,
    #[error("record key does not exist in clic")]
    NonExistentKey(String),
    #[error("id not found while indexing")]
    IdNotFound,
    #[error("no gist id: please initialize sync w/ init-web ")]
    NoGistId,
    #[error("no PAT found")]
    NoPAT,

}
