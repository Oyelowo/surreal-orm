use std::fmt::Display;

mod chain;
mod chain;
mod create;
mod define_database;
mod define_event;
mod define_field;
mod define_index;
mod define_login;
mod define_namespace;
mod define_scope;
mod define_table;
mod define_token;
mod delete;
mod for_;
mod ifelse;
mod info;
mod insert;
mod let_;
mod relate;
mod remove_database;
mod remove_event;
mod remove_field;
mod remove_index;
mod remove_login;
mod remove_namespace;
mod remove_scope;
mod remove_table;
mod remove_token;
mod select;
mod sleep;
mod transaction;
mod update;
mod use_;

pub mod statements {
    pub use super::chain::{chain, QueryChain};
    pub use super::create::{create, CreateStatement};
    pub use super::define_database::{define_database, DefineDatabaseStatement};
    pub use super::define_event::{define_event, DefineEventStatement};
    pub use super::define_field::{define_field, DefineFieldStatement};
    pub use super::define_index::{define_index, DefineIndexStatement};
    pub use super::define_login::{define_login, DefineLoginStatement};
    pub use super::define_namespace::{define_namespace, DefineNamespaceStatement};
    pub use super::define_scope::{define_scope, DefineScopeStatement};
    pub use super::define_table::{define_table, DefineTableStatement};
    pub use super::define_token::{define_token, DefineTokenStatement};
    pub use super::delete::{delete, DeleteStatement};
    pub use super::for_::{for_, For};
    pub use super::ifelse::{ifelse, IfStatement};
    pub use super::info::{info, InfoStatement};
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
