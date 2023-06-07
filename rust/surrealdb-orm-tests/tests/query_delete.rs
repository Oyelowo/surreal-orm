use surrealdb::{
    engine::local::{Db, Mem},
    Surreal,
};
use surrealdb_models::{spaceship_schema, weapon_schema, SpaceShip, Weapon};
use surrealdb_orm::{
    statements::{delete, insert, select, select_value},
    *,
};

async fn create_test_data(db: Surreal<Db>) {
    let space_ships = (0..1000)
        .map(|i| Weapon {
            // id: Weapon::create_id(format!("num-{}", i)),
            name: format!("weapon-{}", i),
            strength: i,
            ..Default::default() // created: chrono::Utc::now(),
        })
        .collect::<Vec<Weapon>>();
    insert(space_ships).run(db.clone()).await.unwrap();
}

#[tokio::test]
async fn test_delete_one_by_id() -> SurrealdbOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    create_test_data(db.clone()).await;

    let weapon_schema::Weapon { id, .. } = &Weapon::schema();

    let total_spaceships = Weapon::find_where(id.is_not(NONE))
        .return_many(db.clone())
        .await?
        .len();
    assert_eq!(total_spaceships, 1000);
    let total_spaceships: Option<i32> = select_value(Field::new("count"))
        .from(
            select(count!(Field::new("strength").gte(500)))
                .from(Weapon::table_name())
                .group_all(),
        )
        // .group_by(id)
        .return_one(db.clone())
        .await?;
    let total_spaceships: Option<i32> = select_value(Field::new("count"))
        .from(
            select(count!(Field::new("strength").gte(500)))
                .from(Weapon::table_name())
                .group_all(),
        )
        // .group_by(id)
        .return_one(db.clone())
        .await?;

    // assert_eq!(
    //     select(All)
    //         .from(count!(SpaceShip::table_name()))
    //         // .group_all()
    //         .to_raw()
    //         .build(),
    //     ""
    // );
    assert_eq!(total_spaceships, Some(1000));
    dbg!(total_spaceships);
    // assert!(false);
    // assert_eq!(total_spaceships, Some(1000));

    delete::<SpaceShip>(SpaceShip::create_id("num-1"))
        .run(db.clone())
        .await
        .unwrap();

    let found_spaceships = SpaceShip::find_by_id(SpaceShip::create_id("num-1"));
    assert_eq!(found_spaceships.return_many(db.clone()).await?.len(), 0);

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
