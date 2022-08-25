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
