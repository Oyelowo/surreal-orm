/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use chrono::{DateTime, Utc};
use geo::line_string;
use geo::point;
use geo::polygon;
use pretty_assertions::assert_eq;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use surreal_models::weapon;
use surreal_models::{alien, space_ship, Alien, SpaceShip, Weapon};
use surreal_orm::statements::insert;
use surreal_orm::statements::order;
use surreal_orm::{statements::select, *};
use surrealdb::engine::local::Mem;
use surrealdb::Surreal;

#[tokio::test]
async fn test_insert_alien_with_id_not_specified_but_generated_by_the_database(
) -> SurrealOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let space_ship = SpaceShip {
        name: "SpaceShip1".to_string(),
        created: Utc::now(),
        ..Default::default()
    };
    assert_eq!(space_ship.id.to_string().starts_with("space_ship"), true);

    let created_ship = insert(space_ship.clone()).get_one(db.clone()).await?;

    // Id is generated after creation
    assert_eq!(
        created_ship
            .clone()
            .id
            .to_string()
            .starts_with("space_ship"),
        true
    );
    assert!(created_ship
        .clone()
        .id
        .to_string()
        .starts_with("space_ship:"));
    assert_eq!(created_ship.clone().name, "SpaceShip1");
    assert_eq!(created_ship.clone().created, space_ship.created);
    Ok(())
}

#[tokio::test]
async fn test_insert_alien_with_id_specified() -> SurrealOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let space_ship = SpaceShip {
        id: SpaceShip::create_id("oyelowo".into()),
        name: "SpaceShip1".to_string(),
        created: Utc::now(),
    };
    // id specified before creation. Will be used by the database.
    assert_eq!(space_ship.id.tb, "space_ship");

    let created_ship = insert(space_ship.clone()).return_one(db.clone()).await?;

    // Id is generated after creation
    assert_eq!(
        created_ship.clone().unwrap().id.to_string(),
        space_ship.id.to_string()
    );
    assert_eq!(created_ship.clone().unwrap().id.tb, "space_ship");
    assert_eq!(
        created_ship.clone().unwrap().id.to_string(),
        "space_ship:oyelowo"
    );
    assert_eq!(created_ship.clone().unwrap().name, "SpaceShip1");
    assert_eq!(created_ship.clone().unwrap().created, space_ship.created);
    Ok(())
}

#[tokio::test]
async fn test_insert_alien_with_id_specified_as_uuid() -> SurrealOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let space_ship = SpaceShip {
        name: "SpaceShip1".to_string(),
        created: Utc::now(),
        ..Default::default()
    };
    // id specified before creation. Will be used by the database.
    assert_eq!(space_ship.id.to_thing().tb, "space_ship");

    let created_ship = insert(space_ship.clone()).get_one(db.clone()).await?;

    // Id is generated after creation
    assert_eq!(created_ship.clone().id.to_thing().tb, "space_ship");
    assert!(created_ship
        .clone()
        .id
        .to_string()
        .starts_with("space_ship:"));
    assert_eq!(created_ship.clone().id.to_string().len(), 55);
    assert_eq!(created_ship.clone().name, "SpaceShip1");
    assert_eq!(created_ship.clone().created, space_ship.created);
    Ok(())
}

#[tokio::test]
async fn test_insert_with_returning_selected_fields() -> SurrealOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let space_ship = SpaceShip {
        name: "SpaceShipCode".to_string(),
        created: Utc::now(),
        ..Default::default()
    };
    // id specified before creation. Will be used by the database.
    assert_eq!(space_ship.id.to_string().starts_with("space_ship:"), true);
    let space_ship::Schema { name, .. } = SpaceShip::schema();

    #[derive(Serialize, Deserialize, Debug, Clone, Default)]
    struct ReturnedSpaceShip {
        name: String,
    }
    // Return only specified fields
    let created_ship = insert(space_ship.clone())
        .return_one_projections::<ReturnedSpaceShip>(db.clone(), arr![name])
        .await?;

    assert_eq!(created_ship.clone().unwrap().name, "SpaceShipCode");
    Ok(())
}

#[tokio::test]
async fn test_insert_alien_with_links() -> SurrealOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

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
        name: "SpaceShip2".to_string(),
        created: Utc::now(),
        ..Default::default()
    };

    let space_ship3 = SpaceShip {
        name: "Oyelowo".to_string(),
        created: Utc::now(),
        ..Default::default()
    };

    /////
    assert_eq!(weapon1.id.to_string().starts_with("weapon"), true);

    // create first record to weapon table
    let created_weapon = insert(weapon1.clone()).get_one(db.clone()).await?;
    // Id should be same as the default generated outside the database
    assert_eq!(created_weapon.id.to_string(), weapon1.id.to_string());

    let select1: Vec<Weapon> = select(All)
        .from(Weapon::table_name())
        .return_many(db.clone())
        .await?;
    // weapon table should have one record
    assert_eq!(select1.len(), 1);

    //  Create second record
    let created_weapon = insert(weapon2).return_one(db.clone()).await?;

    let select2: Vec<Weapon> = select(All)
        .from(Weapon::table_name())
        .return_many(db.clone())
        .await?;
    // weapon table should have two records after second creation
    assert_eq!(select2.len(), 2);

    let created_spaceship1 = insert(space_ship.clone()).return_one(db.clone()).await?;
    let created_spaceship2 = insert(space_ship2.clone()).return_one(db.clone()).await?;
    let created_spaceship3 = insert(space_ship3.clone()).return_one(db.clone()).await?;

    let point = point! {
        x: 40.02f64,
        y: 116.34,
    };

    let territory = line_string![(x: 40.02, y: 116.34), (x: 40.02, y: 116.35), (x: 40.03, y: 116.35), (x: 40.03, y: 116.34), (x: 40.02, y: 116.34)];
    let polygon = polygon![(x: 40.02, y: 116.34), (x: 40.02, y: 116.35), (x: 40.03, y: 116.35), (x: 40.03, y: 116.34), (x: 40.02, y: 116.34)];
    let unsaved_alien = Alien {
        id: Alien::create_simple_id(),
        name: "Oyelowo".to_string(),
        age: 20,
        created: Utc::now(),
        line_polygon: territory,
        life_expectancy: Duration::from_secs(100),
        territory_area: polygon,
        home: point,
        tags: vec!["tag1".into(), "tag2".into()],
        ally: LinkSelf::null(),
        weapon: LinkOne::from(created_weapon.unwrap()),
        space_ships: LinkMany::from(vec![
            created_spaceship1.clone().unwrap(),
            created_spaceship2.clone().unwrap(),
            created_spaceship3.clone().unwrap(),
        ]),
        planets_to_visit: Relate::null(),
    };

    assert!(unsaved_alien.weapon.get_id().is_some());
    assert!(unsaved_alien.weapon.value().is_none());

    // Check fields value fetching
    let weapon = Alien::schema().weapon;
    let created_alien = insert(unsaved_alien.clone())
        .load_links(vec![weapon])?
        .get_one(db.clone())
        .await?;

    let created_alien = &created_alien.clone();
    // id is none  because ally field is not created.
    assert!(created_alien.ally.get_id().is_none());
    // .value() is None because ally is not created.
    assert!(created_alien.ally.value().is_none());
    //
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
            created_spaceship1.unwrap().id.to_string(),
            created_spaceship2.unwrap().id.to_string(),
            created_spaceship3.unwrap().id.to_string(),
        ]
    );
    assert_eq!(created_alien.space_ships.values().len(), 3);
    assert_eq!(
        created_alien
            .space_ships
            .iter()
            .map(|x| x.clone().value().is_some())
            .collect::<Vec<_>>(),
        vec![false, false, false,]
    );

    assert_eq!(created_alien.age, 20);
    assert_eq!(unsaved_alien.id.to_string(), created_alien.id.to_string());

    assert_eq!(
        sql::Geometry::from(created_alien.clone().line_polygon).to_string(),
        "{ type: 'LineString', coordinates: [[40.02, 116.34], [40.02, 116.35], \
            [40.03, 116.35], [40.03, 116.34], [40.02, 116.34]] }"
    );
    assert_eq!(created_alien.name, "Oyelowo");

    Ok(())
}

#[tokio::test]
async fn test_create_fetch_record_links() -> SurrealOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let space_ship = SpaceShip {
        id: SpaceShip::create_id("gbanda".into()),
        name: "SpaceShip1".to_string(),
        created: Utc::now(),
    };

    let space_ship2 = SpaceShip {
        name: "SpaceShip2".to_string(),
        created: Utc::now(),
        ..Default::default()
    };

    let space_ship3 = SpaceShip {
        name: "Oyelowo".to_string(),
        created: Utc::now(),
        ..Default::default()
    };

    let created_spaceship1 = insert(space_ship.clone()).return_one(db.clone()).await?;
    let created_spaceship2 = insert(space_ship2.clone()).return_one(db.clone()).await?;
    let created_spaceship3 = insert(space_ship3.clone()).return_one(db.clone()).await?;

    let point = point! {
        x: 40.02f64,
        y: 116.34,
    };

    let territory = line_string![(x: 40.02, y: 116.34), (x: 40.02, y: 116.35), (x: 40.03, y: 116.35), (x: 40.03, y: 116.34), (x: 40.02, y: 116.34)];
    let polygon = polygon![(x: 40.02, y: 116.34), (x: 40.02, y: 116.35), (x: 40.03, y: 116.35), (x: 40.03, y: 116.34), (x: 40.02, y: 116.34)];
    let unsaved_alien = Alien {
        id: Alien::create_simple_id(),
        name: "Oyelowo".to_string(),
        age: 20,
        created: Utc::now(),
        line_polygon: territory,
        life_expectancy: Duration::from_secs(100),
        territory_area: polygon,
        home: point,
        tags: vec!["tag1".into(), "tag2".into()],
        ally: LinkSelf::null(),
        weapon: LinkOne::null(),
        space_ships: LinkMany::from(vec![
            created_spaceship1.clone().unwrap(),
            created_spaceship2.clone().unwrap(),
            created_spaceship3.clone().unwrap(),
        ]),
        planets_to_visit: Relate::null(),
    };

    // Check fields value fetching
    let alien_schema = Alien::schema();
    let age = Alien::schema().age;
    let name = Alien::schema().name;

    // We specify the exact fields we want from the returned projections(second argument).
    #[derive(Serialize, Deserialize, Clone)]
    struct SpaceShipName {
        age: u8,
        name: String,
        // Alias the retrieved names from foreign tables space_ships returned as array of strings
        aliens_spaceships_names_alias: Vec<String>,
    }

    let aliens_spaceships_names_alias = alien_schema
        .spaceShips(All)
        .name
        .__as__("aliens_spaceships_names_alias");
    assert_eq!(
        aliens_spaceships_names_alias.build(),
        "spaceShips[*].name AS aliens_spaceships_names_alias"
    );

    let space_ship_names: Option<SpaceShipName> = insert(unsaved_alien.clone())
        .return_one_projections(db.clone(), arr![age, name, aliens_spaceships_names_alias])
        .await?;

    let space_ship_names = &space_ship_names.unwrap();
    assert_eq!(space_ship_names.age, 20);
    assert_eq!(space_ship_names.name, "Oyelowo");
    assert_eq!(space_ship_names.aliens_spaceships_names_alias.len(), 3);
    assert_eq!(
        space_ship_names.aliens_spaceships_names_alias,
        vec!["SpaceShip1", "SpaceShip2", "Oyelowo"]
    );

    Ok(())
}

#[tokio::test]
async fn test_create_fetch_values_of_one_to_many_record_links() -> SurrealOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let space_ship = SpaceShip {
        id: SpaceShip::create_id("gbanda".into()),
        name: "SpaceShip1".to_string(),
        created: Utc::now(),
    };

    let space_ship2 = SpaceShip {
        name: "SpaceShip2".to_string(),
        created: Utc::now(),
        ..Default::default()
    };

    let space_ship3 = SpaceShip {
        name: "Oyelowo".to_string(),
        created: Utc::now(),
        ..Default::default()
    };

    let created_spaceship1 = insert(space_ship.clone()).return_one(db.clone()).await?;
    let created_spaceship2 = insert(space_ship2.clone()).return_one(db.clone()).await?;
    let created_spaceship3 = insert(space_ship3.clone()).return_one(db.clone()).await?;

    let point = point! {
        x: 40.02f64,
        y: 116.34,
    };

    let territory = line_string![(x: 40.02, y: 116.34), (x: 40.02, y: 116.35), (x: 40.03, y: 116.35), (x: 40.03, y: 116.34), (x: 40.02, y: 116.34)];
    let polygon = polygon![(x: 40.02, y: 116.34), (x: 40.02, y: 116.35), (x: 40.03, y: 116.35), (x: 40.03, y: 116.34), (x: 40.02, y: 116.34)];
    let unsaved_alien = Alien {
        id: Alien::create_simple_id(),
        name: "Oyelowo".to_string(),
        age: 20,
        created: Utc::now(),
        line_polygon: territory,
        life_expectancy: Duration::from_secs(100),
        territory_area: polygon,
        home: point,
        tags: vec!["tag1".into(), "tag".into()],
        ally: LinkSelf::null(),
        weapon: LinkOne::null(),
        space_ships: LinkMany::from(vec![
            created_spaceship1.clone().unwrap(),
            created_spaceship2.clone().unwrap(),
            created_spaceship3.clone().unwrap(),
        ]),
        planets_to_visit: Relate::null(),
    };

    let alien::Schema { spaceShips, .. } = Alien::schema();

    let created_alien_with_fetched_links = insert(unsaved_alien.clone())
        .load_links(vec![spaceShips])?
        .return_one(db.clone())
        .await?;

    let created_alien_with_fetched_links = &created_alien_with_fetched_links.unwrap();
    let alien_spaceships = created_alien_with_fetched_links.space_ships.values();

    assert_eq!(created_alien_with_fetched_links.space_ships.keys().len(), 3);
    assert_eq!(
        created_alien_with_fetched_links
            .space_ships
            .keys_truthy()
            .len(),
        3
    );
    assert_eq!(created_alien_with_fetched_links.age, 20);
    assert_eq!(created_alien_with_fetched_links.name, "Oyelowo");
    assert_eq!(created_alien_with_fetched_links.space_ships.len(), 3);
    assert_eq!(alien_spaceships[0].unwrap().name, "SpaceShip1");
    assert_eq!(alien_spaceships[1].unwrap().name, "SpaceShip2");
    assert_eq!(alien_spaceships[2].unwrap().name, "Oyelowo");

    Ok(())
}

#[tokio::test]
async fn test_create_fetch_values_of_one_to_many_record_links_with_alias() -> SurrealOrmResult<()> {
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
    };

    let space_ship3 = SpaceShip {
        id: spaceship_id_3,
        name: "Oyelowo".to_string(),
        created: Utc::now(),
    };

    let point = point! {
        x: 40.02f64,
        y: 116.34,
    };

    let territory = line_string![(x: 40.02, y: 116.34), (x: 40.02, y: 116.35), (x: 40.03, y: 116.35), (x: 40.03, y: 116.34), (x: 40.02, y: 116.34)];
    let polygon = polygon![(x: 40.02, y: 116.34), (x: 40.02, y: 116.35), (x: 40.03, y: 116.35), (x: 40.03, y: 116.34), (x: 40.02, y: 116.34)];
    let unsaved_alien = Alien {
        id: Alien::create_simple_id(),
        name: "Oyelowo".to_string(),
        age: 20,
        created: Utc::now(),
        line_polygon: territory,
        life_expectancy: Duration::from_secs(100),
        territory_area: polygon,
        home: point,
        tags: vec!["tag1".into(), "tag".into()],
        ally: LinkSelf::null(),
        weapon: LinkOne::null(),
        space_ships: LinkMany::from(vec![
            space_ship1.clone(),
            space_ship2.clone(),
            space_ship3.clone(),
        ]),
        planets_to_visit: Relate::null(),
    };

    let created_alien_with_fetched_links = insert(unsaved_alien.clone())
        .load_link_manys()?
        .return_one(db.clone())
        .await?;

    let created_alien_with_fetched_links = &created_alien_with_fetched_links.unwrap();
    let alien_spaceships = &created_alien_with_fetched_links.space_ships;
    // Reference ids exist, but we tried to fetch the keys before they were created.
    // so, now we dont have either the keys nor the values since the values don't yet exist.
    assert_eq!(alien_spaceships.keys_truthy().len(), 0);
    assert_eq!(alien_spaceships.keys().len(), 3);
    assert_eq!(alien_spaceships.values().len(), 3);
    assert_eq!(alien_spaceships.values_truthy().len(), 0);
    assert_eq!(
        alien_spaceships
            .values()
            .into_iter()
            .filter(Option::is_some)
            .collect::<Vec<_>>()
            .len(),
        0
    );
    assert_eq!(alien_spaceships.keys().len(), 3);
    assert_eq!(alien_spaceships.keys_truthy().len(), 0);

    // the are now created, so the reference should be valid now.
    insert(space_ship1.clone()).return_one(db.clone()).await?;
    insert(space_ship2.clone()).return_one(db.clone()).await?;
    insert(space_ship3.clone()).return_one(db.clone()).await?;

    let selected_aliens: Option<Alien> = select(All)
        .from(Alien::table_name())
        .return_first(db.clone())
        .await?;
    let selected_aliens_spaceships = &selected_aliens.unwrap().space_ships;
    // The spaceship values not fetched, so, only ids present
    assert_eq!(selected_aliens_spaceships.keys_truthy().len(), 3);
    assert_eq!(alien_spaceships.keys().len(), 3);
    assert_eq!(selected_aliens_spaceships.values_truthy().len(), 0);
    assert_eq!(alien_spaceships.values().len(), 3);
    assert_eq!(
        alien_spaceships
            .values()
            .into_iter()
            .filter(Option::is_some)
            .collect::<Vec<_>>()
            .len(),
        0
    );
    assert_eq!(
        selected_aliens_spaceships
            .keys()
            .into_iter()
            .filter(Option::is_some)
            .collect::<Vec<_>>()
            .len(),
        3
    );
    assert_eq!(selected_aliens_spaceships.keys().len(), 3);
    assert_eq!(selected_aliens_spaceships.keys_truthy().len(), 3);

    let selected_aliens: Option<Alien> = select(arr![All, Alien::schema().spaceShips(All).all()])
        .from(Alien::table_name())
        .fetch(Alien::schema().spaceShips(All).all())
        .return_first(db.clone())
        .await?;

    let selected_aliens_spaceships = &selected_aliens.unwrap().space_ships;
    assert_eq!(selected_aliens_spaceships.values_truthy().len(), 3);
    assert_eq!(selected_aliens_spaceships.values().len(), 3);
    assert_eq!(
        selected_aliens_spaceships
            .values()
            .into_iter()
            .filter(Option::is_some)
            .collect::<Vec<_>>()
            .len(),
        3
    );
    assert_eq!(
        selected_aliens_spaceships
            .keys()
            .into_iter()
            .filter(Option::is_some)
            .collect::<Vec<_>>()
            .len(),
        3
    );
    assert_eq!(selected_aliens_spaceships.keys().len(), 3);
    let selected_aliens_spaceships_values = &selected_aliens_spaceships.values();

    assert_eq!(selected_aliens_spaceships_values.len(), 3);
    assert_eq!(
        selected_aliens_spaceships_values[0].unwrap().name,
        "SpaceShip1"
    );
    assert_eq!(
        selected_aliens_spaceships_values[1].unwrap().name,
        "SpaceShip2"
    );
    assert_eq!(
        selected_aliens_spaceships_values[2].unwrap().name,
        "Oyelowo"
    );
    Ok(())
}

#[tokio::test]
async fn test_alien_build_output() -> SurrealOrmResult<()> {
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
    };

    let space_ship3 = SpaceShip {
        id: spaceship_id_3,
        name: "Oyelowo".to_string(),
        created: Utc::now(),
    };

    let created_spaceship1 = insert(space_ship1.clone()).get_one(db.clone()).await?;
    let created_spaceship2 = insert(space_ship2.clone()).get_one(db.clone()).await?;
    let created_spaceship3 = insert(space_ship3.clone()).get_one(db.clone()).await?;
    let point = point! {
        x: 40.02f64,
        y: 116.34,
    };

    let territory = line_string![(x: 40.02, y: 116.34), (x: 40.02, y: 116.35), (x: 40.03, y: 116.35), (x: 40.03, y: 116.34), (x: 40.02, y: 116.34)];
    let polygon = polygon![(x: 40.02, y: 116.34), (x: 40.02, y: 116.35), (x: 40.03, y: 116.35), (x: 40.03, y: 116.34), (x: 40.02, y: 116.34)];
    let unsaved_alien = Alien {
        id: Alien::create_simple_id(),
        name: "Oyelowo".to_string(),
        age: 20,
        created: DateTime::parse_from_rfc3339("2020-01-01T00:00:00Z")
            .unwrap()
            .with_timezone(&Utc),
        line_polygon: territory,
        life_expectancy: Duration::from_secs(100),
        territory_area: polygon,
        home: point,
        tags: vec!["tag1".into(), "tag".into()],
        ally: LinkSelf::null(),
        weapon: LinkOne::null(),
        space_ships: LinkMany::from(vec![
            created_spaceship1,
            created_spaceship2,
            created_spaceship3,
        ]),
        planets_to_visit: Relate::null(),
    };

    let build = insert(unsaved_alien);

    assert_eq!(build.get_bindings().len(), 12);
    insta::assert_display_snapshot!(build.get_bindings()[0].get_raw_value());
    assert_eq!(
        build.fine_tune_params(),
        "INSERT INTO alien (age, ally, created, home, id, lifeExpectancy, linePolygon, \
            name, spaceShips, tags, territoryArea, weapon) VALUES \
            ($_param_00000001, $_param_00000002, $_param_00000003, $_param_00000004, \
            $_param_00000005, $_param_00000006, $_param_00000007, $_param_00000008, $_param_00000009, \
            $_param_00000010, $_param_00000011, $_param_00000012);"
    );
    insta::assert_display_snapshot!(build.fine_tune_params());
    insta::assert_display_snapshot!(build.to_raw().build().len());
    Ok(())
}

#[tokio::test]
async fn test_access_array_record_links_with_some_null_links() -> SurrealOrmResult<()> {
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
    };

    let space_ship3 = SpaceShip {
        id: spaceship_id_3,
        name: "Oyelowo".to_string(),
        created: Utc::now(),
    };

    let point = point! {
        x: 40.02f64,
        y: 116.34,
    };

    // only 2 have been created. the third one is not, so when we reference it,
    // it will point to a table that does not exist
    let created_spaceship1 = insert(space_ship1.clone()).return_one(db.clone()).await?;
    let created_spaceship2 = insert(space_ship2.clone()).return_one(db.clone()).await?;
    // let created_spaceship3 = create(space_ship3.clone()).return_one(db.clone()).await?;

    let territory = line_string![(x: 40.02, y: 116.34), (x: 40.02, y: 116.35), (x: 40.03, y: 116.35), (x: 40.03, y: 116.34), (x: 40.02, y: 116.34)];
    let polygon = polygon![(x: 40.02, y: 116.34), (x: 40.02, y: 116.35), (x: 40.03, y: 116.35), (x: 40.03, y: 116.34), (x: 40.02, y: 116.34)];
    let unsaved_alien = Alien {
        id: Alien::create_simple_id(),
        name: "Oyelowo".to_string(),
        age: 20,
        created: Utc::now(),
        line_polygon: territory,
        life_expectancy: Duration::from_secs(100),
        territory_area: polygon,
        home: point,
        tags: vec!["tag1".into(), "tag".into()],
        ally: LinkSelf::null(),
        weapon: LinkOne::null(),
        space_ships: LinkMany::from(vec![
            created_spaceship1.unwrap().clone(),
            created_spaceship2.unwrap().clone(),
            space_ship3.clone(),
        ]),
        planets_to_visit: Relate::null(),
    };

    let created_alien_with_fetched_links = insert(unsaved_alien.clone())
        .load_all_links()?
        .return_one(db.clone())
        .await?;

    let created_alien_with_fetched_links = &created_alien_with_fetched_links.unwrap();
    // Has not yet been saved.
    let alien_spaceships = &created_alien_with_fetched_links.space_ships;
    assert_eq!(alien_spaceships.iter().count(), 3);
    assert_eq!(alien_spaceships.values_truthy().len(), 2);
    assert_eq!(alien_spaceships.keys_truthy().len(), 2);
    assert_eq!(alien_spaceships.keys_checked().len(), 2);
    assert!(!alien_spaceships.keys_truthy().is_empty());

    let selected_aliens: Option<Alien> = select(All)
        .from(Alien::table_name())
        .return_first(db.clone())
        .await?;

    let selected_aliens_spaceships = &selected_aliens.unwrap().space_ships;
    assert_eq!(selected_aliens_spaceships.values().len(), 3);
    assert_eq!(selected_aliens_spaceships.values_truthy().len(), 0);
    assert_eq!(selected_aliens_spaceships.values_truthy_count(), 0);
    assert_eq!(selected_aliens_spaceships.keys().len(), 3);
    assert_eq!(selected_aliens_spaceships.keys_truthy().len(), 3);

    // We are fetching all fields and all values in space_ships array.
    let selected_aliens: Option<Alien> = select(arr![All, Alien::schema().spaceShips(All).all()])
        .from(Alien::table_name())
        .return_first(db.clone())
        .await?;
    let selected_aliens_spaceships = &selected_aliens.unwrap().space_ships;
    assert_eq!(selected_aliens_spaceships.values().len(), 3);
    assert_eq!(selected_aliens_spaceships.values_truthy().len(), 2);
    assert_eq!(selected_aliens_spaceships.values_truthy_count(), 2);
    // Empty keys because we values have being fetched leaving [None, None, None]
    assert_eq!(selected_aliens_spaceships.keys().len(), 3);
    assert!(selected_aliens_spaceships.keys()[0].is_some());
    assert!(selected_aliens_spaceships.keys()[1].is_some());
    assert!(selected_aliens_spaceships.keys()[2].is_none());
    // Nones have been filtered out.
    assert_eq!(selected_aliens_spaceships.keys_truthy().len(), 2);
    assert_eq!(selected_aliens_spaceships.keys_checked().len(), 2);
    let selected_aliens_spaceships_values = &selected_aliens_spaceships.values();

    assert_eq!(selected_aliens_spaceships_values.len(), 3);
    assert_eq!(
        selected_aliens_spaceships_values[0].unwrap().name,
        "SpaceShip1"
    );
    assert_eq!(
        selected_aliens_spaceships_values[1].unwrap().name,
        "SpaceShip2"
    );
    assert!(selected_aliens_spaceships_values[2].is_none());

    let selected_aliens_spaceships_values = &selected_aliens_spaceships.values_truthy();
    assert_eq!(selected_aliens_spaceships_values[0].name, "SpaceShip1");
    assert_eq!(selected_aliens_spaceships_values[1].name, "SpaceShip2");
    Ok(())
}

#[tokio::test]
async fn test_return_non_null_links() -> SurrealOrmResult<()> {
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
    };

    let space_ship3 = SpaceShip {
        id: spaceship_id_3,
        name: "Oyelowo".to_string(),
        created: Utc::now(),
    };

    let point = point! {
        x: 40.02f64,
        y: 116.34,
    };

    // only 2 have been created. the third one is not, so when we reference it,
    // it will point to a table that does not exist
    let created_spaceship1 = insert(space_ship1.clone()).return_one(db.clone()).await?;
    let created_spaceship2 = insert(space_ship2.clone()).return_one(db.clone()).await?;

    let territory = line_string![(x: 40.02, y: 116.34), (x: 40.02, y: 116.35), (x: 40.03, y: 116.35), (x: 40.03, y: 116.34), (x: 40.02, y: 116.34)];
    let polygon = polygon![(x: 40.02, y: 116.34), (x: 40.02, y: 116.35), (x: 40.03, y: 116.35), (x: 40.03, y: 116.34), (x: 40.02, y: 116.34)];
    let unsaved_alien = Alien {
        id: Alien::create_simple_id(),
        name: "Oyelowo".to_string(),
        age: 20,
        created: Utc::now(),
        line_polygon: territory,
        life_expectancy: Duration::from_secs(100),
        territory_area: polygon,
        home: point,
        tags: vec!["tag1".into(), "tag".into()],
        ally: LinkSelf::null(),
        weapon: LinkOne::null(),
        space_ships: LinkMany::from(vec![
            created_spaceship1.unwrap().clone(),
            created_spaceship2.unwrap().clone(),
            space_ship3.clone(),
        ]),
        planets_to_visit: Relate::null(),
    };

    // Non-null links filter out null links
    let created_alien_with_fetched_links = insert(unsaved_alien.clone())
        .load_link_manys()?
        .return_one(db.clone())
        .await?;

    let created_alien_with_fetched_links = &created_alien_with_fetched_links.unwrap();
    // Has not yet been saved.
    let alien_spaceships = &created_alien_with_fetched_links.space_ships;
    assert_eq!(alien_spaceships.iter().count(), 3);
    // Two present values and one null
    assert_eq!(alien_spaceships.values().len(), 3);
    // Two present values
    assert_eq!(alien_spaceships.values_truthy().len(), 2);
    // array of 3 none keys
    assert_eq!(alien_spaceships.keys().len(), 3);
    // no valid keys
    assert_eq!(alien_spaceships.keys_truthy().len(), 2);

    let selected_aliens: Option<Alien> = select(All)
        .from(Alien::table_name())
        .fetch(Alien::schema().spaceShips)
        .return_first(db.clone())
        .await?;
    let selected_aliens_spaceships = &selected_aliens.unwrap().space_ships;
    assert_eq!(selected_aliens_spaceships.values().len(), 3);
    assert_eq!(selected_aliens_spaceships.values_truthy().len(), 2);
    assert_eq!(selected_aliens_spaceships.keys().len(), 3);
    assert_eq!(
        selected_aliens_spaceships
            .keys()
            .into_iter()
            .filter(Option::is_none)
            .collect::<Vec<_>>()
            .len(),
        1
    );
    assert_eq!(
        selected_aliens_spaceships
            .keys()
            .into_iter()
            .filter(Option::is_some)
            .collect::<Vec<_>>()
            .len(),
        2
    );
    assert_eq!(selected_aliens_spaceships.keys_truthy().len(), 2);

    let selected_aliens: Option<Alien> = select(All)
        .from(Alien::table_name())
        .fetch(Alien::schema().spaceShips)
        .return_first(db.clone())
        .await?;
    let selected_aliens_spaceships = &selected_aliens.unwrap().space_ships;
    assert_eq!(selected_aliens_spaceships.values().len(), 3);
    assert_eq!(selected_aliens_spaceships.values_truthy().len(), 2);
    assert_eq!(selected_aliens_spaceships.keys().len(), 3);
    assert_eq!(selected_aliens_spaceships.keys_truthy().len(), 2);

    let selected_aliens_spaceships_values = &selected_aliens_spaceships.values_truthy();
    assert_eq!(selected_aliens_spaceships_values.len(), 2);
    assert_eq!(selected_aliens_spaceships_values[0].name, "SpaceShip1");
    assert_eq!(selected_aliens_spaceships_values[1].name, "SpaceShip2");

    // You can also achieve the same result by using the `select` method
    // with the `All` argument i.e "spaceShips(All).all() which is equivalent to
    // `spaceShips[*].*` in the raw query
    let selected_aliens: Option<Alien> = select(arr![All, Alien::schema().spaceShips(All).all()])
        .from(Alien::table_name())
        .fetch(Alien::schema().spaceShips)
        .return_first(db.clone())
        .await?;
    let selected_aliens_spaceships = &selected_aliens.unwrap().space_ships;
    assert_eq!(selected_aliens_spaceships.values().len(), 3);
    assert_eq!(selected_aliens_spaceships.values_truthy().len(), 2);
    assert_eq!(selected_aliens_spaceships.keys().len(), 3);
    assert_eq!(selected_aliens_spaceships.keys_truthy().len(), 2);

    let selected_aliens_spaceships_values = &selected_aliens_spaceships.values_truthy();
    assert_eq!(selected_aliens_spaceships_values.len(), 2);
    assert_eq!(selected_aliens_spaceships_values[0].name, "SpaceShip1");
    assert_eq!(selected_aliens_spaceships_values[1].name, "SpaceShip2");
    Ok(())
}

#[tokio::test]
async fn test_insert_multiple_nodes_return_non_null_links() -> SurrealOrmResult<()> {
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
    };

    let space_ship3 = SpaceShip {
        id: spaceship_id_3,
        name: "Oyelowo".to_string(),
        created: Utc::now(),
    };

    let point = point! {
        x: 40.02f64,
        y: 116.34,
    };

    // only 2 have been created. the third one is not, so when we reference it,
    // it will point to a table that does not exist
    let created_spaceship1 = insert(space_ship1.clone()).get_one(db.clone()).await?;
    let created_spaceship2 = insert(space_ship2.clone()).get_one(db.clone()).await?;

    let territory = line_string![(x: 40.02, y: 116.34), (x: 40.02, y: 116.35), (x: 40.03, y: 116.35), (x: 40.03, y: 116.34), (x: 40.02, y: 116.34)];
    let polygon = polygon![(x: 40.02, y: 116.34), (x: 40.02, y: 116.35), (x: 40.03, y: 116.35), (x: 40.03, y: 116.34), (x: 40.02, y: 116.34)];
    let unsaved_alien1 = Alien {
        id: Alien::create_simple_id(),
        name: "Oyelowo".to_string(),
        age: 20,
        created: Utc::now(),
        line_polygon: territory.clone(),
        life_expectancy: Duration::from_secs(100),
        territory_area: polygon.clone(),
        home: point,
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

    let unsaved_alien2 = Alien {
        id: Alien::create_simple_id(),
        name: "Oyedayo".to_string(),
        age: 109,
        created: Utc::now(),
        line_polygon: territory,
        life_expectancy: Duration::from_secs(43),
        territory_area: polygon,
        home: point,
        tags: vec!["alien2_tag1".into(), "alien2_tag".into()],
        ally: LinkSelf::null(),
        weapon: LinkOne::null(),
        space_ships: LinkMany::from(vec![created_spaceship1.clone(), space_ship3.clone()]),
        planets_to_visit: Relate::null(),
    };

    let alien::Schema {
        spaceShips,
        age,
        created,
        ..
    } = Alien::schema();

    // We are trying to fetch fields that are not linked fields type. Catch the error
    let created_alien_with_fetched_links =
        insert(vec![unsaved_alien1.clone(), unsaved_alien2.clone()])
            .load_links(arr![spaceShips, age, created]);

    assert!(created_alien_with_fetched_links.is_err());

    assert_eq!(
        created_alien_with_fetched_links.unwrap_err().to_string(),
        "The following fields could not be fetched as they are not linked to a \
            foreign table: age, created. Please ensure that all fields provided are of types \
            'link_self', 'link_one' or 'link_many' to allow fetching of linked values from other tables."
    );

    let created_alien_with_fetched_links = insert(vec![unsaved_alien1, unsaved_alien2])
        .load_links(Alien::get_linked_fields())?
        .return_many(db.clone())
        .await?;

    let created_alien_with_fetched_links = &created_alien_with_fetched_links;
    // Has not yet been saved.
    let alien_spaceships = &created_alien_with_fetched_links[0].space_ships;
    assert_eq!(alien_spaceships.iter().count(), 3);
    // Two present values and one null
    assert_eq!(alien_spaceships.values().len(), 3);
    // Two present values
    assert_eq!(alien_spaceships.values_truthy().len(), 2);
    // array of 3 none keys
    assert_eq!(alien_spaceships.keys().len(), 3);
    // no valid keys
    assert_eq!(alien_spaceships.keys_truthy().len(), 2);

    let alien::Schema { created, .. } = Alien::schema();
    let selected_aliens: Option<Alien> = select(All)
        .from(Alien::table_name())
        .fetch(Alien::schema().spaceShips)
        .order_by(order(&created).desc())
        .return_first(db.clone())
        .await?;
    let selected_aliens_spaceships = &selected_aliens.unwrap().space_ships;
    assert_eq!(selected_aliens_spaceships.values().len(), 2);
    assert_eq!(selected_aliens_spaceships.values_truthy().len(), 1);
    assert_eq!(selected_aliens_spaceships.keys().len(), 2);
    assert_eq!(selected_aliens_spaceships.keys_truthy().len(), 1);
    assert_eq!(
        selected_aliens_spaceships
            .keys()
            .into_iter()
            .filter(Option::is_none)
            .collect::<Vec<_>>()
            .len(),
        1
    );
    assert_eq!(
        selected_aliens_spaceships
            .keys()
            .into_iter()
            .filter(Option::is_some)
            .collect::<Vec<_>>()
            .len(),
        1
    );

    let selected_aliens: Option<Alien> = select(arr![All, Alien::schema().spaceShips.all().all()])
        .from(Alien::table_name())
        .order_by(created.asc())
        .return_first(db.clone())
        .await?;
    let selected_aliens_spaceships = &selected_aliens.unwrap().space_ships;
    assert_eq!(selected_aliens_spaceships.values().len(), 3);
    assert_eq!(selected_aliens_spaceships.values_truthy().len(), 2);
    assert_eq!(selected_aliens_spaceships.keys().len(), 3);
    assert_eq!(selected_aliens_spaceships.keys_truthy().len(), 2);

    let selected_aliens_spaceships_values = &selected_aliens_spaceships.values_truthy();
    assert_eq!(selected_aliens_spaceships_values.len(), 2);
    assert_eq!(selected_aliens_spaceships_values[0].name, "SpaceShip1");
    assert_eq!(selected_aliens_spaceships_values[1].name, "SpaceShip2");
    Ok(())
}

#[derive(Node, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "strong_weapon")]
pub struct StrongWeapon {
    pub id: SurrealSimpleId<Self>,
    pub name: String,
    pub strength: u64,
    pub created: DateTime<Utc>,
}

#[tokio::test]
async fn test_insert_from_another_table() {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    // Weapon
    let weapons = (0..1000)
        .map(|i| Weapon {
            name: format!("Weapon{}", i),
            created: Utc::now(),
            strength: i as f64,
            ..Default::default()
        })
        .collect::<Vec<_>>();

    let created_weapons = insert(weapons).return_many(db.clone()).await.unwrap();
    assert_eq!(created_weapons.len(), 1000);

    let weapon::Schema { strength, .. } = Weapon::schema();
    let select_statement = select(All)
        .from(Weapon::table_name())
        .where_(cond(strength.greater_than_or_equal(800)).and(strength.less_than(950)));
    let result: Vec<Weapon> = select_statement.return_many(db.clone()).await.unwrap();
    assert_eq!(result.len(), 150);
    assert!(
        result[0].id.to_string().starts_with("weapon:"),
        "Original should start with weapon:"
    );

    let strong_weapons = insert::<StrongWeapon>(select_statement)
        .return_many(db.clone())
        .await
        .unwrap();

    assert_eq!(strong_weapons.len(), 150);
    assert!(
        strong_weapons[0]
            .id
            .to_string()
            .starts_with("strong_weapon:"),
        "copied should start with new table name - strong_weapon:"
    );

    let strong_weapons: Vec<StrongWeapon> = select(All)
        .from(StrongWeapon::table_name())
        .return_many(db.clone())
        .await
        .unwrap();

    assert_eq!(strong_weapons.len(), 150);

    let strong_weapon::Schema { strength, .. } = StrongWeapon::schema();

    let strong_weapons_count: Vec<StrongWeapon> = select(All)
        .from(StrongWeapon::table_name())
        .order_by(order(strength).desc())
        .return_many(db.clone())
        .await
        .unwrap();

    assert_eq!(strong_weapons_count.len(), 150);
    assert_eq!(strong_weapons_count[0].strength, 949);
}
