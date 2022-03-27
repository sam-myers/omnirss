#[derive(Debug, Deserialize)]
pub struct Settings {
    pub(crate) base_url: String,

    pub(crate) spotify_client_id: String,
    pub(crate) spotify_client_secret: String,

    pub(crate) redis_endpoint: String,
    pub(crate) redis_password: String,
    pub(crate) redis_port: u32,

    pub(crate) sentry_env: String,
    pub(crate) sentry_url: String,
}

impl Settings {
    pub(crate) fn redis_connection_url(&self) -> crate::error::Result<String> {
        if self.redis_password.is_empty() {
            return Err(crate::error::OmniRssError::MissingConfigValue(
                "redis password",
            ));
        }

        debug!(
            "Connecting to URL: rediss://:[PASSWORD_REDACTED]@{}:{}",
            self.redis_endpoint, self.redis_port
        );
        Ok(format!(
            "rediss://:{}@{}:{}",
            self.redis_password, self.redis_endpoint, self.redis_port
        ))
    }
}
