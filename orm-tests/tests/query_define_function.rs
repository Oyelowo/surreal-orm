/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */
// -- Define a global function which can be used in any query

use chrono::TimeZone;
use pretty_assertions::assert_eq;
use surreal_models::SpaceShip;
use surreal_orm::{
    cond, define_function, object,
    statements::{create, if_, select},
    All, Buildable, Field, Model, Operatable, Param, SchemaGetter, SetterAssignable, ToRaw, NONE,
};

define_function!(get_or_create_spaceship(
    first_arg: string,
    last_arg: string,
    birthday_arg: datetime,
    _very_complex_type: int | option<float> | array<option<string>|int|null, 10> | set<option<number>|float|null, 10> | option<array> | option<set<option<int>>>
) {
    let person = select(All)
        .from(SpaceShip::table())
        .where_(
            cond(SpaceShip::schema().id.equal(first_arg))
                .and(SpaceShip::schema().name.equal(last_arg))
                .and(SpaceShip::schema().created.equal(birthday_arg)),
        );

    if person.with_path::<SpaceShip>([0]).id.is_not(NONE) {
        return person;
    } else {
        return create::<SpaceShip>().set(
                    object!(SpaceShip {
                        id: first_arg,
                        name: last_arg,
                        created: birthday_arg,
                    })
                );
    };
});

#[test]
fn test_function_definition_with_idiomatic_if_statement() {
    let dt = chrono::Utc.timestamp_opt(61, 0).unwrap();
    let spaceship = get_or_create_spaceship_fn("Oyelowo", "Oyedayo", dt, 5);
    insta::assert_display_snapshot!(spaceship.to_raw().build());
    insta::assert_display_snapshot!(spaceship.fine_tune_params());

    let spaceship = get_or_create_spaceship_fn("Oyelowo", "Oyedayo", Param::new("birthday"), 5);
    insta::assert_display_snapshot!(spaceship.to_raw().build());
    insta::assert_display_snapshot!(spaceship.fine_tune_params());

    let spaceship = get_or_create_spaceship_fn("Oyelowo", "Oyedayo", Field::new("birthday"), 5);
    insta::assert_display_snapshot!(spaceship.to_raw().build());
    insta::assert_display_snapshot!(spaceship.fine_tune_params());

    let spaceship = get_or_create_spaceship!("Oyelowo", "Oyedayo", dt, 5);
    insta::assert_display_snapshot!(spaceship.to_raw().build());
    insta::assert_display_snapshot!(spaceship.fine_tune_params());

    let spaceship = get_or_create_spaceship!(
        Field::new("first_name"),
        "Oyedayo",
        Param::new("birthday"),
        5
    );
    insta::assert_display_snapshot!(spaceship.to_raw().build());
    insta::assert_display_snapshot!(spaceship.fine_tune_params());

    let spaceship = get_or_create_spaceship!(
        "Oyelowo",
        Param::new("last_name"),
        Field::new("birthday"),
        5
    );
    insta::assert_display_snapshot!(spaceship.to_raw().build());
    insta::assert_display_snapshot!(spaceship.fine_tune_params());

    assert_eq!(
        spaceship.to_raw().build(),
        "fn::get_or_create_spaceship('Oyelowo', $last_name, birthday, 5)"
    );
    assert_eq!(
        spaceship.fine_tune_params(),
        "fn::get_or_create_spaceship($_param_00000001, $last_name, birthday, $_param_00000002)"
    );

    let spaceship_statement = get_or_create_spaceship_statement();
    insta::assert_display_snapshot!(spaceship_statement.to_raw().build());
    insta::assert_display_snapshot!(spaceship_statement.fine_tune_params());
}

define_function!(get_person(first_arg: string, last_arg: string, birthday_arg: string) {
    let person = select(All)
        .from(SpaceShip::table())
        .where_(
            cond(SpaceShip::schema().id.equal(first_arg))
                .and(SpaceShip::schema().name.equal(last_arg))
                .and(SpaceShip::schema().created.equal(birthday_arg)),
        );

    return if_(person.with_path::<SpaceShip>([0]).id.is_not(NONE))
                .then(person)
            .else_(
                create::<SpaceShip>().set(
                    vec![
                        SpaceShip::schema().id.equal_to(first_arg),
                        SpaceShip::schema().name.equal_to(last_arg),
                        SpaceShip::schema().created.equal_to(birthday_arg),
                    ]
                )
            ).end();
});

#[test]
fn test_function_definition() {
    let person = get_person!("Oyelowo", "Oyedayo", "2022-09-21");
    insta::assert_display_snapshot!(person.to_raw().build());
    insta::assert_display_snapshot!(person.fine_tune_params());
    assert_eq!(
        person.to_raw().build(),
        "fn::get_person('Oyelowo', 'Oyedayo', '2022-09-21')"
    );
    assert_eq!(
        person.fine_tune_params(),
        "fn::get_person($_param_00000001, $_param_00000002, $_param_00000003)"
    );

    let person_statement = get_person_statement();
    insta::assert_display_snapshot!(person_statement.to_raw().build());
    insta::assert_display_snapshot!(person_statement.fine_tune_params());
}
