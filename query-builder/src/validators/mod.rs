pub use num_traits::{Float, Num, PrimInt as Int};
// pub use static_assertions::assert_fields;
pub use static_assertions::assert_impl_all;
pub use static_assertions::assert_impl_any;
// pub use static_assertions::assert_impl_one;
pub use static_assertions::assert_trait_sub_all;
pub use static_assertions::assert_trait_super_all;
// pub use static_assertions::assert_type_eq_all;
use std::any::TypeId;
use std::collections::HashSet;

// macro_rules! assert_fields {
//     ($ty:ty: $($field:ident),+) => {
//         {
//             let _ = |_: $ty| {};
//             $(
//                 let _ = |val: $ty| { let _ = val.$field; };
//             )+
//         }
//     };
// }

/// Checks that struct has all the fields specified
#[macro_export]
macro_rules! assert_fields {
    ($ty:ty: $($field:ident),+) => {
        {
            let _ = |_: $ty| {};
                let _ = |val: $ty| {
          $(
              let _ = val.$field;
           )+
        };
        }
    };
}

#[macro_export]
macro_rules! assert_type_eq_all {
    ($a:ty, $b:ty) => {
        let _a: $a = unimplemented!();
        let _b: $b = _a;
    };
}

pub use assert_type_eq_all;

#[macro_export]
macro_rules! assert_impl_one {
    ($ty:ty; $trait:path) => {{
        struct AssertTraitImpl<T: $trait>(PhantomData<T>);
        AssertTraitImpl::<$ty>(std::marker::PhantomData);
    }};
}

pub use assert_impl_one;

/// Validate that type is a number at compile time
///
/// # Example
/// ```
/// # use surreal_query_builder::validators::is_number;
/// is_number::<i8>();
/// is_number::<i16>();
/// is_number::<i32>();
/// is_number::<i64>();
/// is_number::<i128>();
/// is_number::<isize>();
/// is_number::<u8>();
/// is_number::<u16>();
/// is_number::<u32>();
/// is_number::<u64>();
/// is_number::<u128>();
/// is_number::<usize>();
/// is_number::<f32>();
/// is_number::<f64>();
/// ```
pub fn is_number<T: Num>() {}

/// Validate that type is a primitive integer at compile time
///
/// # Example
/// ```
/// # use surreal_query_builder::validators::is_int;
/// is_int::<i8>();
/// is_int::<i16>();
/// is_int::<i32>();
/// is_int::<i64>();
/// is_int::<i128>();
/// is_int::<isize>();
/// ```
pub fn is_int<T: Int>() {}

/// Validate that type is a primitive float at compile time
///
/// # Example
/// ```
/// # use surreal_query_builder::validators::is_float;
///
/// is_float::<f32>();
/// is_float::<f64>();
/// ```
pub fn is_float<T: Float>() {}

/// Validate that type is a vector at compile time
///
/// # Example
/// ```
/// # use surreal_query_builder::validators::assert_is_vec;
/// assert_is_vec::<Vec<i8>>();
/// assert_is_vec::<Vec<String>>();
/// assert_is_vec::<Vec<i32>>();
/// assert_is_vec::<Vec<i64>>();
/// assert_is_vec::<Vec<i128>>();
/// assert_is_vec::<Vec<isize>>();
/// assert_is_vec::<Vec<u8>>();
/// assert_is_vec::<Vec<u16>>();
/// assert_is_vec::<Vec<u32>>();
/// assert_is_vec::<Vec<u64>>();
/// assert_is_vec::<Vec<u128>>();
/// assert_is_vec::<Vec<usize>>();
/// assert_is_vec::<Vec<f32>>();
/// assert_is_vec::<Vec<f64>>();
/// ```
pub fn assert_is_vec<T: IntoIterator>() {
    let _ = <T as IntoIterator>::into_iter;
}

/// Validate that type is a hashset at compile time
pub fn assert_is_hashset<T: 'static>(_: &T) -> bool {
    // Here, you can change i32 to any other type based on what you are comparing with
    TypeId::of::<T>() == TypeId::of::<HashSet<i32>>()
}

/// This function can only be called with two arrays of the same length.
pub fn assert_same_length_arrays<T, const N: usize>(_array1: [T; N], _array2: [T; N]) {
    println!("Both arrays have the same length of {}", N);
}

/// check if a type is an Option
pub trait IsOption {}

impl<T> IsOption for Option<T> {}

/// Validate that type is an Option at compile time
pub fn assert_option<T: IsOption>() {
    // This function doesn't need to do anything; it's just here to enforce the type constraint.
}

/// Check if a type is an array
pub trait IsArray {}

impl<T> IsArray for Vec<T> {}

/// Validate that type is an Vec at compile time
pub fn assert_vec<T: IsArray>() {
    // This function doesn't need to do anything; it's just here to enforce the type constraint.
}
/// Checks that all idents are unique.
#[macro_export]
macro_rules! check_unique_idents {
    // Base case: single element, always unique
    ($_ident:ident) => {};

    // Recursive case: check head against the rest and recurse
    ($head:ident, $($tail:ident),+ $(,)?) => {
        // Generate a unique constant for $head
        $crate::internal_tools::paste! {
        #[allow(non_upper_case_globals)]
            const [<UNIQUE_ $head>]: () = ();
        }

        // Recurse with the tail
        $crate::check_unique_idents!($($tail),*);
    };
}
