# Node

In Surreal, your database is represented using Nodes, Edges, and Objects:

- Nodes: These correspond to database tables, defined as Rust structs
  implementing the `Node` trait. Nodes can link to other Nodes and incorporate
  Objects for complex nested data structures.

- Edges: Edges represent relationships between Nodes and are used for modeling
  many-to-many relationships or storing additional information about the
  relationship itself.

- Objects: These are complex nested data structures embedded within Nodes. While
  they don't represent standalone tables, they facilitate complex data modeling
  within a Node.

Nodes are the heart of your database model in Surreal. They're Rust structs
decorated with `Node` attributes for overall configuration and field-specific
attributes for property definition. There are three types of links that you can
use to define relationships between Nodes: `LinkSelf`, `LinkOne`, and
`LinkMany`.

- `LinkSelf`: This is a self-referential link within the same Node (table). For
  example, if an `Alien` can be friends with other aliens, you would use
  `LinkSelf`.

- `LinkOne`: This creates a one-to-one relationship between two different Nodes.
  If every `Alien` has exactly one `Weapon`, you would use `LinkOne`.

- `LinkMany`: This creates a one-to-many relationship between two Nodes. If an
  `Alien` can have multiple `SpaceShip`s, you would use `LinkMany`.

For example:

```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surreal_orm::{LinkMany, LinkOne, LinkSelf, SurrealSimpleId, Node};

#[derive(Node, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
#[orm(table = "alien")]
pub struct Alien {
    pub id: SurrealSimpleId<Self>,

    #[orm(link_self = "Alien")]
    pub friend: LinkSelf<Alien>,

    #[orm(link_one = "Weapon")]
    pub weapon: LinkOne<Weapon>,

    #[orm(link_many = "SpaceShip")]
    pub space_ships: LinkMany<SpaceShip>,
}

#[derive(Node, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
#[orm(table = "weapon")]
pub struct Weapon {
    pub id: SurrealSimpleId<Self>,
    pub name: String,
    pub strength: u64,
}

#[derive(Node, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[orm(table = "space_ship")]
pub struct SpaceShip {
    pub id: SurrealId<Self, String>,
    pub name: String,
    pub created: DateTime<Utc>,
}
```

In this `Alien` Node, an alien can have an friend (another alien), a weapon
(one-to-one relationship with `Weapon` Node), and multiple spaceships
(one-to-many relationship with `SpaceShip` Node).

In summary, Nodes in Surreal provide a powerful way to model your database
schema directly in Rust, with type safety, automatic
serialization/deserialization, and the ability to define complex relationships
between different tables.
