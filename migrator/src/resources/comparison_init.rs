/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use serde::{Deserialize, Serialize};
use surreal_query_builder::DbResources;

use crate::*;

macro_rules! define_top_level_resource {
    ($($resource_name:ident, $resource_type:ident);*) => {
        paste::paste! {
            $(
                #[derive(Debug)]
                pub struct [<Comparison $resource_type>]<'a> {
                    pub resources: &'a ComparisonsInit<'a>
                }

                impl<'a> DbResourcesMeta<[<$resource_type>]> for [<Comparison $resource_type>] <'a>{
                    fn get_left(&self) -> [<$resource_type>] {
                        self.resources.left_resources.[<$resource_name>]()
                    }

                    fn get_right(&self) -> [<$resource_type>] {
                        self.resources.right_resources.[<$resource_name>]()
                    }
                }
            )*
        }
    };
}

define_top_level_resource!(
    // tables, Tables;
    analyzers, Analyzers;
    functions, Functions;
    params, Params;
    scopes, Scopes;
    tokens, Tokens;
    users, Users
);

#[derive(Debug, Clone)]
pub struct ComparisonsInit<'a> {
    // Migrations directoy latest state tables
    pub left_resources: &'a LeftFullDbInfo,
    // Codebase latest state tables
    pub right_resources: &'a RightFullDbInfo,
    pub prompter: &'a dyn Prompter,
}

impl<'a> ComparisonsInit<'a> {
    pub fn new_tables<R: DbResources>(&self, codebase_resources: &'a R) -> ComparisonTables<R> {
        // log::info!("comparing resources {:#?}", self.left_resources);

        ComparisonTables {
            resources: self,
            codebase_resources,
            prompter: self.prompter,
        }
    }

    pub fn new_analyzers(&self) -> ComparisonAnalyzers {
        ComparisonAnalyzers { resources: self }
    }

    pub fn new_params(&self) -> ComparisonParams {
        ComparisonParams { resources: self }
    }

    pub fn new_scopes(&self) -> ComparisonScopes {
        ComparisonScopes { resources: self }
    }

    pub fn new_tokens(&self) -> ComparisonTokens {
        ComparisonTokens { resources: self }
    }

    pub fn new_users(&self) -> ComparisonUsers {
        ComparisonUsers { resources: self }
    }

    pub fn new_functions(&self) -> ComparisonFunctions {
        ComparisonFunctions { resources: self }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct DbInfo {
    pub analyzers: Analyzers,
    pub functions: Functions,
    pub params: Params,
    pub scopes: Scopes,
    pub tables: Tables,
    pub tokens: Tokens,
    pub users: Users,
}

impl DbInfo {
    pub fn analyzers(&self) -> Analyzers {
        self.analyzers.clone()
    }

    pub fn functions(&self) -> Functions {
        self.functions.clone()
    }

    pub fn params(&self) -> Params {
        self.params.clone()
    }

    pub fn scopes(&self) -> Scopes {
        self.scopes.clone()
    }

    pub fn tables(&self) -> Tables {
        self.tables.clone()
    }

    pub fn tokens(&self) -> Tokens {
        self.tokens.clone()
    }

    pub fn users(&self) -> Users {
        self.users.clone()
    }
}
