use surreal_simple_querybuilder::prelude::*;

struct Account {
    id: Option<String>,
    handle: String,
    password: String,
    email: String,
    friends: Foreign<Vec<Account>>,
}

model!(Account {
  id,
  handle,
  password,
  friends<Account>
});

fn main() {
    // the schema module is created by the macro
    use schema::model as account;

    let query = format!("select {} from {account}", account.handle);
    assert_eq!("select handle from Account", query);
}
