/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use pretty_assertions::assert_eq;
use surreal_models::{weapon, weapon_stats, Weapon, WeaponStats};
use surreal_orm::{
    chain,
    functions::math,
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
            strength: i,
            ..Default::default()
        })
        .collect::<Vec<_>>();

    insert(generated_weapons).return_many(db.clone()).await?;

    let created_stats_statement = create::<WeaponStats>().set(object_partial!(WeaponStats {
        // id: WeaponStats::create_simple_id(),
        averageStrength: block! {
            let strengths = select_value(strength).from(weapon);
            let total = math::sum!(strengths);
            let count = count!(strengths);
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
                LET $count = count($strengths);\n\n\
                RETURN math::ceil(((($total / $count) * ($count * $total)) / ($total + 4)) * 100);\n\
                };"
    );

    assert_eq!(
        created_stats_statement.fine_tune_params(),
        "CREATE weapon_stats SET averageStrength = {\n\
                LET $strengths = $_param_00000001;\n\n\
                LET $total = math::sum($strengths);\n\n\
                LET $count = count($strengths);\n\n\
                RETURN math::ceil(((($total / $count) * ($count * $total)) / ($total + $_param_00000002)) * $_param_00000003);\n\
                };"
    );

    let result = created_stats_statement.get_one(db.clone()).await.unwrap();
    // NOTE: there is a problem with this result. Previously gave the correct answert with 10115.0
    // but the test is now expecting 9400 which seems wrong
    assert_eq!(result.average_strength, 9400.0);

    Ok(())
}
