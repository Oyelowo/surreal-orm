use super::ExtractorResult;
use crate::models::MyFieldReceiver;
use syn::{spanned::Spanned, Attribute};

impl MyFieldReceiver {
    pub fn attributes(&self) -> &Vec<Attribute> {
        &self.attrs
    }

    pub fn validate_attributes(&self) -> ExtractorResult<()> {
        if self.relate.is_some() {
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
        Ok(())
    }
}
