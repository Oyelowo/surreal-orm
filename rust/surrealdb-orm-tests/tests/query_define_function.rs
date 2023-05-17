// -- Define a global function which can be used in any query
// DEFINE FUNCTION fn::get_person($first: string, $last: string, $birthday: string) {
//
// 	LET $person = SELECT * FROM person WHERE [first, last, birthday] = [$first, $last, $birthday];
//
// 	RETURN IF $person[0].id THEN
// 		$person[0]
// 	ELSE
// 		CREATE person SET first = $first, last = $last, birthday = $birthday
// 	END;
//
// };
//
// -- Call the global custom function, receiving the returned result
// LET $person = fn::get_person('Tobie', 'Morgan Hitchcock', '2022-09-21');
// define more complex function using select statement within like the raw above this file

use surrealdb_models::{spaceship_schema, SpaceShip};
use surrealdb_orm::{
    cond, index,
    statements::{create, define_function, if_, select},
    All, Operatable, SchemaGetter, SetterAssignable, SurrealdbModel, Table,
};

// const SPACE_SHIP: SpaceShip = SpaceShip::schema();
fn spaceship() -> Table {
    SpaceShip::table_name()
}

define_function!(get_person(first_arg: string, last_arg: string, birthday_arg: string) {
    let person = select(All)
        .from(spaceship())
        .where_(
            cond(SpaceShip::schema().id.equal(&first_arg))
                .and(SpaceShip::schema().name.equal(&last_arg))
                .and(SpaceShip::schema().created.equal(&birthday_arg)),
        );

    return if_(person.with_path::<SpaceShip>(index(0)).id)
        .then_(person.with_path::<SpaceShip>(index(0)))
    .else_(
            create::<SpaceShip>(
                vec![
                    SpaceShip::schema().id.equal_to(&first_arg),
                    SpaceShip::schema().name.equal_to(&last_arg),
                    SpaceShip::schema().created.equal_to(&birthday_arg),
                ]
            )
        ).end();
});

#[test]
fn test_function_definition() {}
