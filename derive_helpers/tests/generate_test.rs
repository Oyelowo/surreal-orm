
use quote::{format_ident, quote, ToTokens};
use darling::FromDeriveInput;
use surreal_derive_helpers::models::NodeToken;
use test_case::test_case;

enum ModelType {
    Node,
    Edge,
}
use ModelType::*;

// General test for the Node derive macro
#[test_case(Node ; "node model common attributes")]
#[test_case(Edge ; "edge model common attributes")]
fn test_node_trait_derive(model_type: ModelType) {
    let model_type = match model_type {
        Node => quote!(Node),
        Edge => quote!(Edge),
    };

    let input = quote!(
        #[derive(#model_type)]
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
        format!("{model_type}_trait_derive"),
        format!("{:#}", node_token.to_token_stream())
    );
}
