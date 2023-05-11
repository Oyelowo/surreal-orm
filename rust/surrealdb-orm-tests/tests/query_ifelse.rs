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
use surrealdb_models::{SpaceShip, Weapon};
use surrealdb_orm::{
    statements::{chain, if_, insert, let_, select},
    All, Buildable, Operatable, ReturnableStandard, Runnable, SurrealdbModel, SurrealdbOrmResult,
    ToRaw,
};

#[tokio::test]
async fn test_if_else_statement() -> SurrealdbOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let generated_spaceships = (0..10)
        .map(|i| SpaceShip {
            id: SpaceShip::create_id(format!("spaceship-{}", i)),
            name: format!("spaceship-{}", i),
            created: chrono::Utc::now(),
        })
        .collect::<Vec<_>>();
    let created_spaceships = insert(generated_spaceships).return_many(db.clone()).await?;

    let generated_weapons = (0..10)
        .map(|i| Weapon {
            strength: i,
            ..Default::default()
        })
        .collect::<Vec<_>>();
    let created_weapons = insert(generated_weapons).return_many(db.clone()).await?;

    let let_val = let_("val").equal(7);
    let val = || let_val.get_param();

    let let_name = let_("name").equal("Oyelowo");
    let if_statement = if_(val().greater_than(8))
        .then(select(All).from(SpaceShip::table_name()))
        .else_if(let_name.get_param().equal("Oyelowo"))
        .then(select(All).from(Weapon::table_name()))
        .else_(5)
        .end();

    let queries = chain(let_val).chain(let_name).chain(if_statement);

    // insta::assert_display_snapshot!(queries.to_raw().build());
    // insta::assert_display_snapshot!(queries.fine_tune_params());
    assert_eq!(
        queries.fine_tune_params(),
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
        None,
    }
    // let result = queries.run(db.clone()).await?.take::<Vec<SpaceShip>>(2);
    let result = queries
        .run(db.clone())
        .await?
        .take::<Vec<SpaceShipOrWeapon>>(2)
        .unwrap();
    let result = result.first().unwrap();
    match result {
        SpaceShipOrWeapon::SpaceShip(s) => {
            dbg!("Fist", s);
        }
        SpaceShipOrWeapon::Weapon(w) => {
            dbg!("Giust", w);
        }
        SpaceShipOrWeapon::None => {
            dbg!("None");
        }
    };
    dbg!(&result);
    assert!(false);

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
