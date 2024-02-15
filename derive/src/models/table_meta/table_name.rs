use convert_case::{Case, Casing};
use quote::format_ident;

use crate::models::create_ident_wrapper;

use super::derive_attributes::StructIdent;

create_ident_wrapper!(TableName);

impl TableName {
    pub(crate) fn validate_and_return(
        &self,
        struct_name_ident: &StructIdent,
        relax_table_name: &Option<bool>,
    ) -> ExtractorResult<&Self> {
        let expected_table_name = struct_name_ident.to_string().to_case(Case::Snake);
        if !relax_table_name.unwrap_or(false) && self.to_string() != expected_table_name {
            return Err(syn::Error::new(
                table_name.span(),
                "table name must be in snake case of the current struct name. 
        Try: `{expected_table_name}`.
        
        If you don't want to follow this convention, use attribute `relax_table_name`. ",
            )
            .into());
        };

        Ok(self)
    }
}
