#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
use serde::Deserialize;
use serde::Serialize;

use surreal_simple_querybuilder::prelude::*;

/* 

#[derive(SurrealdbModel, Debug, Serialize, Deserialize, Default)]
struct Account {    
// #[surreal(skip_serialize)] // or maybe this?
  #[surreal(primary_key)]
  id: Option<String>,

  handle: String,
  password: String,
  
  #[surreal(skip)]
  email: String,

  // #[surreal(skip_serialize)] // or maybe ths?
  #[surreal(skip_input)]
  something: Something,

  #[surreal(skip_input)]
  friend: Foreign<Project>,
  
  #[surreal(relate(->manage->Project))]
  managed_projects: ForeignVec<Project>,
} 

#[derive(SurrealdbModel ,Debug, Serialize, Deserialize, Default)]
struct Project {    
  #[surreal(primary_key)]
  id: Option<String>,
  name: String,

  #[surreal(relate(<-manage<-Project))]
  authors: ForeignVec<Account>,
} 


#[derive(SurrealdbModel ,Debug, Serialize, Deserialize, Default)]
struct Account {
  id: Option<String>,
  handle: String,
  password: String,
  email: String,

  projects: ForeignVec<Project>,
} 

#[derive(SurrealdbModel ,Debug, Serialize, Deserialize, Default)]
struct Project {
  id: Option<String>,
  name: String,

  releases: ForeignVec<Release>,
  pub ->has->Release as releases,
pub <-manage<-Account as authors
} 












mod account {
  use super::project::schema::Project;
  use surreal_simple_querybuilder::prelude::*;

  model!(Account {
    pub handle,
    pub password,
    pub email,
    friend<Account>,

    ->manage->Project as managed_projects,
  });
}

*/


#[derive(Debug, Serialize, Deserialize, Default)]
struct Account {
  id: Option<String>,
  handle: String,
  password: String,
  email: String,

  projects: ForeignVec<Project>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct Project {
  id: Option<String>,
  name: String,

  releases: ForeignVec<Release>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct Release {
  id: Option<String>,
  name: String,
}

mod release {
  use surreal_simple_querybuilder::prelude::*;
   use super::account::schema::Account;

   fn mm() {
let super::account::schema::Account{email, friend, handle,managed_projects,password, ..} = super::account::schema::model;
    //   super::release::):
    //   let x = Account::;
  }

  model!(Release {
    pub name
  });
}

mod project {
  use super::account::schema::Account;
  use super::release::schema::Release;
  use surreal_simple_querybuilder::prelude::*;

  model!(Project {
    pub name,

    pub ->has->Release as releases,
    pub <-manage<-Account as authors
  });
}

mod account {
  use super::project::schema::Project;
  use surreal_simple_querybuilder::prelude::*;

  model!(Account {
    pub handle,
    pub password,
    pub email,
    pub friend<Account>,

    pub ->manage->Project as managed_projects,
  });
}

fn er() {
    account.friend().email.g
}
use account::schema::model as account;
use project::schema::model as project;

#[derive(Debug, Serialize, Deserialize)]
struct File {
  name: String,
  author: Foreign<Account>,
}

impl IntoKey<String> for Project {
  fn into_key<E>(&self) -> Result<String, E>
  where
    E: serde::ser::Error,
  {
    self
      .id
      .as_ref()
      .map(String::clone)
      .ok_or(serde::ser::Error::custom("The project has no ID"))
  }
}

impl IntoKey<String> for Release {
  fn into_key<E>(&self) -> Result<String, E>
  where
    E: serde::ser::Error,
  {
    self
      .id
      .as_ref()
      .map(String::clone)
      .ok_or(serde::ser::Error::custom("The release has no ID"))
  }
}

impl IntoKey<String> for Account {
  fn into_key<E>(&self) -> Result<String, E>
  where
    E: serde::ser::Error,
  {
    self
      .id
      .as_ref()
      .map(String::clone)
      .ok_or(serde::ser::Error::custom("The account has no ID"))
  }
}


fn main()-> Result<(), SqlSerializeError> {
  let query = QueryBuilder::new()
    .create(account.handle.as_named_label(&account.to_string()))
    .set_model(&account)
    .unwrap()
    .build();

    let xx = Account {
        id: Some("x".to_string()),
        ..Default::default()
    };
    // xx.projects.value().map(|x|x[0].releases.value().)

  println!("query: {query}");

    let v = vec![
    Foreign::new_value(Account {
      id: Some("Account:John".to_owned()),
      ..Default::default()
    //   projects: ForeignVec::new()
    }),
    Foreign::new_key("Account:John".to_owned()),
  ];

  // once called, the Foreign should deserialize even the Values without calling
  // IntoKeys
//   v.allow_value_serialize();

  println!("query: {v:?}");


  Ok(())
}
