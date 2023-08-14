/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use crate::{
    count,
    statements::{
        create::{create, CreateStatement},
        delete::{delete, DeleteStatementMini},
        select::{select, SelectStatementCount},
        select_value,
        update::{update, UpdateStatement},
        SelectStatementMini,
    },
    Alias, All, Conditional, Field, Filter, NodeClause, Raw, SurrealId, SurrealOrmResult,
    SurrealSimpleId, SurrealUlid, SurrealUuid, Table, Valuex,
};
use serde::{de::DeserializeOwned, Serialize};
use surrealdb::sql::{self, Thing};

/// SurrealModel is a trait signifying superset of Node and Edge.
/// i.e both are SurrealModel
pub trait SurrealModel: Sized {
    /// The id of the model/table
    type Id;
    /// Used for updating a model/table. Useful when you want to skip optional fields
    /// when updating a model/table.
    type NonNullUpdater;
    /// For checking renamed struct field names
    type StructRenamedCreator;
    /// The name of the model/table
    fn table_name() -> Table;

    /// Returns id of the model/table
    fn get_id(self) -> Self::Id;

    /// Returns id of the model/table as a Thing
    fn get_id_as_thing(&self) -> sql::Thing;

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
        SurrealId::new(id)
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
    fn from_thing(thing: sql::Thing) -> SurrealOrmResult<SurrealId<Self, sql::Id>> {
        SurrealId::try_from(thing)
    }

    // /// Create a new surreal Thing/compound id from a Uuid
    // fn create_thing_uuid() -> Thing {
    //     Thing::from((Self::table_name().to_string(), Uuid::new_v4().to_string()))
    // }
}

/// DB convenience helper methods.
pub trait SurrealCrud: Sized + Serialize + DeserializeOwned + SurrealModel {
    /// Creates or updates a model/table in the database.
    fn save(self) -> UpdateStatement<Self> {
        update::<Self>(self.get_id_as_thing()).content(self)
    }

    /// Finds a record by id.
    fn find_by_id(id: impl Into<Thing>) -> SelectStatementMini<Self> {
        select(All).from(id.into()).into()
    }

    /// Finds records by filtering.
    fn find_where(filter: impl Conditional + Clone) -> SelectStatementMini<Self> {
        select(All).from(Self::table_name()).where_(filter).into()
    }

    /// Count filtered records.
    fn count_where(filter: impl Conditional + Clone) -> SelectStatementCount {
        let selection = select_value(Field::new("count")).from(
            select(count!(Filter::new(filter)))
                .from(Self::table_name())
                .group_all(),
        );
        selection.into()
    }

    /// Count all records.
    fn count_all() -> SelectStatementCount {
        let selection = select_value(Field::new("count"))
            .from(select(count!()).from(Self::table_name()).group_all());
        selection.into()
    }

    /// Delete the current record by instance.
    fn delete(&self) -> DeleteStatementMini<Self> {
        delete::<Self>(self.get_id_as_thing()).into()
    }

    /// Deletes a record by id.
    fn delete_by_id(id: impl Into<Thing>) -> DeleteStatementMini<Self> {
        delete::<Self>(id.into()).into()
    }

    /// Deletes records by filtering.
    fn delete_where(filter: impl Conditional + Clone) -> DeleteStatementMini<Self> {
        delete::<Self>(Self::table_name()).where_(filter).into()
    }
}

impl<T> SurrealCrud for T where T: Sized + Serialize + DeserializeOwned + SurrealModel {}

/// DB convenience helper methods.
pub trait SurrealCrudNode: Sized + Serialize + DeserializeOwned + Node {
    /// Creates or updates a model/table in the database.
    fn create(self) -> CreateStatement<Self> {
        create().content(self)
    }
}
impl<T> SurrealCrudNode for T where T: Sized + Serialize + DeserializeOwned + Node {}

/// Node is a trait signifying a node in the graph
#[async_trait::async_trait]
pub trait Node: SurrealModel + Serialize + SchemaGetter {
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
    /// #[derive(Node, Serialize, Deserialize, Debug, Clone)]
    /// struct UserSchema {
    ///    ...,
    ///    #[surreal_orm(relate(model = "StudentPublishedBook", connection = "->published->book"))]
    ///    published_books: Relate<Book>,
    ///    
    ///    #[surreal_orm(relate(model = "StudentWritesBlog", connection = "->writes->blog"))]
    ///    wrriten_blogs: Relate<Blog>,
    /// }
    /// ```
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
    /// ```
    fn with(clause: impl Into<NodeClause>) -> Self::Schema;

    /// returns the relations aliases of the model in the format `->edge->graph AS alias`.
    ///
    /// Example:
    /// ```rust, ignore
    /// /// field relations would be `->published->book AS published_books` and `->writes->blog AS written_blogs` in this case.
    ///
    /// #[derive(Node, Serialize, Deserialize, Debug, Clone)]
    /// struct UserSchema {
    ///    ...,
    ///    #[surreal_orm(relate(model = "StudentPublishedBook", connection = "->published->book"))]
    ///    published_books: Relate<Book>,
    ///    
    ///    #[surreal_orm(relate(model = "StudentWritesBlog", connection = "->writes->blog"))]
    ///    wrriten_blogs: Relate<Blog>,
    /// }
    /// ```
    fn get_fields_relations_aliased() -> Vec<Alias>;
}

/// Edge is a trait signifying an edge in the graph
pub trait Edge: SurrealModel + Serialize + SchemaGetter {
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

/// Object is a trait signifying a nested object in the graph
pub trait Object: Serialize + SchemaGetter {
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
