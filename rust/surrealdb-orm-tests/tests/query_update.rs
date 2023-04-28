use chrono::{DateTime, Utc};
use geo::line_string;
use geo::point;
use geo::polygon;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use surrealdb::engine::local::Mem;
use surrealdb::Surreal;
use surrealdb_models::weapon_schema;
use surrealdb_models::{alien_schema, spaceship_schema, Alien, SpaceShip, Weapon};
use surrealdb_orm::statements::insert;
use surrealdb_orm::statements::update;
use surrealdb_orm::{
    statements::{create, select},
    *,
};
// -- Update all records in a table
// UPDATE person SET skills += ['breathing'];
//
// -- Update or create a record with a specific numeric id
// UPDATE person:100 SET name = 'Tobie', company = 'SurrealDB', skills = ['Rust', 'Go', 'JavaScript'];
//
// -- Update or create a record with a specific string id
// UPDATE person:tobie SET name = 'Tobie', company = 'SurrealDB', skills = ['Rust', 'Go', 'JavaScript'];
//

// test Increment update and decrement update
#[tokio::test]
async fn test_increment_and_decrement_update() -> SurrealdbOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let weapon = Weapon {
        name: "Laser".to_string(),
        created: Utc::now(),
        strength: 0,
        ..Default::default()
    };

    let created_weapon = create(weapon).return_one(db.clone()).await.unwrap();
    assert_eq!(created_weapon.as_ref().unwrap().name, "Laser");
    assert_eq!(created_weapon.as_ref().unwrap().strength, 0);

    // Increment by 5;
    let ref id = created_weapon.unwrap().clone().id.unwrap();
    let weapon_schema::Weapon { ref strength, .. } = Weapon::schema();

    update::<Weapon>(id)
        .set(updater(strength).increment_by(5))
        .run(db.clone())
        .await?;

    update::<Weapon>(id)
        .set(updater(strength).increment_by(5))
        .run(db.clone())
        .await?;

    let updated = update::<Weapon>(id)
        .set(updater(strength).decrement_by(2))
        .return_one(db.clone())
        .await?;

    let selected: Option<Weapon> = select(All)
        .from(Weapon::table_name())
        .return_one(db.clone())
        .await?;
    assert_eq!(updated.unwrap().strength, 8);
    assert_eq!(selected.unwrap().strength, 8);

    // Try setting
    let updated = update::<Weapon>(id)
        .set(updater(strength).equal(923))
        .return_one(db.clone())
        .await?;

    let selected: Option<Weapon> = select(All)
        .from(Weapon::table_name())
        .return_one(db.clone())
        .await?;
    assert_eq!(updated.unwrap().strength, 923);
    assert_eq!(selected.unwrap().strength, 923);
    Ok(())
}

#[tokio::test]
async fn test_add_and_remove_to_array() -> SurrealdbOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let point = point! {
        x: 40.02f64,
        y: 116.34,
    };
    let territory = line_string![(x: 40.02, y: 116.34), (x: 40.02, y: 116.35), (x: 40.03, y: 116.35), (x: 40.03, y: 116.34), (x: 40.02, y: 116.34)];
    let polygon = polygon![(x: 40.02, y: 116.34), (x: 40.02, y: 116.35), (x: 40.03, y: 116.35), (x: 40.03, y: 116.34), (x: 40.02, y: 116.34)];
    let unsaved_alien = Alien {
        id: None,
        name: "Oyelowo".to_string(),
        age: 20,
        created: Utc::now(),
        line_polygon: territory.into(),
        life_expectancy: Duration::from_secs(100),
        territory_area: polygon.into(),
        home: point.into(),
        tags: vec!["tag1".into(), "tag2".into()],
        ally: LinkSelf::null(),
        weapon: LinkOne::null(),
        space_ships: LinkMany::null(),
        planets_to_visit: Relate::null(),
    };

    let created_alien = create(unsaved_alien).return_one(db.clone()).await.unwrap();
    assert_eq!(created_alien.as_ref().unwrap().name, "Oyelowo");
    assert_eq!(
        created_alien.as_ref().unwrap().tags,
        vec!["tag1".to_string(), "tag2".to_string()]
    );
    assert!(created_alien.as_ref().unwrap().weapon.get_id().is_none());
    assert!(created_alien.as_ref().unwrap().space_ships.is_empty());

    // Try append
    let ref alien_id = created_alien.as_ref().unwrap().clone().id.unwrap();
    let alien_schema::Alien {
        ref age,
        ref tags,
        ref ally,
        ref weapon,
        ref spaceShips,
        ..
    } = Alien::schema();

    update::<Alien>(alien_id)
        .set(updater(tags).append("tag3"))
        .set(updater(weapon).equal(Weapon::create_id("agi")))
        .set(updater(spaceShips).append(SpaceShip::create_id("cali")))
        .set(updater(spaceShips).append(SpaceShip::create_id("codebreather")))
        .set(updater(spaceShips).append(SpaceShip::create_id("blayz")))
        .set(updater(spaceShips).append(SpaceShip::create_id("anam")))
        .run(db.clone())
        .await?;

    update::<Alien>(alien_id)
        .set(updater(tags).append("rust"))
        .set(updater(spaceShips).append(SpaceShip::create_id("anam")))
        .run(db.clone())
        .await?;

    let ref updated = update::<Alien>(alien_id)
        .set(updater(tags).plus_equal("rice"))
        .set(updater(spaceShips).append(SpaceShip::create_id("cali")))
        .return_one(db.clone())
        .await?;

    let selected: Option<Alien> = select(All).from(alien_id).return_one(db.clone()).await?;
    assert_eq!(
        updated.as_ref().unwrap().tags,
        vec!["tag1", "tag2", "tag3", "rust", "rice"]
    );
    assert_eq!(
        selected.unwrap().tags,
        vec!["tag1", "tag2", "tag3", "rust", "rice"]
    );
    assert!(updated.as_ref().unwrap().weapon.get_id().is_some());
    assert_eq!(
        updated
            .as_ref()
            .unwrap()
            .weapon
            .get_id()
            .unwrap()
            .to_string(),
        "weapon:agi"
    );
    assert_eq!(updated.as_ref().unwrap().space_ships.len(), 6);
    assert_eq!(
        updated
            .as_ref()
            .unwrap()
            .space_ships
            .keys_truthy()
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>(),
        vec![
            "space_ship:cali",
            "space_ship:codebreather",
            "space_ship:blayz",
            "space_ship:anam",
            "space_ship:anam",
            "space_ship:cali",
        ]
    );

    // Try removing
    let updated = update::<Alien>(alien_id)
        .set(updater(tags).remove("tag1"))
        .set(updater(spaceShips).remove(SpaceShip::create_id("cali")))
        .set(updater(spaceShips).remove(SpaceShip::create_id("nonexistent")))
        .return_one(db.clone())
        .await?;

    let ref selected: Option<Alien> = select(All).from(alien_id).return_one(db.clone()).await?;
    assert_eq!(
        updated.as_ref().unwrap().tags,
        vec!["tag2", "tag3", "rust", "rice"]
    );
    assert_eq!(
        selected.as_ref().unwrap().tags,
        vec!["tag2", "tag3", "rust", "rice"]
    );

    // Try setting
    let updated = update::<Alien>(alien_id)
        .set(updater(tags).equal(vec!["oye", "dayo"]))
        .return_one(db.clone())
        .await?;

    let selected: Option<Alien> = select(All)
        .from(Alien::table_name())
        .where_(Alien::schema().id.equal(alien_id.to_owned()))
        .return_one(db.clone())
        .await?;
    assert_eq!(updated.as_ref().unwrap().tags, vec!["oye", "dayo"]);
    assert_eq!(selected.as_ref().unwrap().tags, vec!["oye", "dayo"]);
    assert_eq!(updated.as_ref().unwrap().space_ships.len(), 5);
    assert_eq!(
        selected
            .as_ref()
            .unwrap()
            .space_ships
            .keys_truthy()
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>(),
        vec![
            "space_ship:codebreather",
            "space_ship:blayz",
            "space_ship:anam",
            "space_ship:anam",
            "space_ship:cali",
        ]
    );
    Ok(())
}

#[tokio::test]
async fn test_create_alien_with_id_not_specified_but_generated_by_the_database(
) -> SurrealdbOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let weapon = Weapon {
        name: "Laser".to_string(),
        created: Utc::now(),
        ..Default::default()
    };

    let space_ship = SpaceShip {
        id: Some(SpaceShip::create_id("gbanda")),
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
    // No id specified before creation. Will be autogenerated by the database.
    assert_eq!(weapon.clone().id.is_some(), false);

    // create first record to weapon table
    let created_weapon = insert(weapon.clone()).return_one(db.clone()).await?;
    // Id is generated after creation
    assert_eq!(created_weapon.unwrap().id.is_some(), true);

    let select1: Vec<Weapon> = select(All)
        .from(Weapon::table_name())
        .return_many(db.clone())
        .await?;
    // weapon table should have one record
    assert_eq!(select1.len(), 1);

    //  Create second record
    let created_weapon = insert(weapon.clone()).return_one(db.clone()).await?;

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
        id: None,
        name: "Oyelowo".to_string(),
        age: 20,
        created: Utc::now(),
        line_polygon: territory.into(),
        life_expectancy: Duration::from_secs(100),
        territory_area: polygon.into(),
        home: point.into(),
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

    Ok(())
}
