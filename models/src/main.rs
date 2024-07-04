use surreal_models::spaceship::SpaceShip;
use surreal_models::user;
use surreal_models::Rocket;
use surreal_orm::*;
// use surreal_orm::PartialUpdater;
//
// type Lala<'a, T> = <Weapon<'a, T> as PartialUpdater>::StructPartial;
//
fn main() {
    let x = 5;
    let rocket = Rocket::partial_builder()
        .something(907)
        .something2(Some("Mars"))
        .build();

    // let x = Weapon::partial_builder()
    //     .name("Oyelowo".into())
    //     .something(45)
    //     // .strength(2.0)
    //     .rocket(rocket)
    //     .build();
    // // Weapon::pa
    // println!("rocket{:?}", serde_json::to_string(&x).unwrap());
    //
    let sh = SpaceShip::partial_builder().name("Banff".into()).build();
    println!("Rocket: {:?}", serde_json::to_string(&rocket).unwrap());
    println!("spaceship{:?}", serde_json::to_string(&sh).unwrap());
}
