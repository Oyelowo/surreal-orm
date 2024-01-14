use darling::FromDeriveInput;
use proc_macros_helpers::get_crate_name;
use quote::{quote, ToTokens};
use syn::parse_macro_input;

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(surreal_orm, serde), forward_attrs(allow, doc, cfg))]
pub struct TableMigrationSchemaDeriveAttributes {
    pub(crate) ident: syn::Ident,
    // pub(crate) attrs: Vec<syn::Attribute>,
    pub(crate) generics: syn::Generics,
    // Receives the body of the struct or enum. We don't care about
    // struct fields because we previously told darling we only accept structs.
    // pub data: Data<util::Ignored, self::MyFieldReceiver>,
}

impl ToTokens for TableMigrationSchemaDeriveAttributes {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let crate_name = get_crate_name(false);
        let TableMigrationSchemaDeriveAttributes {
            ident: struct_name_ident,
            generics,
        } = self;

        tokens.extend(quote! {
            impl #generics #crate_name::TableResources for #struct_name_ident #generics {}
        });
        // tokens.extend(quote! {
        //     // impl<In: #crate_name::Node, Out: #crate_name::Node> #crate_name::TableResources  for #struct_name_ident<In, Out> {}
        // });
    }
}
// fn ferf(){
//                     impl<In: #crate_name::Node, Out: #crate_name::Node> #crate_name::TableResources  for #struct_name_ident<In, Out> {}
//
//
// }
//

pub fn generate_table_resources_trait(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input);
    let output = match TableMigrationSchemaDeriveAttributes::from_derive_input(&input) {
        Ok(out) => out,
        Err(err) => return proc_macro::TokenStream::from(err.write_errors()),
    };
    quote!(#output).into()
}
