use surrealdb::{engine::local::Mem, Surreal};
use surrealdb_models::{weapon_schema, weaponstats_schema, Weapon, WeaponStats};
use surrealdb_orm::{
    functions::math,
    statements::{chain, create, insert, let_, return_, select, select_value},
    *,
};

macro_rules! code_block {
    ($(let $var:ident = $value:expr;)* return $expr:expr;) => {
        {
            $(
                let $var = let_(stringify!($var)).equal_to($value);
            )*

            $(
                chain(&$var).
            )*

            chain(return_($expr)).as_block()
        }
    };
    // () => {};
}

#[tokio::test]
async fn test_code_block_with_sweet_macro_block() -> SurrealdbOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let weapon = Weapon::table_name();
    let weapon_schema::Weapon { name, strength, .. } = Weapon::schema();
    let weaponstats_schema::WeaponStats {
        averageStrength, ..
    } = WeaponStats::schema();

    let generated_weapons = (0..=14)
        .map(|i| Weapon {
            name: format!("weapon_{}", i),
            strength: i,
            ..Default::default()
        })
        .collect::<Vec<_>>();

    insert(generated_weapons).return_many(db.clone()).await?;

    let code_block = code_block! {
        let strengths = select_value(strength).from(weapon);
        let total = math::sum!(&strengths);
        let count = count!(&strengths);
        return total.divide(count);
    };

    let created_stats_statement = create::<WeaponStats>(averageStrength.equal_to(code_block));

    insta::assert_display_snapshot!(created_stats_statement.to_raw());
    insta::assert_display_snapshot!(created_stats_statement.fine_tune_params());
    assert_eq!(
        created_stats_statement.to_raw().build(),
        "CREATE weapon_stats SET averageStrength = {\n\
                LET $strengths = (SELECT VALUE strength FROM weapon);\n\n\
                LET $total = math::sum($strengths);\n\n\
                LET $count = count($strengths);\n\n\
                RETURN $total / $count;\n\
                };"
    );

    let result = created_stats_statement.get_one(db.clone()).await.unwrap();
    assert_eq!(result.average_strength, 7.5);

    Ok(())
}
#[tokio::test]
async fn test_code_block_with_macro() -> SurrealdbOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let weapon = Weapon::table_name();
    let weapon_schema::Weapon { name, strength, .. } = Weapon::schema();
    let weaponstats_schema::WeaponStats {
        averageStrength, ..
    } = WeaponStats::schema();

    let generated_weapons = (0..=14)
        .map(|i| Weapon {
            name: format!("weapon_{}", i),
            strength: i,
            ..Default::default()
        })
        .collect::<Vec<_>>();

    insert(generated_weapons).return_many(db.clone()).await?;

    let_!(strengths = select_value(strength).from(weapon));
    let_!(total = math::sum!(&strengths));
    let_!(count = count!(&strengths));
    let returned = return_(bracket(total.divide(&count)));
    let code_block = block(chain(strengths).chain(total).chain(count).chain(returned));

    let created_stats_statement = create::<WeaponStats>(averageStrength.equal_to(code_block));

    insta::assert_display_snapshot!(created_stats_statement.to_raw());
    insta::assert_display_snapshot!(created_stats_statement.fine_tune_params());
    assert_eq!(
        created_stats_statement.to_raw().build(),
        "CREATE weapon_stats SET averageStrength = {\n\
                LET $strengths = (SELECT VALUE strength FROM weapon);\n\n\
                LET $total = math::sum($strengths);\n\n\
                LET $count = count($strengths);\n\n\
                RETURN ($total / $count);\n\
                };"
    );

    let result = created_stats_statement.get_one(db.clone()).await.unwrap();
    assert_eq!(result.average_strength, 7.5);

    Ok(())
}

#[tokio::test]
async fn test_code_block_simplified() -> SurrealdbOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let weapon = Weapon::table_name();
    let weapon_schema::Weapon { name, strength, .. } = Weapon::schema();
    let weaponstats_schema::WeaponStats {
        averageStrength, ..
    } = WeaponStats::schema();

    let generated_weapons = (0..=14)
        .map(|i| Weapon {
            name: format!("weapon_{}", i),
            strength: i,
            ..Default::default()
        })
        .collect::<Vec<_>>();

    insert(generated_weapons).return_many(db.clone()).await?;

    let ref strengths = let_("strengths").equal_to(select_value(strength).from(weapon));
    let ref total = let_("total").equal_to(math::sum!(strengths));
    let ref count = let_("count").equal_to(count!(strengths));
    let return_value = return_(bracket(total.divide(count)));

    let code_block = block(
        chain(strengths)
            .chain(total)
            .chain(count)
            .chain(return_value),
    );

    let created_stats_statement = create::<WeaponStats>(averageStrength.equal_to(code_block));

    insta::assert_display_snapshot!(created_stats_statement.to_raw());
    insta::assert_display_snapshot!(created_stats_statement.fine_tune_params());
    assert_eq!(
        created_stats_statement.to_raw().build(),
        "CREATE weapon_stats SET averageStrength = {\n\
                LET $strengths = (SELECT VALUE strength FROM weapon);\n\n\
                LET $total = math::sum($strengths);\n\n\
                LET $count = count($strengths);\n\n\
                RETURN ($total / $count);\n\
                };"
    );

    let result = created_stats_statement.get_one(db.clone()).await.unwrap();
    assert_eq!(result.average_strength, 7.5);

    Ok(())
}

#[tokio::test]
async fn test_code_block() -> SurrealdbOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let weapon = Weapon::table_name();
    let weapon_schema::Weapon { name, strength, .. } = Weapon::schema();
    let generated_weapons = (0..=14)
        .map(|i| Weapon {
            name: format!("weapon_{}", i),
            strength: i,
            ..Default::default()
        })
        .collect::<Vec<_>>();

    insert(generated_weapons).return_many(db.clone()).await?;

    let ref step1_assign_strengths =
        let_("strengths").equal_to(select_value(strength).from(weapon));
    let ref strengths = step1_assign_strengths.get_param();

    let ref step2_assign_total = let_("total").equal_to(math::sum!(strengths));
    let total = step2_assign_total.get_param();

    let ref step3_assign_count = let_("count").equal_to(count!(strengths));
    let count = step3_assign_count.get_param();

    let step4_return_last = return_(bracket(total.divide(count)));

    let weaponstats_schema::WeaponStats {
        averageStrength, ..
    } = WeaponStats::schema();
    let created_stats_statement = create::<WeaponStats>(
        averageStrength.equal_to(block(
            chain(step1_assign_strengths)
                .chain(step2_assign_total)
                .chain(step3_assign_count)
                .chain(step4_return_last),
        )),
    );

    insta::assert_display_snapshot!(created_stats_statement.to_raw());
    insta::assert_display_snapshot!(created_stats_statement.fine_tune_params());
    assert_eq!(
        created_stats_statement.to_raw().build(),
        "CREATE weapon_stats SET averageStrength = {\n\
                LET $strengths = (SELECT VALUE strength FROM weapon);\n\n\
                LET $total = math::sum($strengths);\n\n\
                LET $count = count($strengths);\n\n\
                RETURN ($total / $count);\n\
                };"
    );

    let result = created_stats_statement.get_one(db.clone()).await.unwrap();
    assert_eq!(result.average_strength, 7.5);

    Ok(())
}
