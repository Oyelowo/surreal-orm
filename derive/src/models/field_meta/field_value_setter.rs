use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use proc_macros_helpers::get_crate_name;
use quote::{format_ident, quote, ToTokens};

use crate::{
    errors::ExtractorResult,
    models::{derive_attributes::TableDeriveAttributes, MyFieldReceiver, RelationType},
};

pub struct FieldSetterNumericImpl(TokenStream);

impl ToTokens for FieldSetterNumericImpl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens);
    }
}

pub struct ArrayElementFieldSetterToken(TokenStream);
// let mut update_field_names_fields_types_kv = |array_element: Option<TokenStream>| {

pub struct FieldSetterImplTokens(TokenStream);

impl MyFieldReceiver {
    pub fn get_field_value_setter_impl(
        &self,
        table_attributes: TableDeriveAttributes,
    ) -> ExtractorResult<FieldSetterImplTokens> {
        let crate_name = get_crate_name(false);
        let struct_level_casing = table_attributes.struct_level_casing();
        let field_ident_serialized_fmt = self
            .normalize_ident(struct_level_casing)
            .field_ident_serialized_fmt;

        let field_name_as_camel = format_ident!(
            "{}x",
            field_ident_serialized_fmt.to_string().to_case(Case::Pascal)
        );

        let numeric_trait = if self.is_numeric() {
            Some(self.numeric_trait_token())
        } else {
            None
        };

        // Only works for vectors
        let array_trait = if field_receiver.is_list() {
        } else {
            quote!()
        };

        store.field_wrapper_type_custom_implementations
            .push(quote!(
            #[derive(Debug, Clone)]
            pub struct #field_name_as_camel(pub #crate_name::Field);

            impl ::std::convert::From<&str> for #field_name_as_camel {
            fn from(field_name: &str) -> Self {
            Self(#crate_name::Field::new(field_name))
            }
            }

            impl ::std::convert::From<#crate_name::Field> for #field_name_as_camel {
            fn from(field_name: #crate_name::Field) -> Self {
            Self(field_name)
            }
            }

            impl ::std::convert::From<&#field_name_as_camel> for #crate_name::ValueLike {
            fn from(value: &#field_name_as_camel) -> Self {
            let field: #crate_name::Field = value.into();
            field.into()
            }
            }

            impl ::std::convert::From<#field_name_as_camel> for #crate_name::ValueLike {
            fn from(value: #field_name_as_camel) -> Self {
            let field: #crate_name::Field = value.into();
            field.into()
            }
            }

            impl ::std::convert::From<&#field_name_as_camel> for #crate_name::Field {
            fn from(field_name:& #field_name_as_camel) -> Self {
            field_name.0.clone()
            }
            }

            impl ::std::convert::From<#field_name_as_camel> for #crate_name::Field {
            fn from(field_name: #field_name_as_camel) -> Self {
            field_name.0
            }
            }

            impl ::std::ops::Deref for #field_name_as_camel {
            type Target = #crate_name::Field;

            fn deref(&self) -> &Self::Target {
            &self.0
            }
            }

            impl ::std::ops::DerefMut for #field_name_as_camel {
            fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
            }
            }

            impl<T: #crate_name::serde::Serialize> ::std::convert::From<self::#field_name_as_camel> for #crate_name::SetterArg<T> {
            fn from(value: self::#field_name_as_camel) -> Self {
            Self::Field(value.into())
            }
            }

            impl<T: #crate_name::serde::Serialize> ::std::convert::From<&self::#field_name_as_camel> for #crate_name::SetterArg<T> {
            fn from(value: &self::#field_name_as_camel) -> Self {
            Self::Field(value.into())
            }
            }

            impl #field_impl_generics #crate_name::SetterAssignable<#field_type> for self::#field_name_as_camel  #field_where_clause {}

            impl #field_impl_generics #crate_name::Patchable<#field_type> for self::#field_name_as_camel  #field_where_clause {}

            #numeric_trait

            #array_trait
        ));
        todo!()
    }

    pub fn array_trait_impl(
        &self,
        table_attributes: &TableDeriveAttributes,
    ) -> ArrayElementFieldSetterToken {
        let crate_name = get_crate_name(false);
        // let field_name_as_camel = format_ident!(
        //     "{}x",
        //     self.normalize_ident(struct_level_casing)
        //         .field_ident_serialized_fmt
        //         .to_string()
        //         .to_case(Case::Pascal)
        // );
        // let field_type = self.rust_field_type();
        // let field_impl_generics = self.field_impl_generics();
        // let field_where_clause = self.field_where_clause();
        // let array_trait = quote!(
        //     impl #field_impl_generics #crate_name::SetterArray<#field_type> for self::#field_name_as_camel
        //     #field_where_clause {}
        // );
        let (generics_meta, array_item_type) = match self.to_relation_type() {
            RelationType::LinkMany(foreign_node) => {
                let generics_meta = foreign_node.get_generics_meta(table_attributes);
                (
                    Some(generics_meta),
                    Some(quote!(<#foreign_node as #crate_name::Model>::Id)),
                )
            }
            RelationType::NestArray(foreign_object) => {
                let generics_meta = foreign_object.get_generics_meta(table_attributes);
                (Some(generics_meta), Some(quote!(#foreign_object)))
            }
            _ => self
                .ty
                .get_array_inner_type()
                .map(|item| {
                    let generics_meta = item.get_generics_meta(table_attributes);
                    (Some(generics_meta), Some(quote!(#item)))
                })
                .or_else(|| {
                     self
                        .type_
                        .map(|t| t.get_array_item_type())
                        .flatten()
                        .map(|t| Some(None, Some(t.as_db_sql_value_tokenstream())))
                        .ok_or(|e| {
                            syn::Error::new_spanned(field_type, "Could not infer array type. Explicitly specify the type e.g ty = array<string>")
                        })
                })
                .map(|item_type| quote!(#item_type)),
        };

        let array_setter_impl = array_item_type.map(|item_type| {
            quote!(
                impl #crate_name::SetterArray<#item_type> for self::#field_name_as_camel  {}
            )
        });

        ArrayElementFieldSetterToken(array_trait)
    }

    pub fn numeric_trait_token(&self) -> FieldSetterNumericImpl {
        let numeric_trait = {
            quote!(
                impl #field_impl_generics #crate_name::SetterNumeric<#field_type> for self::#field_name_as_camel
                #field_where_clause {}

                impl ::std::convert::From<self::#field_name_as_camel> for #crate_name::NumberLike {
                    fn from(val: self::#field_name_as_camel) -> Self {
                        val.0.into()
                    }
                }

                impl ::std::convert::From<&self::#field_name_as_camel> for #crate_name::NumberLike {
                    fn from(val: &self::#field_name_as_camel) -> Self {
                        val.clone().0.into()
                    }
                }

                impl<T: ::std::convert::Into<#crate_name::NumberLike>> ::std::ops::Add<T> for #field_name_as_camel {
                    type Output = #crate_name::Operation;

                    fn add(self, rhs: T) -> Self::Output {
                        let rhs: #crate_name::NumberLike = rhs.into();

                        #crate_name::Operation {
                                query_string: format!("{} + {}", self.build(), rhs.build()),
                                bindings: [self.get_bindings(), rhs.get_bindings()].concat(),
                                errors: vec![],
                            }
                        }
                }

                impl<T: ::std::convert::Into<#crate_name::NumberLike>> ::std::ops::Sub<T> for #field_name_as_camel {
                    type Output = #crate_name::Operation;

                    fn sub(self, rhs: T) -> Self::Output {
                        let rhs: #crate_name::NumberLike = rhs.into();

                        #crate_name::Operation {
                            query_string: format!("{} - {}", self.build(), rhs.build()),
                            bindings: [self.get_bindings(), rhs.get_bindings()].concat(),
                            errors: vec![],
                        }
                    }
                }

                impl<T: ::std::convert::Into<#crate_name::NumberLike>> ::std::ops::Mul<T> for #field_name_as_camel {
                    type Output = #crate_name::Operation;

                    fn mul(self, rhs: T) -> Self::Output {
                        let rhs: #crate_name::NumberLike = rhs.into();

                        #crate_name::Operation {
                            query_string: format!("{} * {}", self.build(), rhs.build()),
                            bindings: [self.get_bindings(), rhs.get_bindings()].concat(),
                            errors: vec![],
                        }
                    }
                }

                impl<T: ::std::convert::Into<#crate_name::NumberLike>> ::std::ops::Div<T> for #field_name_as_camel {
                    type Output = #crate_name::Operation;

                    fn div(self, rhs: T) -> Self::Output {
                        let rhs: #crate_name::NumberLike = rhs.into();

                        #crate_name::Operation {
                            query_string: format!("{} / {}", self.build(), rhs.build()),
                            bindings: [self.get_bindings(), rhs.get_bindings()].concat(),
                            errors: vec![],
                        }
                    }
                }

                impl<T: ::std::convert::Into<#crate_name::NumberLike>> ::std::ops::Add<T> for &#field_name_as_camel {
                    type Output = #crate_name::Operation;

                    fn add(self, rhs: T) -> Self::Output {
                        let rhs: #crate_name::NumberLike = rhs.into();

                        #crate_name::Operation {
                                query_string: format!("{} + {}", self.build(), rhs.build()),
                                bindings: [self.get_bindings(), rhs.get_bindings()].concat(),
                                errors: vec![],
                            }
                        }
                }

                impl<T: ::std::convert::Into<#crate_name::NumberLike>> ::std::ops::Sub<T> for &#field_name_as_camel {
                    type Output = #crate_name::Operation;

                    fn sub(self, rhs: T) -> Self::Output {
                        let rhs: #crate_name::NumberLike = rhs.into();

                        #crate_name::Operation {
                            query_string: format!("{} - {}", self.build(), rhs.build()),
                            bindings: [self.get_bindings(), rhs.get_bindings()].concat(),
                            errors: vec![],
                        }
                    }
                }

                impl<T: ::std::convert::Into<#crate_name::NumberLike>> ::std::ops::Mul<T> for &#field_name_as_camel {
                    type Output = #crate_name::Operation;

                    fn mul(self, rhs: T) -> Self::Output {
                        let rhs: #crate_name::NumberLike = rhs.into();

                        #crate_name::Operation {
                            query_string: format!("{} * {}", self.build(), rhs.build()),
                            bindings: [self.get_bindings(), rhs.get_bindings()].concat(),
                            errors: vec![],
                        }
                    }
                }

                impl<T: ::std::convert::Into<#crate_name::NumberLike>> ::std::ops::Div<T> for &#field_name_as_camel {
                    type Output = #crate_name::Operation;

                    fn div(self, rhs: T) -> Self::Output {
                        let rhs: #crate_name::NumberLike = rhs.into();

                        #crate_name::Operation {
                            query_string: format!("{} / {}", self.build(), rhs.build()),
                            bindings: [self.get_bindings(), rhs.get_bindings()].concat(),
                            errors: vec![],
                        }
                    }
                }
            )
        };
        FieldSetterNumericImpl(numeric_trait)
    }
}
