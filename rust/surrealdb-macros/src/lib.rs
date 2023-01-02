pub trait Edge {
    const edge_relation: &'static str;
    fn to(&self) -> ::proc_macro2::TokenStream;
    fn from(&self) -> ::proc_macro2::TokenStream;
    fn km(&self) -> String;
}
