# Create Statement Documentation

This chapter will cover how to use the `create` statement in our SurrealDB ORM and Query Builder. We'll look at how to create new entries in the database using this statement, as well as various associated features and functionalities.

# CreateStatement

`CreateStatement` allows you to construct a database record creation statement in a fluent style. Here's a high-level overview of its usage:

```rust
create()
   .content(Value) // The record content to be created
   .set(settables) // The values of the fields to be updated
   .return_type(return_types) // The desired return type for the query
   .timeout(seconds) // Timeout for the query execution
   .parallel(); // Indicate if the query should be executed in parallel
```

Here's what each method does:

- `content(Value)`: Sets the content of the record to be created.
- `set(settables)`: Sets the values of the fields to be updated in the record.
- `return_type(return_types)`: Sets the return type for the query.
- `timeout(seconds)`: Sets the timeout duration for the query.
- `parallel()`: Indicates that the query should be executed in parallel.

This fluent style makes your code more readable and easier to maintain.

## Table of Contents

1. [Basic `create` Statement](#basic-create-statement)
2. [Creating Linked Entities](#creating-linked-entities)
3. [`create` with Set Statement](#create-with-set-statement)
4. [Complete Examples](#complete-examples)

### Basic `create` Statement

The `create` statement is primarily used to add new entries into the database. This operation can be done asynchronously. To start a `create` operation, we need to call `create()` method, followed by the `.content()` method that contains the data we want to add:

```rust
let space_ship1 = create()
    .content(space_ship1.clone())
    .get_one(db.clone())
    .await?;
```

This code will create a new entry for `space_ship1` in the database.

### Creating Linked Entities

With the `create` statement, we can also create entries that have links to other entities. In SurrealDB, we have three types of links: `LinkOne`, `LinkMany`, and `LinkSelf`. Here's an example of creating a linked entity:

```rust
let unsaved_alien = Alien {
    ...
    space_ships: LinkMany::from(vec![
        created_spaceship1.clone(),
        created_spaceship2.clone(),
        space_ship3.clone(),
    ]),
    ...
};

let created_alien_with_fetched_links = create()
    .content(unsaved_alien.clone())
    .load_link_manys()? // This loads every `link_many` fields
    .return_one(db.clone())
    .await?;
```

In this example, `unsaved_alien` is being created with links to three different spaceships. We can create the alien and load the linked entities all in one statement using the `.load_link_manys()` method.

### `create` with Set Statement

We can also use the `set` method with the `create` statement, which allows us to set specific fields of the object we're creating. Here's an example:

```rust
let spaceship_schema::SpaceShip {
    id, name, created, ..
} = SpaceShip::schema();

// You can set SpaceShip value using all its fields as a list of setters .
let space_ship1 = create::<SpaceShip>()
    .set(vec![
        id.equal_to(spaceship_id_1),
        name.equal_to("SpaceShip1".to_string()),
        created.equal_to(Utc::now()),
    ])
    .get_one(db.clone())
    .await?;
assert_eq!(space_ship1.name, "SpaceShip1");

// You can also use array const.
let space_ship2 = create::<SpaceShip>()
    .set([
        id.equal_to(spaceship_id_2),
        name.equal_to("SpaceShip2".to_string()),
        created.equal_to(Utc::now()),
    ])
    .get_one(db.clone())
    .await?;
assert_eq!(space_ship2.name, "SpaceShip2");

// You can even chain set methods and it aggregates even if `Vec` is used in any.
let space_ship3 = create::<SpaceShip>()
    .set(id.equal_to(spaceship_id_3))
    .set(name.equal_to("SpaceShip3".to_string()))
    .set(created.equal_to(Utc::now()))
    .get_one(db.clone())
    .await?;
assert_eq!(space_ship2.name, "SpaceShip2");
```

In this example, we're creating a new `SpaceShip` and setting its `id`, `name`, and `created` fields directly in the `set` method.

### Complete Examples

Below you can find two complete examples of how to use `create` statement in different scenarios:

- [Example 1: Creating and Fetching Linked Entities](#example-1-creating-and-fetching-linked-entities)
- [Example 2: Using Create with Set Statement](#example-2-using-create-with-set-statement)

#### Example 1: Creating and Fetching Linked Entities

In this example, we're creating a new `Alien` that has links to three `SpaceShip`s, two of which already exist in the database and one that doesn't. We then fetch the created `Alien` from the database and verify the linked `SpaceShip`s.

Here is the code for this example:

```rust
let db = Surreal::new::<Mem>(()).await.unwrap();
db.use_ns("test").use_db("test").await.unwrap();

let spaceship_id_1 = SpaceShip::create_id("spaceship1".into());
let spaceship_id_2 = SpaceShip::create_id("spaceship2".into());
let spaceship_id_3 = SpaceShip::create_id("spaceship3".into());

let space_ship1 = SpaceShip {
    id: spaceship_id_1,
    name: "SpaceShip1".to_string(),
    created: Utc::now(),
};

let space_ship2 = SpaceShip {
    id: spaceship_id_2,
    name: "SpaceShip2".to_string(),
    created: Utc::now(),
    ..Default::default()
};

let space_ship3 = SpaceShip {
    id: spaceship_id_3,
    name: "Oyelowo".to_string(),
    created: Utc::now(),
    ..Default::default()
};

let point = point! {
    x: 40.02f64,
    y: 116.34,
};

let created_spaceship1 = create()
    .content(space_ship1.clone())
    .get_one(db.clone())
    .await?;
let created_spaceship2 = create()
    .content(space_ship2.clone())
    .get_one(db.clone())
    .await?;

let territory = line_string![(x: 40.02, y: 116.34), (x: 40.02, y: 116.35), (x: 40.03, y: 116.35), (x: 40.03, y: 116.34), (x: 40.02, y: 116.34)];
let polygon = polygon![(x: 40.02, y: 116.34), (x: 40.02, y: 116.35), (x: 40.03, y: 116.35), (x: 40.03, y: 116.34), (x: 40.02, y: 116.34)];
let unsaved_alien = Alien {
    id: Alien::create_simple_id(),
    name: "Oyelowo".to_string(),
    age: 20,
    created: Utc::now(),
    line_polygon: territory.into(),
    life_expectancy: Duration::from_secs(100),
    territory_area: polygon.into(),
    home: point.into(),
    tags: vec!["tag1".into(), "tag".into()],
    ally: LinkSelf::null(),
    weapon: LinkOne::null(),
    space_ships: LinkMany::from(vec![
        created_spaceship1.clone(),
        created_spaceship2.clone(),
        space_ship3.clone(),
    ]),
    planets_to_visit: Relate::null(),
};

let created_alien_with_fetched_links = create()
    .content(unsaved_alien.clone())
    .load_link_manys()?
    .return_one(db.clone())
    .await?;

let ref created_alien_with_fetched_links = created_alien_with_fetched_links.unwrap();
let ref alien_spaceships = created_alien_with_fetched_links.space_ships;
assert_eq!(alien_spaceships.iter().count(), 3);
assert_eq!(alien_spaceships.values().iter().count(), 3);
assert_eq!(alien_spaceships.values_truthy().iter().count(), 2);
assert_eq!(alien_spaceships.keys().len(), 3);
assert_eq!(alien_spaceships.keys_truthy().len(), 0);

let selected_aliens = read::<Alien>()
    .filter(vec![
        id.equal_to(created_alien_with_fetched_links.id.clone()),
    ])
    .load_link_manys()?
    .return_many(db.clone())
    .await?;

assert_eq!(selected_aliens.len(), 1);

Ok(())
}

#[derive(SurrealdbNode, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[surrealdb(table_name = "space_ship")]
pub struct SpaceShip {
    pub id: SurrealId<Self, String>,
    pub name: String,
    pub created: DateTime<Utc>,
}

impl Default for SpaceShip {
    fn default() -> Self {
        Self {
            id: Self::create_id(sql::Uuid::new_v4().to_string()),
            name: Default::default(),
            created: Default::default(),
        }
    }
}

#[derive(SurrealdbNode, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[surrealdb(table_name = "alien")]
pub struct Alien {
    pub id: SurrealSimpleId<Self>,
    pub name: String,
    pub age: u8,
    pub created: DateTime<Utc>,
    pub life_expectancy: Duration,
    pub line_polygon: sql::Geometry,
    pub territory_area: sql::Geometry,
    pub home: sql::Geometry,
    pub tags: Vec<String>,
    // database type attribute is autogenerated for all links of the struct. But you can also provide it
    #[surrealdb(link_self = "Alien")]
    pub friend: LinkSelf<Alien>,

    #[surrealdb(link_one = "Weapon")]
    pub weapon: LinkOne<Weapon>,

    // Again, we dont have to provide the type attribute, it can auto detect
    #[surrealdb(link_many = "SpaceShip")]
    pub space_ships: LinkMany<SpaceShip>,

    // This is a read only field
    #[surrealdb(relate(model = "AlienVisitsPlanet", connection = "->visits->planet"))]
    #[serde(skip_serializing, default)]
    pub planets_to_visit: Relate<Planet>,
}
```

#### Example 2: Using Create with Set Statement

In this example, we're using `create` with `set` to create a new `SpaceShip` entity. This example will cover the creation of a new `SpaceShip` entity and the setting of its `id`, `name`, and `created` fields using the `set` statement.

```rust
let db = Surreal::new::<Mem>(()).await.unwrap();
db.use_ns("test").use_db("test").await.unwrap();

let spaceship_schema::SpaceShip {
    id, name, created, ..
} = SpaceShip::schema();

let spaceship_id_1 = SpaceShip::create_id("spaceship1".into());

let space_ship1 = create::<SpaceShip>()
    .set([
        id.equal_to(spaceship_id_1),
        name.equal_to("SpaceShip1".to_string()),
        created.equal_to(Utc::now()),
    ])
    .get_one(db.clone())
    .await?;

assert_eq!(space_ship1.unwrap().id, spaceship_id_1);

Ok(())
```

This is just a glimpse of the possibilities with the `create` statement in SurrealDB. You can create and manipulate data in many different ways depending on your application's requirements.

```

```

```

```

```

```

```

```
