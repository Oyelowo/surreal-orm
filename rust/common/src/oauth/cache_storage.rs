use lru;
use redis::AsyncCommands;
use std::collections::HashMap;

use super::error::{OauthError, OauthResult};

#[async_trait::async_trait]
pub trait CacheStorage: Send + Sync + 'static {
    /// Load the query by `key`.
    async fn get(&mut self, key: String) -> OauthResult<String>;
    /// Save the query by `key`.
    async fn set(&mut self, key: String, query: String) -> OauthResult<()>;
}

#[derive(Clone, Debug)]
pub struct RedisCache(redis::Client);

impl RedisCache {
    pub fn new(client: redis::Client) -> Self {
        Self(client)
    }
}

#[async_trait::async_trait]
impl CacheStorage for RedisCache {
    async fn get(&mut self, key: String) -> OauthResult<String> {
        let data: String = self.0.get_async_connection().await?.get(key).await?;

        Ok(data)
    }

    async fn set(&mut self, key: String, value: String) -> OauthResult<()> {
        let mut con = self.0.get_async_connection().await?;
        con.set::<String, String, String>(key.clone(), value);

        con.expire::<_, u16>(key, 600).await?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct HashMapCache(HashMap<String, String>);

impl Default for HashMapCache {
    fn default() -> Self {
        Self::new()
    }
}

impl HashMapCache {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
}

#[async_trait::async_trait]
impl CacheStorage for HashMapCache {
    async fn get(&mut self, key: String) -> OauthResult<String> {
        let data = self
            .0
            .get(&key)
            .map(String::from)
            .ok_or(OauthError::AuthUrlDataNotFoundInCache)?;

        Ok(data)
    }

    async fn set(&mut self, key: String, value: String) -> OauthResult<()> {
        self.0.insert(key, value);
        Ok(())
    }
}

/// LRU cache.
#[derive(Debug)]
pub struct LruCache(lru::LruCache<String, String>);

impl Default for LruCache {
    fn default() -> Self {
        Self::new(50000)
    }
}

impl LruCache {
    pub fn new(cap: u16) -> Self {
        Self(lru::LruCache::new(cap as usize))
    }
}

#[async_trait::async_trait]
impl CacheStorage for LruCache {
    async fn get(&mut self, key: String) -> OauthResult<String> {
        let data = self
            .0
            .get(&key)
            .cloned()
            .ok_or(OauthError::AuthUrlDataNotFoundInCache)?;
        Ok(data)
    }

    async fn set(&mut self, key: String, value: String) -> OauthResult<()> {
        self.0.put(key, value);
        Ok(())
    }
}
