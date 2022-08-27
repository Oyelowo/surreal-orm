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
    async fn set(&self, key: String, query: String);
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

    async fn set(&self, key: String, value: String) {
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

    async fn set(&self, key: String, value: String) {
        self.set(key, value).await;
    }
}

// /// LRU cache.
// pub struct LruCache {
//     cap: usize,
// }

// struct LruCacheImpl<K, V>(lru::LruCache<K, V>);

// impl<K, V> CacheStorage for LruCacheImpl<K, V>
// where
//     K: Send + Sync + Clone + Eq + Hash + 'static,
//     V: Send + Sync + Clone + 'static,
// {
//     type Key = K;
//     type Value = V;

//     #[inline]
//     fn get(&mut self, key: &Self::Key) -> Option<&Self::Value> {
//         self.0.get(key)
//     }

//     #[inline]
//     fn insert(&mut self, key: Cow<'_, Self::Key>, val: Cow<'_, Self::Value>) {
//         self.0.put(key.into_owned(), val.into_owned());
//     }

//     #[inline]
//     fn remove(&mut self, key: &Self::Key) {
//         self.0.pop(key);
//     }

//     #[inline]
//     fn clear(&mut self) {
//         self.0.clear();
//     }
// }
