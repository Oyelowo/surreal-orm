// strength.equal_to(923u64)

///
#[macro_export]
macro_rules! object {
    ($struct_name:ident { $($key:ident: $value:expr),* $(,)? }) => {
        {

            // let mut output = vec![];
            // let fields = $struct_name::get_serializable_fields().iter().map(|field| field.to_string()).collect::<std::collections::HashSet<String>>();

            $(
               const _: () = {
                        match (stringify!($key), &$struct_name::ALLOWED_FIELDS) {
                        // match (stringify!($key), &ALLOWED_FIELDS) {
                            (field, fields) if fields.contains(&field) => {},
                            _ => [()][1], // Cause compile-time error
                        }
                    };
            )*

            $crate::validators::assert_fields!($struct_name : $( $key ),*);
            [
                $( $key .equal_to($value) ),*
            ]
        }
    };
}
