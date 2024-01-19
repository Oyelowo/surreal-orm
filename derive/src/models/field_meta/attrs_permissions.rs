#[derive(Debug, Clone)]
pub enum Permissions {
    Full,
    None,
    FnName(LitStr),
}

impl Permissions {
    pub fn get_token_stream(&self) -> TokenStream {
        match self {
            Self::Full => {
                quote!(.permissions_full())
            }
            Self::None => {
                quote!(.permissions_none())
            }
            Self::FnName(permissions) => {
                let permissions =
                    parse_lit_to_tokenstream(permissions).expect("Unable to parse permissions");
                quote!(.permissions(#permissions.to_raw()))
            }
        }
    }
}

impl FromMeta for Permissions {
    fn from_value(value: &Lit) -> darling::Result<Self> {
        match value {
            Lit::Str(str_lit) => {
                let value_str = str_lit.value();

                if value_str.to_lowercase() == "none" {
                    Ok(Self::None)
                } else if value_str.to_lowercase() == "full" {
                    Ok(Self::Full)
                } else {
                    Ok(Self::FnName(LitStr::new(&value_str, str_lit.span())))
                    // Ok(Self::FnName(str_lit.to_owned()))
                }
                // Ok(Self::FnName(LitStr::new(&value_str, str_lit.span())))
            }
            _ => Err(darling::Error::unexpected_lit_type(value)),
        }
    }
}

#[derive(Debug, Clone)]
pub enum PermissionsFn {
    Full,
    None,
    FnPath(Path),
}

impl PermissionsFn {
    pub fn get_token_stream(&self) -> TokenStream {
        match self {
            Self::Full => {
                quote!(.permissions_full())
            }
            Self::None => {
                quote!(.permissions_none())
            }
            Self::FnPath(permissions_fn) => {
                quote!(.permissions(#permissions_fn().to_raw()))
            }
        }
    }
}

impl FromMeta for PermissionsFn {
    fn from_string(value: &str) -> darling::Result<Self> {
        match value.to_lowercase().as_str() {
            "none" => Ok(Self::None),
            "full" => Ok(Self::Full),
            _ => Err(darling::Error::unexpected_type(value)),
        }
    }

    fn from_value(value: &syn::Lit) -> darling::Result<Self> {
        match value {
            Lit::Str(str) => Ok(Self::FnPath(syn::parse_str::<Path>(&str.value())?)),
            _ => Err(darling::Error::unexpected_lit_type(value)),
        }
    }
}
