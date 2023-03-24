use syn::{parse_quote, Expr, Lit, LitStr};

fn main() {
    let input = "\"54+3 * gama()\"";
    let expr: Expr = syn::parse_str(input).unwrap();
    let lit_str = match &expr {
        Expr::Lit(expr_lit) => match &expr_lit.lit {
            Lit::Str(lit_str) => lit_str,
            _ => panic!("Expected string literal"),
        },
        _ => panic!("Expected expression literal"),
    };
    println!("{:?}", lit_str.value());
}
//
// use proc_macro2::TokenStream;
// use quote::{quote, ToTokens};
// use syn::{
//     parse::{Parse, ParseStream},
//     parse_str, ItemFn, Result,
// };
//
// // Define a simple Rust function to parse.
// // Note that it needs to be a valid Rust function definition.
// const CODE: &str = "fn good_fn(x: i32) -> i32 { x + 1 }";
//
// // Define a struct to hold the parsed function.
// struct MyFn {
//     fn_def: ItemFn,
// }
//
// // Implement the `Parse` trait for `MyFn`,
// // which will allow us to use the `parse_str` function to parse the function.
// impl Parse for MyFn {
//     fn parse(input: ParseStream) -> Result<Self> {
//         Ok(Self {
//             fn_def: input.parse()?,
//         })
//     }
// }
//
// // Implement the `ToTokens` trait for `MyFn`,
// // which will allow us to convert the struct into a `TokenStream`.
// impl ToTokens for MyFn {
//     fn to_tokens(&self, tokens: &mut TokenStream) {
//         self.fn_def.to_tokens(tokens);
//     }
// }
//
// fn main() {
//     // Parse the function definition into an `ItemFn` AST node.
//     let my_fn = parse_str::<MyFn>(CODE).unwrap();
//
//     // Convert the function definition AST node into a `TokenStream`.
//     let tokens = quote! {
//         let a = #my_fn;
//     };
//
//     // Print the resulting `TokenStream`.
//     println!("{}", tokens);
// }
