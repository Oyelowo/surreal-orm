/*utils_chain
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::fmt::Display;

pub(crate) mod break_;
pub(crate) mod continue_;
pub(crate) mod create;
pub(crate) mod create_v2;
pub(crate) mod define_analyzer;
pub(crate) mod define_database;
pub(crate) mod define_event;
pub(crate) mod define_field;
pub(crate) mod define_function;
pub(crate) mod define_index;
pub(crate) mod define_model;
pub(crate) mod define_namespace;
pub(crate) mod define_param;
pub(crate) mod define_scope;
pub(crate) mod define_table;
pub(crate) mod define_token;
pub(crate) mod define_user;
pub(crate) mod delete;
pub(crate) mod for_loop;
pub(crate) mod for_permission;
pub(crate) mod ifelse;
pub(crate) mod info;
pub(crate) mod insert;
pub(crate) mod let_;
pub(crate) mod live_select;
pub(crate) mod relate;
pub(crate) mod remove_analyzer;
pub(crate) mod remove_database;
pub(crate) mod remove_event;
pub(crate) mod remove_field;
pub(crate) mod remove_function;
pub(crate) mod remove_index;
pub(crate) mod remove_login;
pub(crate) mod remove_model;
pub(crate) mod remove_namespace;
pub(crate) mod remove_param;
pub(crate) mod remove_scope;
pub(crate) mod remove_table;
pub(crate) mod remove_token;
pub(crate) mod remove_user;
pub(crate) mod return_;
pub(crate) mod select;
pub(crate) mod select_surreal_id_range;
pub(crate) mod show;
pub(crate) mod sleep;
pub(crate) mod subquery;
pub(crate) mod throw;
pub(crate) mod transaction;
pub(crate) mod update;
pub(crate) mod use_;
pub(crate) mod utils_block;
pub(crate) mod utils_chain;

pub use break_::{break_, BreakStatement};
pub use continue_::{continue_, ContinueStatement};
pub use create::{create, create_only, CreateStatement};
pub use create_v2::{create_v2, CreateStatementV2};
pub use define_analyzer::{
    define_analyzer, AnalyzerFilter, DefineAnalyzerStatement, SnowballLanguage, Tokenizer,
};
pub use define_database::{define_database, DefineDatabaseStatement};
pub use define_event::{define_event, DefineEventStatement};
pub use define_field::{define_field, DefineFieldStatement};
pub use define_function::{define_function, DefineFunctionStatement, FunctionArgument};
pub use define_index::{define_index, search_analyzer, DefineIndexStatement, Scoring};
pub use define_model::{define_model, DefineModelStatement, ModelName, ModelVersion};
pub use define_namespace::{define_namespace, DefineNamespaceStatement};
pub use define_param::{define_param, DefineParamStatement};
pub use define_scope::{define_scope, DefineScopeStatement};
pub use define_table::{define_table, DefineTableStatement};
pub use define_token::{define_token, DefineTokenStatement};
pub use define_user::{define_user, DefineUserStatement, UserRole};
pub use delete::{delete, delete_only, DeleteStatement, DeleteStatementMini};
pub use for_loop::{for_, ForLoopStatement};
pub use for_permission::{for_permission, ForPermission, Permissions};
pub use ifelse::{if_, IfElseStatement};
pub use info::{info_for, InfoStatement};
pub use insert::{insert, InsertStatement, Insertables};
pub use let_::{let_, LetStatement};
pub use live_select::{live, LiveSelectStatement};
pub use relate::{relate, relate_only, RelateStatement};
pub use remove_analyzer::{remove_analyzer, RemoveAnalyzerStatement};
pub use remove_database::{remove_database, RemoveDatabaseStatement};
pub use remove_event::{remove_event, RemoveEventStatement};
pub use remove_field::{remove_field, RemoveFieldStatement};
pub use remove_function::{remove_function, RemoveFunctionStatement};
pub use remove_index::{remove_index, RemoveIndexStatement};
pub use remove_login::{remove_login, RemoveLoginStatement};
pub use remove_model::{remove_model, RemoveModelStatement};
pub use remove_namespace::{remove_namespace, RemoveNamespaceStatement};
pub use remove_param::{remove_param, RemoveParamStatement};
pub use remove_scope::{remove_scope, RemoveScopeStatement};
pub use remove_table::{remove_table, RemoveTableStatement};
pub use remove_token::{remove_token, RemoveTokenStatement};
pub use remove_user::{remove_user, RemoveUserStatement, UserPermissionScope};
pub use return_::{return_, ReturnStatement};
pub use select::{
    order, select, select_diff, select_value, Order, Orderables, SelectStatement,
    SelectStatementMini, Selectables, Splittables,
};
pub use show::{show_changes_for_table, ShowChangesStatement};
pub use sleep::{sleep, SleepStatement};
pub use subquery::Subquery;
pub use throw::{throw, ThrowStatement};
pub use transaction::{
    begin_transaction, transaction_deprecated, BeginTransactionStatement, TransactionCompletion,
};
pub use update::{update, update_only, UpdateStatement};
pub use use_::{use_, UseStatement};

/// helpers for statements
pub mod utils {
    pub use super::utils_block::{block, block_deprecated, Block};
    pub use super::utils_chain::{chain, Chainable, QueryChain};
}

#[allow(missing_docs)]
pub enum NamespaceOrDatabase {
    Namespace,
    Database,
}

impl Display for NamespaceOrDatabase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let stringified = match self {
            NamespaceOrDatabase::Namespace => "NAMESPACE",
            NamespaceOrDatabase::Database => "DATABASE",
        };
        write!(f, "{}", stringified)
    }
}
