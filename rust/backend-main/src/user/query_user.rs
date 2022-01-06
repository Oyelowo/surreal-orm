use async_graphql::*;

#[derive(SimpleObject, Clone)]
pub struct User {
    pub id: i32,

    pub name: String,

    /// Value b
    pub age: i32,

    #[graphql(skip)]
    pub family_count: i32,
}

// #[derive(SimpleObject)]
// #[graphql(complex)] // NOTE: If you want the `ComplexObject` macro to take effect, this `complex` attribute is required.
// pub struct MyObj {
//     a: i32,
//     b: i32,
// }

// #[ComplexObject]
// impl MyObj {
//     async fn c(&self) -> i32 {
//         self.a + self.b
//     }
// }
