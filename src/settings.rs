use config::{Config, ConfigError, Environment, File};

#[derive(Debug, Deserialize)]
pub struct Spotify {
    pub(crate) client_id: String,
    pub(crate) client_secret: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub(crate) log_level: String,
    pub(crate) spotify: Spotify,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();
        s.merge(File::with_name("config"))?;
        s.merge(Environment::new())?;
        s.try_into()
    }
}
