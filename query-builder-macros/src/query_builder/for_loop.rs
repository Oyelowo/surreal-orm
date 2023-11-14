use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::{self, Brace},
    Expr, Ident, Token,
};

use super::query_chain::QueriesChain;

struct ForLoop {
    iteration_param: Ident,
    iterable: Expr,
    body: QueriesChain,
}

impl Parse for ForLoop {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // The iteration parameter and the iterable in the start of the for loop
        let iter_content = if input.peek(token::Paren) {
            let iter_content;
            let _paranthesized_iter_content_token = syn::parenthesized!(iter_content in input);
            &iter_content
        } else {
            input
        };

        let iteration_param = iter_content.parse()?;

        iter_content.parse::<syn::Token![in]>()?;

        let iterable = iter_content.parse()?;
        // The body
        let content;
        let _brace_token: Brace = syn::braced!(content in input);

        let body = content.parse()?;

        input.parse::<syn::Token![;]>()?;

        Ok(ForLoop {
            iteration_param,
            iterable,
            body,
        })
    }
}

pub fn for_loop(input: TokenStream) -> TokenStream {
    let ForLoop {
        iteration_param,
        iterable,
        body,
    } = syn::parse_macro_input!(input as ForLoop);

    let generated_code = super::generate_query_chain_code(&body.statements);
    let query_chain = super::generated_bound_query_chain(&body.statements);

    let crate_name = super::get_crate_name(false);

    let whole_stmts = quote!(#crate_name::statements::for_(#iteration_param).in_(#iterable)
    .block(
        #( #query_chain )*
        .parenthesized()
    ));

    quote! {
        {
            #( #generated_code )*

            #whole_stmts
        }
    }
    .into()
}
///
/// A helper function to create a for loop
/// ```
/// use surreal_query_builder as surreal_orm;
/// use surreal_orm::{*, statements::{for_, select, select_value}};
///
/// let ref person_table = Table::from("person");
/// let ref user_name = Field::from("user_name");
///
/// for_!(name in vec!["Oyelowo", "Oyedayo"] {
///    select(All).from(person_table).where_(user_name.eq(name));
///    select(All).from(person_table).where_(user_name.eq(name));
///
///    for_!((name in select_value(user_name).from_only(person_table)) {
///         select(All).from(person_table).where_(user_name.eq(name));
///         select(All).from(person_table).where_(user_name.eq(name));
///    });
/// });
/// ```
// #[macro_export]
macro_rules! for_loop {
    (($param:ident in $iterable:expr) { $($stmt:expr;)+ }) => {{
        // let ref $param = $crate::Param::new(stringify!($param));
        // $crate::statements::for_($param).in_($iterable).block($crate::internal_tools::query_turbo! {
        //     $($stmt;)+
        // })
    }};
    (($param:ident IN $iterable:expr) { $($stmt:expr;)+ }) => {{
        // let ref $param = $crate::Param::new(stringify!($param));
        // $crate::statements::for_($param).in_($iterable).block($crate::internal_tools::query_turbo! {
        //     $($stmt;)+
        // })
    }};
}
// pub use for_loop as for_;

#[cfg(test)]
mod tests {

    // use crate::{
    //     chain, cond, statements::select, traits::Buildable, All, Field, Operatable, ToRaw,
    // };
    //
    // #[test]
    // fn test_for_loop() {
    //     let ref person_table = Table::from("person");
    //     let ref user_name = Field::from("user_name");
    //
    //     for_loop!((name in vec!["Oyelowo", "Oyedayo"]) {
    //         select(All).from(person_table).where_(user_name.eq(name));
    //         select(All).from(person_table).where_(user_name.eq(name));
    //
    //         for_loop!((name in select_value(user_name).from_only(person_table)) {
    //             select(All).from(person_table).where_(user_name.eq(name));
    //             select(All).from(person_table).where_(user_name.eq(name));
    //         });
    //     });
    // }

    // #[test]
    // fn test_for_macro() {
    //     let ref person_table = Table::from("person");
    //     let ref user_name = Field::from("user_name");
    //
    //     let for_loop = for_!((name in vec!["Oyelowo", "Oyedayo"]) {
    //         select(All).from(person_table).where_(user_name.eq(name));
    //     });
    //
    //     assert_eq!(
    //         for_loop.fine_tune_params(),
    //         "FOR $name IN $_param_00000001 {\nSELECT * FROM person WHERE user_name = $name;\n};"
    //     );
    //     assert_eq!(
    //         for_loop.to_raw().build(),
    //         "FOR $name IN ['Oyelowo', 'Oyedayo'] {\nSELECT * FROM person WHERE user_name = $name;\n};"
    //     );
    // }
    //
    // #[test]
    // fn test_for_macro_nested() {
    //     let ref __name = Param::new("name");
    //     let ref person_table = Table::from("person");
    //     let ref user_name = Field::from("user_name");
    //
    //     let for_loop = for_!((__name in vec!["Oyelowo", "Oyedayo"]) {
    //         select(All).from(person_table).where_(user_name.eq(__name));
    //         for_!((__name in vec!["Oyelowo", "Oyedayo"]) {
    //             select(All).from(person_table).where_(user_name.eq(__name));
    //         });
    //     });
    //
    //     insta::assert_snapshot!(for_loop.fine_tune_params());
    //     insta::assert_snapshot!(for_loop.to_raw().build());
    // }
    //
    // #[test]
    // fn test_for_macro_and_block_macro() {
    //     let ref person_table = Table::from("person");
    //     let ref user_name = Field::from("user_name");
    //
    //     // let for_loop = block! {
    //     //     FOR (__name IN vec!["Oyelowo", "Oyedayo"]) {
    //     //         select(All).from(person_table).where_(user_name.eq(__name));
    //     //         select(All).from(person_table).where_(user_name.eq(__name));
    //     //
    //     //         for_!((__moniker IN select_value(user_name).from(person_table)) {
    //     //             select(All).from(person_table).where_(user_name.eq(__moniker));
    //     //             select(All).from(person_table).where_(user_name.eq(__name));
    //     //         });
    //     //
    //     //         for_(__name).in_(vec!["Oyelowo", "Oyedayo"])
    //     //             .block(block! {
    //     //                 select(All).from(person_table).where_(user_name.eq(__name));
    //     //         });
    //     //
    //     //     };
    //     //
    //     //     FOR (__name IN vec!["Oyelowo", "Oyedayo"]) {
    //     //         select(All).from(person_table).where_(user_name.eq(__name));
    //     //         select(All).from(person_table).where_(user_name.eq(__name));
    //     //     };
    //     //
    //     //     FOR (__name IN vec!["Oyelowo", "Oyedayo"]) {
    //     //         select(All).from(person_table).where_(user_name.eq(__name));
    //     //         select(All).from(person_table).where_(user_name.eq(__name));
    //     //     };
    //     //
    //     //     if_(__name.eq("Oyelowo")).then(6).end();
    //     //
    //     //
    //     // };
    //     for_!((__name in vec!["Oyelowo"]) {
    //         select(All).from(person_table).where_(user_name.eq(__name));
    //         select(All).from(person_table).where_(user_name.eq(__name));
    //
    //         for_!((__name in vec!["Oyelowo"]) {
    //             select(All).from(person_table).where_(user_name.eq(__name));
    //             select(All).from(person_table).where_(user_name.eq(__name));
    //
    //             for_!((__name in vec!["Oyelowo"]) {
    //                 select(All).from(person_table).where_(user_name.eq(__name));
    //                 select(All).from(person_table).where_(user_name.eq(__name));
    //             });
    //
    //             for_!((__name in vec!["Oyelowo"]) {
    //                 select(All).from(person_table).where_(user_name.eq(__name));
    //                 select(All).from(person_table).where_(user_name.eq(__name));
    //             });
    //
    //         });
    //     });
    //
    //     insta::assert_snapshot!(for_loop.fine_tune_params());
    //     insta::assert_snapshot!(for_loop.to_raw().build());
    // }
    //
    // use super::for_;
    // use crate::{
    //     statements::{
    //         if_,
    //         select::{select, select_value},
    //     },
    //     *,
    // };
    //
    // #[test]
    // fn test_for_in_block() {
    //     let ref __name = Param::new("name");
    //     let ref person_table = Table::from("person");
    //     let ref user_name = Field::from("user_name");
    //
    //     let for_loop = for_(__name).in_(vec!["Oyelowo", "Oyedayo"]).block(block! {
    //         select(All).from(person_table).where_(user_name.eq(__name));
    //     });
    //
    //     assert_eq!(
    //         for_loop.fine_tune_params(),
    //         "FOR $name IN $_param_00000001 {\nSELECT * FROM person WHERE user_name = $name;\n};"
    //     );
    //     assert_eq!(
    //         for_loop.to_raw().build(),
    //         "FOR $name IN ['Oyelowo', 'Oyedayo'] {\nSELECT * FROM person WHERE user_name = $name;\n};"
    //     );
    // }
    //
    // #[test]
    // fn test_for_in_with_block_macro() {
    //     let ref __name = Param::new("name");
    //     let ref person_table = Table::from("person");
    //     let ref user_name = Field::from("user_name");
    //
    //     let for_loop = for_(__name).in_(vec!["Oyelowo", "Oyedayo"]).block(block! {
    //         LET nick_name = select(user_name).from_only(person_table).where_(user_name.eq(__name));
    //
    //         select(All).from(person_table).where_(user_name.eq(nick_name));
    //     });
    //
    //     assert_eq!(
    //         for_loop.fine_tune_params(),
    //         "FOR $name IN $_param_00000001 {\nLET $nick_name = $_param_00000002;\n\nSELECT * FROM person WHERE user_name = $nick_name;\n};"
    //     );
    //
    //     assert_eq!(
    //         for_loop.to_raw().build(),
    //         "FOR $name IN ['Oyelowo', 'Oyedayo'] {\nLET $nick_name = (SELECT user_name FROM ONLY person WHERE user_name = $name);\n\nSELECT * FROM person WHERE user_name = $nick_name;\n};"
    //     );
    // }
    //
    // #[test]
    // fn test_for_in_block_with_subquery_iterable() {
    //     let ref __name = Param::new("name");
    //     let ref person_table = Table::from("person");
    //     let ref user_name = Field::from("user_name");
    //
    //     let for_loop = for_(__name)
    //         .in_(
    //             select_value(user_name)
    //                 .from(person_table)
    //                 .where_(user_name.eq(__name)),
    //         )
    //         .block(crate::internal_tools::query_turbo! {
    //             let __nick_name = select(user_name).from_only(person_table).where_(user_name.eq(__name));
    //             select(All).from(person_table).where_(user_name.eq(__nick_name));
    //         });
    //
    //     assert_eq!(
    //         for_loop.fine_tune_params(),
    //         "FOR $name IN $_param_00000001 {\nLET $__nick_name = $_param_00000002;\n\nSELECT * FROM person WHERE user_name = $__nick_name;\n};"
    //     );
    //
    //     assert_eq!(
    //         for_loop.to_raw().build(),
    //         "FOR $name IN (SELECT VALUE user_name FROM person WHERE user_name = $name) {\nLET $__nick_name = (SELECT user_name FROM ONLY person WHERE user_name = $name);\n\nSELECT * FROM person WHERE user_name = $__nick_name;\n};"
    //     );
    // }
}
