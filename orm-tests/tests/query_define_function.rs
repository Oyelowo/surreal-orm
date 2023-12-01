/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */
// -- Define a global function which can be used in any query

use pretty_assertions::assert_eq;
use surreal_models::SpaceShip;
use surreal_orm::{
    cond, define_function, index,
    statements::{create, if_, select},
    All, Buildable, Model, Operatable, SchemaGetter, SetterAssignable, ToRaw, NONE,
};

// define_function!(get_person(very_complex_type: string | int , two: option<array<option<int | string >>> ) {
// define_function!(get_person(first: int , very_complex_type: int | option<float> | array<option<string|int|null, 10> | set<option<number>|float|null, 10> | option<array> | option<set<option<int>, 34>>) {
// define_function!(get_person(one: int | option<float> | array<option<string>|int|null, 10> | set<option<number>|float|null, 10> | option<array> | option<set<option<int>>>,
// first: int, two: array<option<string | int | null> | number, 65>  | set<option<number>|float|null, 10>  | duration | option<datetime>, third: float) {
// define_function!(get_person(very_complex_type: int | option<float> | array<option<string>|int|null, 10> | set<option<number>|float|null, 10> | option<array> | option<set<option<int>>>) {
define_function!(get_person(first_arg: string, last_arg: string, birthday_arg: string) {
    let person = select(All)
        .from(SpaceShip::table_name())
        .where_(
            cond(SpaceShip::schema().id.equal(&first_arg))
                .and(SpaceShip::schema().name.equal(&last_arg))
                .and(SpaceShip::schema().created.equal(&birthday_arg)),
        );

    if person.with_path::<SpaceShip>(index(0)).id.is_not(NONE) {
        return person;
    } else {
        create::<SpaceShip>().set(
                    vec![
                        SpaceShip::schema().id.equal_to(&first_arg),
                        SpaceShip::schema().name.equal_to(&last_arg),
                        SpaceShip::schema().created.equal_to(&birthday_arg),
                    ]
                );
    };
    return 5;
});

// define_function!(get_person(first_arg: string, last_arg: string, birthday_arg: string) {
//     let person = select(All)
//         .from(SpaceShip::table_name())
//         .where_(
//             cond(SpaceShip::schema().id.equal(&first_arg))
//                 .and(SpaceShip::schema().name.equal(&last_arg))
//                 .and(SpaceShip::schema().created.equal(&birthday_arg)),
//         );
//
//     return if_(person.with_path::<SpaceShip>(index(0)).id.is_not(NONE))
//                 .then(person)
//             .else_(
//                 create::<SpaceShip>().set(
//                     vec![
//                         SpaceShip::schema().id.equal_to(&first_arg),
//                         SpaceShip::schema().name.equal_to(&last_arg),
//                         SpaceShip::schema().created.equal_to(&birthday_arg),
//                     ]
//                 )
//             ).end();
// });

#[test]
fn test_function_definition() {
    let person = get_person!("Oyelowo", "Oyedayo", "2022-09-21");
    insta::assert_display_snapshot!(person.to_raw().build());
    insta::assert_display_snapshot!(person.fine_tune_params());
    assert_eq!(
        person.to_raw().build(),
        "get_person('Oyelowo', 'Oyedayo', '2022-09-21')"
    );
    assert_eq!(
        person.fine_tune_params(),
        "get_person($_param_00000001, $_param_00000002, $_param_00000003)"
    );

    let person_statement = get_person_statement();
    insta::assert_display_snapshot!(person_statement.to_raw().build());
    insta::assert_display_snapshot!(person_statement.fine_tune_params());

    assert_eq!(
        person_statement.to_raw().build(),
        "DEFINE FUNCTION get_person($first_arg: string, $last_arg: string, $birthday_arg: string) {\n\
            LET $person = (SELECT * FROM space_ship WHERE (id = $first_arg) AND (name = $last_arg) AND \
            (created = $birthday_arg));\n\n\
            RETURN IF $person[0].id != NONE THEN \
            $person[0] \
            ELSE \
            (CREATE space_ship SET id = $first_arg, name = $last_arg, created = $birthday_arg) \
            END;\n\
            };"
    );

    assert_eq!(person_statement.fine_tune_params(),
    "DEFINE FUNCTION get_person($first_arg: string, $last_arg: string, $birthday_arg: string) {\n\
            LET $person = $_param_00000001;\n\n\
            RETURN $_param_00000002;\n\
            };"
    );
}

/*
*
*
* // define_function!(get_person(first_arg: string, last_arg: string, birthday_arg: string) {
//     let person = select(All)
//         .from(SpaceShip::table_name())
//         .where_(
//             cond(SpaceShip::schema().id.equal(&first_arg))
//                 .and(SpaceShip::schema().name.equal(&last_arg))
//                 .and(SpaceShip::schema().created.equal(&birthday_arg)),
//         );
//
//     return if_(person.with_path::<SpaceShip>(index(0)).id.is_not(NONE))
//                 .then(person)
//             .else_(
//                 create::<SpaceShip>().set(
//                     vec![
//                         SpaceShip::schema().id.equal_to(&first_arg),
//                         SpaceShip::schema().name.equal_to(&last_arg),
//                         SpaceShip::schema().created.equal_to(&birthday_arg),
//                     ]
//                 )
//             ).end();
// });
//
// #[test]
// fn test_function_definition() {
//     let person = get_person("Oyelowo", "Oyedayo", "2022-09-21");
//     insta::assert_display_snapshot!(person.to_raw().build());
//     insta::assert_display_snapshot!(person.fine_tune_params());
//     assert_eq!(
//         person.to_raw().build(),
//         "get_person('Oyelowo', 'Oyedayo', '2022-09-21')"
//     );
//     assert_eq!(
//         person.fine_tune_params(),
//         "get_person($_param_00000001, $_param_00000002, $_param_00000003)"
//     );
//
//     let person_statement = get_person_statement();
//     insta::assert_display_snapshot!(person_statement.to_raw().build());
//     insta::assert_display_snapshot!(person_statement.fine_tune_params());
//
//     assert_eq!(
//         person_statement.to_raw().build(),
//         "DEFINE FUNCTION get_person($first_arg: string, $last_arg: string, $birthday_arg: string) {\n\
//             LET $person = (SELECT * FROM space_ship WHERE (id = $first_arg) AND (name = $last_arg) AND \
//             (created = $birthday_arg));\n\n\
//             RETURN IF $person[0].id != NONE THEN \
//             $person[0] \
//             ELSE \
//             (CREATE space_ship SET id = $first_arg, name = $last_arg, created = $birthday_arg) \
//             END;\n\
//             };"
//     );
//
//     assert_eq!(person_statement.fine_tune_params(),
//     "DEFINE FUNCTION get_person($first_arg: string, $last_arg: string, $birthday_arg: string) {\n\
//             LET $person = $_param_00000001;\n\n\
//             RETURN $_param_00000002;\n\
//             };"
//     );
// }
//
//

// define_function!(get_person(first_arg: string, last_arg: string, birthday_arg: string) {
//     let person = select(All)
//         .from(SpaceShip::table_name())
//         .where_(
//             cond(SpaceShip::schema().id.equal(&first_arg))
//                 .and(SpaceShip::schema().name.equal(&last_arg))
//                 .and(SpaceShip::schema().created.equal(&birthday_arg)),
//         );
//
//     return if_(person.with_path::<SpaceShip>(index(0)).id.is_not(NONE))
//                 .then(person)
//             .else_(
//                 create::<SpaceShip>().set(
//                     vec![
//                         SpaceShip::schema().id.equal_to(&first_arg),
//                         SpaceShip::schema().name.equal_to(&last_arg),
//                         SpaceShip::schema().created.equal_to(&birthday_arg),
//                     ]
//                 )
//             ).end();
// });
//
//
// define_function!(get_person(first_arg: string, last_arg: string, birthday_arg: string) {
//     let person = select(All)
//         .from(SpaceShip::table_name())
//         .where_(
//             cond(SpaceShip::schema().id.equal(&first_arg))
//                 .and(SpaceShip::schema().name.equal(&last_arg))
//                 .and(SpaceShip::schema().created.equal(&birthday_arg)),
//         );
//
//     if person.with_path::<SpaceShip>(index(0)).id.is_not(NONE) {
//         return person;
//     } else {
//         return create::<SpaceShip>().set(
//             vec![
//                 SpaceShip::schema().id.equal_to(&first_arg),
//                 SpaceShip::schema().name.equal_to(&last_arg),
//                 SpaceShip::schema().created.equal_to(&birthday_arg),
//             ]
//         );
//     };
// });
// //

*
* */
