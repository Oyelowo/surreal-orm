
        let mut static_assertions = vec![];
        let crate_name = get_crate_name(false);

        if let Some(type_) = &self.type_ {
            let field_type = type_.deref();
            // id: record<student>
            // in: record
            // out: record
            // link_one => record<book> = #crate_name::validators::assert_has_field(<Book as Node>::TableNameChecker, book);
            // link_self => record<student> = #crate_name::validators::assert_has_field(<Student as Node>::TableNameChecker, student);
            // link_many => Vec<Book> => array<record<book>> = #crate_name::validators::assert_has_field(<Book as Node>::TableNameChecker, book);
            // e.g names: Vec<T> => array || array<string> => names: array && names.* : string

            match self {
                MyFieldReceiver {
                    type_: Some(type_),
                    item_assert,
                    item_assert_fn,
                    ..
                } if !type_.is_array() & (item_assert.is_some() || item_assert_fn.is_some()) => {
                    return Err(syn::Error::new_spanned(
                        field_name,
                        "item_assert or item_assert_fn can only be used with array types",
                    )
                    .into());
                }
                MyFieldReceiver {
                    type_: Some(type_),
                    link_one,
                    link_self,
                    link_many,
                    ..
                } => {
                    let linked_node = link_one.clone().or(link_self.clone());
                    let field_type = type_.deref();
                    let ref_node_table_name_checker_ident =
                        format_ident!("I{field_name}RefChecker");
                    // TODO: Check
                    let xx = quote!(#ref_node_table_name_checker_ident );

                    if let Some(link_single_ref_node) = linked_node {
                        // Validate that it is a type - record, when link_one or link_self used,
                        // since those attributes are used for record links. When record type
                        // provided, do static assertions validation to check the inner type e.g
                        // record<book>
                        match field_type {
                            FieldType::Record(link_table_names) => {
                                let link_table_name = format_ident!(
                                    "{}",
                                    link_table_names
                                        .first()
                                        .map(ToString::to_string)
                                        .unwrap_or_default()
                                );
                                // TODO: Remove
                                // let ref_node = NodeTypeName::from(&link_single_ref_node);
                                // let ref_node_token: TokenStream = ref_node.into();
                                // Generate validation for the record type content at compile
                                // time
                                // Check that the link name in the type is same used lin
                                // link_one attribute e.g record(book), when link_one="Book",
                                // which gives <Book as Node>::TableNameChecker
                                static_assertions.push(quote!(
                                type #ref_node_table_name_checker_ident = <#link_single_ref_node as #crate_name::Node>::TableNameChecker;
                                #crate_name::validators::assert_fields!(#ref_node_table_name_checker_ident: #link_table_name);
                                           ));
                            }
                            _ => {
                                return Err(syn::Error::new_spanned(
                                    field_name,
                                    "when link_one or link_self attribute is used, type must be record or record(<ref_node_table_name>)",
                                ).into());
                            }
                        }
                    } else if let Some(link_many_ref_node) = link_many {
                        match field_type.clone() {
                            FieldType::Array(item_type, _) | FieldType::Set(item_type, _) => {
                                // Check content type if of array type. link_many is used for
                                // // array types. e.g link_many = "Blog"

                                match item_type.deref() {
                                    FieldType::Record(array_item_table_name) => {
                                        match array_item_table_name.len() {
                                            1 => {
                                                let array_item_table_name = format_ident!(
                                                    "{}",
                                                    array_item_table_name
                                                        .first()
                                                        .expect("Table should be present here. This is a bug if not so.")
                                                        .to_string()
                                                );
                                                // TODO: Remove
                                                // let ref_node =
                                                //     NodeTypeName::from(link_many_ref_node);
                                                // let ref_node_token: TokenStream = ref_node.into();

                                                static_assertions.push(quote!(
                                            type #ref_node_table_name_checker_ident = <#link_many_ref_node as #crate_name::Node>::TableNameChecker;
                                            #crate_name::validators::assert_fields!(#ref_node_table_name_checker_ident: #array_item_table_name);
                                        ));
                                            }
                                            _ => {
                                                return Err(syn::Error::new_spanned(
                                                    field_name,
                                                    "when link_many attribute is provided, type_ should reference a single table in the format  - array<record<table>>",
                                                ).into());
                                            }
                                        }
                                    }
                                    _ => {
                                        let err = format!("when link_many attribute is provided, type_ must be of type array<record> or array<record<ref_node_table_name>>. Got - {}", item_type.deref());
                                        return Err(syn::Error::new_spanned(field_name, err).into());
                                    }
                                }
                            }
                            _ => {
                                return Err(syn::Error::new_spanned(
                                    field_name,
                                    "when link_many attribute is used, type must be array",
                                )
                                .into());
                            }
                        }
                    }
                }
                _ => {}
            };

            // Gather assertions for all field types
            if let DataType::Edge = model_type {
                match field_name.as_str() {
                    "id" => {
                        if !field_type.is_record_of_the_table(table) && !field_type.is_record_any()
                        {
                            let err = format!(
                                "`id` field must be of type `record<{}>` or `record<any>`",
                                table
                            );
                            return Err(syn::Error::new_spanned(field_name, err.as_str()).into());
                        }
                    }
                    "in" | "out" => {
                        if !field_type.is_record() {
                            let err =
                                format!("`{}` field must be of type `record<any>`", field_name);
                            return Err(syn::Error::new_spanned(field_name, err.as_str()).into());
                        }
                    }
                    _ => {}
                }
            }

            let raw_type = &self.ty;
            static_assertions.push(static_assertion);

            // Get the field type
            // define_field_methods.push(quote!(.type_(#type_.parse::<#crate_name::FieldType>()
            //                                             .expect("Must have been checked at compile time. If not, this is a bug. Please report"))
            //                                  )
            //                           );
            // define_field_methods.push(quote!(.type_(#crate_name::FieldType::String)));
            // let content
            let ft_string = field_type.to_string();
            Ok(Some(DbFieldTypeMeta {
                db_field_type: quote!(#ft_string.parse::<#crate_name::FieldType>()
                                                            .expect("Must have been checked at compile time. If not, this is a bug. Please report")),
                static_assertion: quote!( # ( #static_assertions ) *),
            }))
        } else if self.rust_type().type_is_inferrable(&field_name, model_type) {
            Ok(Some(
                self.rust_type()
                    .infer_surreal_type_heuristically(field_name, model_type),
            ))
        } else {
            return Err(syn::Error::new_spanned(field_name, format!(
                r#"Unable to infer database type for the field. Type must be provided for field - {}.\
            e.g use the annotation #[surreal_orm(type_="int")] to provide the type explicitly."#,
                field_name
            ).as_str()).into());
        }
