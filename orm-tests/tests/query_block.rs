/*
 * Email: oyelowo.oss@gmail.com
* Author: Oyelowo Oyedayo
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use pretty_assertions::assert_eq;
use surreal_models::{weapon, Weapon, WeaponStats};
use surreal_orm::{
    functions::{array, math},
    statements::{create, insert, select_value},
    *,
};
use surrealdb::{engine::local::Mem, Surreal};

#[tokio::test]
async fn test_complex_code_block_with_sweet_macro_block_and_object_partial_and_arithementic_ops(
) -> SurrealOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let weapon = &Weapon::table();
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
            let total = math::sum!(strengths); // 105
            let count = array::len!(strengths);     // 15
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
