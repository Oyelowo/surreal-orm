/*
 *
REMOVE statement

Statement syntax
REMOVE [
    NAMESPACE @name
    | DATABASE @name
    | LOGIN @name ON [ NAMESPACE | DATABASE ]
    | TOKEN @name ON [ NAMESPACE | DATABASE ]
    | SCOPE @name
    | TABLE @name
    | EVENT @name ON [ TABLE ] @table
    | FIELD @name ON [ TABLE ] @table
    | INDEX @name ON [ TABLE ] @table
]
 * */

use std::fmt::Display;

use crate::{query_insert::Buildable, DbField, Queryable};

pub struct Namespace(String);
pub struct Database(String);
pub struct Login(String);
pub struct Token(String);
pub struct Scope(String);
pub struct Table(String);
pub struct Event(String);
pub struct Index(String);

macro_rules! impl_display_for_all {
    ($($types_:ty),*) => {
        $(impl Display for $types_ {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    )*
    };
}
impl_display_for_all!(Namespace, Database, Login, Token, Scope, Table, Event, Index);

enum NamespaceOrDatabase {
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

struct LoginDetails {
    name: Login,
    on: NamespaceOrDatabase,
}
struct TokenDetails {
    name: Token,
    on: NamespaceOrDatabase,
}

pub fn remove_namespace(namespace: impl Into<Namespace>) -> RemoveNamespaceStatement {
    RemoveNamespaceStatement::new(namespace)
}
pub struct RemoveNamespaceStatement {
    namespace: Namespace,
}

impl RemoveNamespaceStatement {
    fn new(namespace: impl Into<Namespace>) -> Self {
        let namespace = namespace.into();
        Self { namespace }
    }
}

impl Buildable for RemoveNamespaceStatement {
    fn build(&self) -> String {
        format!("REMOVE NAMESPACE {}", self.namespace)
    }
}
impl Runnable for RemoveNamespaceStatement {}

pub fn remove_database(database: impl Into<Database>) -> RemoveDatabaseStatement {
    RemoveDatabaseStatement::new(database)
}
pub struct RemoveDatabaseStatement {
    database: Database,
}

impl RemoveDatabaseStatement {
    fn new(database: impl Into<Database>) -> Self {
        Self {
            database: database.into(),
        }
    }
}

impl Buildable for RemoveDatabaseStatement {
    fn build(&self) -> String {
        format!("REMOVE DATABASE {}", self.database)
    }
}
impl Runnable for RemoveDatabaseStatement {}

pub fn remove_login(login: impl Into<Login>) -> RemoveLoginStatement {
    RemoveLoginStatement::new(login)
}
pub struct RemoveLoginStatement {
    login: Login,
    on: Option<NamespaceOrDatabase>,
}

impl RemoveLoginStatement {
    fn new(login: impl Into<Login>) -> Self {
        Self {
            login: login.into(),
            on: None,
        }
    }

    fn on_namespace(mut self) -> Self {
        self.on = Some(NamespaceOrDatabase::Namespace);
        self
    }

    fn on_database(mut self) -> Self {
        self.on = Some(NamespaceOrDatabase::Database);
        self
    }
}

impl Buildable for RemoveLoginStatement {
    fn build(&self) -> String {
        let mut query = format!("REMOVE LOGIN {}", self.login);

        if let Some(on) = &self.on {
            query = format!("{} ON {}", query, on);
        }
        query
    }
}
impl Runnable for RemoveLoginStatement {}

pub fn remove_token(token: impl Into<Token>) -> RemoveTokenStatement {
    RemoveTokenStatement::new(token)
}
pub struct RemoveTokenStatement {
    token: Token,
    on: Option<NamespaceOrDatabase>,
}

impl RemoveTokenStatement {
    fn new(token: impl Into<Token>) -> Self {
        Self {
            token: token.into(),
            on: None,
        }
    }

    fn on_namespace(mut self) -> Self {
        self.on = Some(NamespaceOrDatabase::Namespace);
        self
    }

    fn on_database(mut self) -> Self {
        self.on = Some(NamespaceOrDatabase::Database);
        self
    }
}

impl Buildable for RemoveTokenStatement {
    fn build(&self) -> String {
        let mut query = format!("REMOVE TOKEN {}", self.token);

        if let Some(on) = &self.on {
            query = format!("{} ON {}", query, on);
        }
        query
    }
}
impl Runnable for RemoveTokenStatement {}

pub fn remove_scope(scope: impl Into<Scope>) -> RemoveScopeStatement {
    RemoveScopeStatement::new(scope)
}
pub struct RemoveScopeStatement {
    scope: Scope,
}

impl RemoveScopeStatement {
    fn new(scope: impl Into<Scope>) -> Self {
        Self {
            scope: scope.into(),
        }
    }
}

impl Queryable for RemoveScopeStatement {}

impl Buildable for RemoveScopeStatement {
    fn build(&self) -> String {
        format!("REMOVE SCOPE {}", self.scope)
    }
}
impl Runnable for RemoveScopeStatement {}

pub fn remove_table(table: impl Into<Table>) -> RemoveTableStatement {
    RemoveTableStatement::new(table)
}
pub struct RemoveTableStatement {
    table: Table,
}

impl RemoveTableStatement {
    fn new(table: impl Into<Table>) -> Self {
        Self {
            table: table.into(),
        }
    }
}

impl Buildable for RemoveTableStatement {
    fn build(&self) -> String {
        format!("REMOVE TABLE {}", self.table)
    }
}
impl Runnable for RemoveTableStatement {}

pub fn remove_event(event: impl Into<Event>) -> RemoveEventStatement {
    RemoveEventStatement::new(event)
}
pub struct RemoveEventStatement {
    event: Event,
    table: Option<Table>,
}

impl RemoveEventStatement {
    fn new(event: impl Into<Event>) -> Self {
        Self {
            table: None,
            event: event.into(),
        }
    }

    fn on_table(mut self, table: impl Into<Table>) -> Self {
        self.table = Some(table.into());
        self
    }
}

impl Buildable for RemoveEventStatement {
    fn build(&self) -> String {
        let query = format!("REMOVE EVENT {}", self.event);
        if let Some(table) = &self.table {
            let query = format!("{} ON TABLE {}", query, table);
        }
        query
    }
}
impl Runnable for RemoveEventStatement {}

pub fn remove_field(field: impl Into<DbField>) -> RemoveFieldStatement {
    RemoveFieldStatement::new(field)
}
pub struct RemoveFieldStatement {
    field: DbField,
    table: Option<Table>,
}

impl RemoveFieldStatement {
    fn new(field: impl Into<DbField>) -> Self {
        Self {
            field: field.into(),
            table: None,
        }
    }

    fn on_table(mut self, table: impl Into<Table>) -> Self {
        self.table = Some(table.into());
        self
    }
}

impl Buildable for RemoveFieldStatement {
    fn build(&self) -> String {
        let query = format!("REMOVE FIELD {}", self.field);
        if let Some(table) = &self.table {
            let query = format!("{} ON TABLE {}", query, table);
        }
        query
    }
}
impl Runnable for RemoveFieldStatement {}

pub fn remove_index(index: impl Into<Index>) -> RemoveIndexStatement {
    RemoveIndexStatement::new(index)
}
pub struct RemoveIndexStatement {
    index: Index,
    table: Option<Table>,
}

impl RemoveIndexStatement {
    fn new(index: impl Into<Index>) -> Self {
        Self {
            index: index.into(),
            table: None,
        }
    }

    fn on_table(mut self, table: impl Into<Table>) -> Self {
        self.table = Some(table.into());
        self
    }
}

impl Buildable for RemoveIndexStatement {
    fn build(&self) -> String {
        let query = format!("REMOVE INDEX {}", self.index);
        if let Some(table) = &self.table {
            let query = format!("{} ON TABLE {}", query, table);
        }
        query
    }
}
impl Runnable for RemoveIndexStatement {}

#[async_trait::async_trait]
pub trait Runnable
where
    Self: Buildable,
{
    async fn run(
        &self,
        db: surrealdb::Surreal<surrealdb::engine::local::Db>,
    ) -> surrealdb::Result<()> {
        let query = self.build();
        db.query(query).await?;
        Ok(())
    }
}
#[test]
fn test() {}
