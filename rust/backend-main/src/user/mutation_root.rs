// use super::{User, UserInput};
use super::User;
use async_graphql::*;

#[derive(SimpleObject)]
#[graphql(complex)] // NOTE: If you want the `ComplexObject` macro to take effect, this `complex` attribute is required.
pub struct MyObj {
    a: i32,
    b: i32,
}

#[ComplexObject]
impl MyObj {
    async fn c(&self) -> i32 {
        self.a + self.b
    }
}

#[derive(Default)]
pub struct UserMutationRoot;

#[Object]
impl UserMutationRoot {
    async fn create_user(&self, #[graphql(desc = "id of the droid")] user_input: User) -> User {
        User::builder()
            .first_name("Oyelowo".into())
            .last_name("Oyedayo".into())
            .email("nothingexits@fang.com".into())
            .age(120)
            .build()
    }
}
