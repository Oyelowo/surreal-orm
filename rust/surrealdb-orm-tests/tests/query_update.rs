use chrono::Utc;
use geo::line_string;
use geo::point;
use geo::polygon;
use std::time::Duration;
use surrealdb::engine::local::Mem;
use surrealdb::Surreal;
use surrealdb_models::RocketNonNullUpdater;
use surrealdb_models::{
    alien_schema, weapon_schema, weaponold_schema, Alien, SpaceShip, Weapon, WeaponNonNullUpdater,
    WeaponOld,
};
use surrealdb_orm::{
    statements::{create, insert, patch, select, update},
    *,
};

fn create_test_alien(age: u8, name: String) -> Alien {
    let point = point! {
        x: 40.02f64,
        y: 116.34,
    };
    let territory = line_string![(x: 40.02, y: 116.34), (x: 40.02, y: 116.35), (x: 40.03, y: 116.35), (x: 40.03, y: 116.34), (x: 40.02, y: 116.34)];
    let polygon = polygon![(x: 40.02, y: 116.34), (x: 40.02, y: 116.35), (x: 40.03, y: 116.35), (x: 40.03, y: 116.34), (x: 40.02, y: 116.34)];
    Alien {
        id: SurrealId::default(),
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
        strength: 0,
        created: Utc::now(),
        ..Default::default()
    };

    let created_weapon = create(weapon).return_one(db.clone()).await.unwrap();
    assert_eq!(created_weapon.as_ref().unwrap().name, "Laser");
    assert_eq!(created_weapon.as_ref().unwrap().strength, 0);

    // Increment by 5;
    let ref id = created_weapon.unwrap().clone().id;
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
        // .set(updater(strength).equal(923))
        .set(strength.equal(34u64))
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
            .where_(cond(alien.weapon(E).strength.equal(5u64)).and(age.greater_than(3)))
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
        .set(name.equal("Rook"))
        .set(updater(tags).append("street"))
        .where_(cond(alien.weapon(E).strength.equal(5u64)).and(age.greater_than(3)))
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
        .where_(cond(alien.weapon(E).strength.equal(5u64)).and(age.greater_than(3)))
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
    let ref alien_id = created_alien.as_ref().unwrap().clone().id;
    let alien_schema::Alien {
        ref tags,
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
        id: Weapon::create_id("original_id"),
        name: "Laser".to_string(),
        created: Utc::now(),
        strength: 20,
        ..Default::default()
    };
    let weapon_to_update = Weapon {
        id: Weapon::create_id("lowo"),
        name: "Oyelowo".to_string(),
        created: Utc::now(),
        strength: 1000,
        ..Default::default()
    };

    // Create weapon
    let created_weapon = create(weapon.clone()).get_one(db.clone()).await?;
    assert_eq!(created_weapon.name, "Laser");
    assert_eq!(created_weapon.id.to_string(), "weapon:original_id");
    assert_eq!(created_weapon.strength, 20);
    assert_eq!(created_weapon.id.to_string(), weapon.id.to_string());

    let get_selected_weapon = || async {
        let selected_weapon: Option<Weapon> = select(All)
            .from(Weapon::table_name())
            .where_(Weapon::schema().id.equal(created_weapon.id.to_owned()))
            .return_one(db.clone())
            .await
            .unwrap();
        selected_weapon
    };
    assert_eq!(get_selected_weapon().await.unwrap().name, "Laser");

    let updated_weapon = update::<Weapon>(created_weapon.clone().id)
        .content(weapon_to_update)
        .get_one(db.clone())
        .await?;
    assert_eq!(updated_weapon.name, "Oyelowo");
    assert_eq!(updated_weapon.strength, 1000);
    // Id must not be changed to weapon:lowo even if added in the update content
    assert_eq!(updated_weapon.id.to_string(), "weapon:original_id");
    assert_eq!(get_selected_weapon().await.unwrap().name, "Oyelowo");
    assert_eq!(get_selected_weapon().await.unwrap().strength, 1000);
    // Id field must not be updated even if provided to an update statement.
    assert_ne!(
        get_selected_weapon().await.unwrap().id.to_string(),
        "weapon:lowo"
    );
    Ok(())
}

#[tokio::test]
async fn test_update_content_with_filter() -> SurrealdbOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let generated_weapons = (0..20)
        .map(|x| {
            let mut weapon = Weapon {
                name: "Laser".to_string(),
                created: Utc::now(),
                strength: x,
                ..Default::default()
            };
            weapon.id = Weapon::create_id(&format!("weapon:{}", x));
            weapon
        })
        .collect::<Vec<Weapon>>();
    insert(generated_weapons.clone()).run(db.clone()).await?;

    let weapon_schema::Weapon { strength, .. } = Weapon::schema();
    let ref filter = cond(strength.greater_than(5)).and(strength.less_than_or_equal(15));

    let get_selected_weapons = || async {
        let selected_weapons: Vec<Weapon> = select(All)
            .from(Weapon::table_name())
            .where_(filter)
            .return_many(db.clone())
            .await
            .unwrap();
        selected_weapons
    };
    assert_eq!(get_selected_weapons().await.len(), 10);

    // Test update with CONTENT keyoword and not using updater
    let update_weapons_with_filter = update::<Weapon>(Weapon::table_name())
        .content(Weapon {
            name: "Oyelowo".to_string(),
            created: Utc::now(),
            // strength is overriden to zero by ..Default::default() because optional fields are
            // not skipped if None
            ..Default::default()
        })
        .where_(filter)
        .return_many(db.clone())
        .await?;

    assert_eq!(update_weapons_with_filter.len(), 10);
    assert!(
        update_weapons_with_filter
            .iter()
            .all(|x| x.name.to_string() == "Oyelowo"),
        "All not equals Oyelowo"
    );
    assert!(
        update_weapons_with_filter.iter().all(|x| x.strength == 0),
        "All strength not equals 9"
    );

    assert_eq!(get_selected_weapons().await.len(), 0);

    // Test update with CONTENT keyoword and using updater and default values which sets null
    // values for Options
    let update_weapons_with_filter = update::<Weapon>(Weapon::table_name())
        .content(Weapon {
            name: "Oyelowo".to_string(),
            created: Utc::now(),
            // strength is overriden to zero by ..Default::default() because optional fields are
            // not skipped if None
            ..Default::default()
        })
        .where_(filter)
        .return_many(db.clone())
        .await?;

    assert_eq!(update_weapons_with_filter.len(), 0);
    assert!(
        update_weapons_with_filter
            .iter()
            .all(|x| x.name.to_string() == "Oyelowo"),
        "All not equals Oyelowo"
    );
    assert!(
        update_weapons_with_filter.iter().all(|x| x.strength == 0),
        "All strength not equals 9"
    );

    assert_eq!(get_selected_weapons().await.len(), 0);

    Ok(())
}

#[tokio::test]
async fn test_update_merge_with_filter() -> SurrealdbOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let generated_weapons = (0..20)
        .map(|x| {
            let mut weapon = Weapon {
                name: "Laser".to_string(),
                created: Utc::now(),
                strength: x,
                ..Default::default()
            };
            weapon.id = Weapon::create_id(&format!("weapon:{}", x));
            weapon
        })
        .collect::<Vec<Weapon>>();
    insert(generated_weapons.clone()).run(db.clone()).await?;

    let weapon_schema::Weapon { strength, .. } = Weapon::schema();
    let ref filter = cond(strength.greater_than(5)).and(strength.less_than_or_equal(15));

    let get_selected_weapons = || async {
        let selected_weapons: Vec<Weapon> = select(All)
            .from(Weapon::table_name())
            .where_(filter)
            .return_many(db.clone())
            .await
            .unwrap();
        selected_weapons
    };
    assert_eq!(get_selected_weapons().await.len(), 10);

    // Test update with MERGE keyoword and using updater
    let rocket_update_object = RocketNonNullUpdater {
        name: Some("Bruno".to_string()),
        ..Default::default()
    };
    let update_weapons_with_filter = update::<Weapon>(Weapon::table_name())
        .merge(WeaponNonNullUpdater {
            name: Some("Oyelowo".to_string()),
            strength: Some(16),
            rocket: Some(rocket_update_object),
            ..Default::default()
        })
        .where_(filter)
        .return_many(db.clone())
        .await?;

    assert_eq!(update_weapons_with_filter.len(), 10);
    assert!(
        update_weapons_with_filter
            .iter()
            .all(|x| x.name.to_string() == "Oyelowo"),
        "All not equals Oyelowo"
    );
    assert_eq!(
        update_weapons_with_filter
            .iter()
            .filter(|x| x.strength == 16)
            .collect::<Vec<&Weapon>>()
            .len(),
        10,
    );

    assert_eq!(get_selected_weapons().await.len(), 0);

    Ok(())
}
#[tokio::test]
async fn test_update_single_id_merge_no_fields_skip() -> SurrealdbOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let weapon = Weapon {
        id: Weapon::create_id("original_id"),
        name: "Laser".to_string(),
        created: Utc::now(),
        strength: 20,
        ..Default::default()
    };
    let weapon_to_update = Weapon {
        id: Weapon::create_id("lowo"),
        name: "Oyelowo".to_string(),
        created: Utc::now(),
        strength: 1000,
        ..Default::default()
    };

    // Create weapon
    let created_weapon = create(weapon.clone()).get_one(db.clone()).await?;
    assert_eq!(created_weapon.name, "Laser");
    assert_eq!(created_weapon.id.to_string(), "weapon:original_id");
    assert_eq!(created_weapon.strength, 20);
    assert_eq!(created_weapon.id.to_string(), weapon.id.to_string());

    let get_selected_weapon = || async {
        let selected_weapon: Option<Weapon> = select(All)
            .from(Weapon::table_name())
            .where_(Weapon::schema().id.equal(created_weapon.id.to_owned()))
            .return_one(db.clone())
            .await
            .unwrap();
        selected_weapon
    };
    assert_eq!(get_selected_weapon().await.unwrap().name, "Laser");

    let updated_weapon = update::<Weapon>(created_weapon.clone().id)
        .merge(weapon_to_update)
        .get_one(db.clone())
        .await?;
    assert_eq!(updated_weapon.name, "Oyelowo");
    assert_eq!(updated_weapon.strength, 1000);
    // Id must not be changed to weapon:lowo even if added in the update content
    assert_eq!(updated_weapon.id.to_string(), "weapon:original_id");
    assert_eq!(get_selected_weapon().await.unwrap().name, "Oyelowo");
    assert_eq!(get_selected_weapon().await.unwrap().strength, 1000);
    // Id field must not be updated even if provided to an update statement.
    assert_ne!(
        get_selected_weapon().await.unwrap().id.to_string(),
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
        id: Weapon::create_id("original_id"),
        name: "Laser".to_string(),
        created: Utc::now(),
        strength: 20,
        ..Default::default()
    };
    // Will only override the name
    let weapon_to_update = WeaponNonNullUpdater {
        name: Some("Oyelowo".to_string()),
        ..Default::default()
    };

    // Create weapon
    let created_weapon = create(weapon.clone()).get_one(db.clone()).await?;
    assert_eq!(created_weapon.name, "Laser");
    assert_eq!(created_weapon.id.to_string(), "weapon:original_id");
    assert_eq!(created_weapon.strength, 20);
    assert_eq!(created_weapon.id.tb, "weapon");

    let get_selected_weapon = || async {
        let selected_weapon: Option<Weapon> = select(All)
            .from(Weapon::table_name())
            .where_(Weapon::schema().id.equal(created_weapon.id.to_owned()))
            .return_one(db.clone())
            .await
            .unwrap();
        selected_weapon
    };
    assert_eq!(get_selected_weapon().await.unwrap().name, "Laser");

    let updated_weapon = update::<Weapon>(created_weapon.clone().id)
        .merge(weapon_to_update)
        .get_one(db.clone())
        .await?;
    // Only name should be updated
    assert_eq!(updated_weapon.name, "Oyelowo");
    assert_eq!(updated_weapon.strength, 20);
    // Id must not be changed to weapon:lowo even if added in the update content
    assert_eq!(updated_weapon.id.to_string(), "weapon:original_id");
    assert_eq!(get_selected_weapon().await.unwrap().name, "Oyelowo");
    assert_eq!(get_selected_weapon().await.unwrap().strength, 20);
    // Id field must not be updated even if provided to an update statement.
    assert_ne!(
        get_selected_weapon().await.unwrap().id.to_string(),
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
        id: WeaponOld::create_id("original_id"),
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
    assert_eq!(old_weapon.id.to_string(), "weapon:original_id");
    assert_eq!(old_weapon.strength, 20);
    assert_eq!(old_weapon.id.tb, "weapon");
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
            "created",
            "rocket"
        ]
    );

    let selected_weapon: Option<WeaponOld> = select(All)
        .from(WeaponOld::table_name())
        .where_(WeaponOld::schema().id.equal(old_weapon.id.to_owned()))
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
            "created",
            "rocket"
        ]
    );

    // Will replace the whole weapon.
    let weapon_to_update_with_new_fields = Weapon {
        id: Weapon::create_id("original_id"),
        name: "Oyelowo".to_string(),
        created: Utc::now(),
        strength: 823,
        ..Default::default()
    };

    // Fully replace weapon table with completely new object and data. This will remove all fields
    // that are not present in the new object. This is a destructive operation.
    let ref updated_weapon = update::<Weapon>(old_weapon.clone().id)
        .replace(weapon_to_update_with_new_fields)
        .get_one(db.clone())
        .await?;

    let selected_weapon: Option<Weapon> = select(All)
        .from(Weapon::table_name())
        .where_(Weapon::schema().id.equal(old_weapon.id.to_owned()))
        .return_one(db.clone())
        .await
        .unwrap();
    assert_eq!(selected_weapon.as_ref().unwrap().name, "Oyelowo");

    // Only name should be updated
    assert_eq!(updated_weapon.strength, 823);
    assert_eq!(updated_weapon.name, "Oyelowo");
    assert_eq!(updated_weapon.id.to_string(), "weapon:original_id");

    assert_eq!(selected_weapon.as_ref().unwrap().name, "Oyelowo");
    assert_eq!(
        serde_json::to_value(&updated_weapon)
            .unwrap()
            .as_object()
            .unwrap()
            .keys()
            .collect::<Vec<&String>>(),
        vec!["id", "name", "strength", "created", "rocket"]
    );
    assert_eq!(
        serde_json::to_value(&selected_weapon)
            .unwrap()
            .as_object()
            .unwrap()
            .keys()
            .collect::<Vec<&String>>(),
        vec!["id", "name", "strength", "created", "rocket"]
    );
    assert_ne!(selected_weapon.unwrap().id.to_string(), "weapon:lowo");
    Ok(())
}

#[tokio::test]
async fn test_update_single_id_patch_replace_change() -> SurrealdbOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let weapon = Weapon {
        id: Weapon::create_id("original_id"),
        name: "test".to_string(),
        created: Utc::now(),
        strength: 20,
        ..Default::default()
    };

    // Create weapon
    let created_weapon = create(weapon.clone()).get_one(db.clone()).await?;
    assert_eq!(created_weapon.name, "test");
    assert_eq!(created_weapon.id.to_string(), "weapon:original_id");
    assert_eq!(created_weapon.strength, 20);

    let get_selected_weapon = || async {
        let selected_weapon: Option<Weapon> = select(All)
            .from(Weapon::table_name())
            .where_(Weapon::schema().id.equal(created_weapon.id.to_owned()))
            .return_one(db.clone())
            .await
            .unwrap();
        selected_weapon
    };
    assert_eq!(get_selected_weapon().await.unwrap().name, "test");

    let weapon_schema::Weapon {
        ref name,
        ref strength,
        ..
    } = Weapon::schema();
    let updated_weapon = update::<Weapon>(created_weapon.clone().id)
        .patch(vec![
            patch(name).change("@@ -1,4 +1,4 @@\n te\n-s\n+x\n t\n"),
            patch(strength).replace(34),
        ])
        .get_one(db.clone())
        .await?;
    assert_eq!(updated_weapon.name, "text");
    assert_eq!(updated_weapon.strength, 34);
    // Id must not be changed to weapon:lowo even if added in the update content
    assert_eq!(updated_weapon.id.to_string(), "weapon:original_id");
    assert_eq!(get_selected_weapon().await.unwrap().name, "text");
    assert_eq!(get_selected_weapon().await.unwrap().strength, 34);
    // Id field must not be updated even if provided to an update statement.
    assert_ne!(
        get_selected_weapon().await.unwrap().id.to_string(),
        "weapon:lowo"
    );
    let updated_weapon = update::<Weapon>(created_weapon.clone().id)
        .patch(patch(strength).replace(921))
        .patch(patch(name).change("@@ -1,4 +1,4 @@\n te\n-x\n+o\n t\n"))
        .get_one(db.clone())
        .await?;
    assert_eq!(updated_weapon.name, "teot");
    assert_eq!(updated_weapon.strength, 921);
    Ok(())
}

#[tokio::test]
async fn test_update_single_id_patch_remove() -> SurrealdbOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();
    let weapon_old = WeaponOld {
        id: WeaponOld::create_id("original_id"),
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
    assert_eq!(old_weapon.id.to_string(), "weapon:original_id");
    assert_eq!(old_weapon.strength, 20);
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
            "created",
            "rocket"
        ]
    );

    let selected_weapon: Option<WeaponOld> = select(All)
        .from(WeaponOld::table_name())
        .where_(WeaponOld::schema().id.equal(old_weapon.id.to_owned()))
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
            "created",
            "rocket"
        ]
    );

    // Remove some fields from WeaponOld struct.
    let weaponold_schema::WeaponOld {
        ref bunchOfOtherFields,
        ref nice,
        ..
    } = WeaponOld::schema();
    // Deserializing with old struct should now cause error since there are fields missing.
    // This is not how you should use the patch remove. Merely used for testing. Check the latter
    // place for a good example where the new weapon struct is used.
    let ref updated_weapon_with_old = update::<WeaponOld>(old_weapon.clone().id)
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
    let ref updated_weapon = update::<Weapon>(old_weapon.clone().id)
        .patch(patch(nice).remove())
        .patch(patch(bunchOfOtherFields).remove())
        .get_one(db.clone())
        .await?;

    let selected_weapon: Option<Weapon> = select(All)
        .from(Weapon::table_name())
        .where_(Weapon::schema().id.equal(old_weapon.id.to_owned()))
        .return_one(db.clone())
        .await?;
    assert_eq!(selected_weapon.as_ref().unwrap().name, "Laser");

    // Only name should be updated
    assert_eq!(updated_weapon.strength, 20);
    assert_eq!(updated_weapon.name, "Laser");
    assert_eq!(updated_weapon.id.to_string(), "weapon:original_id");

    assert_eq!(selected_weapon.as_ref().unwrap().name, "Laser");
    assert_eq!(
        serde_json::to_value(&updated_weapon)
            .unwrap()
            .as_object()
            .unwrap()
            .keys()
            .collect::<Vec<&String>>(),
        vec!["id", "name", "strength", "created", "rocket"]
    );
    assert_eq!(
        serde_json::to_value(&selected_weapon)
            .unwrap()
            .as_object()
            .unwrap()
            .keys()
            .collect::<Vec<&String>>(),
        vec!["id", "name", "strength", "created", "rocket"]
    );
    assert_ne!(selected_weapon.unwrap().id.to_string(), "weapon:lowo");
    //
    Ok(())
}

#[tokio::test]
async fn test_update_single_id_patch_add() -> SurrealdbOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let weapon_old = Weapon {
        id: Weapon::create_id("original_id"),
        name: "Laser".to_string(),
        created: Utc::now(),
        strength: 20,
        ..Default::default()
    };
    // Create weapon
    let old_weapon = create(weapon_old).get_one(db.clone()).await?;
    assert_eq!(old_weapon.name, "Laser");
    assert_eq!(old_weapon.id.to_string(), "weapon:original_id");
    assert_eq!(old_weapon.strength, 20);
    assert_eq!(
        serde_json::to_value(&old_weapon)
            .unwrap()
            .as_object()
            .unwrap()
            .keys()
            .collect::<Vec<&String>>(),
        vec!["id", "name", "strength", "created", "rocket"]
    );

    let selected_weapon: Option<Weapon> = select(All)
        .from(Weapon::table_name())
        .where_(Weapon::schema().id.equal(old_weapon.id.to_owned()))
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
        vec!["id", "name", "strength", "created", "rocket"]
    );

    let weaponold_schema::WeaponOld {
        ref bunchOfOtherFields,
        ref nice,
        ..
    } = WeaponOld::schema();
    let selected_weapon = select(All)
        .from(WeaponOld::table_name())
        .where_(Weapon::schema().id.equal(old_weapon.id.to_owned()))
        .return_one::<WeaponOld>(db.clone())
        .await;

    assert!(
        selected_weapon.unwrap_err().to_string().contains(
        "Unable to parse data returned from the database. \
            Check that all fields are complete and the types are able to deserialize surrealdb data types properly.")
    );

    // bunchOfOtherFields is not string
    let ref updated_weapon = update::<WeaponOld>(old_weapon.clone().id)
        .patch(patch(nice).add(true))
        .patch(patch(bunchOfOtherFields).add("test"))
        .return_one(db.clone())
        .await;
    assert!(
        updated_weapon
            .as_ref()
            .unwrap_err()
            .to_string()
            .contains("expected i32"),
        "Wrong type"
    );
    assert!(
        updated_weapon.as_ref().unwrap_err().to_string().contains(
            "Unable to parse data returned from the database. \
            Check that all fields are complete and the types are able to deserialize surrealdb data types properly."
        ),
        "Should not be able to deserialize with old struct"
    );

    let ref updated_weapon = update::<WeaponOld>(old_weapon.clone().id)
        .patch(patch(nice).add(true))
        .patch(patch(bunchOfOtherFields).add(45))
        .get_one(db.clone())
        .await?;

    let selected_weapon: Option<WeaponOld> = select(All)
        .from(WeaponOld::table_name())
        .where_(WeaponOld::schema().id.equal(old_weapon.id.to_owned()))
        .return_one(db.clone())
        .await?;
    assert_eq!(selected_weapon.as_ref().unwrap().name, "Laser");

    // Only name should be updated
    assert_eq!(updated_weapon.bunch_of_other_fields, 45);
    assert_eq!(updated_weapon.nice, true);
    assert_eq!(updated_weapon.strength, 20);
    assert_eq!(updated_weapon.name, "Laser");
    assert_eq!(updated_weapon.id.to_string(), "weapon:original_id");

    assert_eq!(selected_weapon.as_ref().unwrap().name, "Laser");
    assert_eq!(selected_weapon.as_ref().unwrap().nice, true);
    assert_eq!(selected_weapon.as_ref().unwrap().bunch_of_other_fields, 45);
    assert_eq!(
        serde_json::to_value(&updated_weapon)
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
            "created",
            "rocket"
        ]
    );
    assert_eq!(
        serde_json::to_value(&selected_weapon)
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
            "created",
            "rocket"
        ]
    );
    assert_ne!(selected_weapon.unwrap().id.to_string(), "weapon:lowo");
    //
    Ok(())
}
