use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::ToTokens;
use surreal_query_builder::FieldType;
use syn::Expr;

pub enum ExprOrPath {
    Expr(Expr),
    Path(syn::Path),
}

impl FromMeta for ExprOrPath {
    fn from_expr(expr: &syn::Expr) -> darling::Result<Self> {
        Ok(Self::Expr(expr.clone()))
    }

    fn from_meta(item: &syn::Meta) -> darling::Result<Self> {
        match item {
            syn::Meta::Path(path) => Ok(Self::Path(path.clone())),
            syn::Meta::NameValue(name_value) => {
                let lit = &name_value.lit;
                match lit {
                    syn::Lit::Str(str_lit) => {
                        let value_str = str_lit.value();
                        Ok(Self::Path(syn::parse_str(&value_str)?))
                    }
                    _ => Err(darling::Error::custom("Invalid value").with_span(lit)),
                }
            }
            _ => Err(darling::Error::custom("Invalid value").with_span(item)),
        }
    }
}

impl ToTokens for ExprOrPath {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            ExprOrPath::Expr(expr) => expr.to_tokens(tokens),
            ExprOrPath::Path(path) => path.to_tokens(tokens),
        }
    }
}

pub struct AttributeValue(ExprOrPath);
pub struct AttributeAssert(ExprOrPath);
pub struct AttributeItemAssert(ExprOrPath);
pub struct AttributeAs(ExprOrPath);
pub struct AttributeDefine(ExprOrPath);

macro_rules! impl_from_expr_or_path {
    ($ty:ty) => {
        impl FromMeta for $ty {
            fn from_expr(expr: &syn::Expr) -> darling::Result<Self> {
                Ok(Self(ExprOrPath::Expr(expr.clone())))
            }

            fn from_meta(item: &syn::Meta) -> darling::Result<Self> {
                Ok(Self(ExprOrPath::from_meta(item)?))
            }
        }

        impl ToTokens for $ty {
            fn to_tokens(&self, tokens: &mut TokenStream) {
                self.0.to_tokens(tokens)
            }
        }
    };
}

impl_from_expr_or_path!(AttributeValue);
impl_from_expr_or_path!(AttributeAssert);
impl_from_expr_or_path!(AttributeItemAssert);
impl_from_expr_or_path!(AttributeAs);
impl_from_expr_or_path!(AttributeDefine);

impl AttributeValue {
    pub fn get_static_assrtion(&self, db_field_type: FieldType) -> StaticAssertionToken {
        let value_expr = &self;
        let convertible_values_to_db_type = match db_field_type {
            FieldType::Bytes => quote!(#crate_name::sql::Bytes::from(#value_expr)),
            FieldType::Null => quote!(#crate_name::sql::Value::Null),
            // FieldType::Union(_) => quote!(#crate_name::sql::Value::from(#value_expr)),
            FieldType::Union(_) => quote!(),
            // FieldType::Option(_) => quote!(#crate_name::sql::Value::from(#value_expr)),
            FieldType::Option(_) => quote!(),
            FieldType::Uuid => quote!(#crate_name::sql::Uuid::from(#value_expr)),
            FieldType::Duration => quote!(#crate_name::sql::Duration::from(#value_expr)),
            FieldType::String => quote!(#crate_name::sql::String::from(#value_expr)),
            FieldType::Int => quote!(#crate_name::sql::Number::from(#value_expr)),
            FieldType::Float => quote!(#crate_name::sql::Number::from(#value_expr)),
            FieldType::Bool => quote!(#crate_name::sql::Bool::from(#value_expr)),
            FieldType::Array(_, _) => quote!(),
            FieldType::Set(_, _) => quote!(),
            // FieldType::Array => quote!(#crate_name::sql::Value::from(#value)),
            FieldType::Datetime => quote!(#crate_name::sql::Datetime::from(#value_expr)),
            FieldType::Decimal => quote!(#crate_name::sql::Number::from(#value_expr)),
            FieldType::Number => quote!(#crate_name::sql::Number::from(#value_expr)),
            FieldType::Object => quote!(),
            // FieldType::Object => quote!(#crate_name::sql::Value::from(#value_expr)),
            FieldType::Record(_) => quote!(#crate_name::sql::Thing::from(#value_expr)),
            FieldType::Geometry(_) => quote!(#crate_name::sql::Geometry::from(#value_expr)),
            FieldType::Any => quote!(#crate_name::sql::Value::from(#value_expr)),
        };

        quote!(let _ = #convertible_values_to_db_type;)
    }
}
