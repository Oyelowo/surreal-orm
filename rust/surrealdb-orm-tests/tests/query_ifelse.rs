/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use serde::{Deserialize, Serialize};
use surrealdb::{engine::local::Mem, sql, Surreal};
use surrealdb_models::{spaceship_schema, weapon_schema, SpaceShip, Weapon};
use surrealdb_orm::{
    block, chain, queries,
    statements::{if_, insert, let_, order, select, update, LetStatement},
    All, Buildable, Operatable, Param, QueryChain, ReturnableSelect, ReturnableStandard, Runnable,
    SchemaGetter, SetterAssignable, SurrealdbModel, SurrealdbOrmResult, ToRaw,
};

#[tokio::test]
async fn test_if_else_statement_and_let_with_block_macro() -> SurrealdbOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let generated_spaceships = (0..7)
        .map(|i| SpaceShip {
            id: SpaceShip::create_id(format!("num-{}", i)),
            name: format!("spaceship-{}", i),
            created: chrono::Utc::now(),
        })
        .collect::<Vec<_>>();
    insert(generated_spaceships).run(db.clone()).await?;

    let generated_weapons = (0..10)
        .map(|i| Weapon {
            strength: i,
            name: format!("weapon-{}", i),
            ..Default::default()
        })
        .collect::<Vec<_>>();
    insert(generated_weapons).run(db.clone()).await?;

    let ref space_ship = SpaceShip::table_name();
    let ref weapon = Weapon::table_name();
    let spaceship_schema::SpaceShip { name, .. } = SpaceShip::schema();
    let weapon_schema::Weapon {
        ref name,
        ref strength,
        ..
    } = Weapon::schema();

    let queries_1 = block! {
        let val = 7;
        let oye_name = "Oyelowo";
        // You can even assign a statement
        let select_space_ship = select(All).from(space_ship).order_by(order(name).desc());

        let query_result = if_(val.greater_than(5))
            .then(select_space_ship)
            .else_if(oye_name.equal("Oyelowo"))
            .then(
                select(All)
                    .from(weapon)
                    .order_by(order(strength).desc()),
            )
            .else_(2505)
            .end();
        return query_result;
    };
    insta::assert_display_snapshot!(queries_1.to_raw().build());
    insta::assert_display_snapshot!(queries_1.fine_tune_params());

    assert_eq!(
        queries_1.to_raw().build(),
        "{\n\
            LET $val = 7;\n\n\
            LET $oye_name = 'Oyelowo';\n\n\
            LET $select_space_ship = (SELECT * FROM space_ship ORDER BY name DESC);\n\n\
            LET $query_result = IF $val > 5 THEN \
            $select_space_ship \
            ELSE IF $oye_name = 'Oyelowo' THEN \
            (SELECT * FROM weapon ORDER BY strength DESC) \
            ELSE \
            2505 \
            END;\n\n\
            RETURN $query_result;\n\
            }"
    );

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(untagged)]
    enum SpaceShipOrWeapon {
        Weapon(Weapon),
        SpaceShip(SpaceShip),
        Number(u32),
    }
    let query_result_1 = select(All)
        .from(queries_1)
        .return_many::<SpaceShipOrWeapon>(db.clone())
        .await?;

    if let SpaceShipOrWeapon::SpaceShip(s) = &query_result_1[0] {
        assert_eq!(s.name, "spaceship-6");
        assert_eq!(s.id.to_string(), "space_ship:⟨num-6⟩");
    };

    // A good way to share a query across multiple blocks
    let if_else_external = |val: &LetStatement, oye_name: &LetStatement| {
        if_(val.greater_than(5))
            .then(select(All).from(space_ship).order_by(order(name).desc()))
            .else_if(oye_name.equal("Oyelowo"))
            .then(select(All).from(weapon).order_by(order(strength).desc()))
            .else_(2505)
            .end()
    };
    // If declared outside of a block
    // let_!(val = 4);
    // let_!(oye_name = "Oyelowo");
    let queries_2 = block! {
        let val = 4;
        let oye_name = "Oyelowo";
        return if_else_external(val, oye_name);
    };

    let query_result_2: Vec<SpaceShipOrWeapon> =
        select(All).from(queries_2).return_many(db.clone()).await?;

    assert_eq!(query_result_2.len(), 10);
    if let SpaceShipOrWeapon::Weapon(w) = &query_result_2[0] {
        assert_eq!(w.name, "weapon-9");
        assert!(w.id.to_string().starts_with("weapon:"));
        assert_eq!(w.strength, 9);
    };

    // Matches Else
    let queries_3 = block! {
        let val = 4;
        let oye_name = "Not Oyelowo";

        return if_(val.greater_than(5))
            .then(
                select(All)
                    .from(space_ship)
                    .order_by(order(name).desc()),
            )
            .else_if(oye_name.equal("Oyelowo"))
            .then(
                select(All)
                    .from(weapon)
                    .order_by(order(strength).desc()),
            )
            .else_(2505)
            .end();
    };

    let query_result_3 = select(All)
        .from(queries_3)
        .run(db.clone())
        .await?
        .take::<Option<SpaceShipOrWeapon>>(0)
        .unwrap();

    if let Some(SpaceShipOrWeapon::Number(n)) = &query_result_3 {
        assert_eq!(*n, 2505);
    };

    Ok(())
}

#[tokio::test]
async fn test_if_else_statement_and_let_macro() -> SurrealdbOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let generated_spaceships = (0..7)
        .map(|i| SpaceShip {
            id: SpaceShip::create_id(format!("num-{}", i)),
            name: format!("spaceship-{}", i),
            created: chrono::Utc::now(),
        })
        .collect::<Vec<_>>();
    insert(generated_spaceships).run(db.clone()).await?;

    let generated_weapons = (0..10)
        .map(|i| Weapon {
            strength: i,
            name: format!("weapon-{}", i),
            ..Default::default()
        })
        .collect::<Vec<_>>();
    insert(generated_weapons).run(db.clone()).await?;

    let space_ship = SpaceShip::table_name();
    let weapon = Weapon::table_name();
    let spaceship_schema::SpaceShip { name, .. } = SpaceShip::schema();
    let weapon_schema::Weapon { name, strength, .. } = Weapon::schema();

    let_!(val = 7);
    let_!(name = "Oyelowo");
    let if_statement = if_(val.greater_than(5))
        .then(
            select(All)
                .from(SpaceShip::table_name())
                .order_by(order(SpaceShip::schema().name).desc()),
        )
        .else_if(name.equal("Oyelowo"))
        .then(
            select(All)
                .from(Weapon::table_name())
                .order_by(order(Weapon::schema().strength).desc()),
        )
        .else_(2505)
        .end();

    let queries_1 = chain(val.clone())
        .chain(name.clone())
        .chain(if_statement.clone());

    insta::assert_display_snapshot!(queries_1.to_raw().build());
    insta::assert_display_snapshot!(queries_1.fine_tune_params());
    assert_eq!(
        queries_1.fine_tune_params(),
        "\
    LET $val = $_param_00000001;\n\n\
    LET $name = $_param_00000002;\n\n\
    IF $val > $_param_00000003 THEN\n\
    \t$_param_00000004\n\
    ELSE IF $name = $_param_00000005 THEN\n\
    \t$_param_00000006\n\
    ELSE\n\
    \t$_param_00000007\n\
    END"
    );

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(untagged)]
    enum SpaceShipOrWeapon {
        Weapon(Weapon),
        SpaceShip(SpaceShip),
        Number(u32),
    }
    let query_result_1 = queries_1
        .run(db.clone())
        .await?
        .take::<Vec<SpaceShipOrWeapon>>(2)
        .unwrap();

    assert_eq!(query_result_1.len(), 7);
    if let SpaceShipOrWeapon::SpaceShip(s) = &query_result_1[0] {
        assert_eq!(s.name, "spaceship-6");
        assert_eq!(s.id.to_string(), "space_ship:⟨num-6⟩");
    };

    let val = let_("val").equal_to(4);
    let name = let_("name").equal_to("Oyelowo");

    let queries_2 = chain(val).chain(name).chain(if_statement.clone());

    let query_result_2 = queries_2
        .run(db.clone())
        .await?
        .take::<Vec<SpaceShipOrWeapon>>(2)
        .unwrap();

    assert_eq!(query_result_2.len(), 10);
    if let SpaceShipOrWeapon::Weapon(w) = &query_result_2[0] {
        assert_eq!(w.name, "weapon-9");
        assert!(w.id.to_string().starts_with("weapon:"));
        assert_eq!(w.strength, 9);
    };

    // Matches Else
    let_!(val = 4);
    let_!(name = "Not Oyelowo");

    let queries_3 = chain(val).chain(name).chain(if_statement);

    let query_result_3 = queries_3
        .run(db.clone())
        .await?
        .take::<Vec<SpaceShipOrWeapon>>(2)
        .unwrap();

    assert_eq!(query_result_3.len(), 1);
    if let SpaceShipOrWeapon::Number(n) = &query_result_3[0] {
        assert_eq!(*n, 2505);
    };

    Ok(())
}

#[tokio::test]
async fn test_if_else_statement() -> SurrealdbOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let generated_spaceships = (0..7)
        .map(|i| SpaceShip {
            id: SpaceShip::create_id(format!("num-{}", i)),
            name: format!("spaceship-{}", i),
            created: chrono::Utc::now(),
        })
        .collect::<Vec<_>>();
    insert(generated_spaceships).run(db.clone()).await?;

    let generated_weapons = (0..10)
        .map(|i| Weapon {
            strength: i,
            name: format!("weapon-{}", i),
            ..Default::default()
        })
        .collect::<Vec<_>>();
    insert(generated_weapons).run(db.clone()).await?;

    let let_val = let_("val").equal_to(7);
    let val = || let_val.clone().get_param();

    let let_name = let_("name").equal_to("Oyelowo");
    let name = || let_name.get_param();

    let if_statement = if_(val().greater_than(5))
        .then(
            select(All)
                .from(SpaceShip::table_name())
                .order_by(order(SpaceShip::schema().name).desc()),
        )
        .else_if(name().equal("Oyelowo"))
        .then(
            select(All)
                .from(Weapon::table_name())
                .order_by(order(Weapon::schema().strength).desc()),
        )
        .else_(2505)
        .end();

    let queries_1 = chain(let_val.clone())
        .chain(let_name.clone())
        .chain(if_statement.clone());

    // insta::assert_display_snapshot!(queries.to_raw().build());
    // insta::assert_display_snapshot!(queries.fine_tune_params());
    assert_eq!(
        queries_1.fine_tune_params(),
        "\
LET $val = $_param_00000001;\n\n\
LET $name = $_param_00000002;\n\n\
IF $val > $_param_00000003 THEN\n\
\t$_param_00000004\n\
ELSE IF $name = $_param_00000005 THEN\n\
\t$_param_00000006\n\
ELSE\n\
\t$_param_00000007\n\
END"
    );

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(untagged)]
    enum SpaceShipOrWeapon {
        Weapon(Weapon),
        SpaceShip(SpaceShip),
        Number(u32),
    }
    let query_result_1 = queries_1
        .run(db.clone())
        .await?
        .take::<Vec<SpaceShipOrWeapon>>(2)
        .unwrap();

    assert_eq!(query_result_1.len(), 7);
    if let SpaceShipOrWeapon::SpaceShip(s) = &query_result_1[0] {
        assert_eq!(s.name, "spaceship-6");
        assert_eq!(s.id.to_string(), "space_ship:⟨num-6⟩");
    };

    let let_val = let_val.equal_to(4);

    let queries_2 = chain(let_val.clone())
        .chain(let_name.clone())
        .chain(if_statement.clone());

    let query_result_2 = queries_2
        .run(db.clone())
        .await?
        .take::<Vec<SpaceShipOrWeapon>>(2)
        .unwrap();
    assert_eq!(query_result_2.len(), 10);
    if let SpaceShipOrWeapon::Weapon(w) = &query_result_2[0] {
        assert_eq!(w.name, "weapon-9");
        assert!(w.id.to_string().starts_with("weapon:"));
        assert_eq!(w.strength, 9);
    };

    // Matches Else
    let let_val = let_val.equal_to(4);
    let let_name = let_name.equal_to("Not Oyelowo");

    let queries_3 = chain(let_val.clone())
        .chain(let_name.clone())
        .chain(if_statement);

    let query_result_3 = queries_3
        .run(db.clone())
        .await?
        .take::<Vec<SpaceShipOrWeapon>>(2)
        .unwrap();

    assert_eq!(query_result_3.len(), 1);
    if let SpaceShipOrWeapon::Number(n) = &query_result_3[0] {
        assert_eq!(*n, 2505);
    };

    Ok(())
}

#[tokio::test]
async fn test_if_else_in_update_statement_setter() -> SurrealdbOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();
    let weapon_schema::Weapon {
        ref strength, name, ..
    } = Weapon::schema();
    let ref weapon = Weapon::table_name();

    let generated_weapons = (0..=100)
        .map(|i| Weapon {
            strength: i,
            name: format!("weapon-{}", i),
            ..Default::default()
        })
        .collect::<Vec<_>>();
    insert(generated_weapons).run(db.clone()).await?;

    let weapons: Vec<Weapon> = select(All)
        .from(weapon)
        .order_by(order(strength).asc())
        .return_many(db.clone())
        .await?;

    assert_eq!(weapons.len(), 101);
    assert_eq!(weapons[0].name, "weapon-0");
    assert_eq!(weapons[10].name, "weapon-10");

    assert_eq!(weapons[11].name, "weapon-11");
    assert_eq!(weapons[20].name, "weapon-20");
    assert_eq!(weapons[21].name, "weapon-21");

    assert_eq!(weapons[22].name, "weapon-22");
    assert_eq!(weapons[64].name, "weapon-64");

    assert_eq!(weapons[65].name, "weapon-65");
    assert_eq!(weapons[100].name, "weapon-100");

    update::<Weapon>(weapon)
        .set(
            name.equal_to(
                if_(strength.less_than_or_equal(10))
                    .then("junior")
                    .else_if(strength.less_than_or_equal(21))
                    .then("student")
                    .else_if(strength.greater_than_or_equal(65))
                    .then("senior")
                    .else_("NULL")
                    .end(),
            ),
        )
        .run(db.clone())
        .await?;

    let weapons: Vec<Weapon> = select(All)
        .from(weapon)
        .order_by(order(strength).asc())
        .return_many(db.clone())
        .await?;

    assert_eq!(weapons.len(), 101);
    assert_eq!(weapons[0].name, "junior");
    assert_eq!(weapons[10].name, "junior");

    assert_eq!(weapons[11].name, "student");
    assert_eq!(weapons[20].name, "student");
    assert_eq!(weapons[21].name, "student");

    assert_eq!(weapons[22].name, "NULL");
    assert_eq!(weapons[64].name, "NULL");

    assert_eq!(weapons[65].name, "senior");
    assert_eq!(weapons[100].name, "senior");

    Ok(())
}
