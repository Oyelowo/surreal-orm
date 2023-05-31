use surrealdb_models::{spaceship_schema, weapon_schema, SpaceShip, Weapon};
use surrealdb_orm::{Field, SchemaGetter};

#[test]
fn rerer() {
    let age = Field::new("age");
    let name = Field::new("name");
    let email = Field::new("email");
    let surname = Field::new("surname");
    // let age_name = age + name + email + surname;
    // let age_name = (age + surname) / 3;
    // let age_name = (age + surname) / (name + email);
    // let age_name = (age + surname) * (name + email);
    // let age_name = (age + surname) + (name + email);
    // dbg!(age_name);
    // assert!(false);
    // let age_name = (age + surname) / name;
    // let age_name = age + name + email + surname;
    let weapon_schema::Weapon {
        id, ref strength, ..
    } = Weapon::schema();

    // let age_name = id + created + email;
    let age_name = strength + strength;
    // let age_name = id + created + email;
    // let age_name = age + name + email + surname;
}
