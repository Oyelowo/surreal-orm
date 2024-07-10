use darling::FromDeriveInput;
use quote::{format_ident, quote, ToTokens};
use std::fmt::Display;
use surreal_derive_helpers::models::NodeToken;
use surreal_query_builder::assert_not;

// Test table name
use test_case::test_case;
#[derive(Debug)]
enum Validity {
    Valid,
    Invalid,
}
impl Display for Validity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Valid => "Valid",
                Invalid => "Invalid",
            }
        )
    }
}

use Validity::*;

#[derive(Debug)]
enum RelaxTable {
    Relax,
    NoRelax,
}

impl Display for RelaxTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Relax => "Relax",
                NoRelax => "NoRelax",
            }
        )
    }
}

use RelaxTable::*;

enum TableNameFormat {
    Raw,
    Stringified,
}

impl Display for TableNameFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Raw => "Raw",
                Stringified => "Stringified",
            }
        )
    }
}
use TableNameFormat::*;

#[test_case("student_table", Raw, NoRelax, Valid; "Node table snake case raw table name with struct name even without relax")]
#[test_case("student_table", Raw, Relax, Valid; "Node table snake case raw table name with struct name with relax")]
#[test_case("student_table", Stringified, NoRelax, Valid; "Node table snake case stringified table name version of struct name cannot be used without relax")]
#[test_case("student_table", Stringified, Relax, Valid; "Node table snake case stringified table name version of struct name can be used with relax")]

#[test_case("snake_case_but_wrong_name", Stringified, NoRelax, Invalid; "snake case different from even snake case version of struct name cannot be used with relax")]
#[test_case("snake_case_but_wrong_name", Stringified, Relax, Valid; "snake case different from even snake case version of struct name can be used with relax")]

#[test_case("StudentTable",  Raw, NoRelax, Invalid; "non snake case raw table name version of struct name cannot be used without relax")]
#[test_case("StudentTable",  Raw, Relax, Valid; "non snake case raw table name version of struct name cann be used with relax")]
#[test_case("StudentTable",  Stringified, NoRelax, Invalid; "non snake case stringified table name version of struct name cannot be used without relax")]
#[test_case("StudentTable",  Stringified, Relax, Valid; "non snake case stringified table name version of struct name cann be used with relax")]
fn test_node_table_name(
    table_name: &str,
    table_name_format: TableNameFormat,
    relaxation: RelaxTable,
    validity: Validity,
) {
    let relax_table = match relaxation {
        Relax => quote!(, relax_table),
        NoRelax => quote!(),
    };
    let table_name = match table_name_format {
        Raw => {
            let table_name = format_ident!("{table_name}");
            quote!(#table_name)
        }
        Stringified => quote!(#table_name),
    };
    let input = quote!(
        #[derive(Node)]
        #[surreal_orm(table = #table_name #relax_table)]
        pub struct StudentTable {
            id: SurrealSimpleId<Self>,
        }
    );

    let derive_input = syn::parse2(input).unwrap();
    let node_token = NodeToken::from_derive_input(&derive_input).unwrap();

    // insta::assert_snapshot!(
    //     format!("node_table_name_tests-{table_name}-{table_name_format}-{relaxation}-{validity}"),
    //     format!("{:#}", node_token.to_token_stream())
    // );

    let must_be_in_snake_case_error = || {
        node_token
            .to_token_stream()
            .to_string()
            .contains("table name must be in snake case of the current struct name")
    };

    match validity {
        Valid => assert_not!(must_be_in_snake_case_error()),
        Invalid => assert!(must_be_in_snake_case_error()),
    }
}

#[test_case("big_writes", Raw, NoRelax, Valid)]
#[test_case("big_writes", Raw, Relax, Valid)]
#[test_case("big_writes", Stringified, NoRelax, Valid)]
#[test_case("big_writes", Stringified, Relax, Valid)]

#[test_case("snake_case_but_wrong_name", Stringified, NoRelax, Invalid)]
#[test_case("snake_case_but_wrong_name", Stringified, Relax, Valid)]

#[test_case("BigWrites",  Raw, NoRelax, Invalid)]
#[test_case("BigWrites",  Raw, Relax, Valid)]
#[test_case("BigWrites",  Stringified, NoRelax, Invalid)]
#[test_case("BigWrites",  Stringified, Relax, Valid)]
fn test_edge_table_name(
    table_name: &str,
    table_name_format: TableNameFormat,
    relaxation: RelaxTable,
    validity: Validity,
) {
    let relax_table = match relaxation {
        Relax => quote!(, relax_table),
        NoRelax => quote!(),
    };
    let table_name = match table_name_format {
        Raw => {
            let table_name = format_ident!("{table_name}");
            quote!(#table_name)
        }
        Stringified => quote!(#table_name),
    };
    let input = quote!(
        #[derive(Edge)]
        #[surreal_orm(table = #table_name #relax_table)]
        pub struct BigWrites<In, Out> {
            id: SurrealSimpleId<Self>,

            #[surreal_orm(link_one = In)]
            r#in: LinkOne<In>,

            #[surreal_orm(link_one = Out)]
            out: LinkOne<Out>,
        }
    );

    let derive_input = syn::parse2(input).unwrap();
    let node_token = NodeToken::from_derive_input(&derive_input).unwrap();

    // insta::assert_snapshot!(
    //     format!("edge_table_name_tests-{table_name}-{table_name_format}-{relaxation}-{validity}"),
    //     format!("{:#}", node_token.to_token_stream())
    // );

    let must_be_in_snake_case_error = || {
        node_token
            .to_token_stream()
            .to_string()
            .contains("table name must be in snake case of the current struct name")
    };

    match validity {
        Valid => assert_not!(must_be_in_snake_case_error()),
        Invalid => assert!(must_be_in_snake_case_error()),
    }
}

