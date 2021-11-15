use std::rc::Rc;
use std::cell::RefCell;
use async_graphql::*;
use super::query::User;
use super::model::UserData;
use super::inputs::UserInput;

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
    async fn create_user(&self, #[graphql(desc = "new user")]user: UserInput) -> User {
        User {
            id: user.id,
            name: user.name,
            age: user.age,
            family_count: 5
        }
    }
 
}
