use std::any::Any;

use serde::Serialize;

#[derive(Debug, Serialize)]
struct Person<'a, T: 'a, U: 'a> {
    name: String,
    age: u8,
    some: &'a T,
    another: &'a U,
}

trait PersonPickable {
    type name;
    type age;
    type some;
    type another;
}

// impl<'a, T> PersonPicker for Person<'a, T> {
impl<'a, T: 'a, U: 'a> PersonPickable for Person<'a, T, U> {
    type name = String;
    type age = u8;
    type some = &'a T;
    type another = &'a U;
}
// struct PickedPerson<'a, T> {
//     name: <Person<'a, T> as PersonPicker>::name,
// }
struct PickedPerson<'a> {
    name: <Person<'a, std::marker::PhantomData<dyn Any>, std::marker::PhantomData<dyn Any>> as PersonPickable>::name,
    // __phantom_data: std::marker::PhantomData<&'a T>,
    // kaka: T
}

struct PickedPersonAll<'a, U> {
    // name: <Person<'a, std::marker::PhantomData<dyn Any>> as PersonPickable>::name,
    name: <Person<'a, std::marker::PhantomData<dyn Any>, U> as PersonPickable>::name,
    // kaka: &'a std::marker::PhantomData<dyn Any>, U
    // some: <Person<'a, std::marker::PhantomData<dyn Any>, U> as PersonPickable>::some,
    another: <Person<'a, std::marker::PhantomData<dyn Any>, U, > as PersonPickable>::another,
}

fn main() {
    // let person = Person<'a, T> {
    let person = Person {
        name: "Oyelowo".into(),
        age: 25,
        some: &43,
        another: &"kaka",
    };

    // let something = PickedPerson::<'_, u32>{
    let something = PickedPerson {
        name: "Oyelowo".into(),
        // name: person.name,
        // kaka: 43,
        // __phantom_data: std::marker::PhantomData,
    };
    // std::marker::PhantomData<dyn Any>
    let p2 = PickedPersonAll {
        name: "Oyelowo".into(),
        // some: &43,
        another: &"kaka",
    };
}

// pick!(PickedPerson, Person, [name]);
//
// #[pick(Person, [name])]
// #[pick(AnotherPerson, [age])]
// struct PickedPerson {
//     more_fields: u8,
// }
//
//
// #[derive(Serialize, Deserialize)]
// struct NewPerson {
//     #[serde(flatten)]
//     picked_person: PickedPerson,
//     more_fields: u8,
// }
