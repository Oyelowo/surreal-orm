/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use chrono::Utc;
use geo::{line_string, point, polygon};
use pretty_assertions::assert_eq;
use std::time::Duration;
use surreal_models::{
    alien, weapon, weapon_old, Alien, Rocket, SpaceShip, Weapon, WeaponOld,
};
use surreal_orm::{
    statements::{create, insert, select, update},
    *,
};
use surrealdb::{engine::local::Mem, Surreal};

fn create_test_alien(age: u8, name: String) -> Alien {
    let point = point! {
        x: 40.02f64,
        y: 116.34,
    };
    let territory = line_string![(x: 40.02, y: 116.34), (x: 40.02, y: 116.35), (x: 40.03, y: 116.35), (x: 40.03, y: 116.34), (x: 40.02, y: 116.34)];
    let polygon = polygon![(x: 40.02, y: 116.34), (x: 40.02, y: 116.35), (x: 40.03, y: 116.35), (x: 40.03, y: 116.34), (x: 40.02, y: 116.34)];
    Alien {
        id: Alien::create_simple_id(),
        name,
        age,
        created: Utc::now(),
        line_polygon: territory,
        life_expectancy: Duration::from_secs(100),
        territory_area: polygon,
        home: point,
        tags: vec!["tag1".into(), "tag2".into()],
        ally: LinkSelf::null(),
        weapon: LinkOne::null(),
        space_ships: LinkMany::null(),
        planets_to_visit: Relate::null(),
    }
}

// test Increment update and decrement update
#[tokio::test]
async fn test_increment_and_decrement_update_set_with_object_partial() -> SurrealOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let weapon = Weapon {
        name: "Laser".to_string(),
        strength: 0.0,
        created: Utc::now(),
        ..Default::default()
    };

    let created_weapon = create().content(weapon).get_one(db.clone()).await.unwrap();
    assert_eq!(created_weapon.name, "Laser");
    assert_eq!(created_weapon.strength, 0.0);
    //
    // // Increment by 5;
    let id = &created_weapon.clone().id;
    let weapon::Schema { strength, .. } = Weapon::schema();

    update::<Weapon>(created_weapon)
        .set(strength.increment_by(5f64))
        .run(db.clone())
        .await?;

    update::<Weapon>(id)
        .set(strength.increment_by(5f64))
        .run(db.clone())
        .await?;

    let updated = update::<Weapon>(id)
        .set(strength.decrement_by(2f64))
        .return_one(db.clone())
        .await?;

    let selected: Option<Weapon> = select(All)
        .from(Weapon::table())
        .return_one(db.clone())
        .await?;
    assert_eq!(updated.unwrap().strength, 8.0);
    assert_eq!(selected.unwrap().strength, 8.0);

    // Try setting
    let updated = update::<Weapon>(id)
        .set(object_partial!(Weapon { strength: 923f64 }))
        .return_one(db.clone())
        .await?;

    let selected: Option<Weapon> = select(All)
        .from(Weapon::table())
        .return_one(db.clone())
        .await?;
    assert_eq!(updated.unwrap().strength, 923.0);
    assert_eq!(selected.unwrap().strength, 923.0);
    Ok(())
}
#[tokio::test]
async fn test_increment_and_decrement_update() -> SurrealOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let weapon = Weapon {
        name: "Laser".to_string(),
        strength: 0.0,
        created: Utc::now(),
        ..Default::default()
    };

    let created_weapon = create().content(weapon).get_one(db.clone()).await.unwrap();
    assert_eq!(created_weapon.name, "Laser");
    assert_eq!(created_weapon.strength, 0.0);
    //
    // // Increment by 5;
    let id = &created_weapon.clone().id;
    let weapon::Schema { strength, .. } = Weapon::schema();

    update::<Weapon>(created_weapon)
        .set(strength.increment_by(5f64))
        .run(db.clone())
        .await?;

    update::<Weapon>(id)
        .set(strength.increment_by(5f64))
        .run(db.clone())
        .await?;

    let updated = update::<Weapon>(id)
        .set(strength.decrement_by(2f64))
        .return_one(db.clone())
        .await?;

    let selected: Option<Weapon> = select(All)
        .from(Weapon::table())
        .return_one(db.clone())
        .await?;
    assert_eq!(updated.unwrap().strength, 8.0);
    assert_eq!(selected.unwrap().strength, 8.0);

    // Try setting
    let updated = update::<Weapon>(id)
        .set(strength.equal_to(923f64))
        .return_one(db.clone())
        .await?;

    let selected: Option<Weapon> = select(All)
        .from(Weapon::table())
        .return_one(db.clone())
        .await?;
    assert_eq!(updated.unwrap().strength, 923.0);
    assert_eq!(selected.unwrap().strength, 923.0);
    Ok(())
}

#[tokio::test]
async fn test_increment_and_decrement_update_conditionally() -> SurrealOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let weapon1 = Weapon {
        name: "Laser".to_string(),
        created: Utc::now(),
        strength: 5.0,
        ..Default::default()
    };
    let weapon2 = Weapon {
        name: "Weapon2".to_string(),
        created: Utc::now(),
        strength: 20.0,
        ..Default::default()
    };
    let weapon3 = Weapon {
        name: "Weapon3".to_string(),
        created: Utc::now(),
        strength: 42.0,
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
    assert_eq!(created_aliens[0].weapon.value().unwrap().strength, 5.0);
    assert_eq!(created_aliens[1].weapon.value().unwrap().strength, 20.0);

    let alien::Schema {
        ref age,
        ref name,
        ref tags,
        ..
    } = Alien::schema();
    let alien = Alien::schema();

    // Select all aliens with weapon strength 5
    let select_weak_aliens = || async {
        let weak_aliens: Vec<Alien> = select(All)
            .from(Alien::table())
            .where_(cond(alien.weapon().strength.equal_to(5f64)).and(age.greater_than(3)))
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

    let weak_aliens = update::<Alien>(Alien::table())
        .set([name.equal_to("Rook"), tags.append("street")])
        .where_(cond(alien.weapon().strength.equal_to(5f64)).and(age.greater_than(3)))
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

    let weak_aliens = update::<Alien>(Alien::table())
        .set([name.equal_to("Kiwi"), tags.remove("street")])
        .where_(cond(alien.weapon().strength.equal_to(5f64)).and(age.greater_than(3)))
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
async fn test_add_and_remove_to_array() -> SurrealOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let unsaved_alien = create_test_alien(20, "Oyelowo".into());
    let created_alien = create().content(unsaved_alien).get_one(db.clone()).await?;
    assert_eq!(created_alien.name, "Oyelowo");
    assert_eq!(
        created_alien.tags,
        vec!["tag1".to_string(), "tag2".to_string()]
    );
    assert!(created_alien.weapon.get_id().is_none());
    assert!(created_alien.space_ships.is_empty());

    // Try append
    let alien_id = &created_alien.clone().id;
    let alien::Schema {
        ref tags,
        ref weapon,
        ref spaceShips,
        ..
    } = Alien::schema();

    update::<Alien>(alien_id)
        .set([
            tags.append("tag3"),
            weapon.equal_to(Weapon::create_id("agi")),
            spaceShips.append(SpaceShip::create_id("cali".into())),
            spaceShips.append(SpaceShip::create_id("codebreather".into())),
            spaceShips.append(SpaceShip::create_id("blayz".into())),
            spaceShips.append(SpaceShip::create_id("anam".into())),
        ])
        .run(db.clone())
        .await?;

    update::<Alien>(alien_id)
        .set([
            tags.append("rust"),
            spaceShips.append(SpaceShip::create_id("anam".into())),
        ])
        .run(db.clone())
        .await?;

    let updated = &(update::<Alien>(alien_id)
        .set([
            tags.append("rice"),
            spaceShips.append(SpaceShip::create_id("cali".into())),
        ])
        .return_one(db.clone())
        .await?);

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
        .set([
            tags.remove("tag1"),
            // removes one of calis. There should be 2 before this
            spaceShips.remove(SpaceShip::create_id("cali".into())),
            spaceShips.remove(SpaceShip::create_id("nonexistent".into())),
        ])
        .return_one(db.clone())
        .await?;

    let selected: &Option<Alien> = &(select(All).from(alien_id).return_one(db.clone()).await?);
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
        .set(tags.equal_to(vec!["oye".into(), "dayo".into()]))
        .return_one(db.clone())
        .await?;

    let selected: Option<Alien> = select(All)
        .from(Alien::table())
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
async fn test_update_single_id_content() -> SurrealOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let weapon = Weapon {
        id: Weapon::create_simple_id(),
        name: "Laser".to_string(),
        created: Utc::now(),
        strength: 20f64,
        ..Default::default()
    };
    let weapon_to_update = Weapon {
        id: weapon.id.clone(),
        name: "Oyelowo".to_string(),
        created: Utc::now(),
        strength: 1000f64,
        ..Default::default()
    };

    // Create weapon
    let created_weapon = create().content(weapon.clone()).get_one(db.clone()).await?;
    assert_eq!(created_weapon.name, "Laser");
    assert_eq!(created_weapon.id.to_thing(), weapon.id.to_thing());
    assert_eq!(created_weapon.strength, 20f64);
    assert_eq!(created_weapon.id.to_string(), weapon.id.to_string());

    let weapon::Schema { id, .. } = Weapon::schema();

    let get_selected_weapon = || async {
        let selected_weapon: Option<Weapon> = select(All)
            .from(Weapon::table())
            .where_(id.equal(created_weapon.id.to_owned()))
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
    assert_eq!(updated_weapon.strength, 1000.0);
    // Id must not be changed to weapon:lowo even if added in the update content
    assert_eq!(updated_weapon.id.to_string(), created_weapon.id.to_string());
    assert_eq!(get_selected_weapon().await.unwrap().name, "Oyelowo");
    assert_eq!(get_selected_weapon().await.unwrap().strength, 1000.0);
    // Id field must not be updated even if provided to an update statement.
    assert_ne!(
        get_selected_weapon().await.unwrap().id.to_string(),
        "weapon:lowo"
    );
    Ok(())
}

#[tokio::test]
async fn test_update_content_with_filter() -> SurrealOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let generated_weapons = (0..20)
        .map(|x| Weapon {
            name: "Laser".to_string(),
            created: Utc::now(),
            strength: x as f64,
            ..Default::default()
        })
        .collect::<Vec<Weapon>>();
    insert(generated_weapons.clone()).run(db.clone()).await?;

    let weapon::Schema { strength, .. } = Weapon::schema();
    let filter = &cond(strength.greater_than(5)).and(strength.less_than_or_equal(15));

    let get_selected_weapons = || async {
        let selected_weapons: Vec<Weapon> = select(All)
            .from(Weapon::table())
            .where_(filter)
            .return_many(db.clone())
            .await
            .unwrap();
        selected_weapons
    };
    assert_eq!(get_selected_weapons().await.len(), 10);

    // Test update with CONTENT keyoword and not using updater
    let update_weapons_with_filter = update::<Weapon>(Weapon::table())
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
            .all(|x| x.name == "Oyelowo"),
        "All not equals Oyelowo"
    );
    assert!(
        update_weapons_with_filter.iter().all(|x| x.strength == 0.0),
        "All strength not equals 9"
    );

    assert_eq!(get_selected_weapons().await.len(), 0);

    // Test update with CONTENT keyoword and using updater and default values which sets null
    // values for Options
    let update_weapons_with_filter = update::<Weapon>(Weapon::table())
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
            .all(|x| x.name == "Oyelowo"),
        "All not equals Oyelowo"
    );
    assert!(
        update_weapons_with_filter.iter().all(|x| x.strength == 0.0),
        "All strength not equals 9"
    );

    assert_eq!(get_selected_weapons().await.len(), 0);

    Ok(())
}

#[tokio::test]
async fn test_update_merge_with_filter() -> SurrealOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let generated_weapons = (0..20)
        .map(|x| Weapon {
            name: "Laser".to_string(),
            created: Utc::now(),
            strength: x as f64,
            ..Default::default()
        })
        .collect::<Vec<Weapon>>();
    insert(generated_weapons.clone()).run(db.clone()).await?;

    let weapon::Schema { strength, .. } = Weapon::schema();
    let filter = &cond(strength.greater_than(5)).and(strength.less_than_or_equal(15));

    let get_selected_weapons = || async {
        let selected_weapons: Vec<Weapon> = select(All)
            .from(Weapon::table())
            .where_(filter)
            .return_many(db.clone())
            .await
            .unwrap();
        selected_weapons
    };
    assert_eq!(get_selected_weapons().await.len(), 10);

    // Test update with MERGE keyoword and using updater
    let update_weapons_with_filter = update::<Weapon>(Weapon::table())
        .merge(
            Weapon::partial_builder()
                .name("Oyelowo".into())
                .strength(16.0)
                .rocket(Rocket::partial_builder().name("Bruno".into()).build()
                .build(),
        )
        .where_(filter)
        .return_many(db.clone())
        .await?;
    assert_eq!(update_weapons_with_filter.len(), 10);
    assert!(
        update_weapons_with_filter
            .iter()
            .all(|x| x.name == "Oyelowo"),
        "All not equals Oyelowo"
    );
    assert_eq!(
        update_weapons_with_filter
            .iter()
            .filter(|x| x.strength == 16.0)
            .collect::<Vec<&Weapon>>()
            .len(),
        10,
    );

    assert_eq!(get_selected_weapons().await.len(), 0);

    Ok(())
}
#[tokio::test]
async fn test_update_single_id_merge_no_fields_skip() -> SurrealOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let weapon = Weapon {
        name: "Laser".to_string(),
        created: Utc::now(),
        strength: 20.0,
        ..Default::default()
    };
    let weapon_to_update = Weapon {
        id: weapon.id.clone(),
        name: "Oyelowo".to_string(),
        created: Utc::now(),
        strength: 1000.0,
        ..Default::default()
    };

    // Create weapon
    let created_weapon = create().content(weapon.clone()).get_one(db.clone()).await?;
    assert_eq!(created_weapon.name, "Laser");
    assert_eq!(created_weapon.id.to_string(), weapon.id.to_string());
    assert_eq!(created_weapon.strength, 20.0);
    assert_eq!(created_weapon.id.to_string(), weapon.id.to_string());

    let get_selected_weapon = || async {
        let selected_weapon: Option<Weapon> = select(All)
            .from(Weapon::table())
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
    assert_eq!(updated_weapon.strength, 1000.0);
    // Id must not be changed to weapon:lowo even if added in the update content
    assert_eq!(updated_weapon.id.to_string(), created_weapon.id.to_string());
    assert_eq!(get_selected_weapon().await.unwrap().name, "Oyelowo");
    assert_eq!(get_selected_weapon().await.unwrap().strength, 1000.0);
    // Id field must not be updated even if provided to an update statement.
    assert_ne!(
        get_selected_weapon().await.unwrap().id.to_string(),
        "weapon:lowo"
    );
    //
    Ok(())
}

#[tokio::test]
async fn test_update_single_id_merge_skips_fields() -> SurrealOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let weapon = Weapon {
        name: "Laser".to_string(),
        created: Utc::now(),
        strength: 20.0,
        ..Default::default()
    };
    // Will only override the name
    let weapon_to_update = Weapon::partial_builder().name("Oyelowo".to_string());

    // Create weapon
    let created_weapon = create().content(weapon.clone()).get_one(db.clone()).await?;
    assert_eq!(created_weapon.name, "Laser");
    assert_eq!(created_weapon.id.to_thing(), weapon.id.to_thing());
    assert_eq!(created_weapon.strength, 20.0);
    assert_eq!(created_weapon.id.to_thing().tb, "weapon");

    let get_selected_weapon = || async {
        let selected_weapon: Option<Weapon> = select(All)
            .from(Weapon::table())
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
    assert_eq!(updated_weapon.strength, 20.0);
    // Id must not be changed to weapon:lowo even if added in the update content
    assert_eq!(updated_weapon.id.to_string(), created_weapon.id.to_string());
    assert_eq!(get_selected_weapon().await.unwrap().name, "Oyelowo");
    assert_eq!(get_selected_weapon().await.unwrap().strength, 20.0);
    // Id field must not be updated even if provided to an update statement.
    assert_ne!(
        get_selected_weapon().await.unwrap().id.to_string(),
        "weapon:lowo"
    );
    //
    Ok(())
}

#[tokio::test]
async fn test_update_single_id_replace() -> SurrealOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    // Create old weapon
    let weapon_id = Weapon::create_simple_id();
    create::<Weapon>()
        .set(object_partial!(Weapon {
            id: weapon_id.clone(),
            name: "Laser".to_string()
        }))
        .run(db.clone())
        .await?;
    assert_eq!(Weapon::count_all().get(db.clone()).await.unwrap(), 1);

    // Will replace the whole weapon.
    let weapon_to_update_with_new_fields = Weapon {
        id: weapon_id.clone(),
        name: "Oyelowo".to_string(),
        created: Utc::now(),
        strength: 823.0,
        ..Default::default()
    };

    // Fully replace weapon table with completely new object and data. This will remove all fields
    // that are not present in the new object. This is a destructive operation.
    let updated_weapon = &(update::<Weapon>(weapon_id.clone())
        .replace(weapon_to_update_with_new_fields)
        .get_one(db.clone())
        .await?);

    let selected_weapon: Option<Weapon> = select(All)
        .from(Weapon::table())
        .where_(Weapon::schema().id.equal(weapon_id.clone()))
        .return_one(db.clone())
        .await
        .unwrap();
    assert_eq!(selected_weapon.as_ref().unwrap().name, "Oyelowo");

    // Only name should be updated
    assert_eq!(updated_weapon.strength, 823.0);
    assert_eq!(updated_weapon.name, "Oyelowo");
    assert_eq!(updated_weapon.id.to_string(), weapon_id.clone().to_string());

    assert_eq!(selected_weapon.as_ref().unwrap().name, "Oyelowo");
    assert_eq!(
        serde_json::to_value(updated_weapon)
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
async fn test_update_single_id_patch_replace_change() -> SurrealOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let weapon = Weapon {
        name: "test".to_string(),
        created: Utc::now(),
        strength: 20.0,
        ..Default::default()
    };

    // Create weapon
    let created_weapon = create().content(weapon.clone()).get_one(db.clone()).await?;
    assert_eq!(created_weapon.name, "test");
    assert_eq!(created_weapon.id.to_thing(), weapon.id.to_thing());
    assert_eq!(created_weapon.strength, 20.0);

    let get_selected_weapon = || async {
        let selected_weapon: Option<Weapon> = select(All)
            .from(Weapon::table())
            .where_(Weapon::schema().id.equal(created_weapon.id.to_owned()))
            .return_one(db.clone())
            .await
            .unwrap();
        selected_weapon
    };
    assert_eq!(get_selected_weapon().await.unwrap().name, "test");

    let weapon::Schema {
        ref name,
        ref strength,
        ..
    } = Weapon::schema();
    let updated_weapon = update::<Weapon>(created_weapon.clone().id)
        .patch(vec![
            name.patch_change("@@ -1,4 +1,4 @@\n te\n-s\n+x\n t\n"),
            strength.patch_replace(34f64),
        ])
        .get_one(db.clone())
        .await?;
    assert_eq!(updated_weapon.name, "text");
    assert_eq!(updated_weapon.strength, 34.0);
    // Id must not be changed to weapon:lowo even if added in the update content
    assert_eq!(updated_weapon.id.to_thing(), created_weapon.id.to_thing());
    assert_eq!(get_selected_weapon().await.unwrap().name, "text");
    assert_eq!(get_selected_weapon().await.unwrap().strength, 34.0);
    // Id field must not be updated even if provided to an update statement.
    assert_ne!(
        get_selected_weapon().await.unwrap().id.to_string(),
        "weapon:lowo"
    );
    let updated_weapon = update::<Weapon>(created_weapon.clone().id)
        .patch([
            strength.patch_replace(921f64),
            name.patch_change("@@ -1,4 +1,4 @@\n te\n-x\n+o\n t\n"),
        ])
        .get_one(db.clone())
        .await?;
    assert_eq!(updated_weapon.name, "teot");
    assert_eq!(updated_weapon.strength, 921.0);
    Ok(())
}

#[tokio::test]
async fn test_update_single_id_patch_remove() -> SurrealOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();
    let weapon_old = WeaponOld {
        name: "Laser".to_string(),
        created: Utc::now(),
        strength: 20.0,
        bunch_of_other_fields: 34,
        nice: false,
        ..Default::default()
    };
    // Create old weapon
    let old_weapon = create()
        .content(weapon_old.clone())
        .get_one(db.clone())
        .await?;
    assert_eq!(old_weapon.name, "Laser");
    assert_eq!(old_weapon.id.to_string(), weapon_old.id.to_string());
    assert_eq!(old_weapon.strength, 20.0);
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
            "rocket",
        ]
    );

    let selected_weapon: Option<WeaponOld> = select(All)
        .from(WeaponOld::table())
        .where_(WeaponOld::schema().id.equal(old_weapon.id.to_owned()))
        .return_one(db.clone())
        .await
        .unwrap();
    assert_eq!(selected_weapon.as_ref().unwrap().name, "Laser");
    assert_eq!(
        serde_json::to_value(selected_weapon.as_ref().unwrap())
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
            "rocket",
        ]
    );

    // Remove some fields from WeaponOld struct.
    let weapon_old::Schema {
        ref bunchOfOtherFields,
        ref nice,
        ..
    } = WeaponOld::schema();
    // Deserializing with old struct should now cause error since there are fields missing.
    // This is not how you should use the patch remove. Merely used for testing. Check the latter
    // place for a good example where the new weapon struct is used.
    let updated_weapon_with_old = &update::<WeaponOld>(old_weapon.clone().id)
        .patch([nice.patch_remove(), bunchOfOtherFields.patch_remove()])
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
    let updated_weapon = &(update::<Weapon>(old_weapon.clone().id)
        .patch([nice.patch_remove(), bunchOfOtherFields.patch_remove()])
        .get_one(db.clone())
        .await?);

    let selected_weapon: Option<Weapon> = select(All)
        .from(Weapon::table())
        .where_(Weapon::schema().id.equal(old_weapon.id.to_owned()))
        .return_one(db.clone())
        .await?;
    assert_eq!(selected_weapon.as_ref().unwrap().name, "Laser");

    // Only name should be updated
    assert_eq!(updated_weapon.strength, 20.0);
    assert_eq!(updated_weapon.name, "Laser");
    assert_eq!(updated_weapon.id.to_string(), old_weapon.id.to_string());

    assert_eq!(selected_weapon.as_ref().unwrap().name, "Laser");
    assert_eq!(
        serde_json::to_value(updated_weapon)
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
async fn test_update_single_id_patch_add() -> SurrealOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let weapon_old = Weapon {
        name: "Laser".to_string(),
        created: Utc::now(),
        strength: 20.0,
        ..Default::default()
    };
    // Create weapon
    let old_weapon = create()
        .content(weapon_old.clone())
        .get_one(db.clone())
        .await?;
    assert_eq!(old_weapon.name, "Laser");
    assert_eq!(old_weapon.id.to_thing(), weapon_old.id.to_thing());
    assert_eq!(old_weapon.strength, 20.0);
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
        .from(Weapon::table())
        .where_(Weapon::schema().id.equal(old_weapon.id.to_owned()))
        .return_one(db.clone())
        .await
        .unwrap();
    assert_eq!(selected_weapon.as_ref().unwrap().name, "Laser");
    assert_eq!(
        serde_json::to_value(selected_weapon.as_ref().unwrap())
            .unwrap()
            .as_object()
            .unwrap()
            .keys()
            .collect::<Vec<&String>>(),
        vec!["id", "name", "strength", "created", "rocket"]
    );

    let weapon_old::Schema {
        ref bunchOfOtherFields,
        ref nice,
        ..
    } = WeaponOld::schema();
    let selected_weapon = select(All)
        .from(WeaponOld::table())
        .where_(Weapon::schema().id.equal(old_weapon.id.to_owned()))
        .return_one::<WeaponOld>(db.clone())
        .await;

    assert!(
        selected_weapon.unwrap_err().to_string().contains(
        "Unable to parse data returned from the database. \
            Check that all fields are complete and the types are able to deserialize surrealdb data types properly.")
    );

    // bunchOfOtherFields is not string
    let _updated_weapon = &(update::<WeaponOld>(old_weapon.clone().id)
        .patch([nice.patch_add(true), bunchOfOtherFields.patch_add(56)])
        .return_one(db.clone())
        .await);

    let updated_weapon = &(update::<WeaponOld>(old_weapon.clone().id)
        .patch([nice.patch_add(true), bunchOfOtherFields.patch_add(45)])
        .get_one(db.clone())
        .await?);

    let selected_weapon: Option<WeaponOld> = select(All)
        .from(WeaponOld::table())
        .where_(WeaponOld::schema().id.equal(old_weapon.id.to_owned()))
        .return_one(db.clone())
        .await?;
    assert_eq!(selected_weapon.as_ref().unwrap().name, "Laser");

    // Only name should be updated
    assert_eq!(updated_weapon.bunch_of_other_fields, 45);
    assert_eq!(updated_weapon.nice, true);
    assert_eq!(updated_weapon.strength, 20.0);
    assert_eq!(updated_weapon.name, "Laser");
    assert_eq!(updated_weapon.id.to_string(), old_weapon.id.to_string());

    assert_eq!(selected_weapon.as_ref().unwrap().name, "Laser");
    assert_eq!(selected_weapon.as_ref().unwrap().nice, true);
    assert_eq!(selected_weapon.as_ref().unwrap().bunch_of_other_fields, 45);
    assert_eq!(
        serde_json::to_value(updated_weapon)
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
            "rocket",
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
            "rocket",
        ]
    );
    assert_ne!(selected_weapon.unwrap().id.to_string(), "weapon:lowo");

    //
    Ok(())
}
