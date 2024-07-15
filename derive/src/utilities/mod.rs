// pub mod pick;
//
// use syn;
// use quote::quote;
//
// pub fn generate_pick_macro_fn_tokens(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
//     let output = match syn::parse2::<PickedMeta>(input.into()) {
//         Ok(out) => out,
//         Err(err) => return err.to_compile_error().into()
//     };
//
//     quote!(#output).into()
// }
