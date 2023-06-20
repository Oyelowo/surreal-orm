// strength.equal_to(923u64)

///
#[macro_export]
macro_rules! object {
    ($struct_name:ident { $($key:ident: $value:expr),* $(,)? }) => {
        {

            $crate::validators::assert_fields!($struct_name : $( $key ),*);
            [
                $( $key .equal_to($value) ),*
            ]
        }
    };
}
