#![allow(dead_code)]
#![allow(non_snake_case)]

use surreal_simple_querybuilder::model;
// use surreal_simple_querybuilder::model;
use surrealdb_derive::SurrealdbModel;
// use surrealdb_macros::FieldsGetter;
use surrealdb_macros::SurrealdbModel;
use serde::{Deserialize, Serialize};

use surreal_simple_querybuilder::prelude::*;

mod account {
    // use surrealdb_macros::SurrealdbModel;
    use surrealdb_derive::SurrealdbModel;
    use serde::{Deserialize, Serialize};
    use surreal_simple_querybuilder::prelude::*;

    #[derive(SurrealdbModel , Debug, Serialize, Deserialize, Default)]
    pub struct Account {
        id: Option<String>,
        handle: String,
        password: String,
        email: String,
        
        // projects: ForeignVec<Project>,
    }
}


mod project {
    use surrealdb_derive::SurrealdbModel;
    use serde::{Deserialize, Serialize};
    use surreal_simple_querybuilder::prelude::*;

    #[derive(SurrealdbModel ,Debug, Serialize, Deserialize, Default)]
    pub struct Project {
    id: Option<String>,
    name: String,

      releases: ForeignVec<super::release::Release>,
    }
}


mod release {
    use surrealdb_derive::SurrealdbModel;
    use serde::{Deserialize, Serialize};
    use surreal_simple_querybuilder::prelude::*;

    #[derive(SurrealdbModel ,Debug, Serialize, Deserialize, Default)]
    pub struct Release {
    id: Option<String>,
    name: String,
    }
}


mod file {
    use surrealdb_derive::SurrealdbModel;
    use serde::{Deserialize, Serialize};
    use surreal_simple_querybuilder::prelude::*;
    use super::account::Account;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct File {
    name: String,
    author: Foreign<Account>,
    }
}
fn do_it() {
// account::schema::model.email.
// project::schema::model.releases.name().
}

/* 
mod account {
  struct Account {
      pub handle,
      pub password,
      pub email,

    //    #[serde(rename(serialize = "age"))]
     #[surrealdb(reference(one_to_one ="Account"))]
     #[surrealdb(record_link(one_to_one ="Account"))]
     friend: Foreign<Account>,
     //   friend<Account>,
    
      #[surrealdb->manage->Project]
      managed_projects,
  }
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

 
// mod xx {
//         use surreal_simple_querybuilder::prelude::*;

//         model!(Accountt {
//             pub email
//         });

// fn xx()  {
//     // use account::schema::model as account;
//     use schema::model as account;

//         let x = schema::model.email.contains_one(value);
// }
//     }


// mod yy {
//         use surreal_simple_querybuilder::prelude::*;
//         use super::{SurrealdbModel, Serialize, Deserialize};

//     #[derive(SurrealdbModel, Serialize, Deserialize)]
//     #[serde(rename_all = "camelCase")]
//     pub struct Consumer {
//         pub name_of_me: String,

//         pub age: u8,
//     }

// fn xx()  {
//     // use account::schema::model as account;
//     use schema::model as account;

//     // schema::model.nameOfMe.co
//         // let x = schema::model.email.contains_one(value);
// }
//     }



// #[test]
// fn ddefault_to_how_fields_are_written_if_no_rename_all_struct_attribute_specifieda() {


   
//     schema::model.nameOfMe;
//     // schema::model.
//     // schema::model.
//     // schema::model.
// // schema::model.
//     // model!(Axx {
//     //     lowo
//     // });
//     // schema::Axx::
//     // schema::model.
//     // schema::
// // schema::model
//     // schema::
// // model::
//     let ConsumerFields { nameOfMe, age } = Consumer::get_fields_serialized();

//     assert_eq!(nameOfMe, "nameOfMe");
//     assert_eq!(age, "age");
// }
