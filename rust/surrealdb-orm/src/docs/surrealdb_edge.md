# SurrealdbEdge

Edges in Surrealdb represent relationships between Nodes. They are useful when you want to model many-to-many relationships or when you want to store additional information about the relationship itself. Edges can be seen as "relationship tables" in a relational database context, holding metadata about the relationship between two entities. Edges are defined by a Rust struct that implements the `SurrealdbEdge` trait.

Here's a detailed example:

```rust
#[derive(SurrealdbNode, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[surrealdb(table_name = "alien")]
pub struct Alien {
    pub id: SurrealSimpleId<Self>,
    pub name: String,

    // This is a read-only field
    #[surrealdb(relate(model = "AlienVisitsPlanet", connection = "->visits->planet"))]
    #[serde(skip_serializing, default)]
    pub planets_to_visit: Relate<Planet>,
}

#[derive(SurrealdbNode, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surrealdb(table_name = "planet")]
pub struct Planet {
    pub id: SurrealSimpleId<Self>,
    pub population: u64,
}

// Visits
#[derive(SurrealdbEdge, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surrealdb(table_name = "visits")]
pub struct Visits<In: SurrealdbNode, Out: SurrealdbNode> {
    pub id: SurrealSimpleId<Self>,
    #[serde(rename = "in")]
    pub in_: LinkOne<In>,
    pub out: LinkOne<Out>,
    pub time_visited: Duration,
}

// Connects Alien to Planet via Visits
pub type AlienVisitsPlanet = Visits<Alien, Planet>;
```

The `Alien` Node has a field `planets_to_visit` which is of type `Relate<Planet>`. This field doesn't represent a direct link from `Alien` to `Planet`. Instead, it represents an indirect relationship via the `Visits` Edge. This indirect relationship is defined by the `Relate` annotation on the `planets_to_visit` field in the `Alien` Node.

The `#[surrealdb(relate(model = "AlienVisitsPlanet", connection = "->visits->planet"))]` attribute on the `planets_to_visit` field in the `Alien` Node tells Surrealdb that this field represents the `Planet` Nodes that are connected to the `Alien` Node via the `AlienVisitsPlanet` Edge. The `connection = "->visits->planet"` part defines the path of the relationship from the `Alien` Node, through the `Visits` Edge (represented by "visits"), and finally to the `Planet` Node.

The `Visits` Edge struct defines the structure of this relationship. It implements `SurrealdbEdge` and specifies two type parameters: `In` and `Out` which represent the source and target Node types of the relationship, respectively. In this example, `Alien` is the source and `Planet` is the target. The `Visits` Edge also has a `time_visited` field, which can store additional information about each visit.

In summary, Surrealdb Edges provide a flexible way to model complex relationships between Nodes, such as when an `Alien` visits a `Planet`. They allow for relationships to be modeled with additional information (like the `time_visited` field in the `Visits` Edge) and can represent both direct and indirect

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
| type              | Specify the valid surrealdb field's type. One of any, array, bool, datetime, decimal, duration, float, int, number, object, string, record.                                                                                                                                                                                                                                  | surrealdb field type                          | Y        |
| assert            | Assert the field's value meets a certain criteria using the an filter using `value()` function as an operation (e.g `value().is_not(NONE)`) or in `cond` helper function for more complex filter assertion. e.g `cond(value().is_not(NONE)).and(value().like("@codebreather"))`.                                                                                             | inline code string                            | Y        |
| assert_fn         | Provide a function to assert the field's value meets a certain criteria. This is similar to `assert` but is intended for an already created external function which is useful when reusing an assertion e.g `is_email`.                                                                                                                                                      | function name string                          | Y        |
| content_type      | Only when for nested array. Specifies the type of the items of the array.                                                                                                                                                                                                                                                                                                    | `Option<FieldTypeWrapper>`                    | Y        |
| content_assert    | Only used for nested array. Asserts a condition on the content.                                                                                                                                                                                                                                                                                                              | `Option<syn::LitStr>`                         | Y        |
| content_assert_fn | Only used for nested array. Specifies the function to assert a condition on the content.                                                                                                                                                                                                                                                                                     | `Option<syn::Path>`                           | Y        |
| define            | Generates a `DEFINE FIELD` statement for the table. This overrides other specific definitions to prevent confusion and collision. You can also invoke an external function directly rather than inlining the function e.g `define = "define_age()"`                                                                                                                          | inline code string                            | Y        |
| define_fn         | Generates a `DEFINE FIELD` statement for the table. This overrides other specific definitions to prevent confusion and collision. Same as `define` attribute but expects the function name instead rather than invocation i.e `define_age` instead of `define_age()`. You can also invoke an external function directly rather than inlining the function e.g `define = "def |
| skip_serializing  | When true, this field will be omitted when serializing the struct.                                                                                                                                                                                                                                                                                                           | bool                                          | Y        |
