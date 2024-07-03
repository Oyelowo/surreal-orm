
    fn based_on_field_relation_type(
    fn based_on_field_relation_type(
        &self,
        field_ty: &CustomType,
        relation_type: &RelationType,
    ) -> ExtractorResult<Option<DbFieldTypeAstMeta>> {
        let crate_name = get_crate_name(false);
        let ty = &field_ty.into_inner_ref();

        let meta = match relation_type {
                RelationType::Relate(_ref_node) => {
                    // Relation are not stored on nodes, but
                    // on edges. Just used on nodes for convenience
                    // during deserialization
                    None
                }
    RelationType::LinkOne(ref_node) => {
        let db_field_type_ast_meta = DbFieldTypeAstMeta {
                    field_type_db_original: FieldType::Record(vec![]),
                    field_type_db_token: quote!(#crate_name::FieldType::Record(::std::vec![#ref_node::table()])).into(),
                    static_assertion_token: quote!(
    #crate_name::validators::assert_type_eq_all!(#field_ty, #crate_name::LinkOne<#ref_node>);
                ).into(),
                };
        Some(db_field_type_ast_meta)
    },
            RelationType::LinkSelf(self_node) => {
                let current_struct_type = self.model_attrs.struct_no_bounds()?;
                let db_field_type_ast_meta = DbFieldTypeAstMeta {
                            field_type_db_original: FieldType::Record(vec![]),
                            field_type_db_token: quote!(#crate_name::FieldType::Record(::std::vec![Self::table()])).into(),
                            static_assertion_token: quote!(
                    quote!(#crate_name::validators::assert_type_eq_all!(#current_struct_type, #crate_name::LinkSelf<#self_node>);),
                    quote!(#crate_name::validators::assert_type_eq_all!(#field_ty, #crate_name::LinkSelf<#self_node>);),

                        ).into(),
                        };
                Some(db_field_type_ast_meta)
            },
            RelationType::LinkMany(ref_node) | RelationType::LinkManyInAndOutEdgeNodesInert(ref_node) => DbFieldTypeAstMeta {
                field_type_db_original: FieldType::Array(
                    ::std::boxed::Box::new(FieldType::Record(vec![])),
                    ::std::option::Option::None
                ),
                field_type_db_token: quote!(#crate_name::FieldType::Array(
                    ::std::boxed::Box::new(#crate_name::FieldType::Record(::std::vec![#ref_node::table()])),
                    ::std::option::Option::None
                )).into(),
                static_assertion_token: quote!(
                    #crate_name::validators::assert_type_eq_all!(#field_ty, #crate_name::LinkMany<#ref_node>);
            ).into(),
            },
            RelationType::NestObject(_ref_object) => DbFieldTypeAstMeta {
                field_type_db_original: FieldType::Object,
                field_type_db_token: quote!(#crate_name::FieldType::Object).into(),
                static_assertion_token:
                quote!(#crate_name::validators::assert_type_is_object::<#ty>();).into(),
            },
            RelationType::NestArray(foreign_array_object) => {

        let nesting_level = Self::count_vec_nesting(field_ty.to_basic_type());
                let nested_vec_type =
                    Self::generate_nested_vec_type(&foreign_array_object, nesting_level);

                DbFieldTypeAstMeta {
                            // provide the inner type for when the array part start recursing
                            field_type_db_original: FieldType::Object,
                            field_type_db_token: quote!(#crate_name::FieldType::Object).into(),
                            // db_field_type: quote!(#crate_name::FieldType::Array(
                            //     ::std::boxed::Box::new(#crate_name::FieldType::Object),
                            //     ::std::option::Option::None
                            // )),
                            static_assertion_token: quote!(#crate_name::validators::assert_type_eq_all!(#foreign_array_object, #nested_vec_type);).into(),
                            // static_assertion_token:
                            //     quote!(#crate_name::validators::assert_type_is_array::<#ty>();).into(),
                        }
            },
                // We already did for list/array/set earlier. 
                // TODO: Consider removing the concept of list altogether to 
                // avoid confusion/ambiguity
                RelationType::List(_) | RelationType::None => {
                    return Err(syn::Error::new(
                        ty.span(),
                        format!("Could not infer type for the field. Specify explicitly by using e.g ty = \"array<any>\". You can choose from one of these types: {}", FieldType::variants().join(", ")),
                    )
                    .into())
                }
            };

        Ok(meta)
    }

    fn generate_nested_vec_type(
        foreign_node: &CustomType,
        nesting_level: usize,
    ) -> proc_macro2::TokenStream {
        if nesting_level == 0 {
            quote!(#foreign_node)
        } else {
            let inner_type = Self::generate_nested_vec_type(foreign_node, nesting_level - 1);
            quote!(::std::vec::Vec<#inner_type>)
        }
    }

    fn count_vec_nesting(field_type: &syn::Type) -> usize {
        match field_type {
            syn::Type::Path(type_path) => {
                if let Some(segment) = type_path.path.segments.last() {
                    if segment.ident == "Vec" {
                        if let syn::PathArguments::AngleBracketed(angle_args) = &segment.arguments {
                            if let Some(syn::GenericArgument::Type(inner_type)) =
                                angle_args.args.first()
                            {
                                1 + Self::count_vec_nesting(inner_type)
                            } else {
                                0
                            }
                        } else {
                            0
                        }
                    } else {
                        0
                    }
                } else {
                    0
                }
            }
            _ => 0,
        }
    }
}
        &self,
        field_ty: &CustomType,
        relation_type: &RelationType,
    ) -> ExtractorResult<Option<DbFieldTypeAstMeta>> {
        let crate_name = get_crate_name(false);
        let ty = &field_ty.into_inner_ref();

        let meta = match relation_type {
                RelationType::Relate(_ref_node) => {
                    // Relation are not stored on nodes, but
                    // on edges. Just used on nodes for convenience
                    // during deserialization
                    None
                }
    RelationType::LinkOne(ref_node) => {
        let db_field_type_ast_meta = DbFieldTypeAstMeta {
                    field_type_db_original: FieldType::Record(vec![]),
                    field_type_db_token: quote!(#crate_name::FieldType::Record(::std::vec![#ref_node::table()])).into(),
                    static_assertion_token: quote!(
    #crate_name::validators::assert_type_eq_all!(#field_ty, #crate_name::LinkOne<#ref_node>);
                ).into(),
                };
        Some(db_field_type_ast_meta)
    },
            RelationType::LinkSelf(self_node) => {
                let current_struct_type = self.model_attrs.struct_no_bounds()?;
                let db_field_type_ast_meta = DbFieldTypeAstMeta {
                            field_type_db_original: FieldType::Record(vec![]),
                            field_type_db_token: quote!(#crate_name::FieldType::Record(::std::vec![Self::table()])).into(),
                            static_assertion_token: quote!(
                    quote!(#crate_name::validators::assert_type_eq_all!(#current_struct_type, #crate_name::LinkSelf<#self_node>);),
                    quote!(#crate_name::validators::assert_type_eq_all!(#field_ty, #crate_name::LinkSelf<#self_node>);),

                        ).into(),
                        };
                Some(db_field_type_ast_meta)
            },
            RelationType::LinkMany(ref_node) | RelationType::LinkManyInAndOutEdgeNodesInert(ref_node) => DbFieldTypeAstMeta {
                field_type_db_original: FieldType::Array(
                    ::std::boxed::Box::new(FieldType::Record(vec![])),
                    ::std::option::Option::None
                ),
                field_type_db_token: quote!(#crate_name::FieldType::Array(
                    ::std::boxed::Box::new(#crate_name::FieldType::Record(::std::vec![#ref_node::table()])),
                    ::std::option::Option::None
                )).into(),
                static_assertion_token: quote!(
                    #crate_name::validators::assert_type_eq_all!(#field_ty, #crate_name::LinkMany<#ref_node>);
            ).into(),
            },
            RelationType::NestObject(_ref_object) => DbFieldTypeAstMeta {
                field_type_db_original: FieldType::Object,
                field_type_db_token: quote!(#crate_name::FieldType::Object).into(),
                static_assertion_token:
                quote!(#crate_name::validators::assert_type_is_object::<#ty>();).into(),
            },
            RelationType::NestArray(foreign_array_object) => {

        let nesting_level = Self::count_vec_nesting(field_ty.to_basic_type());
                let nested_vec_type =
                    Self::generate_nested_vec_type(&foreign_array_object, nesting_level);

                DbFieldTypeAstMeta {
                            // provide the inner type for when the array part start recursing
                            field_type_db_original: FieldType::Object,
                            field_type_db_token: quote!(#crate_name::FieldType::Object).into(),
                            // db_field_type: quote!(#crate_name::FieldType::Array(
                            //     ::std::boxed::Box::new(#crate_name::FieldType::Object),
                            //     ::std::option::Option::None
                            // )),
                            static_assertion_token: quote!(#crate_name::validators::assert_type_eq_all!(#foreign_array_object, #nested_vec_type);).into(),
                            // static_assertion_token:
                            //     quote!(#crate_name::validators::assert_type_is_array::<#ty>();).into(),
                        }
            },
                // We already did for list/array/set earlier. 
                // TODO: Consider removing the concept of list altogether to 
                // avoid confusion/ambiguity
                RelationType::List(_) | RelationType::None => {
                    return Err(syn::Error::new(
                        ty.span(),
                        format!("Could not infer type for the field. Specify explicitly by using e.g ty = \"array<any>\". You can choose from one of these types: {}", FieldType::variants().join(", ")),
                    )
                    .into())
                }
            };

        Ok(meta)
    }

    fn generate_nested_vec_type(
        foreign_node: &CustomType,
        nesting_level: usize,
    ) -> proc_macro2::TokenStream {
        if nesting_level == 0 {
            quote!(#foreign_node)
        } else {
            let inner_type = Self::generate_nested_vec_type(foreign_node, nesting_level - 1);
            quote!(::std::vec::Vec<#inner_type>)
        }
    }

    fn count_vec_nesting(field_type: &syn::Type) -> usize {
        match field_type {
            syn::Type::Path(type_path) => {
                if let Some(segment) = type_path.path.segments.last() {
                    if segment.ident == "Vec" {
                        if let syn::PathArguments::AngleBracketed(angle_args) = &segment.arguments {
                            if let Some(syn::GenericArgument::Type(inner_type)) =
                                angle_args.args.first()
                            {
                                1 + Self::count_vec_nesting(inner_type)
                            } else {
                                0
                            }
                        } else {
                            0
                        }
                    } else {
                        0
                    }
                } else {
                    0
                }
            }
            _ => 0,
        }
    }
}
