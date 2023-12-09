/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use proc_macro::TokenStream;
use surreal_query_builder::sql;
use syn::{parse_macro_input, LitStr};

mod query;
mod query_builder;
mod statement_parser;

/// Checks a query at compile time and returns the query as a string.
/// Unlike, query! macro, this macro does not allow variable interpolation.
///
/// # Arguments
/// * `query` - The query to be checked at compile time.
///
/// # Example
/// ```rust
/// # use query_builder_macros as surreal_orm;
/// use surreal_orm::{query_raw, statements::select};
///
/// let query = query_raw!("SELECT name, age, * FROM users");
/// let query = query_raw!("SELECT name, age, * FROM users WHERE name = $1 AND name = 'Oyelowo'");
/// let query = query_raw!("SELECT ->purchased->product<-purchased<-person->purchased->product FROM person:tobie PARALLEL");
/// ```
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

/// Checks one or multiple quer(ies) at compile time.
/// It also allows interpolations of variables and can
/// be run.
///
/// # Arguments
/// * `db` - The database connection to be used.
/// * `quer(ies)` - A single or list of queries to be checked at compile time.
/// * `params` - The parameters to be used for the query.
///
/// # Example
/// ```rust
/// # use query_builder_macros as surreal_orm;
/// use surreal_orm::{query};
/// use surrealdb::{engine::local::Mem, Surreal};
///
/// let db = Surreal::new::<Mem>(()).await.unwrap();
/// db.use_ns("test").use_db("test").await.unwrap();
///
/// let _query = query!(db, "SELECT * FROM users").await;
/// let _query = query!(db, "SELECT * FROM users", {}).await;
/// let _query = query!(db, "SELECT * FROM users WHERE id = $id", {id : 1} ).await;
/// let username = "Oyelowo";
/// let _query = query!(db, "SELECT name, age FROM users WHERE id = $id AND name = $name", {
///     id : 1,
///     name : username
/// })
/// .await;
///
/// ```
///
/// Also supports multiple queries in a single call.
/// ```rust
/// # use query_builder_macros as surreal_orm;
/// use surreal_orm::{query};
/// use surrealdb::{engine::local::Mem, Surreal};
///
/// let db = Surreal::new::<Mem>(()).await.unwrap();
/// db.use_ns("test").use_db("test").await.unwrap();
///
/// let _queries = query!(
///     db,
///     [
///         "SELECT * FROM users WHERE id = $id",
///         "CREATE user:oyelowo SET name = $name, company = 'Codebreather', skills = $skills"
///     ],
///     {
///         id: 1,
///         name: "Oyelowo",
///         skills: vec!["Rust", "python", "typescript"]
///     }
/// )
/// .await;
/// ```
#[proc_macro]
pub fn query(raw_input: TokenStream) -> TokenStream {
    query::query(raw_input.into()).into()
}

/// A novel approach to writing SQL queries in Rust. It allows you to write SQL queries
/// in a native-like Rust Syntax. It supports mixing query builder functions with native looking
/// syntax such as if, for, let, return statements and standard Rust expressions which can be
/// a normal surreal_orm query builder function statement.
///
/// It dynamically returns a query chain, transaction or a block depending on the usage.
/// By default, it returns a query chain. If you want to return a transaction, you start
/// with a `begin transaction;` and end with a `commit transaction;` or `cancel transaction;`
/// keywords.
/// To return a block, you have to return typically at the end.
///
/// # Arguments
/// * `queries` - Series of queries to be checked at compile time.
///
/// # Example
///
/// ```rust, ignore
/// query_turbo! {
///     let balance1 = create_only().content(Balance {
///     id: Balance::create_id("balance1".to_string()),
///         amount: amount_to_transfer,
///     });
///
///     create_only().content(Balance {
///         id: Balance::create_id("balance2".to_string()),
///         amount: amount_to_transfer,
///     });
///
///     if balance.greater_than(100) {
///         let first_name = "Oyelowo";
///         let score = 100;
///         select(All).from(Account::table_name()).where_(acc.balance.eq(5));
///     } else if balance.gte(100) {
///         let first_name = "Oyelowo";
///         let score = 100;
///         select(All).from(Account::table_name()).where_(acc.balance.eq(5));
///     } else {
///         let first_name = "Oyelowo";
///         let score = 100;
///         select(All).from(Account::table_name()).where_(acc.balance.eq(5));
///     };
///
///     for name in vec!["Oyelowo", "Oyedayo"] {
///         let first = "Oyelowo";
///         select(All).from(Account::table_name()).where_(acc.balance.eq(5));
///         
///         let good_stmt = select(All).from(Account::table_name()).where_(acc.balance.eq(64));
///
///         if balance.gt(50) {
///             let first_name = "Oyelowo";
///         };
///
///         select(All).from(Account::table_name()).where_(acc.balance.eq(34));
///
///         let numbers = vec![23, 98];
///
///         for age in numbers {
///             let score = 100;
///             let first_stmt = select(All).from(Account::table_name()).where_(acc.balance.eq(5));
///
///             let second_stmt = select(All).from(Account::table_name()).where_(acc.balance.eq(25));
///             select(All).from(Account::table_name()).where_(acc.balance.eq(923));
///             
///         };
///     };
///
///     // You can reference the balance object by using the $balance variable and pass the amount
///     // as a parameter to the decrement_by function. i.e $balance.amount
///     let updated1 = update::<Account>(id1).set(acc.balance.increment_by(balance1.with_path::<Balance>(E).amount));
///     update::<Account>(id1).set(acc.balance.increment_by(balance1.with_path::<Balance>(E).amount));
///     update::<Account>(id1).set(acc.balance.increment_by(45.3));
///     
///     // You can also pass the amount directly to the decrement_by function. i.e 300.00
///     update::<Account>(id2).set(acc.balance.decrement_by(amount_to_transfer));
///     update::<Account>(id2).set(acc.balance.decrement_by(50));
/// };
///
/// // as transaction.
/// query_turbo! {
///    BEGIN TRANSACTION;
///    let balance1 = create_only().content(Balance {
///         id: Balance::create_id("balance1".to_string()),
///         amount: amount_to_transfer,
///    });
///
///    COMMIT TRANSACTION;
/// };
///
/// // as block.
/// query_turbo! {
///   let result = select(All).from(Account::table_name()).where_(acc.balance.eq(5));
///
///   return result;
/// };
/// ```
#[proc_macro]
pub fn query_turbo(input: TokenStream) -> TokenStream {
    query_builder::query_turbo(input)
}

/// A block is a series of queries that are executed as a single
/// unit, surrounded by curly braces and typically returns a value. It can be assigned to a field or variable.
/// It supports mixing query builder functions with native looking
/// syntax such as if, for, let, return statements and standard Rust expressions which can be
/// a normal surreal_orm query builder function statement.
///
/// # Arguments
/// * `queries` - Series of queries to be checked at compile time.
///
/// # Example
/// ```rust, ignore
/// let created_stats_statement = create::<WeaponStats>().set(object_partial!(WeaponStats {
///     averageStrength: block! {
///         let strengths = select_value(strength).from(weapon);
///         let total = math::sum!(strengths); // 105
///         let count = array::len!(strengths);     // 15
///         return math::ceil!((((total / count) * (count * total)) / (total + 4)) * 100);
///     }
/// }));
/// ```
#[proc_macro]
pub fn block(input: TokenStream) -> TokenStream {
    query_builder::query_block(input)
}

/// A transaction is a series of queries that are executed as a single
/// unit. It supports mixing query builder functions with native looking
/// syntax such as if, for, let, return statements and standard Rust expressions which can be
/// a normal surreal_orm query builder function statement.
///
/// It must start with a `begin transaction;` and end with a `commit transaction;` or `cancel transaction;`
/// keywords.
///
/// # Arguments
/// * `queries` - Series of queries to be checked at compile time.
///
/// # Example
///
/// ```rust, ignore
///
/// let db = Surreal::new::<Mem>(()).await.unwrap();
/// db.use_ns("test").use_db("test").await.unwrap();
///
/// let id1 = &Account::create_id("one".to_string());
/// let id2 = &Account::create_id("two".to_string());
/// let amount_to_transfer = 300.00;
///
/// let acc = Account::schema();
/// transaction! {
///     BEGIN TRANSACTION;
///     
///     let balance = create_only().content(Balance {
///         id: Balance::create_id("balance1".into()),
///         amount: amount_to_transfer,
///     });
///
///     create_only().content(Account {
///         id: id1.clone(),
///         balance: 135_605.16,
///     });
///     create_only().content(Account {
///         id: id2.clone(),
///         balance: 91_031.31,
///     });
///
///     // You can reference the balance object by using the $balance variable and pass the amount
///     // as a parameter to the decrement_by function. i.e $balance.amount
///     update::<Account>(id1).set(acc.balance.increment_by(balance.with_path::<Balance>(E).amount));
///
///     // You can also pass the amount directly to the decrement_by function. i.e 300.00
///     update::<Account>(id2).set(acc.balance.decrement_by(amount_to_transfer));
///
///     COMMIT TRANSACTION;
/// }
/// .run(db.clone())
/// .await?;
/// ```
#[proc_macro]
pub fn transaction(input: TokenStream) -> TokenStream {
    query_builder::query_transaction(input)
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
/// // The below generates a `get_or_create_spaceship_statement` itself and `get_or_create_spaceship` helper function created by the macro.
/// define_function!(get_or_create_spaceship(
///     first_arg: string,
///     last_arg: string,
///     birthday_arg: datetime,
///     _very_complex_type: int | option<float> | array<option<string>|int|null, 10> | set<option<number>|float|null, 10> | option<array> | option<set<option<int>>>
/// ) {
///     let person = select(All)
///         .from(SpaceShip::table_name())
///         .where_(
///             cond(SpaceShip::schema().id.equal(first_arg))
///                 .and(SpaceShip::schema().name.equal(last_arg))
///                 .and(SpaceShip::schema().created.equal(birthday_arg)),
///         );
///
///     if person.with_path::<SpaceShip>([0]).id.is_not(NONE) {
///         return person;
///     } else {
///         return create::<SpaceShip>().set(
///                     object!(SpaceShip {
///                         id: first_arg,
///                         name: last_arg,
///                         created: birthday_arg,
///                     })
///                 );
///     };
/// });
/// ```
#[proc_macro]
pub fn define_function(input: TokenStream) -> TokenStream {
    statement_parser::define_function(input)
}

/// A helper function to create a standalone for loop if you don't want to define within
/// query_turbo! or block! or transaction! macro. This is almost never necessary. Just use
/// query_turbo! or block! or transaction! macro and define your for loop within it,
///
/// ```rust, ignore
/// use surreal_query_builder as surreal_orm;
/// use surreal_orm::{*, statements::select};
///
/// let ref __name = Param::new("name");
/// let ref person_table = Table::from("person");
/// let ref user_name = Field::from("user_name");
///
/// for_!(__name in vec!["Oyelowo", "Oyedayo"] {
///    let nick_name = select(user_name).from_only(person_table).where_(user_name.eq(__name));
///    select(All).from(person_table).where_(user_name.eq(nick_name));
/// };
/// println!("{}", for_loop);
/// ```
#[proc_macro]
pub fn for_(input: TokenStream) -> TokenStream {
    statement_parser::for_::for_loop_without_for_keyword(input)
}
