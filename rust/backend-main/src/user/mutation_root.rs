// use super::{User, UserInput};
use super::User;
use async_graphql::*;
use mongodb::Database;
use wither::Model;

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
    async fn create_user(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "id of the droid")] user_input: User,
    ) -> User {
        let db = ctx.data_unchecked::<Database>();
        // let mut user = User::builder()
        //     .first_name(user_input.first_name.into())
        //     .last_name(user_input.last_name.into())
        //     .email(user_input.email.into())
        //     .age(user_input.age)
        //     .build();
        let mut user = User { ..user_input };

        user.save(db, None).await.expect("problem storing user");
        user
    }
}
