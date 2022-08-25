#[async_trait::async_trait]
pub trait CacheStorage: Send + Sync + Clone + 'static {
    /// Load the query by `key`.
    async fn get(&self, key: String) -> Option<String>;
    /// Save the query by `key`.
    async fn set(&self, key: String, query: String);
}

use redis::AsyncCommands;

use super::utils::AuthUrlData;

#[derive(Clone)]
pub struct RedisCache(pub(crate) redis::Client);
// pub struct RedisCache<'a>(pub(crate) &'a redis::Client);

// struct RedisCache(redis::aio::Connection);

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

        // Some(serde_json::from_str::<AuthUrlData>(AuthUrlData.as_str()).unwrap())
    }

    async fn set(&self, key: String, value: String) {
        // let key = &Self::redis_key(self.csrf_token.clone());
        // let csrf_state_data_string = serde_json::to_string(&self)?;
        let mut con = self.0.get_async_connection().await.unwrap();
        // let k = AuthUrlData{ }
        // con.set::<oauth2::CsrfToken, AuthUrlData>(key, value).await?;
        con.set::<String, String, String>(key.clone(), value);
        // ::redis::cmd("SET").arg(key.clone()).arg(value);
        con.expire::<_, u16>(key, 600).await.unwrap();
        // Ok(self)
        // todo!()
    }
    // async fn remove(&self, key: String) {
    //     self.0.del(&key);
    // }
}
