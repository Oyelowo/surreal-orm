#[macro_export]
macro_rules! pub_struct {
    ($name:ident {$($field:ident: $t:ty,)*}) => {
        #[derive(Debug, Clone)] // ewww
        pub struct $name {
            $(pub $field: $t),*
        }
    }
}
