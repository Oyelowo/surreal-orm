use super::{User, UserInput};
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
    async fn create_user(
        &self,
        #[graphql(desc = "id of the droid")] user_input: UserInput,
    ) -> User {
        User {
            id: user_input.id,
            name: user_input.name,
            age: user_input.age,
            family_count: 5,
        }
    }
}
