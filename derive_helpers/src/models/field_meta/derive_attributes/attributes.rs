use super::ExtractorResult;
use crate::models::MyFieldReceiver;
use syn::{spanned::Spanned, Attribute};

impl MyFieldReceiver {
    pub fn attributes(&self) -> &Vec<Attribute> {
        &self.attrs
    }

    pub fn validate_attributes(&self) -> ExtractorResult<()> {
        if self.relate.is_some() {
            if !self.skip_serializing {
                let field_name = self.ident()?.to_string();
                return Err(syn::Error::new(
                    self.ident()?.span(),
                        format!(
                            "Missing required serde attribute on `{field_name}. set `#[serde(skip_serializing)]` on the field. \
                                \nThis is because this field is a readonly derived relational field and we don't want to store it in the data"
                        ),
                    )
                    .into());
            }
        }
        Ok(())
    }
}
