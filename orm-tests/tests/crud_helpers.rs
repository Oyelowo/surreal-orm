/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use surreal_models::{space_ship, weapon, SpaceShip, Weapon};
use surreal_orm::{
    statements::{insert, select, select_value},
    *,
};
use surrealdb::{
    engine::local::{Db, Mem},
    Surreal,
};

async fn create_test_data(db: Surreal<Db>) {
    let space_ships = (0..1000)
        .map(|i| Weapon {
            name: format!("weapon-{}", i),
            strength: i as f64,
            ..Default::default() // created: chrono::Utc::now(),
        })
        .collect::<Vec<Weapon>>();
    insert(space_ships).run(db.clone()).await.unwrap();
}
#[tokio::test]
async fn test_create() -> SurrealOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let ss_id = SpaceShip::create_id("num-1".into());
    let spaceship_instance = || SpaceShip {
        id: ss_id.clone(),
        name: "spaceship-1".into(),
        created: chrono::Utc::now(),
    };

    let spaceship = spaceship_instance().create().get_one(db.clone()).await?;
    // Second attempt should fail since it will be duplicate.
    spaceship_instance()
        .create()
        .get_one(db.clone())
        .await
        .expect_err("should fail");

    let saved_spaceship = SpaceShip::find_by_id(ss_id.clone())
        .get_one(db.clone())
        .await?;

    assert_eq!(spaceship.id.to_thing(), saved_spaceship.id.to_thing());
    assert_eq!(spaceship.name, saved_spaceship.name);
    Ok(())
}

#[tokio::test]
async fn test_save() -> SurrealOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let ss_id = SpaceShip::create_id("num-1".into());
    let spaceship = SpaceShip {
        id: ss_id.clone(),
        name: "spaceship-1".into(),
        created: chrono::Utc::now(),
    };

    let spaceship = spaceship.save().get_one(db.clone()).await?;

    let saved_spaceship = SpaceShip::find_by_id(ss_id.clone())
        .get_one(db.clone())
        .await?;

    assert_eq!(spaceship.id.to_thing(), saved_spaceship.id.to_thing());
    assert_eq!(spaceship.name, saved_spaceship.name);
    Ok(())
}

#[tokio::test]
async fn test_find_by_id() -> SurrealOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let spaceship = SpaceShip {
        id: SpaceShip::create_id("num-1".into()),
        name: "spaceship-1".into(),
        created: chrono::Utc::now(),
    };

    spaceship.clone().save().run(db.clone()).await?;

    let found_spaceship = SpaceShip::find_by_id(spaceship.id.clone())
        .get_one(db.clone())
        .await?;

    assert_eq!(spaceship.id.to_thing(), found_spaceship.id.to_thing());
    Ok(())
}

#[tokio::test]
async fn test_find_where() -> SurrealOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let spaceship = SpaceShip {
        id: SpaceShip::create_id("num-1".into()),
        name: "spaceship-1".into(),
        created: chrono::Utc::now(),
    };
    let _spaceship2 = SpaceShip {
        id: SpaceShip::create_id("num-2".into()),
        name: "spaceship-2".into(),
        created: chrono::Utc::now(),
    }
    .save()
    .run(db.clone())
    .await?;

    let _spaceschip = spaceship.clone().save().get_one(db.clone()).await?;
    let space_ship::Schema { name, id, .. } = SpaceShip::schema();

    let found_spaceships = SpaceShip::find_where(id.is_not(NULL))
        .return_many(db.clone())
        .await?;
    assert_eq!(found_spaceships.len(), 2);

    let found_spaceships = SpaceShip::find_where(name.equal("spaceship-1"))
        .return_many(db.clone())
        .await?;

    assert_eq!(found_spaceships.len(), 1);
    assert_eq!(found_spaceships[0].id.to_thing(), spaceship.id.to_thing());

    let found_spaceship = SpaceShip::find_where(name.equal("spaceship-1"))
        .get_one(db.clone())
        .await?;

    assert_eq!(found_spaceship.id.to_thing(), spaceship.id.to_thing());
    Ok(())
}

#[tokio::test]
async fn test_find_where_complex() -> SurrealOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    create_test_data(db.clone()).await;

    let weapon::Schema { id, strength, .. } = &Weapon::schema();
    let count = &Field::new("count");

    let total_spaceships = Weapon::find_where(id.is_not(NONE))
        .return_many(db.clone())
        .await?;
    assert_eq!(total_spaceships.len(), 1000);
    let total_spaceships_query = select_value(count).from(
        select(count!(strength.gte(500)))
            .from(Weapon::table())
            .group_all(),
    );
    assert_eq!(
        total_spaceships_query.to_raw().build(),
        "SELECT VALUE count FROM (SELECT count(strength >= 500) FROM weapon GROUP ALL);"
    );

    let total_spaceships_count: Option<i32> = total_spaceships_query.return_one(db.clone()).await?;

    assert_eq!(total_spaceships_count, Some(500));

    let total_spaceships_count = Weapon::count_where(strength.gte(500))
        .get(db.clone())
        .await?;

    assert_eq!(total_spaceships_count, 500);
    Ok(())
}

#[tokio::test]
async fn test_count_where() -> SurrealOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    create_test_data(db.clone()).await;
    let weapon::Schema { strength, .. } = &Weapon::schema();

    let weapons_query = Weapon::count_where(strength.gte(500));
    let weapons_count = weapons_query.get(db.clone()).await?;

    assert_eq!(
        weapons_query.to_raw().build(),
        "SELECT VALUE count FROM (SELECT count(strength >= 500) FROM weapon GROUP ALL);"
    );

    assert_eq!(weapons_count, 500);

    Ok(())
}

#[tokio::test]
async fn test_count_where_complex() -> SurrealOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    create_test_data(db.clone()).await;
    let weapon::Schema { strength, .. } = &Weapon::schema();

    let weapons_query = Weapon::count_where(cond(strength.gte(500)).and(strength.lt(734)));
    let weapons_count = weapons_query.get(db.clone()).await?;

    assert_eq!(
        weapons_query.to_raw().build(),
        "SELECT VALUE count FROM (SELECT count((strength >= 500) AND (strength < 734)) FROM weapon GROUP ALL);"
    );

    assert_eq!(weapons_count, 234);

    Ok(())
}

#[tokio::test]
async fn test_count_all() -> SurrealOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    create_test_data(db.clone()).await;

    let weapons_query = Weapon::count_all();
    let weapons_count = weapons_query.get(db.clone()).await?;

    assert_eq!(
        weapons_query.to_raw().build(),
        "SELECT VALUE count FROM (SELECT count() FROM weapon GROUP ALL);"
    );

    assert_eq!(weapons_count, 1000);

    Ok(())
}

#[tokio::test]
async fn test_delete() -> SurrealOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let spaceship = SpaceShip {
        id: SpaceShip::create_id("num-1".into()),
        name: "spaceship-1".into(),
        created: chrono::Utc::now(),
    };

    let spaceship = spaceship.save().get_one(db.clone()).await?;

    let found_spaceship = SpaceShip::find_by_id(spaceship.id.clone())
        .return_many(db.clone())
        .await?;
    assert_eq!(found_spaceship.len(), 1);

    spaceship.clone().delete().run(db.clone()).await?;

    let found_spaceship = SpaceShip::find_by_id(spaceship.id.clone())
        .return_many(db.clone())
        .await?;

    assert!(found_spaceship.is_empty());
    assert_eq!(found_spaceship.len(), 0);
    Ok(())
}

#[tokio::test]
async fn test_delete_by_id() -> SurrealOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let spaceship = SpaceShip {
        id: SpaceShip::create_id("num-1".into()),
        name: "spaceship-1".into(),
        created: chrono::Utc::now(),
    };

    let spaceship = spaceship.save().get_one(db.clone()).await?;
    let found_spaceships = SpaceShip::find_by_id(spaceship.id.clone())
        .return_many(db.clone())
        .await?;
    assert_eq!(found_spaceships.len(), 1);

    SpaceShip::delete_by_id(spaceship.id.clone())
        .run(db.clone())
        .await?;

    let found_spaceships = SpaceShip::find_by_id(spaceship.id.clone())
        .return_many(db.clone())
        .await?;
    assert_eq!(found_spaceships.len(), 0);
    Ok(())
}

#[tokio::test]
async fn test_delete_where() -> SurrealOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let spaceship = SpaceShip {
        id: SpaceShip::create_id("num-1".into()),
        name: "spaceship-1".into(),
        created: chrono::Utc::now(),
    };

    spaceship.save().run(db.clone()).await.unwrap();
    let space_ship::Schema { name, .. } = SpaceShip::schema();

    let found_spaceships = SpaceShip::find_where(name.like("spaceship-1"))
        .return_many(db.clone())
        .await?;
    assert_eq!(found_spaceships.len(), 1);

    SpaceShip::delete_where(name.like("spaceship"))
        .run(db.clone())
        .await?;

    let found_spaceships = SpaceShip::find_where(name.like("spaceship-1"))
        .return_many(db.clone())
        .await?;
    assert_eq!(found_spaceships.len(), 0);
    Ok(())
}
