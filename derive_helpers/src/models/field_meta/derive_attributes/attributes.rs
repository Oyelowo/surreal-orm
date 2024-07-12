use super::ExtractorResult;
use crate::models::MyFieldReceiver;
use syn::{spanned::Spanned, Attribute};

impl MyFieldReceiver {
    pub fn attributes(&self) -> &Vec<Attribute> {
        &self.attrs
    }

    pub fn validate_attributes(&self) -> ExtractorResult<()> {
        if self.relate.is_some() {
            let has_required_attributes = self.skip_serializing && self.default;
            
            if !has_required_attributes {
                let field_name = self.ident()?.to_string();
                return Err(syn::Error::new(
                    self.ident()?.span(),
                        format!(
                            "Missing required 'skip_serializing' or 'default' serde attribute(s) on `{field_name}`. set `#[serde(skip_serializing, default)]` on the field. \
                                \nThis is because this field is a readonly derived relational field and we don't want to store it in the database"
                        ),
                    )
                    .into());
            }
        }
        Ok(())
    }
}
