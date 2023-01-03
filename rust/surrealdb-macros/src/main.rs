#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
use quote::format_ident;
use serde::{Deserialize, Serialize};
// #![feature(inherent_associated_types)]
// #![feature(const_mut_refs)]
// use serde::{Deserialize, Serialize};
// use surreal_simple_querybuilder::prelude::*;
use static_assertions::*;

// const_assert!("oylelowo".as_str().len() > 3);
assert_fields!(Account_Manage_Project: r#in, out);

use surrealdb_derive::SurrealdbModel;
#[derive(SurrealdbModel, Default, Serialize, Deserialize, Debug)]
#[surrealdb(rename_all = "camelCase")]
pub struct Account {
    id: Option<String>,
    handle: String,
    // #[surrealdb(rename = "nawao")]
    first_name: String,
    #[surrealdb(rename = "lastName")]
    another_name: String,
    chess: String,
    nice_poa: String,
    password: String,
    email: String,

    // #[surrealdb(relate(edge="Account_Manage_Project", description="->manage->Account"))]
    #[surrealdb(relate(edge = "Account_Manage_project", link = "->runs->Project"))]
    projects: ForeignVec<Project>,
}

// impl Account {
//     fn own_schema(&self) -> Self {
//         self
//     }
// }

#[derive(SurrealdbModel, Default, Serialize, Deserialize, Debug)]
#[surrealdb(rename_all = "camelCase")]
pub struct Project {
    id: Option<String>,
    title: String,
    #[surrealdb(relate = "->run_by->Account")]
    account: ForeignVec<Account>,
}

#[allow(non_camel_case_types)]
// #[derive(SurrealdbModel, Debug, Serialize, Deserialize, Default)]
#[derive(Debug, Serialize, Deserialize, Default)]
// #[surrealdb(edge_relation = "manage")]
struct Account_Manage_Project {
    id: String,
    r#in: Account,
    out: Project,
    when: String,
    destination: String,
}

impl Edga for Account_Manage_Project {
    type In = Account;
    type Out = Project;
}

::static_assertions::assert_type_eq_all!(Account, <Account_Manage_Project as Edga>::In);

trait Edga {
    type In;
    type Out;
}

mod xama {
    use super::Account;
    use super::Project;

    mod edges_types {
        type In = super::Account;
        type Out = super::Project;
    }

    mod nodes_checker {
        pub struct In {
            Account: super::Account,
        }
        pub struct Out {
            Project: super::Project,
        }
    }
    ::static_assertions::assert_type_eq_all!(Project, self::Project);
    // use std::any::Any;

    // use crate::Account_Manage_Project;
    use static_assertions::{const_assert, const_assert_eq};

    const_assert_eq!(false, false);

    struct In {
        Account: super::Account,
    }
    struct Out {
        Project: super::Project,
    }
    ::static_assertions::assert_fields!(In: Account);
    const Nama: &'static str = "tr";
    const xx: &'static str = "tr";

    // const_assert!(xx == Nama);

    const FIVE: usize = 5;

    const_assert!(FIVE * 2 == 10);
    // const kp: String = Account_Manage_Project::sama();
    pub type Kusa = Account;
}
impl Account_Manage_Project {
    fn sama() -> String {
        type Nama = Account;
        assert_type_eq_all!(Account, Nama);
        assert_type_eq_all!(xama::Kusa, Nama);
        assert_type_eq_all!(xama::Kusa, Account);
        // assert_type_eq_all!(xama::Kusa, String);
        assert_fields!(Account_Manage_Project: r#in, out);
        "lowo".to_string()
    }
}

fn xc() {
    let xxx = Account_Manage_Project::default();
    let x = xxx.from();

    println!("{x}");
}

// trait Edge {
//     const edge_relation: &'static str;
//     fn to(&self) -> ::proc_macro2::TokenStream;
//     fn from(&self) -> ::proc_macro2::TokenStream;
// }

// if to().split(->).first() == (struct_name) and ending ===
// description == ending(i.e remaining part of the string)
impl Edge for Account_Manage_Project {
    #[allow(non_upper_case_globals)]
    const EDGE_RELATION: &'static str = "manage";
    fn to(&self) -> ::proc_macro2::TokenStream {
        // Account::;
        // self.out
        let In = self.r#in.own_schema().to_string();
        let Out = self.out.own_schema().to_string();
        let In = format_ident!("{In}");
        let Out = format_ident!("{Out}");
        let edge = format_ident!("{}", Self::EDGE_RELATION);
        let xx = ::quote::quote!(#In->#edge->#Out);
        xx
    }
    fn from(&self) -> ::proc_macro2::TokenStream {
        let In = self.r#in.own_schema().to_string();
        let Out = self.out.own_schema().to_string();
        let In = format_ident!("{In}");
        let Out = format_ident!("{Out}");
        let edge = format_ident!("{}", Self::EDGE_RELATION);
        let xx = ::quote::quote!(#Out<-#edge<-#In);
        xx
    }
    fn km(&self) -> String {
        "dfoyelowo".to_string()
    }
}
use surreal_simple_querybuilder::prelude::*;
use surrealdb_macros::Edge;

fn main() {
    let xxx = Account_Manage_Project::default();
    println!("to: {}", xxx.to());
    println!("from: {}", xxx.from());
    Account::get_schema()
        .projects()
        .title
        .contains_none("values");
}
