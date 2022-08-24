use std::{
    borrow::Cow,
    collections::{hash_map::RandomState, HashMap},
    hash::{BuildHasher, Hash},
    marker::PhantomData,
};

use super::utils::AuthUrlData;
use lru;
use redis::Commands;

//////
//  K: Eq + Hash
#[async_trait::async_trait]
pub trait CacheStorage: Send + Sync + Clone + 'static {
    // pub trait CacheStorage3: Send + Sync + Clone + 'static {
    //  type K: Eq + Hash;
    //  type Value: AuthUrlData;
    /// Load the query by `key`.
    async fn get(&self, key: String) -> Option<AuthUrlData>;
    /// Save the query by `key`.
    async fn save(&self, key: String, AuthUrlData: &AuthUrlData);
    async fn remove(&self, key: String);
}

use redis::{AsyncCommands, RedisError};

// type Haha = impl Future<Output = ()>;

#[derive(Clone)]
pub struct RedisCache(pub(crate) redis::Client);
// pub struct RedisCache<'a>(pub(crate) &'a redis::Client);

// struct RedisCache(redis::aio::Connection);

#[async_trait::async_trait]
impl CacheStorage for RedisCache {
    async fn get(&self, key: String) -> Option<AuthUrlData> {
        let AuthUrlData: String = self
            .0
            .get_async_connection()
            .await
            .unwrap()
            .get(key)
            .await
            .unwrap();

        Some(serde_json::from_str::<AuthUrlData>(AuthUrlData.as_str()).unwrap())
    }

    async fn save(&self, key: String, value: &AuthUrlData) {
        // let key = &Self::redis_key(self.csrf_token.clone());
        // let csrf_state_data_string = serde_json::to_string(&self)?;
        let con = self.0.get_async_connection().await.unwrap();
        // let k = AuthUrlData{ }
        // con.set::<oauth2::CsrfToken, AuthUrlData>(key, value).await?;
        con.set::<String, AuthUrlData>(key, value).await?;
        con.expire::<_, u16>(key, 600).await?;
        // Ok(self)
        // todo!()
    }
    async fn remove(&self, key: String) {
        self.0.del(&key);
    }

    /*
        async fn get(&self, key: &oauth2::CsrfToken) -> Option<&utils::AuthUrlData> {
        // todo!()
        // let key = &Self::redis_key(csrf_token);
        // connection./
        // let AuthUrlData: String = self.connection.get(key).await?;
        let AuthUrlData: String = self.0.get(key.secret().as_str()).await.unwrap();
        // let AuthUrlData: String = self.0.get(key).await?;

        Ok(serde_json::from_str::<Self>(AuthUrlData.as_str())?)
    }

    async fn save(&mut self, key: &oauth2::CsrfToken, value: utils::AuthUrlData) {
        // let key = &Self::redis_key(self.csrf_token.clone());
        // let csrf_state_data_string = serde_json::to_string(&self)?;

        // let k = AuthUrlData{ }
        self.0
            .set::<oauth2::CsrfToken, utils::AuthUrlData>(key, value)
            .await?;
        self.0.expire::<_, u16>(key, 600).await?;
        Ok(self)
    }

    fn remove(&mut self, key: &oauth2::CsrfToken) {
        todo!()
    }
    */
}

#[derive(Clone)]
struct HashMapCache(HashMap<String, AuthUrlData>);

#[async_trait::async_trait]
impl CacheStorage for HashMapCache {
    // type Key = String;
    // type Value = AuthUrlData;

    async fn get(&self, key: String) -> Option<AuthUrlData> {
        self.0.get(&key)
    }

    async fn save(&self, key: String, value: &AuthUrlData) {
        self.0.insert(key.into_owned(), value.into_owned());
    }

    //
    async fn remove(&self, key: String) {
        self.0.remove(&key);
    }

    //
    // fn clear(&mut self) {
    //     self.0.clear();
    // }
}

/// LRU cache.

#[derive(Clone)]
struct LruCache(lru::LruCache<String, AuthUrlData>);

#[async_trait::async_trait]
impl CacheStorage for LruCache {
    async fn get(&self, key: String) -> Option<AuthUrlData> {
        self.0.get(&key)
    }

    async fn save(&self, key: String, value: &AuthUrlData) {
        self.0.put(key, value);
        // self.0.put(key, value.into_owned());
    }

    async fn remove(&self, key: String) {
        self.0.pop(&key);
    }

    //
    // fn remove(&mut self, key: &Self::Key) {
    //     self.0.pop(key);
    // }

    //
    // fn clear(&mut self) {
    //     self.0.clear();
    // }
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
// //         let AuthUrlData: String = connection.get(key).await?;

// //         Ok(serde_json::from_str::<Self>(AuthUrlData.as_str())?)
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
// //     let p = LruCache::<String, String>(lru::LruCache::new(2000));
// //     const p : ::redis::aio::Connection =
// // }

// fn do_something(con: &mut redis::Connection) -> redis::RedisResult<()> {
//     let la  = con.get::<String, u32>("my_key".to_string())?;
//     let le  = con.set::<String, u32, u32>("my_key".to_string(), -42)?;
//     Ok(())
// }
