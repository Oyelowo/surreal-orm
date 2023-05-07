/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use crate::{Alias, Field, NodeClause, Raw, SurrealId, Table};
use serde::Serialize;
use surrealdb::sql::{Id, Thing, Uuid};

/// SurrealdbModel is a trait signifying superset of SurrealdbNode and SurrealdbEdge.
/// i.e both are SurrealdbModel
pub trait SurrealdbModel: Sized {
    /// The name of the model/table
    fn table_name() -> Table;

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
    fn create_thing(id: impl Into<Id>) -> Thing {
        Thing::from((Self::table_name().to_string(), id.into()))
    }

    ///
    fn create_id<V: Into<Id>>(id: V) -> SurrealId<Self, V> {
        // Thing::from((Self::table_name().to_string(), id.into()))
        // SurrealId2::cr
        SurrealId::new(id).into()
    }

    /// Create a new surreal Thing/compound id from a Uuid
    fn create_uuid() -> SurrealId<Self, Uuid> {
        SurrealId::new(Thing::from((
            Self::table_name().to_string(),
            Uuid::new_v4().to_string(),
        )))
        .into()
    }

    fn create_ulid() -> SurrealId<Self, Uuid> {
        SurrealId::new(Thing::from((
            Self::table_name().to_string(),
            Uuid::new_v4().to_string(),
        )))
        .into()
    }

    /// Create a new surreal Thing/compound id from a Uuid
    fn create_thing_uuid() -> Thing {
        Thing::from((Self::table_name().to_string(), Uuid::new_v4().to_string()))
    }
}

/// SurrealdbNode is a trait signifying a node in the graph
pub trait SurrealdbNode: SurrealdbModel + Serialize {
    /// For merge update of object
    type NonNullUpdater;
    /// The type of the schema
    type Schema;
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
    fn schema() -> Self::Schema;
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
    fn get_id<T: From<Thing>>(self) -> T;
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
}

/// SurrealdbEdge is a trait signifying an edge in the graph
pub trait SurrealdbEdge: SurrealdbModel + Serialize {
    /// The Origin node
    type In;
    /// The Destination node
    type Out;
    #[doc(hidden)]
    type TableNameChecker;
    /// The type of the schema
    type Schema;

    /// returns the schema of the edge for generating graph strings e.g
    fn schema() -> Self::Schema;
    /// returns the key of the edge aka id field
    fn get_id<T: From<Thing>>(self) -> T;
    /// returns the table name of the edge
    fn get_table_name() -> Table;
}

/// SurrealdbObject is a trait signifying a nested object in the graph
pub trait SurrealdbObject: Serialize {
    /// For merge update of object
    type NonNullUpdater;
    /// The type of the schema
    type Schema;
    /// returns the schema of a nested object.
    fn schema() -> Self::Schema;
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
