pub trait Edge {
    const EDGE_RELATION: &'static str;
    fn to(&self) -> ::proc_macro2::TokenStream;
    fn from(&self) -> ::proc_macro2::TokenStream;
    fn km(&self) -> String;
}
