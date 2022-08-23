use std::{
    borrow::Cow,
    collections::{hash_map::RandomState, HashMap},
    hash::{BuildHasher, Hash},
    marker::PhantomData,
};

use lru;
use redis::Commands;

/// Factory for creating cache storage.
pub trait CacheFactory: Send + Sync + 'static {
    /// Create a cache storage.
    ///
    /// TODO: When GAT is stable, this memory allocation can be optimized away.
    fn create<K, V>(&self) -> Box<dyn CacheStorage<Key = K, Value = V>>
    where
        K: Send + Sync + Clone + Eq + Hash + 'static,
        V: Send + Sync + Clone + 'static;
}

/// Cache storage for [DataLoader].
pub trait CacheStorage: Send + Sync + 'static {
    /// The key type of the record.
    type Key: Send + Sync + Clone + Eq + Hash + 'static;

    /// The value type of the record.
    type Value: Send + Sync + Clone + 'static;

    /// Returns a reference to the value of the key in the cache or None if it
    /// is not present in the cache.
    fn get(&mut self, key: &Self::Key) -> Option<&Self::Value>;

    /// Puts a key-value pair into the cache. If the key already exists in the
    /// cache, then it updates the key's value.
    fn insert(&mut self, key: Cow<'_, Self::Key>, val: Cow<'_, Self::Value>);

    /// Removes the value corresponding to the key from the cache.
    fn remove(&mut self, key: &Self::Key);

    /// Clears the cache, removing all key-value pairs.
    fn clear(&mut self);
}

#[async_trait::async_trait]
pub trait CacheStorage2: Send + Sync + Clone + 'static {
    /// Load the query by `key`.
    async fn get(&self, key: String) -> Option<String>;
    /// Save the query by `key`.
    async fn set(&self, key: String, query: String);
}

//////
//  K: Eq + Hash
#[async_trait::async_trait]
pub trait CacheStorage3: Send + Sync + Clone + 'static {
    // pub trait CacheStorage3: Send + Sync + Clone + 'static {
    //  type K: Eq + Hash;
    //  type Value: super::utils::Evidence;
    /// Load the query by `key`.
    async fn get(&self, key: String) -> Option<super::utils::Evidence>;
    /// Save the query by `key`.
    async fn save(&self, key: String, evidence: super::utils::Evidence);
}

use redis::{AsyncCommands, RedisError};
use std::future::Future;

// type Haha = impl Future<Output = ()>;

#[derive(Clone)]
pub struct RedisCache(pub(crate) redis::Client);
// pub struct RedisCache<'a>(pub(crate) &'a redis::Client);

impl RedisCache {
    async fn get_connection(self) -> redis::aio::Connection {
        self.0.get_async_connection().await.unwrap()
    }
}
// struct RedisCache(redis::aio::Connection);

// let mut v2: Vec<Box<dyn Fn() -> Box<dyn Future<Output = ()>>>> = vec![];

#[async_trait::async_trait]
impl CacheStorage3 for RedisCache {
    // fn get<'life0, 'async_trait>(
    //     &'life0 self,
    //     key: String,
    // ) ->
    // {
    //     todo!()
    // }
    async fn get(&self, key: String) -> Option<super::utils::Evidence> {
        // todo!()
        // let key = &Self::redis_key(csrf_token);
        // connection./
        // let evidence: String = self.connection.get(key).await?;
        // let evidence: String = self.0.get(key).await.unwrap();
        // let evidence: String = self
        //     .0
        //     .get_async_connection()
        //     .await
        //     .unwrap()
        //     .get(key)
        //     .await
        //     .unwrap();
        let evidence: String = self.get_connection().await.get(key).await.unwrap();
        // let evidence: String = self.0.get(key).await?;

        Some(serde_json::from_str::<super::utils::Evidence>(evidence.as_str()).unwrap())
        // Ok(serde_json::from_str::<Self>(evidence.as_str())?)
    }

    async fn save(&self, key: String, value: super::utils::Evidence) {
        // let key = &Self::redis_key(self.csrf_token.clone());
        // let csrf_state_data_string = serde_json::to_string(&self)?;

        // let k = Evidence{ }
        self.get_connection()
            .await
            .set::<oauth2::CsrfToken, super::utils::Evidence>(key, value)
            .await?;
        self.get_connection()
            .await
            .expire::<_, u16>(key, 600)
            .await?;
        // Ok(self)
        // todo!()
    }

    /*
        async fn get(&self, key: &oauth2::CsrfToken) -> Option<&utils::Evidence> {
        // todo!()
        // let key = &Self::redis_key(csrf_token);
        // connection./
        // let evidence: String = self.connection.get(key).await?;
        let evidence: String = self.0.get(key.secret().as_str()).await.unwrap();
        // let evidence: String = self.0.get(key).await?;

        Ok(serde_json::from_str::<Self>(evidence.as_str())?)
    }

    async fn save(&mut self, key: &oauth2::CsrfToken, value: utils::Evidence) {
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
    */
}

///////

/// [std::collections::HashMap] cache.
pub struct HashMapCache<S = RandomState> {
    _mark: PhantomData<S>,
}

impl<S: Send + Sync + BuildHasher + Default + 'static> HashMapCache<S> {
    /// Use specified `S: BuildHasher` to create a `HashMap` cache.
    pub fn new() -> Self {
        Self { _mark: PhantomData }
    }
}

impl Default for HashMapCache<RandomState> {
    fn default() -> Self {
        Self { _mark: PhantomData }
    }
}

impl<S: Send + Sync + BuildHasher + Default + 'static> CacheFactory for HashMapCache<S> {
    fn create<K, V>(&self) -> Box<dyn CacheStorage<Key = K, Value = V>>
    where
        K: Send + Sync + Clone + Eq + Hash + 'static,
        V: Send + Sync + Clone + 'static,
    {
        Box::new(HashMapCacheImpl::<K, V, S>(HashMap::<K, V, S>::default()))
    }
}

struct HashMapCacheImpl<K, V, S>(HashMap<K, V, S>);

impl<K, V, S> CacheStorage for HashMapCacheImpl<K, V, S>
where
    K: Send + Sync + Clone + Eq + Hash + 'static,
    V: Send + Sync + Clone + 'static,
    S: Send + Sync + BuildHasher + 'static,
{
    type Key = K;
    type Value = V;

    #[inline]
    fn get(&mut self, key: &Self::Key) -> Option<&Self::Value> {
        self.0.get(key)
    }

    #[inline]
    fn insert(&mut self, key: Cow<'_, Self::Key>, val: Cow<'_, Self::Value>) {
        self.0.insert(key.into_owned(), val.into_owned());
    }

    #[inline]
    fn remove(&mut self, key: &Self::Key) {
        self.0.remove(key);
    }

    #[inline]
    fn clear(&mut self) {
        self.0.clear();
    }
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

impl CacheFactory for LruCache {
    fn create<K, V>(&self) -> Box<dyn CacheStorage<Key = K, Value = V>>
    where
        K: Send + Sync + Clone + Eq + Hash + 'static,
        V: Send + Sync + Clone + 'static,
    {
        Box::new(LruCacheImpl(lru::LruCache::new(self.cap)))
    }
}

struct LruCacheImpl<K, V>(lru::LruCache<K, V>);

impl<K, V> CacheStorage for LruCacheImpl<K, V>
where
    K: Send + Sync + Clone + Eq + Hash + 'static,
    V: Send + Sync + Clone + 'static,
{
    type Key = K;
    type Value = V;

    #[inline]
    fn get(&mut self, key: &Self::Key) -> Option<&Self::Value> {
        self.0.get(key)
    }

    #[inline]
    fn insert(&mut self, key: Cow<'_, Self::Key>, val: Cow<'_, Self::Value>) {
        self.0.put(key.into_owned(), val.into_owned());
    }

    #[inline]
    fn remove(&mut self, key: &Self::Key) {
        self.0.pop(key);
    }

    #[inline]
    fn clear(&mut self) {
        self.0.clear();
    }
}

// struct RedisCache(::redis::aio::Connection);

// impl RedisCache {
//     type K = dyn Send + Sync + Clone + Eq + Hash + 'static;
//     type V= dyn Send + Sync + Clone + 'static;
//     fn get(self)->  Self::V{
//         self.get()
//     }
// }

// struct RedisCacheImpl(::redis::aio::Connection);

// impl CacheStorage for RedisCacheImpl {
//     // const CSRF_STATE_REDIS_KEY: &'static str = "CSRF_STATE_REDIS_KEY";

//     // fn redis_key(csrf_token: CsrfToken) -> String {
//     //     format!(
//     //         "{}{:?}",
//     //         Self::CSRF_STATE_REDIS_KEY,
//     //         csrf_token.secret().as_str()
//     //     )
//     // }

// //     pub(crate) async fn verify_csrf_token(
// //         csrf_token: CsrfToken,
// //         connection: &mut redis::aio::Connection,
// //     ) -> OauthResult<Self> {
// //         let key = &Self::redis_key(csrf_token);
// // // connection./
// //         let evidence: String = connection.get(key).await?;

// //         Ok(serde_json::from_str::<Self>(evidence.as_str())?)
// //     }

// //     pub(crate) async fn cache(self, connection: &mut redis::aio::Connection) -> OauthResult<Self> {
// //         let key = &Self::redis_key(self.csrf_token.clone());
// //         let csrf_state_data_string = serde_json::to_string(&self)?;

// //         connection.set(key, csrf_state_data_string).await?;
// //         connection.expire::<_, u16>(key, 600).await?;
// //         Ok(self)
// //     }
// }

// // fn kk() {
// //     let p = LruCacheImpl::<String, String>(lru::LruCache::new(2000));
// //     const p : ::redis::aio::Connection =
// // }

// fn do_something(con: &mut redis::Connection) -> redis::RedisResult<()> {
//     let la  = con.get::<String, u32>("my_key".to_string())?;
//     let le  = con.set::<String, u32, u32>("my_key".to_string(), -42)?;
//     Ok(())
// }
