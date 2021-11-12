use async_graphql::*;

#[derive(SimpleObject)]
pub struct User {
    id: i32,

    name: &'static str,

    /// Value b
    age: i32,

    #[graphql(skip)]
    family_count: i32,
}




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
pub struct UserQueryRoot;

#[Object]
impl UserQueryRoot {
    async fn user(&self, #[graphql(desc = "id of the droid")] id: i32) -> User {
        get_fake_user(id)
    }
    async fn users(&self) -> Vec<User> {
        get_all_users()
    }
}


fn get_fake_user(id: i32) -> User {
   let users = get_all_users();
   let k = users.into_iter().find(|u| u.id == id).unwrap();
   k
}

fn get_all_users() ->Vec<User> {
  return vec![
      User{
            id: 1,
            name: "Oyelowo",
            age: 114,
            family_count: 75,
      },
      User{
            id: 2,
            name: "Oyedayo",
            age: 87,
            family_count: 45,
      },
      User{
            id: 3,
            name: "Jupiter",
            age: 11,
            family_count: 34,
      },
      User{
            id: 4,
            name: "Uranus",
            age: 14,
            family_count: 4,
      },
      User{
            id: 5,
            name: "Mari",
            age: 7,
            family_count: 5,
      },
      User{
            id: 6,
            name: "Saul",
            age: 54,
            family_count: 405,
      },
      User{
            id: 7,
            name: "Olli",
            age: 93,
            family_count: 162,
      },
     
  ]
}