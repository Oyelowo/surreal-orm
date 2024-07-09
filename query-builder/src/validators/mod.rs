use crate::LinkMany;
use crate::LinkOne;
use crate::LinkSelf;
use crate::Node;
use crate::Object;
// pub use crate::assert_fields;
pub use num_traits::{Float, Num, PrimInt as Int};
// pub use static_assertions::assert_fields;
pub use static_assertions::assert_impl_all;
pub use static_assertions::assert_impl_any;
// pub use static_assertions::assert_impl_one;
pub use static_assertions::assert_trait_sub_all;
pub use static_assertions::assert_trait_super_all;
// pub use static_assertions::assert_type_eq_all;
use std::collections::BTreeSet;
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
pub use assert_fields;

#[macro_export]
/// Checks that two types are equal
macro_rules! assert_type_eq_all {
    ($a:ty, $b:ty) => {
        let _a: $a = todo!();
        #[allow(unreachable_code)]
        let _b: $b = _a;
    };
}

pub use assert_type_eq_all;

#[macro_export]
/// Checks that a type implements a trait
macro_rules! assert_impl_one {
    ($ty:ty: $trait:path) => {{
        struct AssertTraitImpl<T: $trait>(::std::marker::PhantomData<T>);
        AssertTraitImpl::<$ty>(::std::marker::PhantomData);
    }};
}

pub use assert_impl_one;

/// Validate that type is a number at compile time
///
/// # Example
/// ```
/// # use surreal_query_builder::validators::assert_type_is_number;
/// assert_type_is_number::<i8>();
/// assert_type_is_number::<i16>();
/// assert_type_is_number::<i32>();
/// assert_type_is_number::<i64>();
/// assert_type_is_number::<i128>();
/// assert_type_is_number::<isize>();
/// assert_type_is_number::<u8>();
/// assert_type_is_number::<u16>();
/// assert_type_is_number::<u32>();
/// assert_type_is_number::<u64>();
/// assert_type_is_number::<u128>();
/// assert_type_is_number::<usize>();
/// assert_type_is_number::<f32>();
/// assert_type_is_number::<f64>();
/// ```
pub fn assert_type_is_number<T: Num>() {}

/// Validate that value is a number at compile time
/// # Example
/// ```
/// # use surreal_query_builder as surreal_orm;
/// use surreal_orm::validators::assert_value_is_number;
/// assert_value_is_number(1);
/// assert_value_is_number(1u8);
/// assert_value_is_number(1u16);
/// assert_value_is_number(1u32);
/// assert_value_is_number(1u64);
/// assert_value_is_number(1u128);
/// assert_value_is_number(1usize);
/// assert_value_is_number(1.0);
/// ```
pub fn assert_value_is_number<T: Num>(_value: T) {}

/// Validate that type is a primitive integer at compile time
///
/// # Example
/// ```
/// # use surreal_query_builder as surreal_orm;
/// # use surreal_orm::validators::assert_type_is_int;
/// assert_type_is_int::<i8>();
/// assert_type_is_int::<i16>();
/// assert_type_is_int::<i32>();
/// assert_type_is_int::<i64>();
/// assert_type_is_int::<i128>();
/// assert_type_is_int::<isize>();
/// ```
pub fn assert_type_is_int<T: Int>() {}

/// Validate that value is an integer at compile time
///
/// # Example
/// ```
/// # use surreal_query_builder as surreal_orm;
/// # use surreal_orm::validators::assert_value_is_int;
/// assert_value_is_int(1);
/// assert_value_is_int(1u8);
/// assert_value_is_int(1u16);
/// assert_value_is_int(1u32);
/// ```
pub fn assert_value_is_int<T: Int>(_value: T) {}

/// Validate that type is a primitive float at compile time
///
/// # Example
/// ```
/// # use surreal_query_builder as surreal_orm;
/// # use surreal_orm::validators::assert_type_is_float;
///
/// assert_type_is_float::<f32>();
/// assert_type_is_float::<f64>();
/// ```
pub fn assert_type_is_float<T: Float>() {}

// Reusing float static check for now
/// Validate that type is a primitive decimal at compile time
pub use assert_type_is_float as assert_type_is_decimal;

/// Validate that value is a float at compile time
/// # Example
/// ```
/// # use surreal_query_builder as surreal_orm;
/// use surreal_orm::validators::assert_value_is_float;
///
/// assert_value_is_float(1.0);
/// assert_value_is_float(1.0f32);
/// assert_value_is_float(1.0f64);
/// ```
pub fn assert_value_is_float<T: Float>(_value: T) {}

/// Validate that type is iterable at compile time
pub fn assert_type_is_iterable<T: IntoIterator>() {
    let _ = <T as IntoIterator>::into_iter;
}

/// Validate that type is a string at compile time
pub trait IsString {}

// impl IsString for String {}
// impl IsString for &String {}
// impl IsString for &str {}
impl<T> IsString for T where T: Into<String> {}

/// Validate that type is a string at compile time
pub fn assert_type_is_string<T: IsString>() {
    // This function doesn't need to do anything; it's just here to enforce the type constraint.
}

/// Validate that value is convertible to a string at compile time
pub fn assert_type_is_stringable<T: ToString>() {
    // This function doesn't need to do anything; it's just here to enforce the type constraint.
}

/// Validate that value is a string at compile time
pub fn assert_value_is_string<T: IsString>(_value: T) {}

/// This function can only be called with two arrays of the same length.
pub fn assert_same_length_arrays<T, const N: usize>(_array1: [T; N], _array2: [T; N]) {
    println!("Both arrays have the same length of {}", N);
}

/// Validate that type is a set at compile time
pub trait IsSet {}

impl<T> IsSet for HashSet<T> {}
impl<T> IsSet for &HashSet<T> {}
impl<T> IsSet for BTreeSet<T> {}
impl<T> IsSet for &BTreeSet<T> {}

/// Validate that type is a Set at compile time
pub fn assert_type_is_set<T: IsSet>() {
    // This function doesn't need to do anything; it's just here to enforce the type constraint.
}

/// Validate that value is a set at compile time
pub fn assert_value_is_set<T: IsSet>(_value: T) {}

/// Validate that type is a HashSet at compile time
pub trait IsHashSet {}

impl<T> IsHashSet for HashSet<T> {}
impl<T> IsHashSet for &HashSet<T> {}

/// Validate that type is a HashSet at compile time
pub fn assert_type_is_hashset<T: IsHashSet>() {
    // This function doesn't need to do anything; it's just here to enforce the type constraint.
}

/// Validate that value is a hashset at compile time
pub fn assert_value_is_hashset<T: IsHashSet>(_value: T) {}

/// check if a type is an Option
pub trait IsOption {}

impl<T> IsOption for Option<T> {}
impl<T> IsOption for &Option<T> {}

/// Validate that type is an Option at compile time
pub fn assert_type_is_option<T: IsOption>() {
    // This function doesn't need to do anything; it's just here to enforce the type constraint.
}

/// Validate that value is an option at compile time
pub fn assert_value_is_option<T: IsOption>(_value: T) {}

/// Check if a type is an array
pub trait IsArray {}

impl<T> IsArray for Vec<T> {}
impl<T> IsArray for &Vec<T> {}
impl<T, const N: usize> IsArray for [T; N] {}
impl<T> IsArray for &[T] {}
impl<T> IsArray for &mut [T] {}
impl<T> IsArray for &mut Vec<T> {}
impl IsArray for crate::sql::Array {}
// impl<T> IsArray for T where T: Into<crate::sql::Array> {}
impl<N: Node> IsArray for LinkMany<N> {}
impl<V: Node> IsLinkMany for LinkMany<V> {}
impl<V: Node> IsLink for LinkMany<V> {}

impl<V: Node> IsLinkSelf for LinkSelf<V> {}
impl<V: Node> IsLink for LinkSelf<V> {}

impl<V: Node> IsLinkOne for LinkOne<V> {}
impl<V: Node> IsLink for LinkOne<V> {}
/// Validate that type is an array at compile time
/// Array can be a Vec or a slice
pub fn assert_type_is_array<T: IsArray>() {
    // This function doesn't need to do anything; it's just here to enforce the type constraint.
}

/// Validate that value is an array at compile time
pub fn assert_value_is_array<T: IsArray>(_value: T) {}

/// Validate that type is a vector at compile time
pub trait IsVec {}

impl<T> IsVec for Vec<T> {}
impl<T> IsVec for &Vec<T> {}

/// Validate that type is an Vec at compile time
/// Validate that type is a vector at compile time
///
/// # Example
/// ```
/// # use surreal_query_builder as surreal_orm;
/// use surreal_orm::validators::assert_type_is_vec;
/// assert_type_is_vec::<Vec<i8>>();
/// assert_type_is_vec::<Vec<String>>();
/// assert_type_is_vec::<Vec<i32>>();
/// assert_type_is_vec::<Vec<i64>>();
/// assert_type_is_vec::<Vec<i128>>();
/// assert_type_is_vec::<Vec<isize>>();
/// assert_type_is_vec::<Vec<u8>>();
/// assert_type_is_vec::<Vec<u16>>();
/// assert_type_is_vec::<Vec<u32>>();
/// assert_type_is_vec::<Vec<u64>>();
/// assert_type_is_vec::<Vec<u128>>();
/// assert_type_is_vec::<Vec<usize>>();
/// assert_type_is_vec::<Vec<f32>>();
/// assert_type_is_vec::<Vec<f64>>();
/// ```
pub fn assert_type_is_vec<T: IsVec>() {
    // This function doesn't need to do anything; it's just here to enforce the type constraint.
}

/// Validate that value is a vector at compile time
pub fn assert_value_is_vec<T: IsVec>(_value: T) {}

/// Validate that type is a Duration at compile time
pub trait IsDuration {}

// impl IsDuration for std::time::Duration {}
// impl IsDuration for crate::sql::Duration {}
impl<T> IsDuration for T where T: Into<crate::sql::Duration> {}

/// Validate that type is a Duration at compile time
pub fn assert_type_is_duration<T: IsDuration>() {
    // This function doesn't need to do anything; it's just here to enforce the type constraint.
}

/// Validate that value is a duration at compile time
pub fn assert_value_is_duration<T: IsDuration>(_value: T) {}

/// Validate that type is a Uuid at compile time
pub trait IsUuid {}

impl<T> IsUuid for T where T: Into<uuid::Uuid> {}

/// Validate that type is a Uuid at compile time
pub fn assert_type_is_uuid<T: IsUuid>() {
    // This function doesn't need to do anything; it's just here to enforce the type constraint.
}

/// Validate that value is a Uuid at compile time
/// # Example
/// ```
/// # use surreal_query_builder as surreal_orm;
/// use surreal_orm::validators::assert_value_is_uuid;
/// assert_value_is_uuid(uuid::Uuid::new_v4());
/// ```
pub fn assert_value_is_uuid<T: IsUuid>(_value: T) {}

/// Validate that type is a Datetime at compile time
pub trait IsDatetime {}

impl IsDatetime for chrono::DateTime<chrono::Utc> {}

/// Validate that type is a Datetime at compile time
pub fn assert_type_is_datetime<T: IsDatetime>() {
    // This function doesn't need to do anything; it's just here to enforce the type constraint.
}

/// Validate that value is a datetime at compile time
/// # Example
/// ```
/// # use surreal_query_builder as surreal_orm;
/// use surreal_orm::validators::assert_value_is_datetime;
/// assert_value_is_datetime(chrono::Utc::now());
/// ```
pub fn assert_value_is_datetime<T: IsDatetime>(_value: T) {}

/// Validate that type is a boolean at compile time
pub trait IsBool {}

impl IsBool for bool {}
impl IsBool for &bool {}

/// Validate that type is a bool at compile time
/// # Example
/// ```
/// # use surreal_query_builder as surreal_orm;
/// use surreal_orm::validators::assert_type_is_bool;
/// assert_type_is_bool::<bool>();
/// ```
pub fn assert_type_is_bool<T: IsBool>() {
    // This function doesn't need to do anything; it's just here to enforce the type constraint.
}

/// Validate that value is a bool at compile time
pub fn assert_value_is_bool<T: IsBool>(_value: T) {}

/// Validate that type is a Surrealdb Thing at compile time
pub trait IsThing {}

impl<T> IsThing for T where T: Into<crate::sql::Thing> {}
impl<N: Node> IsThing for LinkOne<N> {}
impl<N: Node> IsThing for LinkSelf<N> {}

/// Validate that type is a Thing at compile time
pub fn assert_type_is_thing<T: IsThing>() {
    // This function doesn't need to do anything; it's just here to enforce the type constraint.
}

/// Validate that value is a thing at compile time
pub fn assert_value_is_thing<T: IsThing>(_value: T) {}

// Links
/// check if a type is a link
pub trait IsLink {}

/// Validate that type is a link type at compile time
pub fn assert_type_is_link<T: IsLink>() {
    // This function doesn't need to do anything; it's just here to enforce the type constraint.
}

///
/// check if a type is a link one or one-to-one relationship type
pub trait IsLinkOne {}
///
/// Validate that type is a link one or one-to-one relationship type at compile time
pub fn assert_type_is_link_one<T: IsLinkOne>() {
    // This function doesn't need to do anything; it's just here to enforce the type constraint.
}

///
/// check if a type is a link self or self-referential relationship type
pub trait IsLinkSelf {}
/// check if a type is a link self or self-referential relationship type
pub fn assert_type_is_link_self<T: IsLinkSelf>() {
    // This function doesn't need to do anything; it's just here to enforce the type constraint.
}

///
/// check if a type is a link many or one-to-many relationship type
pub trait IsLinkMany {}
/// check if a type is a link many or one-to-many relationship type
pub fn assert_type_is_link_many<T: IsLinkMany>() {
    // This function doesn't need to do anything; it's just here to enforce the type constraint.
}

/// Validates that type is a surrealdb bytes at compile time
pub trait IsBytes {}

impl IsBytes for crate::sql::Bytes {}

/// Validate that type is a Bytes at compile time
pub fn assert_type_is_bytes<T: IsBytes>() {
    // This function doesn't need to do anything; it's just here to enforce the type constraint.
}

/// Validate that value is a bytes at compile time
/// # Example
/// ```
/// # use surreal_query_builder as surreal_orm;
/// use surreal_orm::validators::assert_value_is_bytes;
/// use surreal_orm::sql;
/// assert_value_is_bytes(sql::Bytes::from(vec![1, 2, 3]));
/// ```
pub fn assert_value_is_bytes<T: IsBytes>(_value: T) {}

/// Validates that a type is nullable at compile time
pub trait IsNull {}

impl<T> IsNull for Option<T> {}

/// Validate that type is a Null at compile time
pub fn assert_type_is_null<T: IsNull>() {
    // This function doesn't need to do anything; it's just here to enforce the type constraint.
}

/// Validate that value is a null at compile time
/// # Example
/// ```
/// # use surreal_query_builder as surreal_orm;
/// use surreal_orm::validators::assert_value_is_null;
/// assert_value_is_null(None::<u8>);
/// ```
pub fn assert_value_is_null<T: IsNull>(_value: T) {}

/// Validate that type is a Geometry at compile time
pub trait IsGeometry {}

// impl IsGeometry for crate::sql::Geometry {}
// impl IsGeometry for &crate::sql::Geometry {}
impl<T: Into<crate::sql::Geometry>> IsGeometry for T {}

/// Validate that type is a Geometry at compile time
/// # Example
/// ```
/// # use surreal_query_builder as surreal_orm;
/// use surreal_orm::validators::assert_type_is_geometry;
/// use surreal_orm::sql;
/// assert_type_is_geometry::<sql::Geometry>();
/// ```
pub fn assert_type_is_geometry<T: IsGeometry>() {
    // This function doesn't need to do anything; it's just here to enforce the type constraint.
}

/// Validate that value is a geometry at compile time
pub fn assert_value_is_geometry<T: IsGeometry>(_value: T) {}

/// Validate that type is Any surrealdb field type at compile time
pub trait IsAny {}

impl<T> IsAny for T where T: crate::serde::Serialize {}

/// Validate that type is a Any at compile time
/// # Example
/// ```
/// # use surreal_query_builder as surreal_orm;
/// use surreal_orm::validators::assert_type_is_any;
/// use surreal_orm::sql;
/// assert_type_is_any::<sql::Value>();
/// ```
pub fn assert_type_is_any<T: IsAny>() {
    // This function doesn't need to do anything; it's just here to enforce the type constraint.
}

/// Validate that value is a any at compile time
pub fn assert_value_is_any<T: IsAny>(_value: T) {}

/// Validate that type is a surrealdb orm Object at compile time
pub trait IsObject {}

impl<T: Object> IsObject for T {}

/// Validate that type is an embedded surreal orm Object at compile time
/// # Example
/// ```rust,ignore
/// # use surreal_query_builder as surreal_orm;
/// use surreal_orm::validators::assert_type_is_object;
/// assert_type_is_object::<AnEmbeddedObject>();
/// ```
pub fn assert_type_is_object<T: IsObject>() {
    // This function doesn't need to do anything; it's just here to enforce the type constraint.
}

/// Validate that type is a surrealdb Node at compile time
pub trait IsNode {}

impl<T: crate::Node> IsNode for T {}

/// Validate that type is a Node at compile time
/// # Example
/// ```rust,ignore
/// # use surreal_query_builder as surreal_orm;
/// use surreal_orm::validators::assert_type_is_node;
/// assert_type_is_node::<ANode>();
/// ```
pub fn assert_type_is_node<T: IsNode>() {
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
