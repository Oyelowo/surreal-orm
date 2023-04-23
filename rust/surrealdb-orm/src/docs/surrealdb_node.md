# Attributes

| Attribute         | Description                                                                                                              | Type                       | Optional |
| ----------------- | ------------------------------------------------------------------------------------------------------------------------ | -------------------------- | -------- |
| rename            | Renames the field in the database.                                                                                       | `Option<Rename>`           | Y        |
| relate            | Specifies a link to another object, similar to a foreign key in a relational database. The link is a singular reference. | `Option<Relate>`           | Y        |
| link_one          | Specifies a link to another object, similar to a foreign key in a relational database. The link is a singular reference. | `Option<String>`           | Y        |
| link_self         | Specifies a link to itself, for self-referencing objects.                                                                | `Option<String>`           | Y        |
| link_many         | Specifies a link to another object, similar to a foreign key in a relational database. The link is a plural reference.   | `Option<String>`           | Y        |
| nest_array        | Specifies a nested array of objects.                                                                                     | `Option<String>`           | Y        |
| nest_object       | Specifies a nested object.                                                                                               | `Option<String>`           | Y        |
| skip_serializing  | Specifies if the field should be ignored when serializing to the database.                                               | `bool`                     |          |
| type              | Specifies the type of the field in the database.                                                                         | `Option<FieldTypeWrapper>` | Y        |
| assert            | Asserts a condition on the field.                                                                                        | `Option<syn::LitStr>`      | Y        |
| assert_fn         | Specifies the function to assert a condition on the field.                                                               | `Option<syn::Path>`        | Y        |
| define            | Defines a constant value for the field.                                                                                  | `Option<syn::LitStr>`      | Y        |
| define_fn         | Specifies the function to define a constant value for the field.                                                         | `Option<syn::Path>`        | Y        |
| value             | Specifies the value for the field.                                                                                       | `Option<syn::LitStr>`      | Y        |
| value_fn          | Specifies the function to set the value for the field.                                                                   | `Option<syn::Path>`        | Y        |
| permissions       | Specifies the permissions for the field.                                                                                 | `Option<Permissions>`      | Y        |
| permissions_fn    | Specifies the function to set the permissions for the field.                                                             | `Option<PermissionsFn>`    | Y        |
| content_type      | Specifies the type of the content.                                                                                       | `Option<FieldTypeWrapper>` | Y        |
| content_assert    | Asserts a condition on the content.                                                                                      | `Option<syn::LitStr>`      | Y        |
| content_assert_fn | Specifies the function to assert a condition on the content.                                                             | `Option<syn::Path>`        | Y        |
| rename_all        | Renames all fields in the database according to a case convention.                                                       | `Option<Rename>`           | Y        |
| table_name        | Specifies the name of the table in the database.                                                                         | `Option<String>`           | Y        |
| relax_table_name  | Specifies if the table name should be relaxed, i.e. no table name will be generated if `table_name` is not specified.    | `Option<bool>`             | Y        |
| schemafull        | Specifies if the table should be fully qualified with                                                                    |

# Field attributes

| Attribute        | Description                                                                                                                                                                                                                                   | Type             | Optional |
| ---------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------- | -------- |
| rename           | Renames a field to the given name.                                                                                                                                                                                                            | [Rename](``)     | Y        |
| relate           | Generates the relation helpers for the Current Node struct to an edge and destination node. The corresponding field name is merely used as an alias in code generation and is read only and not serializable. e.g `student:1->writes->book:2` |
| link_one         | Specifies a relationship to a singular record in another node table in the database.                                                                                                                                                          | Option\<String\> | Y        |
| link_self        | Specifies a relationship to a singular record in the same node table in the database.                                                                                                                                                         | Option\<String\> | Y        |
| link_many        | Specifies a relationship to multiple records in another node table in the database.                                                                                                                                                           | Option\<String\> | Y        |
| nest_array       | Specifies a nested array of objects.                                                                                                                                                                                                          | Option\<String\> | Y        |
| nest_object      | Specifies a nested object.                                                                                                                                                                                                                    | Option\<String\> | Y        |
| skip_serializing | When true, this field will be omitted when serializing the struct.                                                                                                                                                                            |
