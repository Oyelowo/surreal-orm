use darling::FromField;
use proc_macro2::Span;
use quote::{IdentFragment, ToTokens};
use syn::{spanned::Spanned, Attribute, Ident, Type};

use crate::models::{ModelAttributes, MyFieldReceiver};

use super::ExtractorResult;

#[derive(Clone, Debug, FromField)]
#[darling(attributes(serde))]
pub struct SerdeAttrs {
    /// Get the ident of the field. For fields in tuple or newtype structs or
    /// enum bodies, this can be `None`.
    ident: Option<Ident>,
    ty: Type,
    attrs: Vec<syn::Attribute>,
    // Serde attributes
    #[darling(default)]
    pub(crate) skip_serializing: bool,

    #[darling(default)]
    pub(crate) default: bool,
}

impl MyFieldReceiver {
    pub fn attributes(&self) -> &Vec<Attribute> {
        &self.attrs
    }

    pub fn validate_attributes(&self, model_attributes: &ModelAttributes) -> ExtractorResult<()> {
        model_attributes.fields()
        let attributes = self.attributes();
// panic!("xxxxxxxjwj ident:{} --- attributes {}endxxx", self.ident.to_token_stream().to_string(), attributes.iter().map(|a|a.to_token_stream().to_string()).collect::<Vec<String>>().join("##"));
        let relation_type = self.to_relation_type(model_attributes);
        // if self.relate.is_some() {}
        let mut serde_present = false;
        if relation_type.is_relate_graph() {
            for attribute in attributes {
                if attribute.path().is_ident("serde") {
                    serde_present = true;
                    // let valid = serde_present && self.default && self.skip_serializing;
                    let has_required_attrs = self.default && self.skip_serializing;
                    if !has_required_attrs {
                        return Err(syn::Error::new(
                    self.ident()?.span(),
                        format!(
                            "serde attribute not set with the required attributes on field - `{}` is a readonly \
                                relate field, you must set `#[serde(skip_serializing, default)]` on the field.",
                            self.ident.as_ref().unwrap().to_string()
                        ),
                    )
                    .into());
                    }
                }
            }

            if !serde_present {
                return Err(syn::Error::new(
                    self.ident()?.span(),
                        format!(
                            "Field `{}` is a readonly relate field, you must set `#[serde(skip_serializing, default)]` on the field.",
                            self.ident.as_ref().unwrap().to_string()
                        ),
                    )
                    .into());

                // return Err(darling::Error::custom(
                //     "Field is a relate field, it must have a serde attribute",
                // )
                // .with_span(attribute.path()));
            }
            // let _ = attribute;
        }
        Ok(())
    }
}
// panic!("xxxxxxxjwj ident:{} --- attributes {}endxxx", self.ident.to_token_stream().to_string(), attributes.iter().map(|a|a.to_token_stream().to_string()).collect::<Vec<String>>().join("##"));
