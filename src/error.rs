use thiserror::Error;

pub type Result<T> = std::result::Result<T, SpotiRssError>;

#[derive(Error, Debug)]
pub enum SpotiRssError {
    #[error("key is missing")]
    MissingKey,

    #[error("key is not 32 digits")]
    BadKeyLength,

    #[error("key has invalid characters")]
    BadKeyCharacters,

    #[error("unspecified env var")]
    UnspecifiedEnvVar(#[from] std::env::VarError),

    #[error("connection error")]
    Connectivity(#[from] reqwest::Error),

    #[error("couldn't parse date")]
    DateParseError(#[from] chrono::ParseError),

    #[error("couldn't validate rss feed")]
    RssValidationError(#[from] rss::validation::ValidationError),
}
