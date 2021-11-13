
use super::model::UserData;
use super::query::User;

use async_graphql::*;

#[derive(Default)]
pub struct UserQueryRoot;

#[Object]
impl UserQueryRoot {
    async fn user(&self, #[graphql(desc = "id of the droid")] id: i32) -> User {
        let user = UserData::new().get_user(id);
        user
    }
    
    
    async fn users(&self) -> Vec<User> {
        let users = UserData::new();
        let users = users.get_users();
        users
    }
}
