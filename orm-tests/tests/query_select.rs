/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use chrono::Utc;
use pretty_assertions::assert_eq;
use surreal_models::{weapon, SpaceShip, Weapon};
use surreal_orm::{
    statements::{insert, order, select, select_value},
    *,
};
use surrealdb::{engine::local::Mem, Surreal};

#[tokio::test]
async fn test_subquery_in_select_statement() -> SurrealOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let generated_spaceships = (1..=10)
        .map(|i| Weapon {
            name: format!("Weapon {}", i),
            strength: i as f64 * 10.0,
            created: Utc::now(),
            ..Default::default()
        })
        .collect::<Vec<_>>();
    insert(generated_spaceships.clone()).run(db.clone()).await?;

    let weapon = &Weapon::table();
    let weapon::Schema { ref strength, .. } = Weapon::schema();

    let statement = select(All)
        .from(weapon)
        .where_(
            strength.inside(
                select_value(strength)
                    .from(weapon)
                    .order_by(strength.asc())
                    .limit(6),
            ),
        )
        .order_by(strength.desc())
        .start(2)
        .limit(10);

    assert_eq!(
        statement.fine_tune_params(),
        "SELECT * FROM weapon WHERE strength INSIDE $_param_00000001 \
            ORDER BY strength DESC LIMIT $_param_00000002 START AT $_param_00000003;"
    );
    assert_eq!(
        statement.to_raw().build(),
        "SELECT * FROM weapon WHERE strength INSIDE \
            (SELECT VALUE strength FROM weapon ORDER BY strength LIMIT 6) \
            ORDER BY strength DESC LIMIT 10 START AT 2;"
    );
    let statement = statement.return_many::<SpaceShip>(db.clone()).await?;

    assert_eq!(&statement[0].name, "Weapon 4");
    assert_eq!(&statement[1].name, "Weapon 3");
    assert_eq!(&statement[2].name, "Weapon 2");
    assert_eq!(&statement[3].name, "Weapon 1");

    assert_eq!(statement.len(), 4);
    assert!(statement[0].id.to_string().starts_with("weapon:"));
    Ok(())
}

#[tokio::test]
async fn test_subquery_in_select_statement_with_order_functions() -> SurrealOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let generated_spaceships = (1..=10)
        .map(|i| Weapon {
            name: format!("Weapon {}", i),
            strength: i as f64 * 10f64,
            created: Utc::now(),
            ..Default::default()
        })
        .collect::<Vec<_>>();
    insert(generated_spaceships.clone()).run(db.clone()).await?;

    let weapon = &Weapon::table();
    let weapon::Schema { ref strength, .. } = Weapon::schema();

    let statement = select(All)
        .from(weapon)
        .where_(
            strength.inside(
                select_value(strength)
                    .from(weapon)
                    .order_by(order(strength).asc())
                    .limit(6),
            ),
        )
        .order_by(order(strength).desc())
        .start(2)
        .limit(10);

    assert_eq!(
        statement.fine_tune_params(),
        "SELECT * FROM weapon WHERE strength INSIDE $_param_00000001 \
            ORDER BY strength DESC LIMIT $_param_00000002 START AT $_param_00000003;"
    );
    assert_eq!(
        statement.to_raw().build(),
        "SELECT * FROM weapon WHERE strength INSIDE \
            (SELECT VALUE strength FROM weapon ORDER BY strength LIMIT 6) \
            ORDER BY strength DESC LIMIT 10 START AT 2;"
    );
    let statement = statement.return_many::<SpaceShip>(db.clone()).await?;

    assert_eq!(&statement[0].name, "Weapon 4");
    assert_eq!(&statement[1].name, "Weapon 3");
    assert_eq!(&statement[2].name, "Weapon 2");
    assert_eq!(&statement[3].name, "Weapon 1");

    assert_eq!(statement.len(), 4);
    assert!(statement[0].id.to_string().starts_with("weapon:"));
    Ok(())
}
