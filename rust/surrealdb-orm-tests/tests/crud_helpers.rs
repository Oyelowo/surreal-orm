use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use surrealdb::{engine::local::Mem, Surreal};
use surrealdb_models::{spaceship_schema, SpaceShip};
use surrealdb_orm::*;

#[tokio::test]
async fn test_save() -> SurrealdbOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let ss_id = SpaceShip::create_id(format!("num-{}", 1));
    let spaceship = SpaceShip {
        id: ss_id.clone(),
        name: format!("spaceship-{}", 1),
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
async fn test_find_by_id() -> SurrealdbOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let spaceship = SpaceShip {
        id: SpaceShip::create_id(format!("num-{}", 1)),
        name: format!("spaceship-{}", 1),
        created: chrono::Utc::now(),
    };

    spaceship.clone().save().run(db.clone()).await?;

    let found_spaceship = SpaceShip::find_by_id(spaceship.id.clone())
        .get_one(db.clone())
        .await
        .unwrap();

    assert_eq!(spaceship.id.to_thing(), found_spaceship.id.to_thing());
    Ok(())
}

// #[tokio::test]
// async fn test_find_where() {
//     let db = Surreal::new::<Mem>(()).await.unwrap();
//     db.use_ns("test").use_db("test").await.unwrap();
//
//     let spaceship = SpaceShip {
//         id: SpaceShip::create_id(format!("num-{}", 1)),
//         name: format!("spaceship-{}", 1),
//         created: chrono::Utc::now(),
//     };
//
//     spaceship.save().run(db.clone()).await.unwrap();
//     let found_spaceships = SpaceShip::find_where(Condition::Equal("name", spaceship.name.clone()))
//         .run(db.clone())
//         .await
//         .unwrap();
//
//     assert_eq!(found_spaceships.len(), 1);
//     assert_eq!(found_spaceships[0].id, spaceship.id);
// }
//
// #[tokio::test]
// async fn test_delete() {
//     let db = Surreal::new::<Mem>(()).await.unwrap();
//     db.use_ns("test").use_db("test").await.unwrap();
//
//     let mut spaceship = SpaceShip {
//         id: SpaceShip::create_id(format!("num-{}", 1)),
//         name: format!("spaceship-{}", 1),
//         created: chrono::Utc::now(),
//     };
//
//     spaceship.save().run(db.clone()).await.unwrap();
//     spaceship.delete().run(db.clone()).await.unwrap();
//
//     let found_spaceship = SpaceShip::find_by_id(spaceship.id.clone())
//         .run(db.clone())
//         .await
//         .unwrap();
//     assert!(found_spaceship.is_none());
// }
//
// #[tokio::test]
// async fn test_delete_by_id() {
//     let db = Surreal::new::<Mem>(()).await.unwrap();
//     db.use_ns("test").use_db("test").await.unwrap();
//
//     let spaceship = SpaceShip {
//         id: SpaceShip::create_id(format!("num-{}", 1)),
//         name: format!("spaceship-{}", 1),
//         created: chrono::Utc::now(),
//     };
//
//     spaceship.save().run(db.clone()).await.unwrap();
//     SpaceShip::delete_by_id(spaceship.id.clone())
//         .run(db.clone())
//         .await
//         .unwrap();
//
//     let found_spaceship = SpaceShip::find_by_id(spaceship.id.clone())
//         .run(db.clone())
//         .await
//         .unwrap();
//     assert!(found_spaceship.is_none());
// }
//
// #[tokio::test]
// async fn test_delete_where() {
//     let db = Surreal::new::<Mem>(()).await.unwrap();
//     db.use_ns("test").use_db("test").await.unwrap();
//
//     let spaceship = SpaceShip {
//         id: SpaceShip::create_id(format!("num-{}", 1)),
//         name: format!("spaceship-{}", 1),
//         created: chrono::Utc::now(),
//     };
//
//     spaceship.save().run(db.clone()).await.unwrap();
//     SpaceShip::delete_where(Condition::Equal("name", spaceship.name.clone()))
//         .run(db.clone())
//         .await
//         .unwrap();
//
//     let found_spaceships = SpaceShip::find_where(Condition::Equal("name", spaceship.name.clone()))
//         .run(db.clone())
//         .await
//         .unwrap();
//     assert_eq!(found_spaceships.len(), 0);
// }
