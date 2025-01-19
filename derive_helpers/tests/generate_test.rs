use darling::FromDeriveInput;
use quote::{quote, ToTokens};
use surreal_derive_helpers::models::{EdgeToken, NodeToken};
use surreal_query_builder::assert_not;

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
            #[serde(skip_serializing, default)]
            written_blogs: Relate<Blog>,
        }
    );

    let derive_input = syn::parse2(input).unwrap();
    let node_token = NodeToken::from_derive_input(&derive_input).unwrap();
    let node_token = node_token.to_token_stream().to_string();

    assert!(node_token.contains("impl surreal_orm :: Node for Student "));
    assert_not!(node_token.contains("impl surreal_orm :: Edge for Student "));
    insta::assert_snapshot!(
        "node_trait_derive",
        format!("{:#}", node_token.to_token_stream())
    );
}

enum RelateFieldAttribute {
    SerdeSkipSerializingOnly,
    DefaultOnly,
    BothAttrs,
}
use test_case::test_case;
use RelateFieldAttribute::*;
#[test_case(SerdeSkipSerializingOnly)]
#[test_case(DefaultOnly)]
#[test_case(BothAttrs)]
fn test_relate_field_has_serde_skip_serializing_but_not_attribute(
    relate_field_attr_provided: RelateFieldAttribute,
) {
    let serde_attr_for_relate = match relate_field_attr_provided {
        SerdeSkipSerializingOnly => quote!(skip_serializing),
        DefaultOnly => quote!(default),
        BothAttrs => quote!(skip_serializing, default),
    };

    let input = quote!(
        #[derive(Node, Serialize, Deserialize, Debug, Clone)]
        #[surreal_orm(table = student)]
        pub struct Student {
            id: SurrealSimpleId<Self>,
            first_name: String,

            #[surreal_orm(link_one = Book)]
            course: LinkOne<Book>,

            #[surreal_orm(link_many = Book)]
            #[serde(rename = "lowo")]
            all_semester_courses: LinkMany<Book>,

            #[surreal_orm(relate(model = "StudentWritesBlog", connection = "->writes->Blog"))]
            #[serde(#serde_attr_for_relate)]
            written_blogs: Relate<Blog>,
        }
    );

    let derive_input = syn::parse2(input).unwrap();
    let node_token = NodeToken::from_derive_input(&derive_input).unwrap();
    let node_token = node_token.to_token_stream().to_string();

    let error_msg = || {
        node_token.contains("Missing required 'skip_serializing' or 'default' serde attribute(s) on `written_blogs`")
    };
    let has_node_trait_valid = || node_token.contains("impl surreal_orm :: Node for Student ");
    match relate_field_attr_provided {
        SerdeSkipSerializingOnly => {
            assert!(
                error_msg(),
                "should have default and skip_serializing attributes"
            );
            assert_not!(has_node_trait_valid());
        }
        DefaultOnly => {
            assert!(
                error_msg(),
                "should have default and skip_serializing attributes"
            );
            assert_not!(has_node_trait_valid());
        }
        BothAttrs => {
            assert_not!(error_msg());
            assert!(has_node_trait_valid());
        }
    }

    insta::assert_snapshot!(
        format!(
            "relate_field_must_have_serde_skip_serializing_attribute-{}",
            serde_attr_for_relate.to_string()
        ),
        format!("{:#}", node_token.to_token_stream())
    );
}

#[test]
fn test_edge_trait_derive() {
    let input = quote!(
        #[derive(Edge)]
        #[surreal_orm(table = writes, drop, schemafull, permissions = perm)]
        pub struct Writes<In, Out> {
            id: SurrealSimpleId<Self>,

            duration_of_write: Duration,

            #[surreal_orm(link_one = In)]
            r#in: LinkOne<In>,

            #[surreal_orm(link_one = Out)]
            out: LinkOne<Out>,

            #[surreal_orm(link_one = Book)]
            course: LinkOne<Book>,

            #[surreal_orm(link_many = Book)]
            field_nother: LinkMany<Book>,
        }
    );

    let derive_input = syn::parse2(input).expect("Failed to parse input");
    let node_token = EdgeToken::from_derive_input(&derive_input).expect("Failed to get node token");
    let node_token = node_token.to_token_stream().to_string();
    let node_no_whitespace = node_token.replace(" ", "");

    assert!(node_no_whitespace.contains("impl<In,Out>surreal_orm::EdgeforWrites<In,Out>"));
    assert!(node_token.contains("impl < In , Out > surreal_orm :: Edge for Writes < In , Out >"));
    assert_not!(node_no_whitespace.contains("impl<In,Out>surreal_orm::NodeforWrites<In,Out>"));
    assert_not!(
        node_token.contains("impl < In , Out > surreal_orm :: Node for Writes < In , Out >")
    );

    insta::assert_snapshot!(
        "edge_trait_derive",
        format!("{:#}", node_token.to_token_stream())
    );
}
