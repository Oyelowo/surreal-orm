/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */

use crate::*;
use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

pub trait DbResourcesMeta<T>
where
    T: Informational,
{
    // Left is from migration dir
    fn get_left(&self) -> T;
    // Right is from codebase
    fn get_right(&self) -> T;

    fn queries(&self) -> MigrationResult<Queries> {
        let mut queries = Queries::default();
        let left = self.get_left().get_names_as_set();
        let right = self.get_right().get_names_as_set();
        let resources = right.union(&left);

        for name in resources {
            let def_right = self.get_right().get_definition(name).cloned();
            let def_left = self.get_left().get_definition(name).cloned();

            match DeltaTypeResource::from((def_left, def_right)) {
                DeltaTypeResource::Create { right } => {
                    queries.add_up(QueryType::Define(right.clone()));
                    queries.add_down(QueryType::Remove(right.as_remove_statement()?));
                }
                DeltaTypeResource::Remove { left } => {
                    queries.add_up(QueryType::Remove(left.as_remove_statement()?));
                    queries.add_down(QueryType::Define(left));
                }
                DeltaTypeResource::Update { left, right } => {
                    queries.add_up(QueryType::Define(right));
                    queries.add_down(QueryType::Define(left));
                }
                DeltaTypeResource::NoChange => {}
            };
        }
        Ok(queries)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
struct Info(BTreeMap<String, DefineStatementRaw>);

use std::collections::BTreeSet;

pub trait Informational {
    // skills[*] is a valid field name in this context
    fn get_names(&self) -> Vec<String>;

    fn get_names_as_set(&self) -> BTreeSet<String>;

    fn get_all_definitions(&self) -> Vec<DefineStatementRaw>;

    // Althought, I dont think u should do this, it is absolutely possible:
    // "skills[*]": "DEFINE FIELD skills[*] ON person TYPE string"
    // Above can be achieved just doing array<string> on the top level field - skills
    // "skills": "DEFINE FIELD skills ON person TYPE option<array>",
    // fn get_definition(&self, name: &str) -> Option<&DefineStatementRaw>;
    fn get_definition<T: AsRef<str>>(&self, name: T) -> Option<&DefineStatementRaw>;
}

impl Informational for Info {
    // skills[*] is a valid field name in this context
    fn get_names(&self) -> Vec<String> {
        self.0.keys().cloned().collect()
    }

    fn get_names_as_set(&self) -> BTreeSet<String> {
        BTreeSet::from_iter(self.get_names())
    }

    fn get_all_definitions(&self) -> Vec<DefineStatementRaw> {
        self.0.values().cloned().collect()
    }

    // Althought, I dont think u should do this, it is absolutely possible:
    // "skills[*]": "DEFINE FIELD skills[*] ON person TYPE string"
    // Above can be achieved just doing array<string> on the top level field - skills
    // "skills": "DEFINE FIELD skills ON person TYPE option<array>",
    // fn get_definition(&self, name: &str) -> Option<&DefineStatementRaw> {
    fn get_definition<T: AsRef<str>>(&self, name: T) -> Option<&DefineStatementRaw> {
        self.0.get(name.as_ref())
    }
}

macro_rules! define_object_info {
    ($($ident: ident),*) => {
        $(
            #[derive(Serialize, Deserialize, Clone, Debug, Default)]
            pub struct $ident(Info);

            // impl Deref for $ident {
            //     type Target = Info;
            //
            //     fn deref(&self) -> &Self::Target {
            //         &self.0
            //     }
            // }


            impl Informational for $ident {
                // skills[*] is a valid field name in this context
                fn get_names(&self) -> Vec<String> {
                    self.0.0.keys().cloned().collect()
                }

                fn get_names_as_set(&self) -> BTreeSet<String> {
                    BTreeSet::from_iter(self.get_names())
                }

                fn get_all_definitions(&self) -> Vec<DefineStatementRaw> {
                    self.0.0.values().cloned().collect()
                }

                // Althought, I dont think u should do this, it is absolutely possible:
                // "skills[*]": "DEFINE FIELD skills[*] ON person TYPE string"
                // Above can be achieved just doing array<string> on the top level field - skills
                // "skills": "DEFINE FIELD skills ON person TYPE option<array>",
                fn get_definition<T: AsRef<str>>(&self, name: T) -> Option<&DefineStatementRaw> {
                    // let name: ::std::string::String = name.to_owned().into();
                    self.0.0.get(name.as_ref())
                }
            }

        )*
    };
}

define_object_info!(
    Analyzers, Functions, Params, Scopes, Tables, Tokens, Users, Fields, Events, Indexes
);
