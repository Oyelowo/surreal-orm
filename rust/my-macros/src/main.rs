// For testing macros made here

use my_macros::{HelloMacro, MyTrait, KeyNamesGetter};
use serde::{Serialize, Deserialize};
// use serde::{Serialize, Deserialize};

#[derive(HelloMacro)]
struct Pancakes;

#[derive(MyTrait)]
#[my_trait(answer = 50, level = "high")]
struct Foo {
    group: String,
}

// #[derive(MyTrait)]
// #[my_trait(answer = 0, name = "lowo")]
// struct Bar;

#[derive(MyTrait)]
#[my_trait(answer = 20, level = "low")]
struct Bar {
    name: String,
    age: u8,
}

// #[my_crate(lorem(dolor = "Hello", sit))]
#[derive(KeyNamesGetter, Serialize, Deserialize)]
// #[mongoye(typee = "Hello")]
// #[mongoye(typee = "Hello", case = "snake")]
#[serde(rename_all = "camelCase")]
pub struct ConsumingType {
    #[serde(rename = "lowo_cool")]
    pub name_of_me: String,
    #[serde(rename = "lmsar")]
    // #[serde(rename(serialize = "ser_name", deserialize = "de_name"))]
    pub age: u8,
}

pub mod pp {
    pub const gg: &str = "34";
}

struct Make {
    name: &'static str,
}

fn main() {

    let ConsumingTypeKeyNames {  lowo_cool, lmsar,.. } = ConsumingType::get_field_names();

    // println!("rerezzzzzzz{name_of_me}, {lmsar}")
    println!("rere{lowo_cool}, {lmsar}")
    // ConsumingType::get_field_names();
    // Pancakes::hello_macro();
    // println!("Hello, world!");
    // println!("Foo::answer() = {}", Foo::answer());
    // println!("Bar::answer() = {}", Bar::answer());

    // println!("Foo::level() = {}", Foo::level());
    // println!("Bar::level() = {}", Bar::level());
}

#[test]
fn default() {
    assert_eq!(Foo::answer(), 50);
    assert!(Foo::level().contains("High"));
    assert!(!Foo::level().contains("Low"));
}

#[test]
fn getter() {
    assert_eq!(Bar::answer(), 20);
    assert!(Bar::level().contains("Low"));
}
