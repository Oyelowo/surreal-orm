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
    account: ForeignVec<Project>,
    // projects: ForeignVec<Project>,
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
mod xama {
    use super::Account;
    pub type Kusa = Account;
}
impl Account_Manage_Project {
    fn sama() -> String {
        type Nama = Account;
        assert_type_eq_all!(Account, Nama);
        assert_type_eq_all!(xama::Kusa, Nama);
        assert_type_eq_all!(xama::Kusa, Account);
        // assert_type_eq_all!(xama::Kusa, String);
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
