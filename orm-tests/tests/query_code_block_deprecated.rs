/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use pretty_assertions::assert_eq;
use surreal_models::{weapon, weapon_stats, Weapon, WeaponStats};
use surreal_orm::{
    block_deprecated as block, chain,
    functions::{array, math},
    statements::{create, insert, let_, return_, select_value},
    *,
};
use surrealdb::{engine::local::Mem, Surreal};

#[tokio::test]
async fn test_complex_code_block_with_sweet_macro_block_and_object_partial_and_arithementic_ops(
) -> SurrealOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let weapon = &Weapon::table_name();
    let weapon::Schema { ref strength, .. } = Weapon::schema();

    let generated_weapons = (0..=14)
        .map(|i| Weapon {
            name: format!("weapon_{}", i),
            strength: i as f64,
            ..Default::default()
        })
        .collect::<Vec<_>>();

    insert(generated_weapons).return_many(db.clone()).await?;
    let created_stats_statement = create::<WeaponStats>().set(object_partial!(WeaponStats {
        // id: WeaponStats::create_simple_id(),
        averageStrength: block! {
            let strengths = select_value(strength).from(weapon);
            let total = math::sum!(strengths);
            let count = array::len!(strengths);
            return math::ceil!((((total / count) * (count * total)) / (total + 4)) * 100);
        }
    }));

    insta::assert_display_snapshot!(created_stats_statement.to_raw());
    insta::assert_display_snapshot!(created_stats_statement.fine_tune_params());
    assert_eq!(
        created_stats_statement.to_raw().build(),
        "CREATE weapon_stats SET averageStrength = {\n\
                LET $strengths = (SELECT VALUE strength FROM weapon);\n\n\
                LET $total = math::sum($strengths);\n\n\
                LET $count = array::len($strengths);\n\n\
                RETURN math::ceil(((($total / $count) * ($count * $total)) / ($total + 4)) * 100);\n\
                };"
    );

    assert_eq!(
        created_stats_statement.fine_tune_params(),
        "CREATE weapon_stats SET averageStrength = {\n\
                LET $strengths = $_param_00000001;\n\n\
                LET $total = math::sum($strengths);\n\n\
                LET $count = array::len($strengths);\n\n\
                RETURN math::ceil(((($total / $count) * ($count * $total)) / ($total + $_param_00000002)) * $_param_00000003);\n\
                };"
    );

    let result = created_stats_statement.get_one(db.clone()).await.unwrap();
    assert_eq!(result.average_strength, 10115.0);

    Ok(())
}
#[tokio::test]
async fn test_complex_code_block_with_sweet_macro_block_and_arithementic_ops(
) -> SurrealOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let weapon = &Weapon::table_name();
    let weapon::Schema { ref strength, .. } = Weapon::schema();
    let weapon_stats::Schema {
        averageStrength, ..
    } = WeaponStats::schema();

    let generated_weapons = (0..=14)
        .map(|i| Weapon {
            name: format!("weapon_{}", i),
            strength: i as f64,
            ..Default::default()
        })
        .collect::<Vec<_>>();

    insert(generated_weapons).return_many(db.clone()).await?;

    let created_stats_statement = create::<WeaponStats>().set(averageStrength.equal_to(block! {
        let strengths = select_value(strength).from(weapon);
        let total = math::sum!(strengths);
        let count = array::len!(strengths);
        return math::ceil!((((total / count) * (count * total)) / (total + 4)) * 100);
    }));

    insta::assert_display_snapshot!(created_stats_statement.to_raw());
    insta::assert_display_snapshot!(created_stats_statement.fine_tune_params());
    assert_eq!(
        created_stats_statement.to_raw().build(),
        "CREATE weapon_stats SET averageStrength = {\n\
                LET $strengths = (SELECT VALUE strength FROM weapon);\n\n\
                LET $total = math::sum($strengths);\n\n\
                LET $count = array::len($strengths);\n\n\
                RETURN math::ceil(((($total / $count) * ($count * $total)) / ($total + 4)) * 100);\n\
                };"
    );

    assert_eq!(
        created_stats_statement.fine_tune_params(),
        "CREATE weapon_stats SET averageStrength = {\n\
                LET $strengths = $_param_00000001;\n\n\
                LET $total = math::sum($strengths);\n\n\
                LET $count = array::len($strengths);\n\n\
                RETURN math::ceil(((($total / $count) * ($count * $total)) / ($total + $_param_00000002)) * $_param_00000003);\n\
                };"
    );

    let result = created_stats_statement.get_one(db.clone()).await.unwrap();
    assert_eq!(result.average_strength, 10115.0);

    Ok(())
}

#[tokio::test]
async fn test_code_block_with_sweet_macro_block_and_arithementic_ops() -> SurrealOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let weapon = &Weapon::table_name();
    let weapon::Schema { ref strength, .. } = Weapon::schema();
    let weapon_stats::Schema {
        averageStrength, ..
    } = WeaponStats::schema();

    let generated_weapons = (0..=14)
        .map(|i| Weapon {
            name: format!("weapon_{}", i),
            strength: i as f64,
            ..Default::default()
        })
        .collect::<Vec<_>>();

    insert(generated_weapons).return_many(db.clone()).await?;
    let created_stats_statement = create::<WeaponStats>().set(averageStrength.equal_to(block! {
        let strengths = select_value(strength).from(weapon);
        let total = math::sum!(strengths);
        let count = array::len!(strengths);
        return total / count;
    }));
    insta::assert_display_snapshot!(created_stats_statement.to_raw());
    insta::assert_display_snapshot!(created_stats_statement.fine_tune_params());
    assert_eq!(
        created_stats_statement.to_raw().build(),
        "CREATE weapon_stats SET averageStrength = {\n\
                LET $strengths = (SELECT VALUE strength FROM weapon);\n\n\
                LET $total = math::sum($strengths);\n\n\
                LET $count = array::len($strengths);\n\n\
                RETURN $total / $count;\n\
                };"
    );

    let result = created_stats_statement.get_one(db.clone()).await.unwrap();
    assert_eq!(result.average_strength, 7.0);

    Ok(())
}

#[tokio::test]
async fn test_code_block_with_sweet_macro_block() -> SurrealOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let weapon = &Weapon::table_name();
    let weapon::Schema { ref strength, .. } = Weapon::schema();
    let weapon_stats::Schema {
        averageStrength, ..
    } = WeaponStats::schema();

    let generated_weapons = (0..=14)
        .map(|i| Weapon {
            name: format!("weapon_{}", i),
            strength: i as f64,
            ..Default::default()
        })
        .collect::<Vec<_>>();

    insert(generated_weapons).return_many(db.clone()).await?;

    let created_stats_statement = create::<WeaponStats>().set(averageStrength.equal_to(block! {
        let strengths = select_value(strength).from(weapon);
        let total = math::sum!(strengths);
        let count = array::len!(strengths);
        return total.divide(count);
    }));
    insta::assert_display_snapshot!(created_stats_statement.to_raw());
    insta::assert_display_snapshot!(created_stats_statement.fine_tune_params());
    assert_eq!(
        created_stats_statement.to_raw().build(),
        "CREATE weapon_stats SET averageStrength = {\n\
                LET $strengths = (SELECT VALUE strength FROM weapon);\n\n\
                LET $total = math::sum($strengths);\n\n\
                LET $count = array::len($strengths);\n\n\
                RETURN $total / $count;\n\
                };"
    );

    let result = created_stats_statement.get_one(db.clone()).await.unwrap();
    assert_eq!(result.average_strength, 7.0);

    Ok(())
}
#[tokio::test]
async fn test_code_block_with_macro() -> SurrealOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let weapon = Weapon::table_name();
    let weapon::Schema { strength, .. } = Weapon::schema();
    let weapon_stats::Schema {
        averageStrength, ..
    } = WeaponStats::schema();

    let generated_weapons = (0..=14)
        .map(|i| Weapon {
            name: format!("weapon_{}", i),
            strength: i as f64,
            ..Default::default()
        })
        .collect::<Vec<_>>();

    insert(generated_weapons).return_many(db.clone()).await?;

    let_!(strengths = select_value(strength).from(weapon));
    let_!(total = math::sum!(&strengths));
    let_!(count = array::len!(&strengths));
    let returned = return_(bracket(total.divide(&count)));
    let code_block = block(chain(strengths).chain(total).chain(count).chain(returned));

    let created_stats_statement = create::<WeaponStats>().set(averageStrength.equal_to(code_block));

    insta::assert_display_snapshot!(created_stats_statement.to_raw());
    insta::assert_display_snapshot!(created_stats_statement.fine_tune_params());
    assert_eq!(
        created_stats_statement.to_raw().build(),
        "CREATE weapon_stats SET averageStrength = {\n\
                LET $strengths = (SELECT VALUE strength FROM weapon);\n\n\
                LET $total = math::sum($strengths);\n\n\
                LET $count = array::len($strengths);\n\n\
                RETURN ($total / $count);\n\
                };"
    );

    let result = created_stats_statement.get_one(db.clone()).await.unwrap();
    assert_eq!(result.average_strength, 7.0);

    Ok(())
}

#[tokio::test]
async fn test_code_block_simplified() -> SurrealOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let weapon = Weapon::table_name();
    let weapon::Schema { strength, .. } = Weapon::schema();
    let weapon_stats::Schema {
        averageStrength, ..
    } = WeaponStats::schema();

    let generated_weapons = (0..=14)
        .map(|i| Weapon {
            name: format!("weapon_{}", i),
            strength: i as f64,
            ..Default::default()
        })
        .collect::<Vec<_>>();

    insert(generated_weapons).return_many(db.clone()).await?;

    let strengths = &let_("strengths").equal_to(select_value(strength).from(weapon));
    let total = &let_("total").equal_to(math::sum!(strengths));
    let count = &let_("count").equal_to(array::len!(strengths));
    let return_value = return_(bracket(total.divide(count)));

    let code_block = block(
        chain(strengths)
            .chain(total)
            .chain(count)
            .chain(return_value),
    );

    let created_stats_statement = create::<WeaponStats>().set(averageStrength.equal_to(code_block));

    insta::assert_display_snapshot!(created_stats_statement.to_raw());
    insta::assert_display_snapshot!(created_stats_statement.fine_tune_params());
    assert_eq!(
        created_stats_statement.to_raw().build(),
        "CREATE weapon_stats SET averageStrength = {\n\
                LET $strengths = (SELECT VALUE strength FROM weapon);\n\n\
                LET $total = math::sum($strengths);\n\n\
                LET $count = array::len($strengths);\n\n\
                RETURN ($total / $count);\n\
                };"
    );

    let result = created_stats_statement.get_one(db.clone()).await.unwrap();
    assert_eq!(result.average_strength, 7.0);

    Ok(())
}

#[tokio::test]
async fn test_code_block() -> SurrealOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let weapon = Weapon::table_name();
    let weapon::Schema { strength, .. } = Weapon::schema();
    let generated_weapons = (0..=14)
        .map(|i| Weapon {
            name: format!("weapon_{}", i),
            strength: i as f64,
            ..Default::default()
        })
        .collect::<Vec<_>>();

    insert(generated_weapons).return_many(db.clone()).await?;

    let step1_assign_strengths = &let_("strengths").equal_to(select_value(strength).from(weapon));
    let strengths = &step1_assign_strengths.get_param();

    let step2_assign_total = &let_("total").equal_to(math::sum!(strengths));
    let total = step2_assign_total.get_param();

    let step3_assign_count = &let_("count").equal_to(array::len!(strengths));
    let count = step3_assign_count.get_param();

    let step4_return_last = return_(bracket(total.divide(count)));

    let weapon_stats::Schema {
        averageStrength, ..
    } = WeaponStats::schema();
    let created_stats_statement = create::<WeaponStats>().set(
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
                LET $count = array::len($strengths);\n\n\
                RETURN ($total / $count);\n\
                };"
    );

    let result = created_stats_statement.get_one(db.clone()).await.unwrap();
    assert_eq!(result.average_strength, 7.0);

    Ok(())
}
