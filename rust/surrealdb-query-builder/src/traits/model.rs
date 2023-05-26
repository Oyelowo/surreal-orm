/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::marker::PhantomData;

use crate::{
    statements::{
        create::{create, CreateStatement},
        select::{select, SelectStatement},
        update::{update, UpdateStatement},
    },
    Alias, All, Buildable, Conditional, Field, NodeClause, Parametric, Queryable, Raw,
    ReturnableSelect, ReturnableStandard, SurrealId, SurrealSimpleId, SurrealUlid, SurrealUuid,
    SurrealdbOrmResult, Table, TestUser, Valuex,
};
use serde::{de::DeserializeOwned, Serialize};
use surrealdb::{
    engine::local::Db,
    sql::{self, Thing},
    Surreal,
};

/// SurrealdbModel is a trait signifying superset of SurrealdbNode and SurrealdbEdge.
/// i.e both are SurrealdbModel
#[async_trait::async_trait]
pub trait SurrealdbModel: Sized + Serialize + DeserializeOwned {
    /// The id of the model/table
    type Id;
    /// The name of the model/table
    fn table_name() -> Table;

    /// Returns id of the model/table
    fn get_id(self) -> Self::Id;

    /// Returns id of the model/table as a Thing
    fn get_id_as_thing(self) -> sql::Thing;

    /// The name of the all fields that are serializable
    /// and can potentially be written to the database.
    fn get_serializable_fields() -> Vec<Field>;

    /// The name of the all fields that are linked i.e line_one, line_many, or line_self.
    fn get_linked_fields() -> Vec<Field>;

    /// The names of link_one fields
    fn get_link_one_fields() -> Vec<Field>;

    /// The names of link_self fields
    fn get_link_self_fields() -> Vec<Field>;

    /// The names of link_one and link_self fields
    fn get_link_one_and_self_fields() -> Vec<Field>;

    /// The names of link_many fields
    fn get_link_many_fields() -> Vec<Field>;

    /// Get model's table definition statement
    fn define_table() -> Raw;
    /// Get model's fields definitions statements as a list
    fn define_fields() -> Vec<Raw>;

    /// Create a new SurrealId from a string
    fn create_thing(id: impl Into<sql::Id>) -> Thing {
        Thing::from((Self::table_name().to_string(), id.into()))
    }

    ///
    fn create_id<V: Into<sql::Id>>(id: V) -> SurrealId<Self, V> {
        SurrealId::new(id).into()
    }

    /// Create a new surreal Thing/compound id from a Uuid
    fn create_uuid() -> SurrealUuid<Self> {
        SurrealUuid::new()
    }

    /// Create a new surreal Thing/compound id from a Ulid
    fn create_ulid() -> SurrealUlid<Self> {
        SurrealUlid::new()
    }

    /// Create a new surreal Thing/compound id from a simple NanoId.
    /// This is the default used by surrealdb engine.
    fn create_simple_id() -> SurrealSimpleId<Self> {
        SurrealSimpleId::new()
    }

    /// Map a Thing to a SurrealId
    fn from_thing(thing: sql::Thing) -> SurrealdbOrmResult<SurrealId<Self, sql::Id>> {
        SurrealId::try_from(thing)
    }

    // /// Create a new surreal Thing/compound id from a Uuid
    // fn create_thing_uuid() -> Thing {
    //     Thing::from((Self::table_name().to_string(), Uuid::new_v4().to_string()))
    // }

    // DB QUERIES HELPERS
    async fn save(self) -> UpdateStatement<Self> {
        // let x = update::<Self>(self);
        // update::<Self>(self).get_one(db)
        update::<Self>(self)
    }

    async fn find_by_id(id: Self::Id, db: Surreal<Db>) -> SelectStatement {
        select(All).from(id)
    }

    async fn find_one(filter: impl Conditional, db: Surreal<Db>) -> ModelSelect<Self> {
        select(All).from(Self::table_name()).where_(filter).into()
    }

    async fn find_many(filter: impl Conditional, db: Surreal<Db>) -> ModelSelect<Self> {
        select(All).from(Self::table_name()).where_(filter).into()
    }
}

#[derive(Debug, Clone)]
struct ModelSelect<T: Serialize + DeserializeOwned + SurrealdbModel>(
    SelectStatement,
    PhantomData<T>,
);

impl<T: Serialize + DeserializeOwned + SurrealdbModel> From<SelectStatement> for ModelSelect<T> {
    fn from(value: SelectStatement) -> Self {
        Self(value, PhantomData)
    }
}

// impl<T: Serialize + DeserializeOwned + SurrealdbModel + std::ops::Deref> std::ops::Deref
//     for ModelSelect<T>
// {
//     type Target = SelectStatement;
//
//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

impl<T: Serialize + DeserializeOwned + SurrealdbModel> Erroneous for ModelSelect<T> {}
impl<T: Serialize + DeserializeOwned + SurrealdbModel> Parametric for ModelSelect<T> {}
impl<T: Serialize + DeserializeOwned + SurrealdbModel> Buildable for ModelSelect<T> {}
impl<T: Serialize + DeserializeOwned + SurrealdbModel> Queryable for ModelSelect<T> {}
impl<T: Serialize + DeserializeOwned + SurrealdbModel> ReturnableStandard<T> for ModelSelect<T> {}

/// SurrealdbNode is a trait signifying a node in the graph
#[async_trait::async_trait]
pub trait SurrealdbNode: SurrealdbModel + Serialize + SchemaGetter {
    /// For merge update of object
    type NonNullUpdater;
    /// The type of the schema
    // type Schema;
    /// The type of the aliases
    type Aliases;
    #[doc(hidden)]
    type TableNameChecker;
    /// returns the schema of the node for generating graph strings e.g
    ///
    /// Example:
    /// ```rust, ignore
    /// let UserSchema { id, age, name, email, writes__ } = User::schema();
    ///
    /// writes__(Empty)
    ///     .book(Book::schema().name.equal("Oyelowo"))
    ///     .title;
    ///
    /// Student::schema()
    ///     .bestFriend(student_id)
    ///     .bestFriend(st_schema.age.between(18, 150))
    ///     .bestFriend(Empty)
    ///     .writes__(StudentWritesBook::schema().timeWritten.greater_than(3422))
    ///     .book(Book::schema().id.equal(RecordId::from(("book", "blaze"))))
    ///     .content
    /// ```
    // fn schema() -> Self::Schema;
    /// Same as schema but prefixed. Useful for traversing the graph for e.g aliases.
    // fn schema_prefixed(prefix: String) -> Self::Schema;
    /// returns the alias names of the relation graph strings of the model
    /// e.g for relation - `->edge->graph AS alias`, the alias would be alias.
    /// the struct definition could be:
    ///
    /// Example:
    /// ```rust, ignore
    /// Aliases would be published_books and written_blogs in this case.
    ///
    /// #[derive(SurrealdbNode, Serialize, Deserialize, Debug, Clone)]
    /// struct UserSchema {
    ///    ...,
    ///    #[surrealdb(relate(model = "StudentPublishedBook", connection = "->published->book"))]
    ///    published_books: Relate<Book>,
    ///    
    ///    #[surrealdb(relate(model = "StudentWritesBlog", connection = "->writes->blog"))]
    ///    wrriten_blogs: Relate<Blog>,
    /// }
    fn aliases() -> Self::Aliases;
    /// returns the key of the node aka id field
    // // fn get_id<T: From<Thing>>(self) -> T;
    // fn get_id<T: Into<Thing>>(self) -> T;
    /// returns the table name of the node
    fn get_table_name() -> Table;
    /// Useful in relate statement for attaching id or statement to a node
    /// Example:
    /// ```rust, ignore
    /// Student::with(Empty).writes__(Empty).book(Empty);
    ///
    /// relate(Student::with(student_id).writes__(Empty).book(book_id)).content(write);
    ///
    /// relate(
    ///     Student::with(select(All).from(Student::get_table_name()))
    ///         .writes__(Empty)
    ///         .book(
    ///             select(All)
    ///                 .from(Book::get_table_name())
    ///                 .where_(Book::schema().title.like("Oyelowo")),
    ///         ),
    /// )
    /// .content(write)
    ///
    /// Student::schema()
    ///     .writes__(Empty)
    ///     .book(Book::schema().id.equal(RecordId::from(("book", "blaze"))))
    ///     .title;
    fn with(clause: impl Into<NodeClause>) -> Self::Schema;

    /// returns the relations aliases of the model in the format `->edge->graph AS alias`.
    ///
    /// Example:
    /// ```rust, ignore
    /// /// field relations would be `->published->book AS published_books` and `->writes->blog AS written_blogs` in this case.
    ///
    /// #[derive(SurrealdbNode, Serialize, Deserialize, Debug, Clone)]
    /// struct UserSchema {
    ///    ...,
    ///    #[surrealdb(relate(model = "StudentPublishedBook", connection = "->published->book"))]
    ///    published_books: Relate<Book>,
    ///    
    ///    #[surrealdb(relate(model = "StudentWritesBlog", connection = "->writes->blog"))]
    ///    wrriten_blogs: Relate<Blog>,
    /// }
    fn get_fields_relations_aliased() -> Vec<Alias>;

    // DB QUERIES HELPERS
    async fn create(content: Self) -> CreateStatement<Self> {
        create(content)
    }
}

/// SurrealdbEdge is a trait signifying an edge in the graph
pub trait SurrealdbEdge: SurrealdbModel + Serialize + SchemaGetter {
    /// The Origin node
    type In;
    /// The Destination node
    type Out;
    #[doc(hidden)]
    type TableNameChecker;
    /// The type of the schema
    // type Schema;

    /// returns the schema of the edge for generating graph strings e.g
    // fn schema() -> Self::Schema;
    // /// returns the key of the edge aka id field
    // fn get_id<T: From<Thing>>(self) -> T;
    /// returns the table name of the edge
    fn get_table_name() -> Table;
}

/// SurrealdbObject is a trait signifying a nested object in the graph
pub trait SurrealdbObject: Serialize + SchemaGetter {
    /// For merge update of object
    type NonNullUpdater;
    // The type of the schema
    // type Schema;
    // returns the schema of a nested object.
    // fn schema() -> Self::Schema;
}

///
pub trait SchemaGetter {
    ///
    type Schema;
    ///
    fn schema() -> Self::Schema;
    ///
    fn schema_prefixed(prefix: impl Into<Valuex>) -> Self::Schema;
}

/// List of error
pub type ErrorList = Vec<String>;

/// Trait for accumulating errors in query building process which is passed
/// to query execution methods to catch runtime errors that cannot be caught
/// at compile time e.g Validating that Id belogs to a node within the context it's
/// used in the statement. This is used in Update statement for example.
pub trait Erroneous {
    /// Get errors
    fn get_errors(&self) -> ErrorList {
        vec![]
    }
}
