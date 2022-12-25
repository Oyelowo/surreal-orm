#[derive(Debug, Clone, Copy)]
pub(crate) struct SkipSerializing(pub(crate) bool);

impl From<SkipSerializing> for ::proc_macro2::TokenStream {
    fn from(value: SkipSerializing) -> Self {
        match value.0 {
            true => ::quote::quote!(),
            false => ::quote::quote!(pub),
        }
    }
}
