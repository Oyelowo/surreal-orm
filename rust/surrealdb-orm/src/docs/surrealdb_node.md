# SurrealdbNode

In Surrealdb, your database is represented using Nodes, Edges, and Objects:

- Nodes: These correspond to database tables, defined as Rust structs implementing the `SurrealdbNode` trait. Nodes can link to other Nodes and incorporate Objects for complex nested data structures.

- Edges: Edges represent relationships between Nodes and are used for modeling many-to-many relationships or storing additional information about the relationship itself.

- Objects: These are complex nested data structures embedded within Nodes. While they don't represent standalone tables, they facilitate complex data modeling within a Node.

Nodes are the heart of your database model in Surrealdb. They're Rust structs decorated with `SurrealdbNode` attributes for overall configuration and field-specific attributes for property definition. There are three types of links that you can use to define relationships between Nodes: `LinkSelf`, `LinkOne`, and `LinkMany`.

- `LinkSelf`: This is a self-referential link within the same Node (table). For example, if an `Alien` can be friends with other aliens, you would use `LinkSelf`.

- `LinkOne`: This creates a one-to-one relationship between two different Nodes. If every `Alien` has exactly one `Weapon`, you would use `LinkOne`.

- `LinkMany`: This creates a one-to-many relationship between two Nodes. If an `Alien` can have multiple `SpaceShip`s, you would use `LinkMany`.

For example:

```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb_orm::{LinkMany, LinkOne, LinkSelf, SurrealSimpleId, SurrealdbNode};

#[derive(SurrealdbNode, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
#[surrealdb(table_name = "alien")]
pub struct Alien {
    pub id: SurrealSimpleId<Self>,

    #[surrealdb(link_self = "Alien")]
    pub friend: LinkSelf<Alien>,

    #[surrealdb(link_one = "Weapon")]
    pub weapon: LinkOne<Weapon>,

    #[surrealdb(link_many = "SpaceShip")]
    pub space_ships: LinkMany<SpaceShip>,
}

#[derive(SurrealdbNode, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
#[surrealdb(table_name = "weapon")]
pub struct Weapon {
    pub id: SurrealSimpleId<Self>,
    pub name: String,
    pub strength: u64,
}

#[derive(SurrealdbNode, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[surrealdb(table_name = "space_ship")]
pub struct SpaceShip {
    pub id: SurrealId<Self, String>,
    pub name: String,
    pub created: DateTime<Utc>,
}
```

In this `Alien` Node, an alien can have an friend (another alien), a weapon (one-to-one relationship with `Weapon` Node), and multiple spaceships (one-to-many relationship with `SpaceShip` Node).

In summary, Nodes in Surrealdb provide a powerful way to model your database schema directly in Rust, with type safety, automatic serialization/deserialization, and the ability to define complex relationships between different tables.

## Struct Attributes

| Attribute        | Description                                                                                                                                                                                                                                                                                                                                                                          | Type               | Optional |
| ---------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ------------------ | -------- |
| rename_all       | Renames all the struct's fields according to the given case convention. The possible values are "lowercase", "UPPERCASE", "PascalCase", "camelCase", "snake_case", "SCREAMING_SNAKE_CASE".                                                                                                                                                                                           | string             | Y        |
| table_name       | Explicitly define the table name. By default, it must correspond with the struct name in snake_case. Use `relax_table_name` if you want to opt out of this but not encouraged.                                                                                                                                                                                                       | Option<String>     | Y        |
| relax_table_name | Determines whether the struct's name is matched to the table name as the snake case by default. This is not encouraged. Using your struct 1:1 to your database tables helps to ensure uniquness and prevent confusion.                                                                                                                                                               | Option<bool>       | Y        |
| schemafull       | Make the table enforce a schema struct.                                                                                                                                                                                                                                                                                                                                              | Option<bool>       | Y        |
| drop             | Drop the table if it exists and create a new one with the same name.                                                                                                                                                                                                                                                                                                                 | Option<bool>       | Y        |
| as               | Inline statement e.g `select(All).from(user)` for creating a projection using the DEFINE TABLE statement. This is useful for copying data from an existing table in the new table definition. This is similar to making a view in a RDBMS.                                                                                                                                           | A select statement | Y        |
| as_fn            | Same as above `as` but defined as external function from the struct e.g `select_reading_from_user` for creating a projection using the DEFINE TABLE statement. This is useful for copying data from an existing table in the new table definition. This is similar to making a view in a RDBMS.                                                                                      | A function name    | Y        |
| permissions      | Specify permissions that apply to the table using the `for` statement.                                                                                                                                                                                                                                                                                                               | ForStatement       | Y        |
| permissions_fn   | Same as `permission` but as an external function from the struct. Specify permissions that apply to the table using the `for` statement.                                                                                                                                                                                                                                             | ForStatement       | Y        |
| define           | Generates a `DEFINE TABLE` statement for the table. This overrides other specific definitions to prevent confusion and collision. You can also invoke an external function directly rather than inlining the function e.g `define = "define_student()"`                                                                                                                              | inline code string | Y        |
| define_fn        | Generates a `DEFINE TABLE` statement for the table. This overrides other specific definitions to prevent confusion and collision. Same as `define` attribute but expects the function name instead rather than invocation i.e `define_student` instead of `define_student()`. You can also invoke an external function directly rather than inlining the function e.g `define = "def |

## Field Attributes

| Attribute         | Description                                                                                                                                                                                                                                                                                                                                                                  | Type                                          | Optional |
| ----------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------- | -------- |
| rename            | Renames the field.                                                                                                                                                                                                                                                                                                                                                           | `string`                                      | Y        |
| link_one          | Specifies a relationship to a singular record in another node table in the database.                                                                                                                                                                                                                                                                                         | `model=NodeEdgeNode, connection ->edge->node` | Y        |
| link_self         | Specifies a relationship to a singular record in the same node table in the database.                                                                                                                                                                                                                                                                                        | `SurrealdbNode`                               | Y        |
| link_many         | Specifies a relationship to multiple records in another node table in the database.                                                                                                                                                                                                                                                                                          | `Vec<S                                        |
| relate            | Generates the relation helpers for the Current Node struct to an edge and destination node. The corresponding field name is merely used as an alias in code generation and is read only and not serializable. e.g `student:1->writes->book:2`                                                                                                                                |
| type              | Specify the valid surrealdb field's type. One of any, array, bool, datetime, decimal, duration, float, int, number, object, string, record.                                                                                                                                                                                                                                  | surrealdb field type                          | Y        |
| assert            | Assert the field's value meets a certain criteria using the an filter using `value()` function as an operation (e.g `value().is_not(NONE)`) or in `cond` helper function for more complex filter assertion. e.g `cond(value().is_not(NONE)).and(value().like("@codebreather"))`.                                                                                             | inline code string                            | Y        |
| assert_fn         | Provide a function to assert the field's value meets a certain criteria. This is similar to `assert` but is intended for an already created external function which is useful when reusing an assertion e.g `is_email`.                                                                                                                                                      | function name string                          | Y        |
| content_type      | Only when for nested array. Specifies the type of the items of the array.                                                                                                                                                                                                                                                                                                    | `Option<FieldTypeWrapper>`                    | Y        |
| content_assert    | Only used for nested array. Asserts a condition on the content.                                                                                                                                                                                                                                                                                                              | `Option<syn::LitStr>`                         | Y        |
| content_assert_fn | Only used for nested array. Specifies the function to assert a condition on the content.                                                                                                                                                                                                                                                                                     | `Option<syn::Path>`                           | Y        |
| define            | Generates a `DEFINE FIELD` statement for the table. This overrides other specific definitions to prevent confusion and collision. You can also invoke an external function directly rather than inlining the function e.g `define = "define_age()"`                                                                                                                          | inline code string                            | Y        |
| define_fn         | Generates a `DEFINE FIELD` statement for the table. This overrides other specific definitions to prevent confusion and collision. Same as `define` attribute but expects the function name instead rather than invocation i.e `define_age` instead of `define_age()`. You can also invoke an external function directly rather than inlining the function e.g `define = "def |
| skip_serializing  | When true, this field will be omitted when serializing the struct.                                                                                                                                                                                                                                                                                                           | bool                                          | Y        |
