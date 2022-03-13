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
        if self.password.is_empty() {
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
    pub(crate) redis: Redis,
    pub(crate) spotify: Spotify,
}
