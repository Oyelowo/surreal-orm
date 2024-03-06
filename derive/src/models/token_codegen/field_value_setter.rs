/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2024 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use proc_macros_helpers::get_crate_name;
use quote::{quote, ToTokens};
use surreal_query_builder::field_type;

use crate::models::*;

create_tokenstream_wrapper!(=>FieldSetterNumericImpl);
create_tokenstream_wrapper!(=>ArrayElementFieldSetterToken);
create_tokenstream_wrapper!(=>FieldTypeSetterPatcherImpls);

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

impl<'a> Codegen<'a> {
    pub fn create_field_setter_impl(&mut self) -> ExtractorResult<()> {
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
        let field_type = field_receiver.ty();
        let casing = &table_attributes.casing()?;
        let binding = field_type.get_generics_from_current_struct(table_attributes);
        let (field_impl_generics, _field_ty_generics, field_where_clause) =
            binding.split_for_impl();
        let field_name_pascalized = field_receiver.field_name_pascalized(casing)?;

        let field_type_setterPatcherImpls: FieldTypeSetterPatcherImpls = if field_receiver
            .db_field_name(casing)?
            .is_id()
        {
            quote!().into()
        } else {
            let field_type =
                field_type.replace_self_with_current_struct_concrete_type(table_attributes);
            quote!(
                impl #field_impl_generics #crate_name::SetterAssignable<#field_type> for self::#field_name_pascalized  #field_where_clause {}

                impl #field_impl_generics #crate_name::Patchable<#field_type> for self::#field_name_pascalized  #field_where_clause {}
            ).into()
        };

        let numeric_trait = if field_receiver.is_numeric() {
            Self::numeric_setter_impl(field_receiver, table_attributes)?
        } else {
            quote!().into()
        };

        let array_trait = if field_receiver.is_list() {
            Self::array_trait_impl(field_receiver, table_attributes)?
        } else {
            quote!().into()
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

            #field_type_setterPatcherImpls

            #numeric_trait

            #array_trait
        );
        Ok(FieldSetterImplTokens(field_setter_impls))
    }

    fn array_trait_impl(
        field_receiver: &MyFieldReceiver,
        model_attributes: &ModelAttributes,
    ) -> ExtractorResult<ArrayElementFieldSetterToken> {
        let crate_name = get_crate_name(false);
        let field_name_as_pascalized =
            field_receiver.field_name_pascalized(&model_attributes.casing()?)?;

        let (generics, array_item_type) = match field_receiver.to_relation_type() {
            RelationType::LinkMany(foreign_node) => {
                let generics_meta = foreign_node.get_generics_from_current_struct(model_attributes);
                (
                    Some(generics_meta),
                    Some(quote!(<#foreign_node as #crate_name::Model>::Id)),
                )
            }
            RelationType::NestArray(foreign_object) => {
                let generics_meta =
                    foreign_object.get_generics_from_current_struct(model_attributes);
                (Some(generics_meta), Some(quote!(#foreign_object)))
            }
            _ => {
                let inferred_type = match field_receiver.ty().get_array_inner_type() {
                    Some(ref ty) => {
                        let generics_meta = ty.get_generics_from_current_struct(model_attributes);
                        (Some(generics_meta), Some(quote!(#ty)))
                    }
                    None => {
                        let array_inner_field_ty = field_receiver
                            .field_type_db
                            .as_ref()
                            .and_then(|db_ty| db_ty.get_array_item_type());

                        let array_inner_ty_db_concrete =  match array_inner_field_ty{
                        Some(ref db_array_item_ty) => (
                            None,
                            Some(db_array_item_ty.as_db_sql_value_tokenstream().to_token_stream()),
                        ),
                        None => {
                            return Err(syn::Error::new_spanned(
                                field_receiver.field_type_db.as_ref(),
                                "Could not infer array type. Explicitly specify the type e.g ty = array<string>",
                            ).into())
                        }
                    };
                        array_inner_ty_db_concrete
                    }
                };
                inferred_type
            }
        };

        let array_setter_impl = array_item_type.map_or(quote!(), |item_type| {
            generics.map_or(
                quote!(
                    impl #crate_name::SetterArray<#item_type> for self::#field_name_as_pascalized {}
                ),
                |this| {
                    let (field_impl_generics, field_ty_generics, field_where_clause) =
                        this.split_for_impl();

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
        model_attributes: &ModelAttributes,
    ) -> ExtractorResult<FieldSetterNumericImpl> {
        let crate_name = get_crate_name(false);
        let field_name_pascalized =
            field_receiver.field_name_pascalized(&model_attributes.casing()?)?;
        let field_type = field_receiver.ty();
        let binding = field_type.get_generics_from_current_struct(model_attributes);
        let (field_impl_generics, field_ty_generics, field_where_clause) = binding.split_for_impl();

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
        Ok(FieldSetterNumericImpl(numeric_trait))
    }
}
