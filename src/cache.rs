use crate::settings::Settings;

use async_trait::async_trait;

pub fn client_from_config(config: &Settings) -> redis::RedisResult<redis::Client> {
    info!("Starting Redis client");
    redis::Client::open(config.redis.get_connection_url().unwrap())
}

#[async_trait]
pub trait Ping {
    async fn ping(&self) -> bool;
}

#[async_trait]
impl Ping for redis::Client {
    async fn ping(&self) -> bool {
        let redis_con = self.get_async_connection().await;
        if let Err(e) = redis_con {
            warn!("Ping to Redis failed: trying to create connection: {}", e);
            return false;
        }
        let mut con = redis_con.unwrap();
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
}