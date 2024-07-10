use std::fmt::Display;

use darling::FromDeriveInput;
use quote::{format_ident, quote, ToTokens};
use surreal_derive_helpers::models::NodeToken;

macro_rules! assert_not {
    ($e:expr) => {
        assert!(!$e)
    };
}

// General test for the Node derive macro
#[test]
fn test_node_trait_derive() {
    let input = quote!(
        #[derive(Node)]
        #[surreal_orm(table = "student", drop, schemafull, permissions = perm)]
        pub struct Student {
            id: SurrealSimpleId<Self>,
            first_name: String,

            #[surreal_orm(link_one = Book)]
            course: LinkOne<Book>,

            #[surreal_orm(link_many = Book)]
            #[serde(rename = "lowo")]
            all_semester_courses: LinkMany<Book>,

            #[surreal_orm(relate(model = "StudentWritesBlog", connection = "->writes->Blog"))]
            written_blogs: Relate<Blog>,
        }
    );

    let derive_input = syn::parse2(input).unwrap();
    let node_token = NodeToken::from_derive_input(&derive_input).unwrap();
    insta::assert_snapshot!(
        "node_trait_derive",
        format!("{:#}", node_token.to_token_stream())
    );
}

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

// #[test_case(Raw("student_table"),  NoRelax, Valid; "snake case table name with struct name even without relax")]
// #[test_case(Raw("student_table"),  Relax, Valid; "snake case table name with struct name with relax")]
// #[test_case(Stringified("StudentTable"),  NoRelax, Invalid; "non snake case table name version of struct name cannot be used without relax")]
// #[test_case(Stringified("StudentTable"),  Relax, Invalid; "non snake case table name version of struct name cann be used with relax")]

#[test_case("student_table", Raw, NoRelax, Valid; "snake case raw table name with struct name even without relax")]
#[test_case("student_table", Raw, Relax, Valid; "snake case raw table name with struct name with relax")]
#[test_case("student_table", Stringified, NoRelax, Valid; "snake case stringified table name version of struct name cannot be used without relax")]
#[test_case("student_table", Stringified, Relax, Valid; "snake case stringified table name version of struct name cann be used with relax")]

#[test_case("StudentTable",  Raw, NoRelax, Invalid; "non snake case raw table name version of struct name cannot be used without relax")]
#[test_case("StudentTable",  Raw, Relax, Valid; "non snake case raw table name version of struct name cann be used with relax")]
#[test_case("StudentTable",  Stringified, NoRelax, Invalid; "non snake case stringified table name version of struct name cannot be used without relax")]
#[test_case("StudentTable",  Stringified, Relax, Valid; "non snake case stringified table name version of struct name cann be used with relax")]
fn test_table_name(
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

    insta::assert_snapshot!(
        format!("node_table_name_tests-{table_name}-{table_name_format}-{relaxation}-{validity}"),
        format!("{:#}", node_token.to_token_stream())
    );

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

// #[test]
// fn test_node_table_name_must_be_snake_case_format_of_struct_name() {
//     let table_name = "student_table";
//     let input = quote!(
//         #[derive(Node)]
//         #[surreal_orm(table = #table_name)]
//         pub struct StudentTable {
//             id: SurrealSimpleId<Self>,
//         }
//     );
//
//     let derive_input = syn::parse2(input).unwrap();
//     let node_token = NodeToken::from_derive_input(&derive_input).unwrap();
//     insta::assert_snapshot!(
//         "node_trait_derive_table_name",
//         format!("{:#}", node_token.to_token_stream())
//     );
//     assert_not!(must_be_in_snake_case_error(node_token));
// }
//
// #[test]
// fn test_node_table_name_must_be_snake_case_format_of_struct_name_no_quotes_in_table_name() {
//     let input = quote!(
//         #[derive(Node)]
//         #[surreal_orm(table = student_table)]
//         pub struct StudentTable {
//             id: SurrealSimpleId<Self>,
//         }
//     );
//
//     let derive_input = syn::parse2(input).unwrap();
//     let node_token = NodeToken::from_derive_input(&derive_input).unwrap();
//     insta::assert_snapshot!(
//         "node_trait_derive_table_name",
//         format!("{:#}", node_token.to_token_stream())
//     );
//     assert_not!(must_be_in_snake_case_error(node_token));
// }
//
// #[test]
// fn test_node_trait_derive_table_name_done_right() {
//     let input_wrong_case = quote!(
//         #[derive(Node)]
//         #[surreal_orm(table = "student_table")]
//         pub struct StudentTable {
//             id: SurrealSimpleId<Self>,
//         }
//     );
//
//     let derive_input = syn::parse2(input_wrong_case).unwrap();
//     let node_token = NodeToken::from_derive_input(&derive_input).unwrap();
//     assert_not!(must_be_in_snake_case_error(node_token));
// }
//
// #[test]
// fn test_node_table_name_not_snake_case_version_of_the_struct_name() {
//     let input_wrong_case = quote!(
//         #[derive(Node)]
//         #[surreal_orm(table = "studentTable")]
//         pub struct StudentTable {
//             id: SurrealSimpleId<Self>,
//         }
//     );
//
//     let derive_input = syn::parse2(input_wrong_case).unwrap();
//     let node_token = NodeToken::from_derive_input(&derive_input).unwrap();
//     assert!(must_be_in_snake_case_error(node_token));
// }
//
// #[test]
// fn test_node_table_name_snake_case_but_completely_different_name() {
//     let input_wrong_case = quote!(
//         #[derive(Node)]
//         #[surreal_orm(table = "snake_case_but_different_name")]
//         pub struct StudentTable {
//             id: SurrealSimpleId<Self>,
//         }
//     );
//
//     let derive_input = syn::parse2(input_wrong_case).unwrap();
//     let node_token = NodeToken::from_derive_input(&derive_input).unwrap();
//     assert!(must_be_in_snake_case_error(node_token));
// }
//
// #[test]
// fn test_node_trait_derive_table_name_error_no_quote() {
//     let input_wrong_case = quote!(
//         #[derive(Node)]
//         #[surreal_orm(table = studentTableWrongCase)]
//         pub struct StudentTable {
//             id: SurrealSimpleId<Self>,
//         }
//     );
//
//     let derive_input = syn::parse2(input_wrong_case).unwrap();
//     let node_token = NodeToken::from_derive_input(&derive_input).unwrap();
//     assert!(must_be_in_snake_case_error(node_token));
// }
//
// #[test]
// fn test_node_relax_table_name_to_opt_out_of_snake_case_name_version_of_struct_convention() {
//     let input_wrong_case = quote!(
//         #[derive(Node)]
//         #[surreal_orm(table = "studentTableWrongCase", relax_table)]
//         pub struct StudentTable {
//             id: SurrealSimpleId<Self>,
//         }
//     );
//
//     let derive_input = syn::parse2(input_wrong_case).unwrap();
//     let node_token = NodeToken::from_derive_input(&derive_input).unwrap();
//     assert_not!(must_be_in_snake_case_error(node_token));
// }
