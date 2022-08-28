use std::{
    borrow::Cow,
    collections::{hash_map::RandomState, HashMap},
    hash::{BuildHasher, Hash},
    marker::PhantomData,
};

// use lru;
use redis::Commands;

#[async_trait::async_trait]
pub trait CacheStorage: Send + Sync + Clone + 'static {
    /// Load the query by `key`.
    async fn get(&self, key: String) -> Option<String>;
    /// Save the query by `key`.
    async fn set(&mut self, key: String, query: String);
}

use redis::AsyncCommands;

#[derive(Clone)]
pub struct RedisCache(pub(crate) redis::Client);

#[async_trait::async_trait]
impl CacheStorage for RedisCache {
    async fn get(&self, key: String) -> Option<String> {
        let data: String = self
            .0
            .get_async_connection()
            .await
            .unwrap()
            .get(key)
            .await
            .unwrap();

        Some(data)
    }

    async fn set(&mut self, key: String, value: String) {
        let mut con = self.0.get_async_connection().await.unwrap();
        con.set::<String, String, String>(key.clone(), value);

        con.expire::<_, u16>(key, 600).await.unwrap();
    }

    // async fn remove(&self, key: String) {
    //     self.0.del(&key);
    // }
}

#[derive(Debug, Clone)]
pub struct HashMapCache(HashMap<String, String>);
// pub struct HashMapCache(HashMap<String, String>);

impl HashMapCache {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
}

#[async_trait::async_trait]
impl CacheStorage for HashMapCache {
    async fn get(&self, key: String) -> Option<String> {
        let data = self.0.get(&key).map(String::from);

        data
    }

    // #[warn(unused_must_use)]
    async fn set(&mut self, key: String, value: String) {
        self.0.insert(key, value);
    }
}

// /// LRU cache.
// pub struct LruCache {
//     cap: usize,
// }

// struct LruCacheImpl(pub(crate) lru::LruCache<String, String>);

// impl CacheStorage for LruCacheImpl {
//     #[inline]
//     fn get(&self, key: String) -> Option<String> {
//         self.0.get(key)
//     }

//     #[inline]
//     fn set(&mut self, key: String, val: String) {
//         self.0.put(key.into_owned(), val.into_owned());
//     }
// }
