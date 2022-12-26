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
    #[surrealdb(relate = "->runs->Project")]
    projects: ForeignVec<Project>,
}

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
    const edge_relation: &'static str = "manage";
    fn to(&self) -> ::proc_macro2::TokenStream {
        // Account::get_schema().chess;
        let In = self.r#in.own_schema().to_string();
        let Out = self.out.own_schema().to_string();
        let In = format_ident!("{In}");
        let Out = format_ident!("{Out}");
        let edge = format_ident!("{}", Self::edge_relation);
        let xx = ::quote::quote!(#In->#edge->#Out);
        xx
    }
    fn from(&self) -> ::proc_macro2::TokenStream {
        let In = self.r#in.own_schema().to_string();
        let Out = self.out.own_schema().to_string();
        let In = format_ident!("{In}");
        let Out = format_ident!("{Out}");
        let edge = format_ident!("{}", Self::edge_relation);
        let xx = ::quote::quote!(#Out<-#edge<-#In);
        xx
    }
}
use surreal_simple_querybuilder::prelude::*;
use surrealdb_macros::Edge;

fn main() {
    let xxx = Account_Manage_Project::default();
    // let x = xxx.to();
    println!("to: {}", xxx.to());
    println!("from: {}", xxx.from());
    // Account::schema.fav_proj()
    // Account::schema.projects().title
    // Account::schema.projects()
    Account::get_schema()
        .projects()
        .title
        .contains_none("values");
    // Account::schema.fav_proj().title.contains_any("values");
    // Account::get_fields_serialized()
    // Account::get_schema().email;
    // Account::get_schema().lastName
    // Account::schema.firstName
    // Account::get_schema().firstName.contains_one("value");
    // Account::get_schema()
    // Account::schema.nicePoa
    // Account::get_schema().firstName
    // Account::get_schema().email.contains_all(values)
    // account::schema::model
}
