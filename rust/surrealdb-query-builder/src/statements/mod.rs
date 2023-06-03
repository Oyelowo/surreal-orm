use std::fmt::Display;

pub(crate) mod create;
pub(crate) mod create_v2;
pub(crate) mod define_database;
pub(crate) mod define_event;
pub(crate) mod define_field;
pub(crate) mod define_function;
pub(crate) mod define_index;
pub(crate) mod define_login;
pub(crate) mod define_namespace;
pub(crate) mod define_param;
pub(crate) mod define_scope;
pub(crate) mod define_table;
pub(crate) mod define_token;
pub(crate) mod delete;
pub(crate) mod for_;
pub(crate) mod ifelse;
pub(crate) mod info;
pub(crate) mod insert;
pub(crate) mod let_;
pub(crate) mod relate;
pub(crate) mod remove_database;
pub(crate) mod remove_event;
pub(crate) mod remove_field;
pub(crate) mod remove_index;
pub(crate) mod remove_login;
pub(crate) mod remove_namespace;
pub(crate) mod remove_scope;
pub(crate) mod remove_table;
pub(crate) mod remove_token;
pub(crate) mod return_;
pub(crate) mod select;
pub(crate) mod select_surreal_id_range;
pub(crate) mod sleep;
pub(crate) mod subquery;
pub(crate) mod transaction;
pub(crate) mod update;
pub(crate) mod use_;
pub(crate) mod utils_block;
pub(crate) mod utils_chain;

pub use create::{create, CreateStatement};
pub use create_v2::{create_v2, CreateStatementV2};
pub use define_database::{define_database, DefineDatabaseStatement};
pub use define_event::{define_event, DefineEventStatement};
pub use define_field::{define_field, DefineFieldStatement};
pub use define_function::{define_function, DefineFunctionStatement};
pub use define_index::{define_index, DefineIndexStatement};
pub use define_login::{define_login, DefineLoginStatement};
pub use define_namespace::{define_namespace, DefineNamespaceStatement};
pub use define_param::{define_param, DefineParamStatement};
pub use define_scope::{define_scope, DefineScopeStatement};
pub use define_table::{define_table, DefineTableStatement};
pub use define_token::{define_token, DefineTokenStatement};
pub use delete::{delete, DeleteStatement, DeleteStatementMini};
pub use for_::{for_, For, PermissionType};
pub use ifelse::{if_, IfElseStatement};
pub use info::{info_for, InfoStatement};
pub use insert::{insert, InsertStatement, Insertables};
pub use let_::{let_, LetStatement};
pub use relate::{relate, RelateStatement};
pub use remove_database::{remove_database, RemoveDatabaseStatement};
pub use remove_event::{remove_event, RemoveEventStatement};
pub use remove_field::{remove_field, RemoveFieldStatement};
pub use remove_index::{remove_index, RemoveIndexStatement};
pub use remove_login::{remove_login, RemoveLoginStatement};
pub use remove_namespace::{remove_namespace, RemoveNamespaceStatement};
pub use remove_scope::{remove_scope, RemoveScopeStatement};
pub use remove_table::{remove_table, RemoveTableStatement};
pub use remove_token::{remove_token, RemoveTokenStatement};
pub use return_::{return_, ReturnStatement};
pub use select::{
    order, select, select_value, Order, Orderables, SelectStatement, SelectStatementMini,
    Selectables, Splittables,
};
pub use sleep::{sleep, SleepStatement};
pub use subquery::Subquery;
pub use transaction::{begin_transaction, transaction, BeginTransactionStatement};
pub use update::{update, UpdateStatement};
pub use use_::{use_, UseStatement};

/// helpers for statements
pub mod utils {
    pub use super::utils_block::{block, Block};
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
