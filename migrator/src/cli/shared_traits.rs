use async_trait::async_trait;
use surrealdb::{engine::any::Any, Surreal};

#[async_trait]
pub trait DbConnection {
    async fn create_and_set_connection(&mut self);
    async fn db(&self) -> Surreal<Any>;
}
