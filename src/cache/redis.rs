use crate::error::{OmniRssError, Result};
use crate::settings::Settings;
use crate::Cache;
use redis::AsyncCommands;

pub struct RedisCache(redis::Client);

impl RedisCache {
    pub fn from_settings(config: &Settings) -> Result<Self> {
        info!("Starting Redis client");
        Ok(Self(redis::Client::open(config.redis.get_connection_url().unwrap())?))
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

    async fn get(&self, key: &String) -> Result<String> {
        let mut con = self.0.get_async_connection().await?;
        con.get(key).await.map_err(OmniRssError::RedisError)
    }

    async fn set(&self, key: &String, value: &String, ttl_seconds: usize) -> Result<()> {
        let mut con = self.0.get_async_connection().await?;
        con.set_ex::<&String, &String, ()>(key, value, ttl_seconds)
            .await
            .map_err(OmniRssError::RedisError)
    }
}
