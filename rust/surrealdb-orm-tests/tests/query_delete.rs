use surrealdb::{
    engine::local::{Db, Mem},
    Surreal,
};
use surrealdb_models::{weapon_schema, Weapon};
use surrealdb_orm::{
    statements::{delete, insert},
    *,
};

async fn create_test_data(db: Surreal<Db>) -> Vec<Weapon> {
    let space_ships = (0..1000)
        .map(|i| Weapon {
            name: format!("weapon-{}", i),
            strength: i,
            ..Default::default() // created: chrono::Utc::now(),
        })
        .collect::<Vec<Weapon>>();
    insert(space_ships).return_many(db.clone()).await.unwrap()
}

#[tokio::test]
async fn test_delete_one_by_id() -> SurrealdbOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let weapons = create_test_data(db.clone()).await;
    let weapon1 = weapons.first().unwrap();
    let ref weapon1_id = weapon1.id.clone();

    let weapon_schema::Weapon { id, .. } = &Weapon::schema();

    let deleted_weapon_count = || async {
        Weapon::count_where(id.eq(weapon1_id))
            .get(db.clone())
            .await
            .unwrap()
    };
    assert_eq!(deleted_weapon_count().await, 1);

    delete::<Weapon>(weapon1_id).run(db.clone()).await?;

    // let found_spaceships = Weapon::find_by_id(weapon1_id);
    // assert_eq!(found_spaceships.return_many(db.clone()).await?.len(), 0);
    assert_eq!(deleted_weapon_count().await, 0);

    Ok(())
}

#[tokio::test]
async fn test_delete_one_by_model() -> SurrealdbOrmResult<()> {
    Ok(())
}

#[tokio::test]
async fn test_delete_many_by_condition() -> SurrealdbOrmResult<()> {
    Ok(())
}
