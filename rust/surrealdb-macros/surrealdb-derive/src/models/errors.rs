use convert_case::{Case, Casing};

pub(crate) fn validate_table_name<'a>(
    struct_name_ident: &proc_macro2::Ident,
    table_name: &'a Option<String>,
    relax_table_name: &Option<bool>,
) -> &'a String {
    let expected_table_name = struct_name_ident.to_string().to_case(Case::Snake);
    let table_name = table_name.as_ref().unwrap();
    if !relax_table_name.unwrap_or(false) && table_name != &expected_table_name {
        panic!(
            "table name must be in snake case of the current struct name. 
        Try: `{expected_table_name}`.
        
        If you don't want to follow this convention, use attribute `relax_table_name`. "
        );
    };

    table_name
}
