/// object! macro is a syntactic sugar for array of setters (e.g`[age.equal_to(4), name.equal_to(param)]`) for setting values in a `create` or `update` but provides
/// much more flexibility than using the basic struct to initialize values
/// This also allows using `parameter` or `field` as value.
#[macro_export]
macro_rules! object {
    ($struct_name:ident { $($key:ident: $value:expr),* $(,)? }) => {
        {
            $crate::validators::assert_same_length_arrays($struct_name:: __get_serializable_field_names(), [ $( stringify!($key) ),*]);
            $crate::check_unique_idents!($($key), *);
            $crate::validators::assert_fields!($struct_name : $( $key ),*);
            let schema = &$struct_name::schema();

            [
                $( schema.$key .equal_to($value) ),*
            ]
        }
    };
}
