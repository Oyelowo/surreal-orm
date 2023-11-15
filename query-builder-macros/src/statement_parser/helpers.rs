use convert_case::{Case, Casing};
use proc_macros_helpers::get_crate_name;
use quote::format_ident;
use syn::Ident;

pub fn generate_variable_name() -> Ident {
    let sanitized_uuid = uuid::Uuid::new_v4().simple();
    let crate_name = get_crate_name(false);
    let name = format!("_{crate_name}__private__internal_variable_prefix__{sanitized_uuid}")
        .to_case(Case::Camel);
    let mut param = format_ident!("{name}");

    // param.truncate(15);

    param
}
