use std::collections::HashMap;

// all the Product related GraphQL types also query and mutation included (export a string containing GraphQL types
use async_graphql::{
    Context, Enum, Interface, Object, Result,
};
use common::pub_struct;
use super::query::{Human, Droid};

pub_struct!(StarWarsChar {
    id: &'static str,
    name: &'static str,
    is_human: bool,
    friends: Vec<usize>,
    appears_in: Vec<Episode>,
    home_planet: Option<&'static str>,
    primary_function: Option<&'static str>,
});


fn rer() {
    // let kk = Human(&StarWarsChar{id: "43", name: "aer", is_human: true, friends: vec![], appears_in: Episode::Empire, home_planet: Some("43"), primary_function: Some("43")});
}

/// One of the films in the Star Wars Trilogy
#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug)]
pub enum Episode {
    /// Released in 1977.
    NewHope,

    /// Released in 1980.
    Empire,

    /// Released in 1983.
    Jedi,
}
pub use Episode::*;


#[derive(Interface)]
#[graphql(
    field(name = "id", type = "&str"),
    field(name = "name", type = "&str"),
    field(name = "friends", type = "Vec<Character<'ctx>>"),
    field(name = "appears_in", type = "&[Episode]")
)]
pub enum Character<'a> {
    Human(Human<'a>),
    Droid(Droid<'a>),
}
