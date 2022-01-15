// For testing macros made here

use hello_macro::{HelloMacro, MyTrait};

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

fn main() {
    Pancakes::hello_macro();
    println!("Hello, world!");
    println!("Foo::answer() = {}", Foo::answer());
    println!("Bar::answer() = {}", Bar::answer());

    println!("Foo::level() = {}", Foo::level());
    println!("Bar::level() = {}", Bar::level());
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
