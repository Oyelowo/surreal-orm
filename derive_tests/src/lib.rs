

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;

    #[test]
    fn test_node_trait_derive() {
        let input = quote!(
            #[derive(Node)]
            #[surreal_orm(table = "student", drop, schemafull, permission, define = "any_fnc")]
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


        let actual = generate_fields_getter_trait(input.into());
        assert_eq!(actual.to_string(), "expected".to_string());
    }
}
