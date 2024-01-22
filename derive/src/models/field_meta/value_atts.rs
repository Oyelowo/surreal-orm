use syn::Expr;

enum Value {
    Expr(Expr),
    Path(syn::Path),
}
