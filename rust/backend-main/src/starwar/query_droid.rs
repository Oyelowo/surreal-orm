use async_graphql::{Context, Object};

use super::model::StarWars;
// use super::Human;
use super::query_human::{Human};
//use super::type_gql::{Character, Episode, StarWarsChar};
// use self::{Character, Episode, StarWarsChar};
use super::type_gql::{Character, Episode, StarWarsChar};

pub struct Droid<'a>(pub &'a StarWarsChar);

/// A mechanical creature in the Star Wars universe.
#[Object]
impl<'a> Droid<'a> {
    /// The id of the droid.
    pub async fn id(&self) -> &str {
        self.0.id
    }

    /// The name of the droid.
    pub async fn name(&self) -> &str {
        self.0.name
    }

    /// The friends of the droid, or an empty list if they have none.
    pub async fn friends<'ctx>(&self, ctx: &Context<'ctx>) -> Vec<Character<'ctx>> {
        let star_wars = ctx.data_unchecked::<StarWars>();
        star_wars
            .friends(self.0)
            .into_iter()
            .map(|ch| {
                if ch.is_human {
                    Human(ch).into()
                } else {
                    Droid(ch).into()
                }
            })
            .collect()
    }

    /// Which movies they appear in.
    pub async fn appears_in(&self) -> &[Episode] {
        &self.0.appears_in
    }

    /// The primary function of the droid.
    pub async fn primary_function(&self) -> &Option<&str> {
        &self.0.primary_function
    }
}
