# Links, Nestings and Relations

- `link_one`: It is an attribute used to define a one-to-one relationship
  between two Nodes. For example, consider the `Alien` struct with the field
  `weapon`:

```rust
use surreal_orm::{Serialize, Deserialize,LinkOne, SurrealSimpleId, Node};

#[derive(Node, Serialize, Deserialize, Debug)]
#[orm(table = "alien")]
pub struct Alien {
    pub id: SurrealSimpleId<Self>,

    // #[orm(link_one = "Weapon", type_ = "record(weapon)")]
    #[orm(link_one = "Weapon")]
    pub best_weapon: LinkOne<Weapon>,
}


#[derive(Node, Serialize, Deserialize, Debug)]
#[orm(table = "weapon")]
pub struct Weapon {
    pub id: SurrealSimpleId<Self>,
}
```

This attribute indicates that an `Alien` can have a single best `Weapon`. The
relationship is represented by a foreign key in the database table, and the
`type` attribute specifies the database type for the relationship.

- `link_many`: It is an attribute used to define a one-to-many relationship
  between two Nodes. For instance, in the `Alien` struct, we have the
  `space_ships` field:

```rust
use surreal_orm::{Serialize, Deserialize, LinkMany, SurrealSimpleId, Node};

#[derive(Node, Serialize, Deserialize, Debug)]
#[orm(table = "alien")]
pub struct Alien {
    pub id: SurrealSimpleId<Self>,
    // #[orm(link_many = "SpaceShip", type_ = "array", item_type = "record(space_ship)")]
    #[orm(link_many = "SpaceShip")]
    pub space_ships: LinkMany<SpaceShip>,
}

#[derive(Node, Serialize, Deserialize, Debug)]
#[orm(table = "space_ship")]
pub struct SpaceShip {
    pub id: SurrealSimpleId<Self>,
}
```

This attribute indicates that an `Alien` can have multiple `SpaceShip` instances
associated with it. The relationship is represented by a foreign key or a join
table in the database, and the `type` attribute specifies the database type for
the relationship.

- `nest_object`: It is an attribute used to embed a single Object within a Node.
  In the `Alien` struct, we have the `weapon` field:

```rust
use surreal_orm::{Serialize, Deserialize,SurrealSimpleId, Node, Object};

#[derive(Node, Serialize, Deserialize, Debug)]
#[orm(table = "alien")]
pub struct Alien {
    pub id: SurrealSimpleId<Self>,
    #[orm(nest_object = "Rocket")]
    pub favorite_rocket: Rocket,
}

#[derive(Object, Serialize, Deserialize, Debug)]
#[orm(table = "rocket")]
pub struct Rocket {
}
```

This attribute specifies that an `Alien2` has a nested `Rocket` object
representing its weapon. The `Rocket` object is stored as part of the `Alien2`
Node in the database.

- `nest_array`: It is an attribute used to embed multiple Objects within a Node.
  Although not explicitly used in the provided code examples, it would be
  similar to `NestObject`, but with a collection type field (e.g.,
  `Vec<Rocket>`).

```rust
use surreal_orm::{Serialize, Deserialize,SurrealSimpleId, Node, Object};

#[derive(Node, Serialize, Deserialize, Debug)]
#[orm(table = "alien")]
pub struct Alien {
    pub id: SurrealSimpleId<Self>,

    #[orm(nest_array = "Rocket")]
    pub big_rockets: Vec<Rocket>,
}

#[derive(Node, Serialize, Deserialize, Debug)]
#[orm(table = "rocket")]
pub struct Rocket {
}
```

- `relate`: It is an attribute used to define a read-only relationship between
  two Nodes. In the `Alien` struct, we have the `planets_to_visit` field:

```rust
use surreal_orm::{Serialize, Deserialize, SurrealSimpleId, Node, Edge, Relate};

#[derive(Node, Serialize, Deserialize, Debug)]
#[orm(table = "alien")]
pub struct Alien {
    pub id: SurrealSimpleId<Self>,

    #[orm(relate(model = "AlienVisitsPlanet", connection = "->visits->planet"))]
    #[serde(skip_serializing, default)]
    pub planets_to_visit: Relate<Planet>,
}

#[derive(Node, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[orm(table = "planet")]
pub struct Planet {
    pub id: SurrealSimpleId<Self>,
    pub population: u64,
}

// Visits
#[derive(Edge, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[orm(table = "visits")]
pub struct Visits<In: Node, Out: Node> {
    pub id: SurrealSimpleId<Self>,
    #[serde(rename = "in")]
    pub in_: LinkOne<In>,
    pub out: LinkOne<Out>,
    pub time_visited: Duration,
}

// Connects Alien to Planet via Visits
pub type AlienVisitsPlanet = Visits<Alien, Planet>;
```

This attribute specifies that an `Alien` has a read-only relationship with
`Planet` through the `AlienVisitsPlanet` model. The `connection` attribute
describes the relationship path between the Nodes. The relationship is read-only
because the `serde(skip_serializing)` attribute is used to prevent it from being
serialized.

These attributes provide additional information to Surreal for modeling
relationships and embedding Objects within Nodes, allowing for more complex and
flexible database schema designs.
