use crate::cache::Cache;
use crate::error::{OmniRssError, Result};
use chrono::prelude::*;
use chrono::{Duration, Utc};
use futures::lock::Mutex;
use std::collections::HashMap;
use std::ops::Add;

pub(crate) struct MemoryCache {
    cache: Mutex<HashMap<String, (String, chrono::DateTime<chrono::Utc>)>>,
    time_offset: i64,
}

impl MemoryCache {
    #[cfg(test)]
    pub(crate) fn new() -> Self {
        Self {
            cache: Mutex::new(HashMap::new()),
            time_offset: 0,
        }
    }

    #[cfg(test)]
    pub(crate) fn time_travel(mut self, seconds: i64) -> Self {
        self.time_offset += seconds;
        self
    }
}

#[rocket::async_trait]
impl Cache for MemoryCache {
    async fn ping(&self) -> bool {
        true
    }

    async fn get(&self, key: &String) -> Result<String> {
        let expire_cutoff: DateTime<Utc> = Utc::now().add(Duration::seconds(self.time_offset));
        let cache = self.cache.lock().await;
        match cache.get(key) {
            Some((v, expire)) if *expire > expire_cutoff => Ok(v.clone()),
            _ => Err(OmniRssError::InMemoryKeyNotFound),
        }
    }

    async fn set(
        &self,
        key: &String,
        value: &String,
        ttl_seconds: usize,
    ) -> crate::error::Result<()> {
        let mut cache = self.cache.lock().await;
        cache.insert(
            key.clone(),
            (
                value.clone(),
                Utc::now().add(chrono::Duration::seconds(
                    ttl_seconds as i64 + self.time_offset,
                )),
            ),
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cache::memory::MemoryCache;
    use crate::error::OmniRssError;
    use crate::Cache;

    #[tokio::test]
    async fn ping() {
        let mem_cache = MemoryCache::new();
        assert!(mem_cache.ping().await);
    }

    #[tokio::test]
    async fn get_nonexistent() {
        let mem_cache = MemoryCache::new();
        assert!(matches!(
            mem_cache.get(&"foo".to_string()).await,
            Err(OmniRssError::InMemoryKeyNotFound)
        ));
    }

    #[tokio::test]
    async fn get_set() {
        let mem_cache = MemoryCache::new();
        let key = "foo".to_string();
        let value = "bar".to_string();

        let set_result = mem_cache.set(&key, &value, 10).await;
        assert!(matches!(set_result, Ok(())));

        let get_result = mem_cache.get(&key).await;
        assert!(matches!(get_result, Ok(_)));
        assert_eq!(get_result.unwrap(), value);
    }

    #[tokio::test]
    async fn set_ttl() {
        let mut mem_cache = MemoryCache::new();
        let key = "foo".to_string();
        let value = "bar".to_string();

        let _ = mem_cache.set(&key, &value, 10).await;
        mem_cache = mem_cache.time_travel(15);

        assert!(matches!(
            mem_cache.get(&key).await,
            Err(OmniRssError::InMemoryKeyNotFound)
        ));
    }
}
