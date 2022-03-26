use thiserror::Error;

pub type Result<T> = std::result::Result<T, OmniRssError>;

#[derive(Error, Debug)]
pub enum OmniRssError {
    #[error("missing config: {0}")]
    MissingConfigValue(&'static str),

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

    #[error("couldn't connect to Redis")]
    RedisError(#[from] redis::RedisError),

    #[error("couldn't deserialize")]
    DeserializeError(#[from] serde_json::Error),

    #[error("in memory data store didn't contain key")]
    InMemoryKeyNotFound,

    #[error("mock spotify has not implemented: {0}")]
    MockSpotifyNotImplemented(&'static str),
}
