use std::{
    borrow::Cow,
    collections::{hash_map::RandomState, HashMap},
    hash::{BuildHasher, Hash},
    marker::PhantomData,
};

use lru;
use redis::{AsyncCommands, RedisError};

use super::utils;
use oauth2;

/// Factory for creating cache storage.
// pub trait CacheFactory: Send + Sync + 'static {
//     /// Create a cache storage.
//     ///
//     /// TODO: When GAT is stable, this memory allocation can be optimized away.
//     fn create<K, V>(&self) -> Box<dyn CacheStorage<Key = K, Value = V>>
//     where
//         K: Send + Sync + Clone + Eq + Hash + 'static,
//         V: Send + Sync + Clone + 'static;
// }

/// Cache storage for [DataLoader].\
/// 
#[async_trait::async_trait]
pub trait CacheStorage: Send + Sync + 'static {
    // type Key;
    // type Value;
    // type Key = oauth2::CsrfToken;
    // type Value = utils::Evidence;

    /// Returns a reference to the value of the key in the cache or None if it
    /// is not present in the cache.
    async fn get(&self, key: &oauth2::CsrfToken) -> Option<&utils::Evidence>;

    /// Puts a key-value pair into the cache. If the key already exists in the
    /// cache, then it updates the key's value.
    fn insert(&self, key: &oauth2::CsrfToken, val: utils::Evidence);
    // fn insert(& self, key: Cow<'_, Self::Key>, val: Cow<'_, Self::Value>);

    /// Removes the value corresponding to the key from the cache.
    fn remove(&self, key: &oauth2::CsrfToken);

    // Clears the cache, removing all key-value pairs.
    // fn clear(&mut self);
}

struct HashMapCacheImpl(HashMap<oauth2::CsrfToken, utils::Evidence>);

impl CacheStorage for HashMapCacheImpl {
    // type Key = oauth2::CsrfToken;
    // type Value = V;

    #[inline]
    fn get(&self, key: &oauth2::CsrfToken) -> Option<&utils::Evidence> {
        self.0.get(key)
    }

    #[inline]
    fn insert(&self, key: oauth2::CsrfToken, val: utils::Evidence) {
        self.0.insert(key.into_owned(), val.into_owned());
    }

    #[inline]
    fn remove(&self, key: &oauth2::CsrfToken) {
        self.0.remove(key);
    }

    // #[inline]
    // fn clear(&mut self) {
    //     self.0.clear();
    // }
}

fn lox() {
    let op = HashMapCacheImpl(HashMap::new());
    // op.
}

// struct redisCache()

struct RedisCacheImpl(redis::aio::Connection);

#[async_trait::async_trait]
impl CacheStorage for RedisCacheImpl {
    async fn get(&self, key: &oauth2::CsrfToken) -> Option<&utils::Evidence> {
        // todo!()
        // let key = &Self::redis_key(csrf_token);
        // connection./
        // let evidence: String = self.connection.get(key).await?;
        let evidence: String = self.0.get(key.secret().as_str()).await.unwrap();
        // let evidence: String = self.0.get(key).await?;

        Ok(serde_json::from_str::<Self>(evidence.as_str())?)
    }

    async fn insert(&mut self, key: &oauth2::CsrfToken, value: utils::Evidence) {
        // let key = &Self::redis_key(self.csrf_token.clone());
        // let csrf_state_data_string = serde_json::to_string(&self)?;

        // let k = Evidence{ }
        self.0
            .set::<oauth2::CsrfToken, utils::Evidence>(key, value)
            .await?;
        self.0.expire::<_, u16>(key, 600).await?;
        Ok(self)
    }

    fn remove(&mut self, key: &oauth2::CsrfToken) {
        todo!()
    }
}

fn get_redis() -> ::redis::aio::Connection {
    todo!()
}
fn get_cs() -> oauth2::CsrfToken {
    todo!()
}
fn get_ev() -> utils::Evidence {
    todo!()
}

fn po() {
    let p = get_redis();
    let k = RedisCacheImpl(&mut p);
    let m = k.insert(&get_cs(), get_ev());
}

/// LRU cache.
pub struct LruCache {
    cap: usize,
}

impl LruCache {
    /// Creates a new LRU Cache that holds at most `cap` items.
    pub fn new(cap: usize) -> Self {
        Self { cap }
    }
}

// impl CacheFactory for LruCache {
//     fn create<C: CacheStorage>(&self) -> C {
//         LruCacheImpl(lru::LruCache::new(self.cap))
//     }
// }

fn ko() {
    let p = LruCacheImpl(::lru::LruCache::new(10000)).insert(&get_cs(), get_ev());
    // let p = LruCache::new(10000).create().insert(&get_cs(), get_ev());

    let x = LruCache::new(10000).create().get(&get_cs()).unwrap();
    // let p = LruCache::new(10000).create();
}
struct LruCacheImpl<K, V>(lru::LruCache<K, V>);

impl CacheStorage for LruCacheImpl<oauth2::CsrfToken, utils::Evidence> {
    fn get(&mut self, key: &oauth2::CsrfToken) -> Option<&utils::Evidence> {
        self.0.get(key)
    }

    fn insert(&mut self, key: &oauth2::CsrfToken, val: utils::Evidence) {
        self.0.put(key.into_owned(), val.into_owned())
    }

    fn remove(&mut self, key: &oauth2::CsrfToken) {
        self.0.pop(key)
    }
}
