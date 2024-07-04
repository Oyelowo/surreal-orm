use surreal_models::{Rocket, Weapon};
use surreal_orm::PartialUpdater;

// type Lala<'a, T> = <Weapon<'a, T> as PartialUpdater>::StructPartial;

fn main() {
    let rocket = Rocket::partial_builder()
        .something(907)
        .something2(None)
        .build();
    let x = Weapon::partial_builder()
        .name("Oyelowo".into())
        .something(45)
        .rocket(rocket)
        .build();
    // Weapon::pa
    println!("{:?}", serde_json::to_string(&x).unwrap());
}
