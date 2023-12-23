use async_trait::async_trait;
use surrealdb::{engine::any::Any, Surreal};

use crate::config::RuntimeConfig;

#[async_trait]
pub trait Command {
    // fn runtime_config(&self) -> RuntimeConfig;

    async fn db(&self) -> Surreal<Any>;
}
