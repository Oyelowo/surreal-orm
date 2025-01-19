# Loaders

Loaders in the Surreal ORM are functions that fetch different kinds of related
records (links) from the database. These loaders provide different ways of
handling these related records, based on their type and their existence in the
database. Here, we discuss some of the "load" types that are part of the
`ReturnableStandard` trait.

## `load_links`

The `load_links` function sets the return type to projections and fetches all
record links. It defaults values to null for referenced records that do not
exist.

For instance, if you have a `User` model that has a `Posts` link (i.e., each
User can have multiple Posts), you can use `load_links` to fetch all the `Posts`
linked to a `User`. If a `Post` does not exist, the function defaults its value
to null.

```rust
let user = User::find(1).load_links(vec!["posts"]).unwrap();
```

```rust
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    #[derive(Node, Serialize, Deserialize, Debug, Clone)]
    #[serde(rename_all = "camelCase")]
    #[orm(table = "alien")]
    pub struct Alien {
        pub id: SurrealSimpleId<Self>,

        #[orm(link_self = "Alien")]
        pub ally: LinkSelf<Alien>,

        #[orm(link_one = "Weapon")]
        pub weapon: LinkOne<Weapon>,

        // Again, we dont have to provide the type attribute, it can auto detect
        #[orm(link_many = "SpaceShip")]
        pub space_ships: LinkMany<SpaceShip>,

        // This is a read only field
        #[orm(relate(model = "AlienVisitsPlanet", connection = "->visits->planet"))]
        #[serde(skip_serializing, default)]
        pub planets_to_visit: Relate<Planet>,
    }

    #[derive(Node, Serialize, Deserialize, Debug, Clone, Default)]
    #[serde(rename_all = "camelCase")]
    #[orm(table = "weapon")]
    pub struct Weapon {
        pub id: SurrealSimpleId<Self>,
        pub name: String,
        // pub strength: u64,
        #[orm(type_ = "int")]
        pub strength: Strength,
        pub created: DateTime<Utc>,
        #[orm(nest_object = "Rocket")]
        pub rocket: Rocket,
    }
    type Strength = u64;


    #[derive(Node, Serialize, Deserialize, Debug, Clone)]
    #[serde(rename_all = "camelCase")]
    #[orm(table = "space_ship")]
    pub struct SpaceShip {
        pub id: SurrealId<Self, String>,
        pub name: String,
        pub created: DateTime<Utc>,
    }

    let weapon = || Weapon {
        name: "Laser".to_string(),
        created: Utc::now(),
        ..Default::default()
    };
    let weapon1 = weapon();
    let weapon2 = weapon();

    let space_ship = SpaceShip {
        id: SpaceShip::create_id("gbanda".into()),
        name: "SpaceShip1".to_string(),
        created: Utc::now(),
    };

    let space_ship2 = SpaceShip {
        id: SpaceShip::create_id("halifax".into()),
        name: "SpaceShip2".to_string(),
        created: Utc::now(),
    };

    let space_ship3 = SpaceShip {
        id: SpaceShip::create_id("alberta".into()),
        name: "Oyelowo".to_string(),
        created: Utc::now(),
    };

    assert_eq!(weapon1.clone().id.to_thing().tb, "weapon");

    // create first record to weapon table
    let created_weapon = create()
        .content(weapon1.clone())
        .get_one(db.clone())
        .await?;
    assert_eq!(created_weapon.id.to_thing(), weapon1.id.to_thing());

    let select1: Vec<Weapon> = select(All)
        .from(Weapon::table())
        .return_many(db.clone())
        .await?;
    // weapon table should have one record
    assert_eq!(select1.len(), 1);

    //  Create second record
    let created_weapon = create()
        .content(weapon2.clone())
        .return_one(db.clone())
        .await?;

    let select2: Vec<Weapon> = select(All)
        .from(Weapon::table())
        .return_many(db.clone())
        .await?;
    // weapon table should have two records after second creation
    assert_eq!(select2.len(), 2);

    let created_spaceship1 = create()
        .content(space_ship.clone())
        .get_one(db.clone())
        .await?;
    let created_spaceship2 = create()
        .content(space_ship2.clone())
        .get_one(db.clone())
        .await?;
    let created_spaceship3 = create()
        .content(space_ship3.clone())
        .get_one(db.clone())
        .await?;

    let point = point! {
        x: 40.02f64,
        y: 116.34,
    };

    let territory = line_string![(x: 40.02, y: 116.34), (x: 40.02, y: 116.35), (x: 40.03, y: 116.35), (x: 40.03, y: 116.34), (x: 40.02, y: 116.34)];
    let polygon = polygon![(x: 40.02, y: 116.34), (x: 40.02, y: 116.35), (x: 40.03, y: 116.35), (x: 40.03, y: 116.34), (x: 40.02, y: 116.34)];
    let unsaved_alien = Alien {
        id: Alien::create_simple_id(),
        ally: LinkSelf::null(),
        weapon: LinkOne::from(created_weapon.unwrap()),
        space_ships: LinkMany::from(vec![
            created_spaceship1.clone(),
            created_spaceship2.clone(),
            created_spaceship3.clone(),
        ]),
        planets_to_visit: Relate::null(),
    };

    assert!(unsaved_alien.weapon.get_id().is_some());
    assert!(unsaved_alien.weapon.value().is_none());

    // Check fields value fetching
    let alien::Schema { weapon, .. } = Alien::schema();
    let created_alien = create()
        .content(unsaved_alien.clone())
        .load_links(vec![weapon])?
        .get_one(db.clone())
        .await?;

    let ref created_alien = created_alien.clone();
    // id is none  because ally field is not created.
    assert!(created_alien.ally.get_id().is_none());
    // .value() is None because ally is not created.
    assert!(created_alien.ally.value().is_none());

    // Weapon is created at weapon field and also loaded.
    // get_id  is None because weapon is loaded.
    assert!(created_alien.weapon.get_id().is_none());
    // .value() is Some because weapon is loaded.
    assert!(created_alien.weapon.value().is_some());

    // Spaceships created at weapon field and also loaded.
    assert_eq!(created_alien.space_ships.is_empty(), false);

    assert_eq!(created_alien.space_ships.len(), 3);
    assert_eq!(
        created_alien
            .space_ships
            .iter()
            .map(|x| x.get_id().unwrap().to_string())
            .collect::<Vec<_>>(),
        vec![
            created_spaceship1.id.to_string(),
            created_spaceship2.id.to_string(),
            created_spaceship3.id.to_string(),
        ]
    );



    let created_alien_with_fetched_links = create()
        .content(unsaved_alien.clone())
        .load_link_manys()?
        .return_one(db.clone())
        .await?;

    let ref created_alien_with_fetched_links = created_alien_with_fetched_links.unwrap();
    let alien_spaceships = created_alien_with_fetched_links.space_ships.values();

    assert_eq!(created_alien_with_fetched_links.space_ships.keys().len(), 3);
    assert_eq!(
        created_alien_with_fetched_links
            .space_ships
            .keys_truthy()
            .len(),
        0
    );
```

## `load_all_links`

The `load_all_links` function sets the return type to projections and fetches
all record link values. For `link_one` and `link_self` types, it returns null if
the link is null or if the reference does not exist. For `link_many` type, it
returns `None` for items that are null or the references that do not exist.

Assume you have a `User` model with `link_one` type `Profile`, `link_self` type
`Friends`, and `link_many` type `Posts`. You can use `load_all_links` to fetch
all these linked records.

```rust
let user = User::find(1).load_all_links().unwrap();
```

## `load_link_manys`

The `load_link_manys` function sets the return type to projections and fetches
all record link values for `link_many` fields, including the null record links.
So, if a `User` has multiple `Posts`, this function fetches all `Posts`
including the ones that are null.

```rust
let user = User::find(1).load_link_manys().unwrap();
```

## `load_link_ones`

The `load_link_ones` function sets the return type to projections and fetches
all record link values for `link_one` fields. It defaults to null if the
reference does not exist.

```rust
let user = User::find(1).load_link_ones().unwrap();
```

## `load_link_selfs`

The `load_line_selfs` function sets the return type to projections and fetches
all record link values for `link_self` fields. It defaults to null if the
reference does not exist.

```rust
let user = User::find(1).load_line_selfs().unwrap();
```

In conclusion, loaders provide a flexible way to handle linked records in your
database. Whether you want to fetch all links, fetch links of a specific type,
or handle null references in a certain way, loaders have got you covered. They
are a powerful tool in the Surreal ORM, simplifying complex database operations.
