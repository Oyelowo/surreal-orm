use chrono::{DateTime, Utc};
use geo::line_string;
use geo::point;
use geo::polygon;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use surrealdb::engine::local::Mem;
use surrealdb::Surreal;
use surrealdb_models::weapon_schema;
use surrealdb_models::weaponold_schema;
use surrealdb_models::WeaponOld;
use surrealdb_models::WeaponUpdater;
use surrealdb_models::{alien_schema, spaceship_schema, Alien, SpaceShip, Weapon};
use surrealdb_orm::statements::insert;
use surrealdb_orm::statements::patch;
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

fn create_test_alien(age: u8, name: String) -> Alien {
    let point = point! {
        x: 40.02f64,
        y: 116.34,
    };
    let territory = line_string![(x: 40.02, y: 116.34), (x: 40.02, y: 116.35), (x: 40.03, y: 116.35), (x: 40.03, y: 116.34), (x: 40.02, y: 116.34)];
    let polygon = polygon![(x: 40.02, y: 116.34), (x: 40.02, y: 116.35), (x: 40.03, y: 116.35), (x: 40.03, y: 116.34), (x: 40.02, y: 116.34)];
    Alien {
        id: None,
        name,
        age,
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
    }
}

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

// // conditionlly link any of the weapons
// if i % 2 == 0 {
//     unsaved_alien.weapon = LinkOne::from(weapon1);
// } else if i % 3 == 0 {
//     unsaved_alien.weapon = LinkOne::from(weapon2);
// } else {
//     unsaved_alien.weapon = LinkOne::from(weapon3);
// }
#[tokio::test]
async fn test_increment_and_decrement_update_conditionally() -> SurrealdbOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let weapon1 = Weapon {
        name: "Laser".to_string(),
        created: Utc::now(),
        strength: 5,
        ..Default::default()
    };
    let weapon2 = Weapon {
        name: "Weapon2".to_string(),
        created: Utc::now(),
        strength: 20,
        ..Default::default()
    };
    let weapon3 = Weapon {
        name: "Weapon3".to_string(),
        created: Utc::now(),
        strength: 42,
        ..Default::default()
    };
    let weapons = insert(vec![weapon1, weapon2, weapon3])
        .return_many(db.clone())
        .await?;

    // generate test aliens
    let generated_aliens = (0..20)
        .map(|i| {
            let mut unsaved_alien = create_test_alien(i, format!("Oyelowo{}", i));
            // Set ally for some
            if i % 2 == 0 {
                unsaved_alien.ally = LinkSelf::from(unsaved_alien.clone());
            }
            // 0 (fulfills the condition) => weapon1 which has a strength of 5.
            // 1
            // 2
            // 3 (fulfills the condition) => weapon1
            // 4
            // 5
            // 6 (fulfills the condition) => weapon1
            // 7
            // 8
            // 9 (fulfills the condition) => weapon1
            // 10
            // 11
            // 12 (fulfills the condition) => weapon1
            // 13
            // 14
            // 15 (fulfills the condition) => weapon1
            // 16
            // 17
            // 18 (fulfills the condition) => weapon1
            // 19
            // 20
            unsaved_alien.weapon = LinkOne::from(weapons[i as usize % 3].clone());
            unsaved_alien
        })
        .collect::<Vec<_>>();

    let created_aliens = insert(generated_aliens)
        .load_link_ones()?
        .return_many(db.clone())
        .await?;
    assert_eq!(created_aliens.len(), 20);
    assert_eq!(created_aliens.last().unwrap().age, 19);
    assert_eq!(created_aliens[0].weapon.value().unwrap().strength, 5);
    assert_eq!(created_aliens[1].weapon.value().unwrap().strength, 20);

    let alien_schema::Alien {
        ref age,
        ref name,
        ref tags,
        ..
    } = Alien::schema();
    let alien = Alien::schema();

    // Select all aliens with weapon strength 5
    let select_weak_aliens = || async {
        let weak_aliens: Vec<Alien> = select(All)
            .from(Alien::table_name())
            .where_(cond(alien.weapon(E).strength.equal(5)).and(age.greater_than(3)))
            .return_many(db.clone())
            .await
            .unwrap();
        weak_aliens
    };
    // None should be rook here
    assert!(select_weak_aliens()
        .await
        .iter()
        .all(|alien| alien.name != "Rook"));
    // assert that none have tag street yet
    assert!(select_weak_aliens()
        .await
        .iter()
        .all(|alien| !alien.tags.contains(&"street".to_string())));
    // assert numbers of tags
    assert!(select_weak_aliens()
        .await
        .iter()
        .all(|alien| alien.tags.len() == 2));

    let weak_aliens = update::<Alien>(Alien::table_name())
        .set(updater(name).equal("Rook"))
        .set(updater(tags).append("street"))
        .where_(cond(alien.weapon(E).strength.equal(5)).and(age.greater_than(3)))
        .return_many(db.clone())
        .await?;

    // Based on the modulo above in the linking, we should have 7 weapons with strength 5.
    // Out of the 7, only 5 have age greater than 3.
    assert_eq!(weak_aliens.len(), 5);
    // Assert that they all now have the name rook
    assert!(weak_aliens.iter().all(|alien| alien.name == "Rook"));
    // Assert that they all now have the tag street
    assert!(weak_aliens
        .iter()
        .all(|alien| alien.tags.contains(&"street".to_string())));

    // They should now all be rooks
    assert!(select_weak_aliens()
        .await
        .iter()
        .all(|alien| alien.name == "Rook"));
    assert!(select_weak_aliens()
        .await
        .iter()
        .all(|alien| alien.tags.len() == 3));

    let weak_aliens = update::<Alien>(Alien::table_name())
        .set(updater(name).equal("Kiwi"))
        .set(updater(tags).remove("street"))
        .where_(cond(alien.weapon(E).strength.equal(5)).and(age.greater_than(3)))
        .return_many(db.clone())
        .await?;
    assert!(weak_aliens
        .iter()
        .all(|alien| !alien.tags.contains(&"street".to_string())));

    // They should now all be kiwi
    assert!(select_weak_aliens()
        .await
        .iter()
        .all(|alien| alien.name == "Kiwi"));
    assert!(select_weak_aliens()
        .await
        .iter()
        .all(|alien| alien.tags.len() == 2));
    Ok(())
}

#[tokio::test]
async fn test_add_and_remove_to_array() -> SurrealdbOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let unsaved_alien = create_test_alien(20, "Oyelowo".into());
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
        // removes one of calis. There should be 2 before this
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
async fn test_update_single_id_content() -> SurrealdbOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let weapon = Weapon {
        id: Some(Weapon::create_id("original_id")),
        name: "Laser".to_string(),
        created: Utc::now(),
        strength: 20,
        ..Default::default()
    };
    let weapon_to_update = Weapon {
        id: Some(Weapon::create_id("lowo")),
        name: "Oyelowo".to_string(),
        created: Utc::now(),
        strength: 1000,
        ..Default::default()
    };

    // Create weapon
    let created_weapon = create(weapon.clone()).get_one(db.clone()).await?;
    assert_eq!(created_weapon.name, "Laser");
    assert_eq!(
        created_weapon.id.as_ref().unwrap().to_string(),
        "weapon:original_id"
    );
    assert_eq!(created_weapon.strength, 20);
    assert!(created_weapon.id.as_ref().is_some());

    let get_selected_weapon = || async {
        let selected_weapon: Option<Weapon> = select(All)
            .from(Weapon::table_name())
            .where_(
                Weapon::schema()
                    .id
                    .equal(created_weapon.id.to_owned().unwrap()),
            )
            .return_one(db.clone())
            .await
            .unwrap();
        selected_weapon
    };
    assert_eq!(get_selected_weapon().await.unwrap().name, "Laser");

    let updated_weapon = update::<Weapon>(created_weapon.clone().id.unwrap())
        .content(weapon_to_update)
        .get_one(db.clone())
        .await?;
    assert_eq!(updated_weapon.name, "Oyelowo");
    assert_eq!(updated_weapon.strength, 1000);
    // Id must not be changed to weapon:lowo even if added in the update content
    assert_eq!(
        updated_weapon.id.as_ref().unwrap().to_string(),
        "weapon:original_id"
    );
    assert_eq!(get_selected_weapon().await.unwrap().name, "Oyelowo");
    assert_eq!(get_selected_weapon().await.unwrap().strength, 1000);
    // Id field must not be updated even if provided to an update statement.
    assert_ne!(
        get_selected_weapon().await.unwrap().id.unwrap().to_string(),
        "weapon:lowo"
    );
    //
    Ok(())
}

#[tokio::test]
async fn test_update_single_id_merge_no_fields_skip() -> SurrealdbOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let weapon = Weapon {
        id: Some(Weapon::create_id("original_id")),
        name: "Laser".to_string(),
        created: Utc::now(),
        strength: 20,
        ..Default::default()
    };
    let weapon_to_update = Weapon {
        id: Some(Weapon::create_id("lowo")),
        name: "Oyelowo".to_string(),
        created: Utc::now(),
        strength: 1000,
        ..Default::default()
    };

    // Create weapon
    let created_weapon = create(weapon.clone()).get_one(db.clone()).await?;
    assert_eq!(created_weapon.name, "Laser");
    assert_eq!(
        created_weapon.id.as_ref().unwrap().to_string(),
        "weapon:original_id"
    );
    assert_eq!(created_weapon.strength, 20);
    assert!(created_weapon.id.as_ref().is_some());

    let get_selected_weapon = || async {
        let selected_weapon: Option<Weapon> = select(All)
            .from(Weapon::table_name())
            .where_(
                Weapon::schema()
                    .id
                    .equal(created_weapon.id.to_owned().unwrap()),
            )
            .return_one(db.clone())
            .await
            .unwrap();
        selected_weapon
    };
    assert_eq!(get_selected_weapon().await.unwrap().name, "Laser");

    let updated_weapon = update::<Weapon>(created_weapon.clone().id.unwrap())
        .merge(weapon_to_update)
        .get_one(db.clone())
        .await?;
    assert_eq!(updated_weapon.name, "Oyelowo");
    assert_eq!(updated_weapon.strength, 1000);
    // Id must not be changed to weapon:lowo even if added in the update content
    assert_eq!(
        updated_weapon.id.as_ref().unwrap().to_string(),
        "weapon:original_id"
    );
    assert_eq!(get_selected_weapon().await.unwrap().name, "Oyelowo");
    assert_eq!(get_selected_weapon().await.unwrap().strength, 1000);
    // Id field must not be updated even if provided to an update statement.
    assert_ne!(
        get_selected_weapon().await.unwrap().id.unwrap().to_string(),
        "weapon:lowo"
    );
    //
    Ok(())
}

#[tokio::test]
async fn test_update_single_id_merge_skips_fields() -> SurrealdbOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let weapon = Weapon {
        id: Some(Weapon::create_id("original_id")),
        name: "Laser".to_string(),
        created: Utc::now(),
        strength: 20,
        ..Default::default()
    };
    // Will only override the name
    let weapon_to_update = WeaponUpdater {
        name: Some("Oyelowo".to_string()),
        ..Default::default()
    };

    // Create weapon
    let created_weapon = create(weapon.clone()).get_one(db.clone()).await?;
    assert_eq!(created_weapon.name, "Laser");
    assert_eq!(
        created_weapon.id.as_ref().unwrap().to_string(),
        "weapon:original_id"
    );
    assert_eq!(created_weapon.strength, 20);
    assert!(created_weapon.id.as_ref().is_some());

    let get_selected_weapon = || async {
        let selected_weapon: Option<Weapon> = select(All)
            .from(Weapon::table_name())
            .where_(
                Weapon::schema()
                    .id
                    .equal(created_weapon.id.to_owned().unwrap()),
            )
            .return_one(db.clone())
            .await
            .unwrap();
        selected_weapon
    };
    assert_eq!(get_selected_weapon().await.unwrap().name, "Laser");

    let updated_weapon = update::<Weapon>(created_weapon.clone().id.unwrap())
        .merge(weapon_to_update)
        .get_one(db.clone())
        .await?;
    // Only name should be updated
    assert_eq!(updated_weapon.name, "Oyelowo");
    assert_eq!(updated_weapon.strength, 20);
    // Id must not be changed to weapon:lowo even if added in the update content
    assert_eq!(
        updated_weapon.id.as_ref().unwrap().to_string(),
        "weapon:original_id"
    );
    assert_eq!(get_selected_weapon().await.unwrap().name, "Oyelowo");
    assert_eq!(get_selected_weapon().await.unwrap().strength, 20);
    // Id field must not be updated even if provided to an update statement.
    assert_ne!(
        get_selected_weapon().await.unwrap().id.unwrap().to_string(),
        "weapon:lowo"
    );
    //
    Ok(())
}

#[tokio::test]
async fn test_update_single_id_replace() -> SurrealdbOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();
    let weapon_old = WeaponOld {
        id: Some(WeaponOld::create_id("original_id")),
        name: "Laser".to_string(),
        created: Utc::now(),
        strength: 20,
        bunch_of_other_fields: 34,
        nice: false,
        ..Default::default()
    };
    // Create old weapon
    let old_weapon = create(weapon_old).get_one(db.clone()).await?;
    assert_eq!(old_weapon.name, "Laser");
    assert_eq!(
        old_weapon.id.as_ref().unwrap().to_string(),
        "weapon:original_id"
    );
    assert_eq!(old_weapon.strength, 20);
    assert!(old_weapon.id.as_ref().is_some());
    assert_eq!(
        serde_json::to_value(&old_weapon)
            .unwrap()
            .as_object()
            .unwrap()
            .keys()
            .collect::<Vec<&String>>(),
        vec![
            "id",
            "name",
            "strength",
            "nice",
            "bunchOfOtherFields",
            "created"
        ]
    );

    let selected_weapon: Option<WeaponOld> = select(All)
        .from(WeaponOld::table_name())
        .where_(
            WeaponOld::schema()
                .id
                .equal(old_weapon.id.to_owned().unwrap()),
        )
        .return_one(db.clone())
        .await
        .unwrap();
    assert_eq!(selected_weapon.as_ref().unwrap().name, "Laser");
    assert_eq!(
        serde_json::to_value(&selected_weapon.as_ref().unwrap())
            .unwrap()
            .as_object()
            .unwrap()
            .keys()
            .collect::<Vec<&String>>(),
        vec![
            "id",
            "name",
            "strength",
            "nice",
            "bunchOfOtherFields",
            "created"
        ]
    );

    // Will replace the whole weapon.
    let weapon_to_update_with_new_fields = Weapon {
        id: Some(Weapon::create_id("original_id")),
        name: "Oyelowo".to_string(),
        created: Utc::now(),
        strength: 823,
        ..Default::default()
    };

    // Fully replace weapon table with completely new object and data. This will remove all fields
    // that are not present in the new object. This is a destructive operation.
    let ref updated_weapon = update::<Weapon>(old_weapon.clone().id.unwrap())
        .replace(weapon_to_update_with_new_fields)
        .return_one(db.clone())
        .await?;

    let selected_weapon: Option<Weapon> = select(All)
        .from(Weapon::table_name())
        .where_(Weapon::schema().id.equal(old_weapon.id.to_owned().unwrap()))
        .return_one(db.clone())
        .await
        .unwrap();
    assert_eq!(selected_weapon.as_ref().unwrap().name, "Oyelowo");

    // Only name should be updated
    assert_eq!(updated_weapon.as_ref().unwrap().strength, 823);
    assert_eq!(updated_weapon.as_ref().unwrap().name, "Oyelowo");
    assert_eq!(
        updated_weapon
            .as_ref()
            .unwrap()
            .id
            .as_ref()
            .unwrap()
            .to_string(),
        "weapon:original_id"
    );

    assert_eq!(selected_weapon.as_ref().unwrap().name, "Oyelowo");
    assert_eq!(
        serde_json::to_value(&updated_weapon)
            .unwrap()
            .as_object()
            .unwrap()
            .keys()
            .collect::<Vec<&String>>(),
        vec!["id", "name", "strength", "created"]
    );
    assert_eq!(
        serde_json::to_value(&selected_weapon)
            .unwrap()
            .as_object()
            .unwrap()
            .keys()
            .collect::<Vec<&String>>(),
        vec!["id", "name", "strength", "created"]
    );
    assert_ne!(
        selected_weapon.unwrap().id.unwrap().to_string(),
        "weapon:lowo"
    );
    Ok(())
}

#[tokio::test]
async fn test_update_single_id_patch_remove() -> SurrealdbOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();
    let weapon_old = WeaponOld {
        id: Some(WeaponOld::create_id("original_id")),
        name: "Laser".to_string(),
        created: Utc::now(),
        strength: 20,
        bunch_of_other_fields: 34,
        nice: false,
        ..Default::default()
    };
    // Create old weapon
    let old_weapon = create(weapon_old).get_one(db.clone()).await?;
    assert_eq!(old_weapon.name, "Laser");
    assert_eq!(
        old_weapon.id.as_ref().unwrap().to_string(),
        "weapon:original_id"
    );
    assert_eq!(old_weapon.strength, 20);
    assert!(old_weapon.id.as_ref().is_some());
    assert_eq!(
        serde_json::to_value(&old_weapon)
            .unwrap()
            .as_object()
            .unwrap()
            .keys()
            .collect::<Vec<&String>>(),
        vec![
            "id",
            "name",
            "strength",
            "nice",
            "bunchOfOtherFields",
            "created"
        ]
    );

    let selected_weapon: Option<WeaponOld> = select(All)
        .from(WeaponOld::table_name())
        .where_(
            WeaponOld::schema()
                .id
                .equal(old_weapon.id.to_owned().unwrap()),
        )
        .return_one(db.clone())
        .await
        .unwrap();
    assert_eq!(selected_weapon.as_ref().unwrap().name, "Laser");
    assert_eq!(
        serde_json::to_value(&selected_weapon.as_ref().unwrap())
            .unwrap()
            .as_object()
            .unwrap()
            .keys()
            .collect::<Vec<&String>>(),
        vec![
            "id",
            "name",
            "strength",
            "nice",
            "bunchOfOtherFields",
            "created"
        ]
    );

    // Remove some fields from WeaponOld struct.
    let weaponold_schema::WeaponOld {
        ref bunchOfOtherFields,
        ref nice,
        ..
    } = WeaponOld::schema();
    // Deserializing with old struct should now cause error since there are fields missing.
    let ref updated_weapon_with_old = update::<WeaponOld>(old_weapon.clone().id.unwrap())
        .patch(patch(nice).remove())
        .patch(patch(bunchOfOtherFields).remove())
        .return_one(db.clone())
        .await
        .unwrap_err();
    assert!(
        updated_weapon_with_old.to_string().contains(
        "Unable to parse data returned from the database. \
            Check that all fields are complete and the types are able to deserialize surrealdb data types properly.")
    );
    assert!(updated_weapon_with_old
        .to_string()
        .contains("missing field `nice`"));

    // that are not present in the new object. This is a destructive operation.
    let ref updated_weapon = update::<Weapon>(old_weapon.clone().id.unwrap())
        .patch(patch(nice).remove())
        .patch(patch(bunchOfOtherFields).remove())
        .return_one(db.clone())
        .await?;

    let selected_weapon: Option<Weapon> = select(All)
        .from(Weapon::table_name())
        .where_(Weapon::schema().id.equal(old_weapon.id.to_owned().unwrap()))
        .return_one(db.clone())
        .await?;
    assert_eq!(selected_weapon.as_ref().unwrap().name, "Laser");

    // Only name should be updated
    assert_eq!(updated_weapon.as_ref().unwrap().strength, 20);
    assert_eq!(updated_weapon.as_ref().unwrap().name, "Laser");
    assert_eq!(
        updated_weapon
            .as_ref()
            .unwrap()
            .id
            .as_ref()
            .unwrap()
            .to_string(),
        "weapon:original_id"
    );

    assert_eq!(selected_weapon.as_ref().unwrap().name, "Laser");
    assert_eq!(
        serde_json::to_value(&updated_weapon)
            .unwrap()
            .as_object()
            .unwrap()
            .keys()
            .collect::<Vec<&String>>(),
        vec!["id", "name", "strength", "created"]
    );
    assert_eq!(
        serde_json::to_value(&selected_weapon)
            .unwrap()
            .as_object()
            .unwrap()
            .keys()
            .collect::<Vec<&String>>(),
        vec!["id", "name", "strength", "created"]
    );
    assert_ne!(
        selected_weapon.unwrap().id.unwrap().to_string(),
        "weapon:lowo"
    );
    //
    Ok(())
}
