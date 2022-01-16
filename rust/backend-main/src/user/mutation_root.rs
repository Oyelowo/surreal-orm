use super::{User, UserInput};
use async_graphql::*;
use mongodb::Database;
use wither::Model;


// #[derive(InputObject)]
// // #[model(index(keys=r#"doc!{"email": 1}"#, options=r#"doc!{"unique": true}"#))]
// pub struct UserInput {
//     pub first_name: String,
//     pub last_name: String,
//     pub email: String,
//     pub age: u8,
// }

#[derive(Default)]
pub struct UserMutationRoot;

#[Object]
impl UserMutationRoot {
    async fn create_user(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "user data")] user_input: UserInput,
    ) -> User {
        let db = ctx.data_unchecked::<Database>();
        let mut user = User::builder()
            .first_name(user_input.first_name.into())
            .last_name(user_input.last_name.into())
            .email(user_input.email.into())
            .age(user_input.age)
            .social_media(user_input.social_media)
            .build();
        // let mut user = User {
        //     id: None,
        //     ..user_input
        // };

        user.save(db, None).await.expect("problem storing user");
        user
    }
}
