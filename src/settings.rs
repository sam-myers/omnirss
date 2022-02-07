use config::{Config, ConfigError, Environment, File};

#[derive(Debug, Deserialize)]
pub struct Spotify {
    pub(crate) client_id: String,
    pub(crate) client_secret: String,
}

#[derive(Debug, Deserialize)]
pub struct Redis {
    pub(crate) endpoint: String,
    pub(crate) password: String,
    pub(crate) port: u32,
}

impl Redis {
    pub(crate) fn get_connection_url(&self) -> crate::error::Result<String> {
        if self.password == "" {
            return Err(crate::error::OmniRssError::MissingConfigValue(
                "redis password",
            ));
        }

        debug!(
            "Connecting to URL: rediss://:[PASSWORD_REDACTED]@{}:{}",
            self.endpoint, self.port
        );
        Ok(format!(
            "rediss://:{}@{}:{}",
            self.password, self.endpoint, self.port
        ))
    }
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub(crate) log_level: String,
    pub(crate) redis: Redis,
    pub(crate) spotify: Spotify,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();
        if let Err(_) = s.merge(File::with_name("config")) {
            info!("No config file found, using only environment variables");
            s = Config::new(); // The library freezes the type after a failed merge
        }
        s.merge(Environment::new())?;

        // Defaults
        let _ = s.set_default("log_level", "info");
        let _ = s.set_default("redis.port", 35884);

        s.try_into()
    }
}
