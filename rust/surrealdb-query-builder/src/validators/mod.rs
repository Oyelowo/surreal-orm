use num_traits::{Float, Num, PrimInt};

pub fn is_number<T: Num>() {}
pub fn is_int<T: PrimInt>() {}
pub fn is_float<T: Float>() {}
