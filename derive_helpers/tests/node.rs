#[cfg(test)]
mod tests {
    use super::*;
    use darling::FromDeriveInput;
    use quote::{quote, ToTokens};
    use surreal_derive_helpers::models::NodeToken;

    #[test]
    fn test_node_trait_derive() {
        let input = quote!(
            #[derive(Node)]
            #[surreal_orm(table = "student", drop, schemafull, permissions = perm)]
            pub struct Student {
                #[serde(skip_serializing_if = "Option::is_none")]
                #[builder(default, setter(strip_option))]
                id: Option<String>,
                first_name: String,

                #[surreal_orm(link_one = "Book", skip_serializing)]
                course: LinkOne<Book>,

                #[surreal_orm(link_many = "Book", skip_serializing)]
                #[serde(rename = "lowo")]
                all_semester_courses: LinkMany<Book>,

                #[surreal_orm(relate(model = "StudentWritesBlog", connection = "->writes->Blog"))]
                written_blogs: Relate<Blog>,
            }
        );

        let derive_input = syn::parse2(input).unwrap();
        let node_token = NodeToken::from_derive_input(&derive_input).unwrap();
       insta::assert_snapshot!(node_token.to_token_stream().to_string(), "node_trait_derive");

    }
}
