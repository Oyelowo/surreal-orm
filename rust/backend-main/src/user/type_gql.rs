use async_graphql::*;

#[derive(InputObject)]
pub struct UserInput {
    pub id: i32,
    pub name: String,
    pub age: i32,
}
