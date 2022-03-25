mod memory;
mod redis;

pub use self::memory::MemoryCache;
pub use self::redis::RedisCache;
use crate::error::Result;

#[allow(clippy::ptr_arg)]
#[rocket::async_trait]
pub trait Cache {
    async fn ping(&self) -> bool;
    async fn get(&self, key: &String) -> Result<String>;
    async fn set(&self, key: &String, value: &String, ttl_seconds: usize) -> Result<()>;
}
