use async_graphql::{Context, ErrorExtensions, Guard, Result};
use bson::oid::ObjectId;
use common::{authentication::TypedSession, error_handling::ApiHttpStatus};

use super::Role;

pub struct RoleGuard {
    role: Role,
}

#[async_trait::async_trait]
impl Guard for RoleGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
        if ctx.data_opt::<Role>() == Some(&self.role) {
            Ok(())
        } else {
            Err(ApiHttpStatus::Unauthorized(
                "You are not authourized to carry out that request.".into(),
            )
            .extend())
        }
    }
}

impl RoleGuard {
    pub fn new(role: Role) -> Self {
        Self { role }
    }
}

pub struct AuthGuard;

#[async_trait::async_trait]
impl Guard for AuthGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
        TypedSession::from_ctx(ctx)?
            .get_user_id::<ObjectId>()
            .map(|_| ())
    }
}
