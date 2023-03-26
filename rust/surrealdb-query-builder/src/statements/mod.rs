use std::fmt::{Display, Formatter};

pub(crate) mod chain;
pub(crate) mod create;
pub(crate) mod define_database;
pub(crate) mod define_event;
pub(crate) mod define_field;
pub(crate) mod define_index;
pub(crate) mod define_login;
pub(crate) mod define_namespace;
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
pub(crate) mod select;
pub(crate) mod sleep;
pub(crate) mod transaction;
pub(crate) mod update;
pub(crate) mod use_;

pub mod statements {
    pub use super::chain::{chain, QueryChain};
    pub use super::create::{create, CreateStatement};
    pub use super::define_database::{define_database, DefineDatabaseStatement};
    pub use super::define_event::{define_event, DefineEventStatement};
    pub use super::define_field::{define_field, DefineFieldStatement, FieldType};
    pub use super::define_index::{define_index, DefineIndexStatement};
    pub use super::define_login::{define_login, DefineLoginStatement};
    pub use super::define_namespace::{define_namespace, DefineNamespaceStatement};
    pub use super::define_scope::{define_scope, DefineScopeStatement};
    pub use super::define_table::{define_table, DefineTableStatement};
    pub use super::define_token::{define_token, DefineTokenStatement};
    pub use super::delete::{delete, DeleteStatement};
    pub use super::for_::{for_, For, ForCrudType};
    pub use super::ifelse::{if_, IfStatement};
    pub use super::info::{info_for, InfoStatement};
    pub use super::insert::{insert, InsertStatement, Insertables};
    pub use super::let_::{let_, LetStatement};
    pub use super::relate::{relate, RelateStatement};
    pub use super::remove_database::{remove_database, RemoveDatabaseStatement};
    pub use super::remove_event::{remove_event, RemoveEventStatement};
    pub use super::remove_field::{remove_field, RemoveFieldStatement};
    pub use super::remove_index::{remove_index, RemoveIndexStatement};
    pub use super::remove_login::{remove_login, RemoveLoginStatement};
    pub use super::remove_namespace::{remove_namespace, RemoveNamespaceStatement};
    pub use super::remove_scope::{remove_scope, RemoveScopeStatement};
    pub use super::remove_table::{remove_table, RemoveTableStatement};
    pub use super::remove_token::{remove_token, RemoveTokenStatement};
    pub use super::select::{select, SelectStatement};
    pub use super::sleep::{sleep, SleepStatement};
    pub use super::transaction::{begin_transaction, BeginTransactionStatement};
    pub use super::update::{update, UpdateStatement};
    pub use super::use_::{use_, UseStatement};
}

pub(crate) enum NamespaceOrDatabase {
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
