use surreal_models::{Rocket, Weapon};
use surreal_orm::PartialUpdater;

// type Lala<'a, T> = <Weapon<'a, T> as PartialUpdater>::StructPartial;

fn main() {
    let rocket = Rocket::partial_builder()
        .strength(907)
        .name("Ye".into())
        .build();
    let x = Weapon::partial_builder()
        .name("Oyelowo".into())
        .strength(45.0)
        .rocket(rocket)
        .build();
    // Weapon::pa
    println!("{:?}", serde_json::to_string(&x).unwrap());
}
