#![allow(clippy::needless_lifetimes)]
//  all the query resolvers

// use crate::starwars::model::{StarWarsChar};
use super::model::StarWars;
use super::query::{Droid, Human};
use super::type_gql::{Character, Episode, StarWarsChar};
use async_graphql::{
    connection::{query, Connection, Edge, EmptyFields},
    Context, Object, Result,
};

#[derive(Default)]
pub struct StarWarQueryRoot;

#[Object]
impl StarWarQueryRoot {
    async fn hero<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(
            desc = "If omitted, returns the hero of the whole saga. If provided, returns the hero of that particular episode."
        )]
        episode: Episode,
    ) -> Character<'a> {
        let star_wars = ctx.data_unchecked::<StarWars>();
        if episode == Episode::Empire {
            Human(star_wars.chars.get(star_wars.luke).expect("Getting starwars cha failed")).into()
        } else {
            Droid(star_wars.chars.get(star_wars.artoo).expect("Getting starwars cha failed")).into()
        }
    }

    async fn human<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(desc = "id of the human")] id: String,
    ) -> Option<Human<'a>> {
        ctx.data_unchecked::<StarWars>().human(&id).map(Human)
    }

    async fn humans<'a>(
        &self,
        ctx: &Context<'a>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<Connection<usize, Human<'a>, EmptyFields, EmptyFields>> {
        let humans = ctx
            .data_unchecked::<StarWars>()
            .humans()
            .iter()
            .copied()
            .collect::<Vec<_>>();

        query_characters(after, before, first, last, &humans)
            .await
            .map(|conn| conn.map_node(Human))
    }

    async fn droid<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(desc = "id of the droid")] id: String,
    ) -> Option<Droid<'a>> {
        ctx.data_unchecked::<StarWars>().droid(&id).map(Droid)
    }

    async fn droids<'a>(
        &self,
        ctx: &Context<'a>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<Connection<usize, Droid<'a>, EmptyFields, EmptyFields>> {
        let droids = ctx
            .data_unchecked::<StarWars>()
            .droids()
            .iter()
            .copied()
            .collect::<Vec<_>>();
        query_characters(after, before, first, last, &droids)
            .await
            .map(|conn| conn.map_node(Droid))
    }
}

async fn query_characters<'a>(
    after: Option<String>,
    before: Option<String>,
    first: Option<i32>,
    last: Option<i32>,
    characters: &[&'a StarWarsChar],
) -> Result<Connection<usize, &'a StarWarsChar, EmptyFields, EmptyFields>> {
    query(
        after,
        before,
        first,
        last,
        |after, before, first, last| async move {
            let mut start = 0usize;
            let mut end = characters.len();

            if let Some(after) = after {
                if after >= characters.len() {
                    return Ok(Connection::new(false, false));
                }
                // Start character from after
                start = after + 1;
            }

            if let Some(before) = before {
                if before == 0 {
                    return Ok(Connection::new(false, false));
                }

                // End from before
                end = before;
            }

            let mut slice = &characters[start..end];

            if let Some(first) = first {
                // Take minimum between the first and the length of the slice. This is in case user
                // asks for more than is available in the slice. e.g  if first= 20. but slice length is just 15,
                // we want to only return first 15 of the slie rather than 20, as 20 is larger than the entire slice.
                slice = &slice[..first.min(slice.len())];

                // Subtract from the end
                // TODO: CROSS-CHECK the statement below. Seems incorrect
                // e.g if user wants first 7, at end of 20, we would want this to end at 20 minus the first set or length of slice,
                // which means the end should be inital end(either the original character length or before specied e.g 20) minus
                // the first set the user wants or lendth or slice i.e new_end = 20 (old_end) - 7(first)or (slice.len) = 14.
                end -= first.min(slice.len());
            } else if let Some(last) = last {
                // similarly here, but from behind. e.g if u want last 20, but slice is only 15 items.
                // Ideally, we would want to return 15 items instead starting from the 0th element. So [15(slice) - 15(minimum btw slice i.e 15 and lasti.e 20) ..]  = [0..]
                // Case 2 e.g: slice is bigger(20) than last(15), this would then be [20 - 15(minimum btw slice i.e 20 and lasti.e 15) ...] = [5 ..]
                // I.O.W, the logic makes sure we get the last items even if larger than the slice size, without reaching outside of the slice size
                slice = &slice[slice.len() - last.min(slice.len())..];

                // Reset start position as the difference between the end and either the last provided by user or slice length.
                start = end - last.min(slice.len());
            }

            let mut connection = Connection::new(start > 0, end < characters.len());
            connection.append(
                slice
                    .iter()
                    .enumerate()
                    .map(|(idx, item)| Edge::new(start + idx, *item)),
            );
            Ok(connection)
        },
    )
    .await
}
