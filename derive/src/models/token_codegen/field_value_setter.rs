use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use proc_macros_helpers::get_crate_name;
use quote::{format_ident, quote, ToTokens};

use crate::{
    errors::ExtractorResult,
    models::{
        create_tokenstream_wrapper, derive_attributes::TableDeriveAttributes, FieldGenerics,
        FieldGenericsMeta, MyFieldReceiver, RelationType,
    },
};

pub struct FieldSetterNumericImpl(TokenStream);

impl ToTokens for FieldSetterNumericImpl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens);
    }
}

pub struct ArrayElementFieldSetterToken(TokenStream);

create_tokenstream_wrapper!(
    /// Generated Field wrapper type implementations for each fiekd around `Field` type
    /// Example value:
    /// ```rust,ignore
    /// struct Email(pub(super) Field);
    ///
    /// impl std::ops::Deref for Email {
    ///     type Target = #crate_name::Field;
    ///
    ///     fn deref(&self) -> &Self::Target {
    ///         &self.0
    ///     }
    /// }
    /// impl #crate_name::SetterAssignable<sql::Duration> for Email {}
    /// ```
=>
FieldSetterImplTokens);

use super::Codegen;

impl Codegen {
    pub fn create_field_setter_impl(&mut self) -> ExtractorResult<()> {
        let table_attributes = self.table_derive_attributes();
        let field_receiver = self.field_receiver();

        match field_receiver.to_relation_type() {
            // Relate fields are readonly and mainly for aliasing connections in select statements
            // To create a relation, we typically use a separate relation statement
            RelationType::Relate(_) => {}
            _ => {
                self.field_wrapper_type_custom_implementations
                    .push(self.get_field_value_setter_impl()?);
            }
        };
        Ok(())
    }

    fn get_field_value_setter_impl(&self) -> ExtractorResult<FieldSetterImplTokens> {
        let crate_name = get_crate_name(false);
        let field_receiver = self.field_receiver();
        let table_attributes = self.table_derive_attributes();

        let field_name_pascalized = field_receiver.field_name_pascalized(table_attributes);

        let numeric_trait = if field_receiver.is_numeric() {
            Self::numeric_setter_impl(field_receiver, table_attributes)
        } else {
            quote!()
        };

        let array_trait = if field_receiver.is_list() {
            Self::array_trait_impl(field_receiver, table_attributes)?
        } else {
            quote!()
        };

        let field_setter_impls = quote!(
            #[derive(Debug, Clone)]
            pub struct #field_name_pascalized(pub #crate_name::Field);

            impl ::std::convert::From<&str> for #field_name_pascalized {
                fn from(field_name: &str) -> Self {
                    Self(#crate_name::Field::new(field_name))
                }
            }

            impl ::std::convert::From<#crate_name::Field> for #field_name_pascalized {
                fn from(field_name: #crate_name::Field) -> Self {
                    Self(field_name)
                }
            }

            impl ::std::convert::From<&#field_name_pascalized> for #crate_name::ValueLike {
                fn from(value: &#field_name_pascalized) -> Self {
                    let field: #crate_name::Field = value.into();
                    field.into()
                }
            }

            impl ::std::convert::From<#field_name_pascalized> for #crate_name::ValueLike {
                fn from(value: #field_name_pascalized) -> Self {
                    let field: #crate_name::Field = value.into();
                    field.into()
                }
            }

            impl ::std::convert::From<&#field_name_pascalized> for #crate_name::Field {
                fn from(field_name:& #field_name_pascalized) -> Self {
                    field_name.0.clone()
                }
            }

            impl ::std::convert::From<#field_name_pascalized> for #crate_name::Field {
                fn from(field_name: #field_name_pascalized) -> Self {
                    field_name.0
                }
            }

            impl ::std::ops::Deref for #field_name_pascalized {
                type Target = #crate_name::Field;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }

            impl ::std::ops::DerefMut for #field_name_pascalized {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.0
                }
            }

            impl<T: #crate_name::serde::Serialize> ::std::convert::From<self::#field_name_pascalized> for #crate_name::SetterArg<T> {
            fn from(value: self::#field_name_pascalized) -> Self {
                    Self::Field(value.into())
                }
            }

            impl<T: #crate_name::serde::Serialize> ::std::convert::From<&self::#field_name_pascalized> for #crate_name::SetterArg<T> {
            fn from(value: &self::#field_name_pascalized) -> Self {
                    Self::Field(value.into())
                }
            }

            impl #field_impl_generics #crate_name::SetterAssignable<#field_type> for self::#field_name_pascalized  #field_where_clause {}

            impl #field_impl_generics #crate_name::Patchable<#field_type> for self::#field_name_pascalized  #field_where_clause {}

            #numeric_trait

            #array_trait
        );
        FieldSetterImplTokens(field_setter_impls).into()
    }

    fn array_trait_impl(
        field_receiver: &MyFieldReceiver,
        table_attributes: &TableDeriveAttributes,
    ) -> ExtractorResult<ArrayElementFieldSetterToken> {
        let crate_name = get_crate_name(false);
        let field_name_as_pascalized = field_receiver.field_name_pascalized(table_attributes);

        let (generics_meta, array_item_type) = match field_receiver.to_relation_type() {
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
            _ => {
                let inferred_type = match field_receiver.ty.get_array_inner_type() {
                    Some(ref ty) => {
                        let generics_meta = ty.get_generics_meta(table_attributes);
                        (Some(generics_meta), Some(quote!(#ty)))
                    }
                    None => {
                        let array_inner_field_ty = field_receiver
                            .field_type_db
                            .map(|db_ty| db_ty.get_array_item_type())
                            .flatten();

                        let array_inner_ty_db_concrete =  match array_inner_field_ty{
                        Some(ref db_array_item_ty) => (
                            None,
                            Some(db_array_item_ty.as_db_sql_value_tokenstream().to_token_stream()),
                        ),
                        None => {
                            return Err(syn::Error::new_spanned(
                                field_receiver.field_type_db,
                                "Could not infer array type. Explicitly specify the type e.g ty = array<string>",
                            ))
                        }
                    };
                        array_inner_ty_db_concrete
                    }
                };
                inferred_type
            }
        };

        let array_setter_impl = array_item_type.map_or(quote!(), |item_type| {
            generics_meta.map_or(
                quote!(
                    impl #crate_name::SetterArray<#item_type> for self::#field_name_as_pascalized {}
                ),
                |generics_meta| {
                    let FieldGenericsMeta {
                        field_impl_generics,
                        field_ty_generics,
                        field_where_clause,
                    } = generics_meta;

                    quote!(
                        impl #field_impl_generics #crate_name::SetterArray<#item_type> for
                        self::#field_name_as_pascalized #field_ty_generics #field_where_clause {}
                    )
                },
            )
        });

        Ok(ArrayElementFieldSetterToken(array_setter_impl))
    }

    fn numeric_setter_impl(
        field_receiver: &MyFieldReceiver,
        table_attributes: &TableDeriveAttributes,
    ) -> FieldSetterNumericImpl {
        let field_name_pascalized = field_receiver.field_name_pascalized(table_attributes);
        let field_type = field_receiver.ty;
        let FieldGenericsMeta {
            field_impl_generics,
            field_ty_generics,
            field_where_clause,
        } = field_type.get_generics_meta(table_attributes);

        let numeric_trait = {
            quote!(
                impl #field_impl_generics #crate_name::SetterNumeric<#field_type> for self::#field_name_pascalized #field_ty_generics
                #field_where_clause {}

                impl ::std::convert::From<self::#field_name_pascalized> for #crate_name::NumberLike {
                    fn from(val: self::#field_name_pascalized) -> Self {
                        val.0.into()
                    }
                }

                impl ::std::convert::From<&self::#field_name_pascalized> for #crate_name::NumberLike {
                    fn from(val: &self::#field_name_pascalized) -> Self {
                        val.clone().0.into()
                    }
                }

                impl<T: ::std::convert::Into<#crate_name::NumberLike>> ::std::ops::Add<T> for #field_name_pascalized {
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

                impl<T: ::std::convert::Into<#crate_name::NumberLike>> ::std::ops::Sub<T> for #field_name_pascalized {
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

                impl<T: ::std::convert::Into<#crate_name::NumberLike>> ::std::ops::Mul<T> for #field_name_pascalized {
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

                impl<T: ::std::convert::Into<#crate_name::NumberLike>> ::std::ops::Div<T> for #field_name_pascalized {
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

                impl<T: ::std::convert::Into<#crate_name::NumberLike>> ::std::ops::Add<T> for &#field_name_pascalized {
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

                impl<T: ::std::convert::Into<#crate_name::NumberLike>> ::std::ops::Sub<T> for &#field_name_pascalized {
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

                impl<T: ::std::convert::Into<#crate_name::NumberLike>> ::std::ops::Mul<T> for &#field_name_pascalized {
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

                impl<T: ::std::convert::Into<#crate_name::NumberLike>> ::std::ops::Div<T> for &#field_name_pascalized {
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
