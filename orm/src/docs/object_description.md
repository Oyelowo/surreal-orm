# Object

In Surreal, an Object is a complex nested data structure that can be embedded
within Nodes, modeled by the `Object` trait in Rust. Unlike Nodes, which
represent database tables, Objects do not represent tables on their own.
However, they are crucial in modeling more complex data within a Node. They can
be used directly as a field type or as an element within an array, enabling you
to encapsulate and manage more intricate data structures within your database
models.

Here's an example of a node named Alien that has a nested Rocket object and an
array of Rocket objects:

```rust, editable
use serde::{Deserialize, Serialize};
use surreal_orm::{SurrealSimpleId, Node};

#[derive(Node, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table = "alien")]
pub struct Alien {
    pub id: SurrealSimpleId<Self>,

    #[surreal_orm(nest_object = "Rocket")]
    pub favorite_rocket: Rocket,

    #[surreal_orm(nest_array = "Rocket")]
    pub strong_rockets: Vec<Rocket>,
}

#[derive(Object, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rocket {
    pub name: String,
    pub strength: u64,
}
```

Objects in Surreal can be used in two ways: as nested objects (`nest_object`)
and as arrays of nested objects (`nest_array`). For instance, in an Alien Node,
a Rocket Object can be a single favorite rocket (`nest_object`) or a collection
of strong rockets (`nest_array`). This powerful feature allows for more complex
nested data to be directly embedded in your models, thus offering a more nuanced
representation of real-world entities in your database.

Notably, the use of `nest_object` or `nest_array` is validated at compile time.
This ensures that `nest_object` is used correctly for the specific Object and
`nest_array` corresponds to a vector of that Object, providing a guarantee of
the validity of your data structures before your program runs.
