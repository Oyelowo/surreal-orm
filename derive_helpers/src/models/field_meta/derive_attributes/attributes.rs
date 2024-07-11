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
                let field_name = self.ident()?.to_string();
                return Err(syn::Error::new(
                    self.ident()?.span(),
                        format!(
                            "Missing required serde attribute on `{field_name}. set `#[serde(skip_serializing, default)]` on the field.
                                This is because this field is a readonly relate field",
                        ),
                    )
                    .into());
            }
        }
        Ok(())
    }
}
