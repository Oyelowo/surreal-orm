//
// IF ELSE statement
// The IF ELSE statement can be used as a main statement, or within a parent statement, to return a value depending on whether a condition, or a series of conditions match. The statement allows for multiple ELSE IF expressions, and a final ELSE expression, with no limit to the number of ELSE IF conditional expressions.
//
// Statement syntax
// IF @condition THEN
// 	@expression
// [ ELSE IF @condition THEN
// 	@expression ... ]
// [ ELSE
// 	@expression ]
// END
// Example usage
// The following query shows example usage of this statement.
//
// IF $scope = "admin" THEN
// 	( SELECT * FROM account )
// ELSE IF $scope = "user" THEN
// 	( SELECT * FROM $auth.account )
// ELSE
// 	[]
// END
// If-else statements can also be used as subqueries within other statements.
//
// UPDATE person SET railcard =
// 	IF age <= 10 THEN
// 		'junior'
// 	ELSE IF age <= 21 THEN
// 		'student'
// 	ELSE IF age >= 65 THEN
// 		'senior'
// 	ELSE
// 		NULL
// 	END
// ;
//

use serde::{Deserialize, Serialize};
use surrealdb::{engine::local::Mem, sql, Surreal};
use surrealdb_models::{spaceship_schema, weapon_schema, SpaceShip, Weapon};
use surrealdb_orm::{
    statements::{chain, if_, insert, let_, order, select, QueryChain},
    All, Buildable, Operatable, ReturnableStandard, Runnable, SchemaGetter, SurrealdbModel,
    SurrealdbOrmResult, ToRaw,
};

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

// #[test]
// fn test_if_else_statement_with_select_statement() {
//     let user = Table::new("user");
//     let statement = if_("name").equal(select(All).from(user));
//
//     assert_eq!(
//         statement.fine_tune_params(),
//         "IF $name = $_param_00000001 THEN"
//     );
//
//     assert_eq!(
//         statement.to_raw().build(),
//         "IF $name = (SELECT * FROM user) THEN"
//     );
//
//     assert_eq!(statement.get_param().build(), "$name");
// }
//
// #[test]
// fn test_if_else_statement_with_else_if() {
//     let user = Table::new("user");
//     let statement = if_("name").equal(select(All).from(user));
//
//     assert_eq!(
//         statement.fine_tune_params(),
//         "IF $name = $_param_00000001 THEN"
//     );
//
//     assert_eq!(
//         statement.to_raw().build(),
//         "IF $name = (SELECT * FROM user) THEN"
//     );
//
//     assert_eq!(statement.get_param().build(), "$name");
// }
