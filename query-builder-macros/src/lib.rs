use proc_macro::TokenStream;
use surreal_query_builder::sql;
use syn::{parse_macro_input, LitStr};

use crate::statement_parser::for_::ForLoopMetaParser;

mod query;
mod query_builder;
mod statement_parser;

#[proc_macro]
pub fn query_raw(raw_input: TokenStream) -> TokenStream {
    let r_input = raw_input.clone();
    let input = parse_macro_input!(r_input as LitStr);
    let input = input.value();
    let sql = sql::parse(input.as_str());

    match sql {
        Ok(value) => value,
        Err(value) => {
            return syn::Error::new_spanned(input, value)
                .to_compile_error()
                .into()
        }
    };
    raw_input
}

#[proc_macro]
pub fn query(raw_input: TokenStream) -> TokenStream {
    query::query(raw_input.into()).into()
}

#[proc_macro]
pub fn query_turbo(input: TokenStream) -> TokenStream {
    query_builder::query_turbo(input.into()).into()
}

#[proc_macro]
pub fn block(input: TokenStream) -> TokenStream {
    query_builder::query_block(input.into()).into()
}

#[proc_macro]
pub fn transaction(input: TokenStream) -> TokenStream {
    query_builder::query_transaction(input.into()).into()
}

/// A macro to generate a function definition statement and the corresponding helper function.
/// e.g. `define_function!(get_it(first: bool, last: string, birthday: string) { let person = "43"; return person; });`
/// generates a `get_it_statement` itself and `get_it` helper function created by the macro.
///
/// # Arguments
/// * `function definition` - The function definition
///
/// # Example
/// ```rust
/// # use surreal_query_builder as surreal_orm;
/// use surreal_orm::{*, statements::define_function};
///
/// // The below generates a `get_it_statement` itself and `get_it` helper function created by the macro.
/// define_function!(get_it(first: bool, last: string, birthday: string) {
///     let person = "43";
///     return person;
/// });
/// ```
///
/// ```rust, ignore   
/// // The below generates a `get_person_statement` itself and `get_person` helper function created by the macro.
/// define_function!(get_person(first_arg: string, last_arg: string, birthday_arg: string) {
///     let person = select(All)
///         .from(SpaceShip::table_name())
///         .where_(
///             cond(SpaceShip::schema().id.equal(&first_arg))
///                 .and(SpaceShip::schema().name.equal(&last_arg))
///                 .and(SpaceShip::schema().created.equal(&birthday_arg)),
///         );
///
///    if person.with_path::<SpaceShip>(index(0)).id.is_not(NONE) {
///         return person.with_path::<SpaceShip>(index(0));
///     } else {
///         create::<SpaceShip>(
///             vec![
///                 SpaceShip::schema().id.equal_to(&first_arg),
///                 SpaceShip::schema().name.equal_to(&last_arg),
///                 SpaceShip::schema().created.equal_to(&birthday_arg),
///             ]
///         )
///     };
/// });
/// ```
#[proc_macro]
pub fn define_function(input: TokenStream) -> TokenStream {
    statement_parser::define_function(input.into()).into()
}

/// A helper function to create a standalone for loop if you don't want to define within
/// query_turbo! or block! or transaction! macro. This is almost never necessary. Just use
/// query_turbo! or block! or transaction! macro and define your for loop within it,
/// ```
/// use surreal_query_builder as surreal_orm;
/// use surreal_orm::{*, statements::{select, for_}};
///
/// let ref __name = Param::new("name");
/// let ref person_table = Table::from("person");
/// let ref user_name = Field::from("user_name");
/// let for_loop = for_(__name).in_(vec!["Oyelowo", "Oyedayo"]).block(block! {
///    LET nick_name = select(user_name).from_only(person_table).where_(user_name.eq(__name));
///    select(All).from(person_table).where_(user_name.eq(nick_name));
/// });
/// println!("{}", for_loop);
#[proc_macro]
pub fn for_(input: TokenStream) -> TokenStream {
    statement_parser::for_::for_loop_without_for_keyword(input.into()).into()
}
