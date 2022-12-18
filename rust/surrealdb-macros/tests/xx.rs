use surreal_simple_querybuilder::prelude::*;
// use surreal_simple_querybuilder::model;
use surrealdb_derive::SurrealdbModel;
// use surrealdb_macros::FieldsGetter;
use surrealdb_macros::SurrealdbModel;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
struct Account {
  id: Option<String>,
  handle: String,
  password: String,
  email: String,


// #[surrealdb(record_link(one_to_one ="Account"))]
#[xxxx(one_to_many ="Project", skip_serialize=true)]
#[xxxx(one_to_many ="Project")]
#[xxxx(one_to_one ="->manage->Project")]
#[xxxx(relate ="->manage->Project" )]
#[xxxx(relate(connection="->manage->Project" ))]
  managed_projects: ForeignVec<Project>,
}

mod account {
  use super::project::schema::Project;
  use surreal_simple_querybuilder::prelude::*;

  model!(Account {
    pub handle,
    pub password,
    pub email,
    projects<Project>,
    friend<Account>,

    ->manage->Project as managed_projects,
  });
}


#[derive(Debug, Serialize, Deserialize, Default)]
struct Project {
  id: Option<String>,
  name: String,

  releases: ForeignVec<Release>,
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

#[derive(Debug, Serialize, Deserialize, Default)]
struct Release {
  id: Option<String>,
  name: String,
}

mod release {
  use surreal_simple_querybuilder::prelude::*;

  model!(Release {
    pub name
  });
}



use account::schema::model as account;
use project::schema::model as project;

#[derive(Debug, Serialize, Deserialize)]
struct File {
  name: String,
  author: Foreign<Account>,
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