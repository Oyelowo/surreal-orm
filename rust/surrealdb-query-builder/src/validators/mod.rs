use num_traits::{Float, Num, PrimInt};

/// Validate that type is a number at compile time
///
/// # Example
/// ```
/// # use surrealdb_query_builder::validators::is_number;
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
/// # use surrealdb_query_builder::validators::is_int;
/// is_int::<i8>();
/// is_int::<i16>();
/// is_int::<i32>();
/// is_int::<i64>();
/// is_int::<i128>();
/// is_int::<isize>();
/// ```
pub fn is_int<T: PrimInt>() {}

/// Validate that type is a primitive float at compile time
///
/// # Example
/// ```
/// # use surrealdb_query_builder::validators::is_float;
///
/// is_float::<f32>();
/// is_float::<f64>();
/// ```
pub fn is_float<T: Float>() {}
