# Links, Nestings and Relations

- `LinkOne`: It is an attribute used to define a one-to-one relationship between
  two Nodes. For example, consider the `Alien` struct with the field `weapon`:

```rust
use surrealdb_orm::{Serialize, Deserialize,LinkOne, SurrealSimpleId, SurrealdbNode};

#[derive(SurrealdbNode, Serialize, Deserialize, Debug)]
#[surrealdb(table_name = "alien")]
pub struct Alien {
    pub id: SurrealSimpleId<Self>,

    // #[surrealdb(link_one = "Weapon", type = "record(weapon)")]
    #[surrealdb(link_one = "Weapon")]
    pub best_weapon: LinkOne<Weapon>,
}


#[derive(SurrealdbNode, Serialize, Deserialize, Debug)]
#[surrealdb(table_name = "weapon")]
pub struct Weapon {
    pub id: SurrealSimpleId<Self>,
}
```

This attribute indicates that an `Alien` can have a single best `Weapon`. The
relationship is represented by a foreign key in the database table, and the
`type` attribute specifies the database type for the relationship.

- `LinkMany`: It is an attribute used to define a one-to-many relationship
  between two Nodes. For instance, in the `Alien` struct, we have the
  `space_ships` field:

```rust
use surrealdb_orm::{Serialize, Deserialize, LinkMany, SurrealSimpleId, SurrealdbNode};

#[derive(SurrealdbNode, Serialize, Deserialize, Debug)]
#[surrealdb(table_name = "alien")]
pub struct Alien {
    pub id: SurrealSimpleId<Self>,
    // #[surrealdb(link_many = "SpaceShip", type = "array", content_type = "record(space_ship)")]
    #[surrealdb(link_many = "SpaceShip")]
    pub space_ships: LinkMany<SpaceShip>,
}

#[derive(SurrealdbNode, Serialize, Deserialize, Debug)]
#[surrealdb(table_name = "space_ship")]
pub struct SpaceShip {
    pub id: SurrealSimpleId<Self>,
}
```

This attribute indicates that an `Alien` can have multiple `SpaceShip` instances
associated with it. The relationship is represented by a foreign key or a join
table in the database, and the `type` attribute specifies the database type for
the relationship.

- `NestObject`: It is an attribute used to embed a single Object within a Node.
  In the `Alien` struct, we have the `weapon` field:

```rust
use surrealdb_orm::{Serialize, Deserialize,SurrealSimpleId, SurrealdbNode, SurrealdbObject};

#[derive(SurrealdbNode, Serialize, Deserialize, Debug)]
#[surrealdb(table_name = "alien")]
pub struct Alien {
    pub id: SurrealSimpleId<Self>,
    #[surrealdb(nest_object = "Rocket")]
    pub favorite_rocket: Rocket,
}

#[derive(SurrealdbObject, Serialize, Deserialize, Debug)]
#[surrealdb(table_name = "rocket")]
pub struct Rocket {
}
```

This attribute specifies that an `Alien2` has a nested `Rocket` object
representing its weapon. The `Rocket` object is stored as part of the `Alien2`
Node in the database.

- `NestArray`: It is an attribute used to embed multiple Objects within a Node.
  Although not explicitly used in the provided code examples, it would be
  similar to `NestObject`, but with a collection type field (e.g.,
  `Vec<Rocket>`).

```rust
use surrealdb_orm::{Serialize, Deserialize,SurrealSimpleId, SurrealdbNode, SurrealdbObject};

#[derive(SurrealdbNode, Serialize, Deserialize, Debug)]
#[surrealdb(table_name = "alien")]
pub struct Alien {
    pub id: SurrealSimpleId<Self>,

    #[surrealdb(nest_array = "Rocket")]
    pub big_rockets: Vec<Rocket>,
}

#[derive(SurrealdbNode, Serialize, Deserialize, Debug)]
#[surrealdb(table_name = "rocket")]
pub struct Rocket {
}
```

- `Relate`: It is an attribute used to define a read-only relationship between
  two Nodes. In the `Alien` struct, we have the `planets_to_visit` field:

```rust
use surrealdb_orm::{Serialize, Deserialize, SurrealSimpleId, SurrealdbNode, SurrealdbEdge, Relate};

#[derive(SurrealdbNode, Serialize, Deserialize, Debug)]
#[surrealdb(table_name = "alien")]
pub struct Alien {
    pub id: SurrealSimpleId<Self>,

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

This attribute specifies that an `Alien` has a read-only relationship with
`Planet` through the `AlienVisitsPlanet` model. The `connection` attribute
describes the relationship path between the Nodes. The relationship is read-only
because the `serde(skip_serializing)` attribute is used to prevent it from being
serialized.

These attributes provide additional information to Surrealdb for modeling
relationships and embedding Objects within Nodes, allowing for more complex and
flexible database schema designs.

---

```rust
use surrealdb_orm::{LinkMany, LinkOne, LinkSelf, NestArray, NestOne, Relate, SurrealSimpleId, SurrealdbNode};

// Alien
#[derive(SurrealdbNode, Serialize, Deserialize, Debug)]
#[surrealdb(table_name = "alien")]
pub struct Alien {
    pub id: SurrealSimpleId<Self>,
    pub name: String,
    pub weapon: LinkOne<Weapon>,
    pub space_ships: LinkMany<SpaceShip>,
    pub allies: LinkMany<Alien>,
    pub best_friend: LinkSelf<Alien>,
    pub rockets: NestMany<Rocket>,
    pub favorite_planet: NestOne<Planet>,
    pub visited_planets: NestArray<Planet>,
    pub enemy_planets: Relate<Planet>,
}

// Weapon
#[derive(SurrealdbNode, Serialize, Deserialize, Debug)]
#[surrealdb(table_name = "weapon")]
pub struct Weapon {
    pub id: SurrealSimpleId<Self>,
    pub name: String,
    pub strength: u64,
}

// SpaceShip
#[derive(SurrealdbNode, Serialize, Deserialize, Debug)]
#[surrealdb(table_name = "space_ship")]
pub struct SpaceShip {
    pub id: SurrealSimpleId<Self>,
    pub name: String,
}

// Rocket
#[derive(SurrealdbNode, Serialize, Deserialize, Debug)]
#[surrealdb(table_name = "rocket")]
pub struct Rocket {
    pub id: SurrealSimpleId<Self>,
    pub name: String,
    pub fuel_capacity: u64,
}

// Planet
#[derive(SurrealdbNode, Serialize, Deserialize, Debug)]
#[surrealdb(table_name = "planet")]
pub struct Planet {
    pub id: SurrealSimpleId<Self>,
    pub name: String,
    pub population: u64,
}
```
