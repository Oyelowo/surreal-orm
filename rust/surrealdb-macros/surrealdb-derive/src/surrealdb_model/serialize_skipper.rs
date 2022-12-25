#[derive(Debug, Clone, Copy)]
pub(crate) enum SkipSerializing {
    Yes,
    No,
}

impl From<bool> for SkipSerializing {
    fn from(value: bool) -> Self {
        match value {
            true => SkipSerializing::Yes,
            false => SkipSerializing::No,
        }
    }
}

impl From<SkipSerializing> for bool {
    fn from(value: SkipSerializing) -> Self {
        match value {
            SkipSerializing::Yes => true,
            SkipSerializing::No => false,
        }
    }
}

impl From<SkipSerializing> for ::proc_macro2::TokenStream {
    fn from(value: SkipSerializing) -> Self {
        match value {
            SkipSerializing::Yes => ::quote::quote!(),
            SkipSerializing::No => ::quote::quote!(pub),
        }
    }
}
