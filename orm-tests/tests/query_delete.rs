/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use pretty_assertions::assert_eq;
use surreal_models::{weapon, Weapon};
use surreal_orm::{
    statements::{delete, insert},
    *,
};
use surrealdb::{
    engine::local::{Db, Mem},
    Surreal,
};

async fn create_test_data(db: Surreal<Db>) -> Vec<Weapon> {
    let space_ships = (0..1000)
        .map(|i| Weapon {
            name: format!("weapon-{}", i),
            strength: i as f64,
            ..Default::default() // created: chrono::Utc::now(),
        })
        .collect::<Vec<Weapon>>();
    insert(space_ships).return_many(db.clone()).await.unwrap()
}

#[tokio::test]
async fn test_delete_by_id_helper_function() -> SurrealOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let weapons = create_test_data(db.clone()).await;
    let weapon1 = weapons.first().unwrap();
    let weapon1_id = &weapon1.id.clone();

    let weapon::Schema { id, .. } = &Weapon::schema();

    let deleted_weapon_count = || async {
        Weapon::count_where(id.eq(weapon1_id))
            .get(db.clone())
            .await
            .unwrap()
    };
    assert_eq!(deleted_weapon_count().await, 1);

    Weapon::delete_by_id(weapon1_id).run(db.clone()).await?;

    assert_eq!(deleted_weapon_count().await, 0);

    Ok(())
}

#[tokio::test]
async fn test_delete_one_by_id() -> SurrealOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let weapons = create_test_data(db.clone()).await;
    let weapon1 = weapons.first().unwrap();
    let weapon1_id = &weapon1.id.clone();

    let weapon::Schema { id, .. } = &Weapon::schema();

    let deleted_weapon_count = || async {
        Weapon::count_where(id.eq(weapon1_id))
            .get(db.clone())
            .await
            .unwrap()
    };
    assert_eq!(deleted_weapon_count().await, 1);

    delete::<Weapon>(weapon1_id).run(db.clone()).await?;

    assert_eq!(deleted_weapon_count().await, 0);

    Ok(())
}

#[tokio::test]
async fn test_delete_one_by_model_instance() -> SurrealOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let weapons = create_test_data(db.clone()).await;
    let weapon1 = weapons.first().unwrap();
    let weapon1_id = &weapon1.id.clone();

    let weapon::Schema { id, .. } = &Weapon::schema();

    let deleted_weapon_count = || async {
        Weapon::count_where(id.eq(weapon1_id))
            .get(db.clone())
            .await
            .unwrap()
    };
    let deleted_weapon = || async {
        Weapon::find_by_id(weapon1_id)
            .return_one(db.clone())
            .await
            .unwrap()
    };

    assert_eq!(deleted_weapon().await.is_some(), true);
    assert_eq!(deleted_weapon_count().await, 1);

    weapon1.delete().run(db.clone()).await?;

    assert_eq!(deleted_weapon().await.is_some(), false);
    assert_eq!(deleted_weapon_count().await, 0);

    Ok(())
}

#[tokio::test]
async fn test_delete_where_model_helper_function() -> SurrealOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    create_test_data(db.clone()).await;

    let weapon::Schema { strength, .. } = &Weapon::schema();

    let weapons_count = || async { Weapon::count_all().get(db.clone()).await.unwrap() };
    assert_eq!(weapons_count().await, 1000);

    Weapon::delete_where(cond(strength.gte(500)).and(strength.lt(600)))
        .run(db.clone())
        .await?;

    assert_eq!(weapons_count().await, 900);

    Ok(())
}

#[tokio::test]
async fn test_delete_many_query_by_condition() -> SurrealOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    create_test_data(db.clone()).await;

    let weapon::Schema { strength, .. } = &Weapon::schema();

    let weapons_count = || async { Weapon::count_all().get(db.clone()).await.unwrap() };
    assert_eq!(weapons_count().await, 1000);

    delete::<Weapon>(Weapon::table_name())
        .where_(cond(strength.gte(500)).and(strength.lt(600)))
        .run(db.clone())
        .await?;

    assert_eq!(weapons_count().await, 900);

    Ok(())
}
