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
