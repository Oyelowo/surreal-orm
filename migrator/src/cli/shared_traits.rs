use async_trait::async_trait;
use surrealdb::{engine::any::Any, Surreal};

#[async_trait]
pub trait Command {
    async fn db(&self) -> Surreal<Any>;
}
