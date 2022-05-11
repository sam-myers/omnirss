use crate::cache::Cache;
use crate::error::{OmniRssError, Result};
use crate::settings::Settings;
use redis::AsyncCommands;
use tracing::{error, instrument};

#[derive(Debug, Clone)]
pub struct RedisCache(redis::Client);

impl RedisCache {
    pub fn from_settings(settings: &Settings) -> Result<Self> {
        info!("Starting Redis client");
        Ok(Self(redis::Client::open(
            settings.redis_connection_url().unwrap(),
        )?))
    }
}

#[rocket::async_trait]
impl Cache for &RedisCache {
    async fn ping(&self) -> bool {
        let con = self.0.get_async_connection().await;
        if let Err(e) = con {
            warn!("Ping to Redis failed: trying to create connection: {}", e);
            return false;
        }
        let mut con = con.unwrap();
        if let Err(e) = redis::pipe()
            .cmd("PING")
            .ignore()
            .query_async::<redis::aio::Connection, ()>(&mut con)
            .await
        {
            warn!("Ping to Redis failed: ping command: {}", e);
            return false;
        }
        true
    }

    #[instrument]
    async fn get(&self, key: &String) -> Result<Option<String>> {
        let mut con = self.0.get_async_connection().await?;
        match con.get(key).await {
            Ok(s) => Ok(s),
            Err(e) => {
                error!("Redis Error: {:?}", e);
                return Err(OmniRssError::RedisError(e));
            }
        }
    }

    #[instrument(err)]
    async fn set(&self, key: &String, value: &String, ttl_seconds: usize) -> Result<()> {
        let mut con = self.0.get_async_connection().await?;
        con.set_ex::<&String, &String, ()>(key, value, ttl_seconds)
            .await
            .map_err(OmniRssError::RedisError)
    }
}
