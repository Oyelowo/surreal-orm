// all the Product related GraphQL types also query and mutation included (export a string containing GraphQL types
//use super::query::{Droid, Human};
use super::query_droid::Droid;
use super::query_human::Human;
use async_graphql::{Enum, Interface};
use common::pub_struct;

pub_struct!(StarWarsChar {
    id: &'static str,
    name: &'static str,
    is_human: bool,
    friends: Vec<usize>,
    appears_in: Vec<Episode>,
    home_planet: Option<&'static str>,
    primary_function: Option<&'static str>,
});

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
