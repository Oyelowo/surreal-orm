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

use crate::{query_insert::Buildable, DbField};

struct Namespace(String);
struct Database(String);
struct Login(String);
struct Token(String);
struct Scope(String);
struct Table(String);
struct Event(String);
struct Index(String);

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
struct RemoveNamespaceStatement {
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

pub fn remove_database(database: impl Into<Database>) -> RemoveDatabaseStatement {
    RemoveDatabaseStatement::new(database)
}
struct RemoveDatabaseStatement {
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

pub fn remove_login(login: impl Into<Login>) -> RemoveLoginStatement {
    RemoveLoginStatement::new(login)
}
struct RemoveLoginStatement {
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

    fn on(mut self, on: impl Into<NamespaceOrDatabase>) -> Self {
        self.on = Some(on.into());
        self
    }
}

impl Buildable for RemoveLoginStatement {
    fn build(&self) -> String {
        let query = format!("REMOVE LOGIN {}", self.login);

        if let Some(on) = self.on {
            query = format!("{} ON {}", query, on);
        }
        query
    }
}

pub fn remove_token(token: impl Into<Token>) -> RemoveTokenStatement {
    RemoveTokenStatement::new(token)
}
struct RemoveTokenStatement {
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

    fn on(mut self, on: impl Into<NamespaceOrDatabase>) -> Self {
        self.on = Some(on.into());
        self
    }
}

impl Buildable for RemoveTokenStatement {
    fn build(&self) -> String {
        let query = format!("REMOVE TOKEN {}", self.token);

        if let Some(on) = self.on {
            query = format!("{} ON {}", query, on);
        }
        query
    }
}

pub fn remove_scope(scope: impl Into<Scope>) -> RemoveScopeStatement {
    RemoveScopeStatement::new(scope)
}
struct RemoveScopeStatement {
    scope: Scope,
}

impl RemoveScopeStatement {
    fn new(scope: impl Into<Scope>) -> Self {
        Self {
            scope: scope.into(),
        }
    }
}

impl Buildable for RemoveScopeStatement {
    fn build(&self) -> String {
        format!("REMOVE SCOPE {}", self.scope)
    }
}

pub fn remove_table(table: impl Into<Table>) -> RemoveTableStatement {
    RemoveTableStatement::new(table)
}
struct RemoveTableStatement {
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

pub fn remove_event(event: impl Into<Event>) -> RemoveEventStatement {
    RemoveEventStatement::new(event)
}
struct RemoveEventStatement {
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
        if let Some(table) = self.table {
            let query = format!("{} ON TABLE {}", query, table);
        }
        query
    }
}

struct RemoveStatement {
    namespace: Option<Namespace>,
    database: Option<Database>,
    login: Option<LoginDetails>,
    token: Option<TokenDetails>,
    scope: Option<Scope>,
    table: Option<Table>,
    event: Option<(Event, Table)>,
    field: Option<(DbField, Table)>,
    index: Option<(Index, Table)>,
}
// remove().namespace("kawaa");
// remove_namespace("kawaa");

impl RemoveStatement {
    fn namespace(mut self, name: Namespace) -> Self {
        self.namespace = Some(name);
        self
    }

    fn database(mut self, name: Database) -> Self {
        self.database = Some(name);
        self
    }

    fn login(mut self, name: impl Into<Login>, on: impl Into<NamespaceOrDatabase>) -> Self {
        let name = name.into();
        let on: NamespaceOrDatabase = on.into();

        let login_details = LoginDetails { name, on };
        self.login = Some(login_details);
        self
    }

    fn token(mut self, name: impl Into<Token>, on: impl Into<NamespaceOrDatabase>) -> Self {
        let name = name.into();
        let on: NamespaceOrDatabase = on.into();

        let token_details = TokenDetails { name, on };
        self.token = Some(token_details);
        self
    }

    fn scope(mut self, name: Scope) -> Self {
        self.scope = Some(name);
        self
    }

    fn table(mut self, name: Table) -> Self {
        self.table = Some(name);
        self
    }

    fn event(mut self, name: impl Into<Event>, on: impl Into<Table>) -> Self {
        let on = on.into();
        let name = name.into();
        self.event = Some((name, on));
        self
    }

    fn field(mut self, name: impl Into<DbField>, on: impl Into<Table>) -> Self {
        let on = on.into();
        let name = name.into();
        self.field = Some((name, on));
        self
    }

    fn index(mut self, name: impl Into<Index>, on: impl Into<Table>) -> Self {
        let on = on.into();
        let name = name.into();
        self.index = Some((name, on));
        self
    }
}

impl Buildable for RemoveStatement {
    fn build(&self) -> String {
        let mut query = String::from("REMOVE");

        if let Some(namespace) = &self.namespace {
            query.push_str(&format!(" NAMESPACE {}", namespace));
        }

        if let Some(database) = &self.database {
            query.push_str(&format!(" DATABASE {}", database));
        }

        if let Some(login) = &self.login {
            query.push_str(&format!(" LOGIN ON {}", login.on));
            if let Some(scope) = &self.scope {
                query.push_str(&format!(" ON {}", scope));
            }
        }

        if let Some(token) = &self.token {
            query.push_str(&format!(" TOKEN ON {}", token.on));
            if let Some(scope) = &self.scope {
                query.push_str(&format!(" ON {}", scope));
            }
        }

        if let Some(table) = &self.table {
            query.push_str(&format!(" TABLE {}", table));
            if let Some(event) = &self.event {
                query.push_str(&format!(" ON TABLE {} {}", event, event));
            } else if let Some(field) = &self.field {
                query.push_str(&format!(" ON TABLE {} {}", field.table(), field));
            } else if let Some(index) = &self.index {
                query.push_str(&format!(" ON TABLE {} {}", index.table(), index));
            }
        }

        query
    }
}
impl std::fmt::Display for RemoveStatement {
    fn to_string(&self) -> String {}
}
#[test]
fn test() {
    let statement = RemoveStatement::new()
        .database("my_database")
        .table("my_table")
        .field("my_field", Some("my_table"));
}
