#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(incomplete_features)]
#![feature(inherent_associated_types)]
#![feature(generic_const_exprs)]
use serde::{Deserialize, Serialize};
// #![feature(inherent_associated_types)]
// #![feature(const_mut_refs)]
// use serde::{Deserialize, Serialize};
// use surreal_simple_querybuilder::prelude::*;
use static_assertions::*;

// const_assert!("oylelowo".as_str().len() > 3);
// assert_fields!(Account_Manage_Project: r#in, out);

use surrealdb_macros::SurrealdbModel;

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
    #[surrealdb(relate(edge = "AccountManageproject", link = "->runs->Project"))]
    managed_projects: ForeignVec<Project>,
}

fn kl() {
    let _po = Account::get_schema()
        .managedProjects()
        .account()
        .managedProjects()
        .account()
        .managedProjects()
        .account()
        .lastName;
}

#[derive(SurrealdbModel, Default, Serialize, Deserialize, Debug)]
#[surrealdb(rename_all = "camelCase")]
pub struct Project {
    id: Option<String>,
    title: String,
    #[surrealdb(relate = "->run_by->Account")]
    account: ForeignVec<Account>,
}

// #[derive(Debug, Serialize, Deserialize, Default)]
#[derive(SurrealdbModel, Debug, Serialize, Deserialize, Default)]
#[surrealdb(relation_name = "manage")]
struct AccountManageProject {
    id: Option<String>,
    // #[serde(rename = "in")]
    r#in: Account,
    out: Project,
    when: String,
    destination: String,
}

fn ki() {
    let xx = AccountManageProject::get_schema().r#in;
    let xm = AccountManageProject::get_schema().out;
    struct Nomax {
        in_: String,
    }
    // let pp = Nomax {
    //     in_: String::from("normal"),
    // };
}

impl Edga for AccountManageProject {
    type In = Account;
    type Out = Project;
}
type Kol = <AccountManageProject as Edga>::In;
::static_assertions::assert_type_eq_all!(Account, <AccountManageProject as Edga>::In);

trait Edga {
    type In;
    type Out;
}

// #[surrealdb(rename_all = "camelCase")]
pub struct Accountt {
    id: Option<String>,
    handle: String,
    // #[surrealdb(rename = "nawao")]
    first_name: String,
    // #[surrealdb(rename = "lastName")]
    another_name: String,
    chess: String,
    nice_poa: String,

    password: String,
    email: String,

    // #[surrealdb(relate(edge="Account_Manage_Project", description="->manage->Account"))]
    // #[surrealdb(relate(edge = "Account_Manage_project", link = "->runs->Project"))]
    projects: ForeignVec<Project>,
}
// impl Accountt {
//     // type Schema = account::schema::Account<0>;
//     // type Schema = #schema_mod_name::schema::#my_struct<0>;
//     const SCHEMA: accountt::schema::Accountt<0> = accountt::schema::Accountt::<0>::new();
//     const fn get_schema() -> accountt::schema::Accountt<0> {
//         // project::schema::model
//         //  account::schema::Account<0>::new()
//         // e.g: account::schema::Account::<0>::new()
//         accountt::schema::Accountt::<0>::new()
//     }
//     // fn own_schema(&self) -> #schema_type_alias_name<0> {
//     //     // project::schema::model
//     //     //  account::schema::Account<0>::new()
//     //     // e.g: account::schema::Account::<0>::new()
//     //     #schema_mod_name::schema::#my_struct::<0>::new()
//     // }
// }
impl Edga2 for Accountt {
    // type Schema = projectt::schema::Projectt<0>;
    // type Schema = projectt::schema::Projectt<0>;
    type Schema<const T: usize> = accountt::schema::Accountt<T>;

    fn get_schema() -> Self::Schema<0> {
        accountt::schema::Accountt::<0>::new()
    }
}
fn cre() {
    let _xx = Accountt::get_schema()
        .managed_projects()
        .manager()
        .managed_projects()
        .manager()
        .managed_projects()
        .manager()
        .managed_projects()
        .manager()
        .managed_projects()
        .email
        .count();

    let _pp = Projectt::get_schema()
        .manager()
        .managed_projects()
        .manager()
        .managed_projects()
        .manager()
        .managed_projects()
        .manager()
        .managed_projects()
        .manager()
        .first_name;
    let _po = Accountt::get_schema()
        .friend()
        .managed_projects()
        .manager()
        .friend()
        .managed_projects()
        .manager()
        .managed_projects;
}
mod accountt {
    // mod xxx
    // use super::projectt::schema::Projectt;
    type Projectt<const T: usize> = <super::Projectt as super::Edga2>::Schema<T>;
    // type Projectt = Mowa;
    // use super::Mowa as Projectt;
    // use super::Projectt;
    // type Mowa = <Projectt as super::Edga2>::Schema;
    use surreal_simple_querybuilder::prelude::*;

    model!( Accountt {
           pub id,
        //    pub _in<Account>,
           pub first_name,
           pub email,
       pub friend<Accountt>,
           pub ->manage->Projectt as managed_projects,
       }
    );
}

mod projectt {
    // use super::accountt::schema::Accountt;
    type Accountt<const T: usize> = <super::Accountt as super::Edga2>::Schema<T>;
    use surreal_simple_querybuilder::prelude::*;

    model!( Projectt {
           pub id,
           pub first_name,
           pub email,
           pub <-run_by<-Accountt as manager
       }
    );
}

#[derive(Default, Serialize, Deserialize, Debug)]
// #[surrealdb(rename_all = "camelCase")]
pub struct Projectt {
    id: Option<String>,
    title: String,
    // #[surrealdb(relate = "->run_by->Account")]
    account: ForeignVec<Account>,
}

pub trait Edga2 {
    type Schema<const T: usize>;
    fn get_schema() -> Self::Schema<0>;
}
impl Edga2 for Projectt {
    // type Schema = projectt::schema::Projectt<0>;
    // type Schema = projectt::schema::Projectt<0>;
    type Schema<const T: usize> = projectt::schema::Projectt<T>;
    fn get_schema() -> Self::Schema<0> {
        // project::schema::model
        //  account::schema::Account<0>::new()
        // e.g: account::schema::Account::<0>::new()
        projectt::schema::Projectt::<0>::new()
    }
}
// type Mowa<const T: usize> = Projectt::Schema<T>;
// type Mowa<const T: usize> = <Projectt as Edga2>::Schema<T>;

// impl Projectt {
//     type Schema<const T: usize> = projectt::schema::Projectt<T>;
//     // type Schema = projectt::schema::Projectt<0>;
//     // type Schema = #schema_mod_name::schema::#my_struct<0>;
//     const SCHEMA: projectt::schema::Projectt<0> = projectt::schema::Projectt::<0>::new();
//     const fn get_schema() -> projectt::schema::Projectt<0> {
//         // project::schema::model
//
//         //  account::schema::Account<0>::new()
//         // e.g: account::schema::Account::<0>::new()
//         projectt::schema::Projectt::<0>::new()
//     }
//     // fn own_schema(&self) -> #schema_type_alias_name<0> {
//     //     // project::schema::model
//     //     //  account::schema::Account<0>::new()
//     //     // e.g: account::schema::Account::<0>::new()
//     //     #schema_mod_name::schema::#my_struct::<0>::new()
//     // }
// }
fn protext() {
    let _xxx = Projectt::get_schema()
        .manager()
        .managed_projects()
        .manager();
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
impl AccountManageProject {
    fn sama() -> String {
        type Nama = Account;
        assert_type_eq_all!(Account, Nama);
        assert_type_eq_all!(xama::Kusa, Nama);
        assert_type_eq_all!(xama::Kusa, Account);
        // assert_type_eq_all!(xama::Kusa, String);
        // assert_fields!(Account_Manage_Project: r#in, out);
        "lowo".to_string()
    }
}

fn xc() {
    let _xxx = AccountManageProject::default();
    // let x = xxx.from();
    // println!("{x}");
}

// trait Edge {
//     const edge_relation: &'static str;
//     fn to(&self) -> ::proc_macro2::TokenStream;
//     fn from(&self) -> ::proc_macro2::TokenStream;
// }

// if to().split(->).first() == (struct_name) and ending ===
// description == ending(i.e remaining part of the string)
impl Edge for AccountManageProject {
    #[allow(non_upper_case_globals)]
    const EDGE_RELATION: &'static str = "manage";
    fn to(&self) -> ::proc_macro2::TokenStream {
        // Account::;
        // self.out
        // let In = self.r#in.own_schema().to_string();
        // let Out = self.out.own_schema().to_string();
        // let In = format_ident!("{In}");
        // let Out = format_ident!("{Out}");
        // let edge = format_ident!("{}", Self::EDGE_RELATION);
        // let xx = ::quote::quote!(#In->#edge->#Out);
        // xx
        todo!()
    }
    fn from(&self) -> ::proc_macro2::TokenStream {
        // let In = self.r#in.own_schema().to_string();
        // let Out = self.out.own_schema().to_string();
        // let In = format_ident!("{In}");
        // let Out = format_ident!("{Out}");
        // let edge = format_ident!("{}", Self::EDGE_RELATION);
        // let xx = ::quote::quote!(#Out<-#edge<-#In);
        // xx
        todo!()
    }
    fn km(&self) -> String {
        "dfoyelowo".to_string()
    }
}
use surreal_simple_querybuilder::prelude::*;
use surrealdb_macros::Edge;

fn main() {
    let _xxx = AccountManageProject::default();
    // println!("to: {}", xxx.to());
    // println!("from: {}", xxx.from());
    Account::get_schema()
        .managedProjects()
        .title
        .contains_none("values");
}
