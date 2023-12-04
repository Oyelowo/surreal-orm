use proc_macro::TokenStream;
use proc_macros_helpers::get_crate_name;
use quote::{quote, ToTokens};
use syn::{
    ext::IdentExt,
    parse::{Parse, ParseStream},
    token::{self, Brace},
    Expr, Ident, Token,
};

use super::{
    helpers::generate_variable_name,
    query_chain::{GeneratedCode, QueriesChainParser},
};

#[derive(Debug, Clone)]
pub enum Iterable {
    Expr(Expr),
    Ident(Ident),
}

impl ToTokens for Iterable {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Iterable::Expr(expr) => expr.to_tokens(tokens),
            Iterable::Ident(ident) => ident.to_tokens(tokens),
        }
    }
}

impl Parse for Iterable {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let fork = input.fork();
        fork.parse::<Ident>()?;

        if (fork.is_empty()) || (fork.peek(token::Brace)) {
            let ident = input.parse()?;
            Ok(Iterable::Ident(ident))
        } else {
            let expr = input.parse()?;
            Ok(Iterable::Expr(expr))
        }
    }
}

#[derive(Debug, Clone)]
pub struct ForLoopMetaParser {
    pub iteration_param: Ident,
    pub iterable: Iterable,
    pub body: QueriesChainParser,
    pub generated_ident: Ident,
}

impl ForLoopMetaParser {
    pub fn has_return_statement(&self) -> bool {
        self.body.is_likely_query_block()
    }
}

impl Parse for ForLoopMetaParser {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let generated_ident = generate_variable_name();

        // The iteration parameter and the iterable in the start of the for loop
        if input.peek(token::Paren) {
            let iter_content;
            let _paranthesized_iter_content_token = syn::parenthesized!(iter_content in input);
            let iteration_param = iter_content.parse()?;

            iter_content.parse::<syn::Token![in]>()?;

            let iterable = iter_content.parse()?;
            // The body
            let content;
            let _brace_token: Brace = syn::braced!(content in input);

            let body = content.parse()?;

            input.parse::<syn::Token![;]>()?;

            return Ok(ForLoopMetaParser {
                iteration_param,
                iterable,
                body,
                generated_ident,
            });
        } else {
            let iteration_param = input.parse()?;

            input.parse::<syn::Token![in]>()?;

            let iterable = input.parse()?;
            // The body
            let content;
            let _brace_token: Brace = syn::braced!(content in input);

            let body = content.parse()?;

            input.parse::<syn::Token![;]>()?;

            return Ok(ForLoopMetaParser {
                iteration_param,
                iterable,
                body,
                generated_ident,
            });
        };
    }
}

#[derive(Debug, Clone)]
pub struct ForLoopStatementParser {
    pub meta_content: Box<ForLoopMetaParser>,
}

impl std::ops::Deref for ForLoopStatementParser {
    type Target = ForLoopMetaParser;

    fn deref(&self) -> &Self::Target {
        &self.meta_content
    }
}

impl Parse for ForLoopStatementParser {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<syn::Token![for]>()?;
        let for_loop = input.parse::<ForLoopMetaParser>()?;
        Ok(ForLoopStatementParser {
            meta_content: Box::new(for_loop),
        })
    }
}

pub struct TokenizedForLoop {
    pub code_to_render: TokenStream,
    pub query_chain: TokenStream,
}

impl ForLoopMetaParser {
    pub fn tokenize(&self) -> TokenizedForLoop {
        let ForLoopMetaParser {
            iteration_param,
            iterable,
            body,
            generated_ident: _,
        } = self;

        // let iterable = match iterable {
        //     Iterable::Expr(expr) => quote!(expr),
        //     Iterable::Ident(ident) => {
        //         let crate_name = get_crate_name(false);
        //         quote!(#crate_name::iterable::Iterable::from(#ident))
        //     }
        // };
        let GeneratedCode {
            query_chain,
            to_render,
        } = body.generate_code();

        let crate_name = get_crate_name(false);

        let whole_stmts = quote!(
        {
            let #iteration_param = #crate_name::Param::new(stringify!(#iteration_param));

            #( #to_render )*

            #crate_name::statements::for_(#iteration_param).in_(#iterable)
            .block(
                #( #query_chain )*
            )
        });

        let to_render = quote! {
            {
                #( #to_render )*

                #whole_stmts
            }
        }
        .into();
        let to_chain = quote!(#whole_stmts);

        TokenizedForLoop {
            code_to_render: to_render,
            query_chain: to_chain.into(),
        }
    }
}

pub fn for_loop_without_for_keyword(input: TokenStream) -> TokenStream {
    let for_loop_content = syn::parse_macro_input!(input as ForLoopMetaParser);

    let z = &for_loop_content.tokenize();
    let to_render: proc_macro2::TokenStream = z.code_to_render.clone().into();
    let to_chain: proc_macro2::TokenStream = z.query_chain.clone().into();

    quote!(
        #to_render

        #to_chain

    )
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
macro_rules! _for_loop {
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
// pub use _for_loop as for_;

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
