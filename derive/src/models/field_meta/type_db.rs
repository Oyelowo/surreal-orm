/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2024 Oyelowo Oyedayo
 * Licensed under the MIT license
 */
#[derive(Debug, Clone, Default)]
pub struct DbFieldTypeMeta {
    pub(crate) db_field_type: TokenStream,
    pub(crate) static_assertion: TokenStream,
}

#[derive(Debug, Clone)]
pub struct FieldTypeWrapper(FieldType);

impl FieldTypeWrapper {
    pub fn into_inner(self) -> FieldType {
        self.0
    }
}

impl FieldTypeWrapper {
    pub fn generate_static_assertions(
        &self,
        rust_field_type: &Type,
        model_type: &DataType,
    ) -> TokenStream {
        let delifed_raw_type = replace_lifetimes_with_underscore(&mut rust_field_type.clone());
        let crate_name = get_crate_name(false);

        let static_assertion = match self.0 {
            FieldType::Any => {
                quote!(#crate_name::validators::assert_impl_one!(#delifed_raw_type: ::std::convert::Into<#crate_name::sql::Value>);)
            }
            FieldType::Null => {
                quote!(#crate_name::validators::assert_impl_one!(#delifed_raw_type: ::std::convert::Into<#crate_name::sql::Value>);)
            }
            FieldType::Uuid => {
                quote!(#crate_name::validators::assert_impl_one!(#delifed_raw_type: ::std::convert::Into<#crate_name::sql::Uuid>);)
            }
            FieldType::Bytes => {
                quote!(#crate_name::validators::assert_impl_one!(#delifed_raw_type: ::std::convert::Into<#crate_name::sql::Bytes>);)
            }
            FieldType::Union(_) => {
                // quote!(#crate_name::validators::assert_impl_one!(#raw_type: ::std::convert::Into<#crate_name::sql::Value>);)
                quote!()
            }
            FieldType::Option(_) => {
                // quote!(#crate_name::validators::assert_impl_one!(#raw_type: ::std::convert::Into<#crate_name::sql::Value>);)
                quote!()
            }
            FieldType::String => {
                quote!(#crate_name::validators::assert_impl_one!(#delifed_raw_type: ::std::convert::Into<::std::string::String>);)
            }
            FieldType::Int => {
                quote!(
                    #crate_name::validators::is_int::<#delifed_raw_type>();
                    // #crate_name::validators::assert_impl_one!(#raw_type: ::std::convert::Into<#crate_name::num_traits>);
                )
            }
            FieldType::Float => {
                quote!(
                    #crate_name::validators::is_float::<#delifed_raw_type>();
                    // #crate_name::validators::assert_impl_one!(#raw_type: ::std::convert::Into<#crate_name::num_traits>);
                )
                // quote!(#crate_name::validators::assert_impl_one!(#raw_type: ::std::convert::Into<#crate_name::sql::Number>);)
            }
            FieldType::Bool => {
                quote!(#crate_name::validators::assert_impl_one!(#delifed_raw_type: ::std::convert::Into<::std::primitive::bool>);)
            }
            FieldType::Array(_, _) => {
                quote!(
                    #crate_name::validators::assert_is_vec::<#delifed_raw_type>();
                )
            }
            FieldType::Set(_, _) => {
                quote!(
                    #crate_name::validators::assert_is_vec::<#delifed_raw_type>();
                )
            }
            FieldType::Datetime => {
                quote!(#crate_name::validators::assert_impl_one!(#delifed_raw_type: ::std::convert::Into<#crate_name::sql::Datetime>);)
            }
            FieldType::Decimal => {
                quote!(#crate_name::validators::assert_impl_one!(#delifed_raw_type: ::std::convert::Into<#crate_name::sql::Number>);)
            }
            FieldType::Duration => {
                quote!(#crate_name::validators::assert_impl_one!(#delifed_raw_type: ::std::convert::Into<#crate_name::sql::Duration>);)
            }
            FieldType::Number => {
                quote!(
                    #crate_name::validators::is_number::<#delifed_raw_type>();
                    // #crate_name::validators::assert_impl_one!(#raw_type: ::std::convert::Into<#crate_name::num_traits>);
                )
                // quote!(#crate_name::validators::assert_impl_one!(#raw_type: ::std::convert::Into<#crate_name::sql::Number>);)
            }
            FieldType::Object => {
                quote!(#crate_name::validators::assert_impl_one!(#delifed_raw_type: ::std::convert::Into<#crate_name::sql::Object>);)
            }
            FieldType::Record(_) => {
                if model_type.is_edge() {
                    quote!()
                } else {
                    quote!(#crate_name::validators::assert_impl_one!(#delifed_raw_type: ::std::convert::Into<Option<#crate_name::sql::Thing>>);)
                }
            }
            FieldType::Geometry(_) => {
                quote!(#crate_name::validators::assert_impl_one!(#delifed_raw_type: ::std::convert::Into<#crate_name::sql::Geometry>);)
            }
        };
        static_assertion
    }
}

impl Display for FieldTypeWrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for FieldTypeWrapper {
    type Target = FieldType;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromMeta for FieldTypeWrapper {
    fn from_string(value: &str) -> darling::Result<Self> {
        match value.parse::<FieldType>() {
            Ok(f) => Ok(Self(f)),
            Err(e) => Err(darling::Error::unknown_value(&e)),
        }
    }
}
