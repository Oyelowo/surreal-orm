use async_graphql::{Context, Object};

use super::model::StarWars;
use super::query_droid::Droid;
use super::type_gql::{Character, Episode, StarWarsChar};

pub struct Human<'a>(pub &'a StarWarsChar);

/// A humanoid creature in the Star Wars universe.
#[Object]
impl<'a> Human<'a> {
    /// The id of the human.
    pub async fn id(&self) -> &str {
        self.0.id
    }

    /// The name of the human.
    pub async fn name(&self) -> &str {
        self.0.name
    }

    /// The friends of the human, or an empty list if they have none.
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

    /// The home planet of the human, or null if unknown.
    pub async fn home_planet(&self) -> &Option<&str> {
        &self.0.home_planet
    }
}
