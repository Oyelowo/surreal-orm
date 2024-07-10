use std::fmt::Display;

use darling::FromDeriveInput;
use quote::{quote, ToTokens};
use surreal_derive_helpers::models::NodeToken;
use surreal_query_builder::assert_not;
use test_case::test_case;

#[test]
fn test_node_use_either_main_define_attr_or_individual_parts_but_not_both() {
    let input = quote!(
        #[derive(Node)]
        #[surreal_orm(table = student, drop, schemafull, permissions = perm, define = define_student)]
        pub struct Student {
            id: SurrealSimpleId<Self>,
        }
    );

    let derive_input = syn::parse2(input).unwrap();
    let node_token = NodeToken::from_derive_input(&derive_input).unwrap();
    let node_token = node_token.to_token_stream().to_string();

    let node_token_no_space = node_token.replace(" ", "");
    assert!(node_token_no_space.contains("compile_error!{\"Invalidcombination.When`define`"));
    insta::assert_snapshot!(node_token,);
}

enum DefinitionStrategy {
    MainDefine,
    IndividualParts,
    Both,
}

impl Display for DefinitionStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MainDefine => write!(f, "MainDefine"),
            IndividualParts => write!(f, "IndividualParts"),
            Both => write!(f, "Both"),
        }
    }
}
use DefinitionStrategy::*;

#[test_case(MainDefine)]
#[test_case(IndividualParts)]
#[test_case(Both)]
fn test_node_use_either_main_define(def_strategy: DefinitionStrategy) {
    let parts = quote!(drop, schemafull, permissions = perm);
    let main_define = quote!(define = define_writes);
    let defs = match def_strategy {
        MainDefine => main_define,
        IndividualParts => parts,
        Both => quote!(#parts, #main_define),
    };

    let input = quote!(
        #[derive(Node)]
        #[surreal_orm(table = student, #defs)]
        pub struct Student {
            id: SurrealSimpleId<Self>,
        }
    );

    let derive_input = syn::parse2(input).unwrap();
    let node_token = NodeToken::from_derive_input(&derive_input).unwrap();
    let node_token = node_token.to_token_stream().to_string();

    let node_token_no_space = node_token.replace(" ", "");
    let is_valid =
        !node_token_no_space.contains("compile_error!{\"Invalidcombination.When`define`");

    match def_strategy {
        MainDefine => {
            assert!(is_valid);
            insta::assert_snapshot!(def_strategy.to_string(), node_token);
        }
        IndividualParts => {
            assert!(is_valid);
            insta::assert_snapshot!(def_strategy.to_string(), node_token);
        }
        Both => {
            assert_not!(is_valid);
            insta::assert_snapshot!(def_strategy.to_string(), node_token);
        }
    }
}

#[test_case(MainDefine)]
#[test_case(IndividualParts)]
#[test_case(Both)]
fn test_edgeuse_either_main_define_attr_or_individual_parts_but_not_both(
    def_strategy: DefinitionStrategy,
) {
    let parts = quote!(drop, schemafull, permissions = perm);
    let main_define = quote!(define = define_writes);
    let defs = match def_strategy {
        MainDefine => main_define,
        IndividualParts => parts,
        Both => quote!(#parts, #main_define),
    };
    let input = quote!(
        #[derive(Edge)]
        #[surreal_orm(table = writes, #defs)]
        pub struct Writes<In, Out> {
            id: SurrealSimpleId<Self>,

            #[surreal_orm(link_one = In)]
            r#in: LinkOne<In>,

            #[surreal_orm(link_one = Out)]
            out: LinkOne<Out>,
        }
    );

    let derive_input = syn::parse2(input).unwrap();
    let node_token = NodeToken::from_derive_input(&derive_input).unwrap();
    let node_token = node_token.to_token_stream().to_string();

    let node_token_no_space = node_token.replace(" ", "");

    let is_valid =
        !node_token_no_space.contains("compile_error!{\"Invalidcombination.When`define`");

    match def_strategy {
        MainDefine => {
            assert!(is_valid);
        }
        IndividualParts => {
            assert!(is_valid);
        }
        Both => {
            assert_not!(is_valid);
            let snapshot_name = format!("edge-{def_strategy}");
            insta::assert_snapshot!(snapshot_name, node_token);
        }
    };
}
