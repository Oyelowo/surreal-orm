
use super::model::UserData;
//use super::query::User;
// use super::User;
use super::query_user::User;

use async_graphql::*;

#[derive(Default)]
pub struct UserQueryRoot;

#[Object]
impl UserQueryRoot {
    async fn user(&self, #[graphql(desc = "id of the droid")] id: i32) -> User {
        
        UserData::new().get_user(id)
    }
    
    
    async fn users(&self) -> Vec<User> {
        let users = UserData::new();
        
        users.get_users()
    }
}
