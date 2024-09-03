/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */

// Math functions
// These functions can be used when analysing numeric data and numeric collections.
//
// Function	Description
// math::abs()	Returns the absolute value of a number
// math::ceil()	Rounds a number up to the next largest integer
// math::fixed()	Returns a number with the specified number of decimal places
// math::floor()	Rounds a number down to the next largest integer
// math::max()	Returns the maximum number in a set of numbers
// math::mean()	Returns the mean of a set of numbers
// math::median()	Returns the median of a set of numbers
// math::min()	Returns the minimum number in a set of numbers
// math::product()	Returns the product of a set of numbers
// math::round()	Rounds a number up or down to the nearest integer
// math::sqrt()	Returns the square root of a number
// math::sum()	Returns the total sum of a set of numbers

use crate::{ArrayLike, Buildable, Erroneous, Function, NumberLike, Parametric};

fn create_fn_with_single_num_arg(number: impl Into<NumberLike>, function_name: &str) -> Function {
    let number: NumberLike = number.into();
    let query_string = format!("math::{function_name}({})", number.build());

    Function {
        query_string,
        bindings: number.get_bindings(),
        errors: number.get_errors(),
    }
}

fn create_fn_with_single_array_arg(value: impl Into<ArrayLike>, function_name: &str) -> Function {
    let value: ArrayLike = value.into();
    let query_string = format!("math::{function_name}({})", value.build());

    Function {
        query_string,
        bindings: value.get_bindings(),
        errors: value.get_errors(),
    }
}

/// The math::fixed function returns a number with the specified number of decimal places.
///
/// math::fixed(number, number) -> number
/// The following example shows this function, and its output, when used in a select statement:
///
/// SELECT * FROM math::fixed(13.146572, 2);
/// 13.15
pub fn fixed_fn(number: impl Into<NumberLike>, decimal_place: impl Into<NumberLike>) -> Function {
    let number: NumberLike = number.into();
    let dp: NumberLike = decimal_place.into();
    let mut bindings = number.get_bindings();
    let mut errors = number.get_errors();
    bindings.extend(dp.get_bindings());
    errors.extend(dp.get_errors());

    let query_string = format!("math::fixed({}, {})", number.build(), dp.build());

    Function {
        query_string,
        bindings,
        errors,
    }
}

/// The math::fixed function returns a number with the specified number of decimal places.
/// Also aliased as `math_fixed`
/// math::fixed(number, number) -> number
/// The following example shows this function, and its output, when used in a select statement:
/// SELECT * FROM math::fixed(13.146572, 2);
/// 13.15
///
/// # Arguments
/// * `number` - The number to be rounded. This can be a number or a field or a parameter.
/// * `decimal_place` - The number of decimal places to round to. This can be a number or a field or a parameter.
///
/// # Example
/// ```rust
/// # use surreal_query_builder as  surreal_orm;
/// use surreal_orm::{*, functions::math};
///
/// math::fixed!(13.146572, 2);
/// # let score_field = Field::new("score_field");
/// # math::fixed!(score_field, 2);
///
/// let score_param = Param::new("score_param");
/// math::fixed!(score_param, 2);
///
/// let decimal_place_param = Param::new("decimal_place_param");
/// math::fixed!(13.146572, decimal_place_param);
/// ```
#[macro_export]
macro_rules! math_fixed {
    ( $number:expr, $decimal_place:expr ) => {
        $crate::functions::math::fixed_fn($number, $decimal_place)
    };
}

pub use math_fixed as fixed;

macro_rules! create_test_for_fn_with_single_arg {
    ($(#[$attr:meta])* => $function_name: expr) => {
        paste::paste! {
            // Although, surrealdb technically accepts stringified number also,
            // I dont see why that should be allowed at the app layer in rust
            // Obviously, if a field has stringified number that would work
            // during query execution
            $(#[$attr])*
            pub fn [<$function_name _fn>](number: impl Into<NumberLike>) -> Function {
                create_fn_with_single_num_arg(number, $function_name)
            }

            $(#[$attr])*
            #[macro_export]
            macro_rules!  [<math_ $function_name>] {
                ( $value:expr ) => {
                    $crate::functions::math::[<$function_name _fn>]($value)
                };
            }

            pub use [<math_ $function_name>] as [<$function_name>];


            #[cfg(test)]
            mod [<test_ $function_name>] {
                use super::*;
                use crate::*;
                use crate::functions::math;

                #[test]
                fn [<test_ $function_name _fn_with_field_data >] () {
                    let temparate = Field::new("temperature");
                    let result = math::[<$function_name _fn>](temparate);

                    assert_eq!(result.fine_tune_params(), format!("math::{}(temperature)", $function_name));
                    assert_eq!(result.to_raw().build(), format!("math::{}(temperature)", $function_name));
                }

                #[test]
                fn [<test_ $function_name _fn_with_fraction>]() {
                    let result = math::[<$function_name _fn>](45.23);
                    assert_eq!(result.fine_tune_params(), format!("math::{}($_param_00000001)", $function_name));
                    assert_eq!(result.to_raw().build(), format!("math::{}(45.23f)", $function_name));
                }

                #[test]
                fn [<test_ $function_name _fn_with_negative_number>]() {
                    let result = math::[<$function_name _fn>](-454);
                    assert_eq!(result.fine_tune_params(), format!("math::{}($_param_00000001)", $function_name));
                    assert_eq!(result.to_raw().build(), format!("math::{}(-454)", $function_name));
                }

                // Macro version
                #[test]
                fn [<test_ $function_name _macro_with_field_data >] () {
                    let temparate = Field::new("temperature");
                    let result = math::[<$function_name>]!(temparate);

                    assert_eq!(result.fine_tune_params(), format!("math::{}(temperature)", $function_name));
                    assert_eq!(result.to_raw().build(), format!("math::{}(temperature)", $function_name));
                }

                #[test]
                fn [<test_ $function_name _macro_with_param >] () {
                    let temparate = Param::new("temperature");
                    let result = math::[<$function_name>]!(temparate);

                    assert_eq!(result.fine_tune_params(), format!("math::{}($temperature)", $function_name));
                    assert_eq!(result.to_raw().build(), format!("math::{}($temperature)", $function_name));
                }

                #[test]
                fn [<test_ $function_name _macro_with_fraction>]() {
                    let result = math::[<$function_name>]!(45.23);
                    assert_eq!(result.fine_tune_params(), format!("math::{}($_param_00000001)", $function_name));
                    assert_eq!(result.to_raw().build(), format!("math::{}(45.23f)", $function_name));
                }

                #[test]
                fn [<test_ $function_name _macro_with_negative_number>]() {
                    let result = math::[<$function_name>]!(-454);
                    assert_eq!(result.fine_tune_params(), format!("math::{}($_param_00000001)", $function_name));
                    assert_eq!(result.to_raw().build(), format!("math::{}(-454)", $function_name));
                }
            }
        }
    };
}

create_test_for_fn_with_single_arg!(
    /// The math::abs function returns the absolute value of a number.
    /// The function is also aliased as `math_abs!`
    ///
    /// math::abs(number) -> number
    /// The following example shows this function, and its output, when used in a select statement:
    ///
    /// SELECT * FROM math::abs(13.746189);
    /// 13
    ///
    /// # Arguments
    /// * `number` - The number to get the absolute value of. The number can be a positive or negative value. Can be a number or field or param.
    ///
    /// # Example
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// # use surreal_orm::{*, functions::math};
    ///
    /// math::abs!(45.23);
    /// # let score_field = Field::new("score_field");
    /// # math::abs!(score_field);
    ///
    /// # let score_param = Param::new("score_param");
    /// # math::abs!(score_param);
    /// ```
=>
    "abs"
);

create_test_for_fn_with_single_arg!(
    /// The math::ceil function rounds a number up to the next largest integer.
    /// The function is also aliased as `math_ceil!`
    ///
    /// math::ceil(number) -> number
    /// The following example shows this function, and its output, when used in a select statement:
    ///
    /// SELECT * FROM math::ceil(13.146572);
    /// 14
    ///
    /// # Arguments
    /// * `number` - The number to round up. The number can be a positive or negative value. Can be a number or field or param.
    ///
    /// # Example
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// # use surreal_orm::{*, functions::math};
    ///
    /// math::ceil!(45.23);
    /// # let score_field = Field::new("score_field");
    /// # math::ceil!(score_field);
    ///
    /// # let score_param = Param::new("score_param");
    /// # math::ceil!(score_param);
    /// ```
=>
    "ceil"
);

create_test_for_fn_with_single_arg!(
    /// The math::floor function rounds a number down to the next largest integer.
    /// The function is also aliased as `math_floor!`
    ///
    /// math::floor(number) -> number
    /// The following example shows this function, and its output, when used in a select statement:
    ///
    /// SELECT * FROM math::floor(13.746189);
    /// 13
    ///
    /// # Arguments
    /// * `number` - The number to round down. The number can be a positive or negative value. Can be a number or field or param.
    ///
    /// # Example
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// # use surreal_orm::{*, functions::math};
    ///
    /// math::floor!(45.23);
    /// # let score_field = Field::new("score_field");
    /// # math::floor!(score_field);
    ///
    /// # let score_param = Param::new("score_param");
    /// # math::floor!(score_param);
    /// ```
=>
    "floor"
);

create_test_for_fn_with_single_arg!(
    /// The math::round function rounds a number up or down to the nearest integer.
    /// The function is also aliased as `math_round!`
    ///
    /// math::round(number) -> number
    /// The following example shows this function, and its output, when used in a select statement:
    ///
    /// SELECT * FROM math::round(13.53124);
    /// 14
    ///
    /// # Arguments
    /// * `number` - The number to round. The number can be a positive or negative value. Can be a number or field or param.
    ///
    /// # Example
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// # use surreal_orm::{*, functions::math};
    ///
    /// math::round!(45.23);
    /// # let score_field = Field::new("score_field");
    /// # math::round!(score_field);
    ///
    /// # let score_param = Param::new("score_param");
    /// # math::round!(score_param);
    /// ```
=>
    "round"
);

create_test_for_fn_with_single_arg!(
    /// The math::sqrt function returns the square root of a number.
    ///
    /// math::sqrt(number) -> number
    /// The following example shows this function, and its output, when used in a select statement:
    ///
    /// SELECT * FROM math::sqrt(15);
    /// 3.872983346207417
    ///
    /// # Arguments
    /// * `number` - The number to get the square root of. The number can be a positive or negative value. Can be a number or field or param.
    ///
    /// # Example
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// # use surreal_orm::{*, functions::math};
    ///
    /// math::sqrt!(45.23);
    /// # let score_field = Field::new("score_field");
    /// # math::sqrt!(score_field);
    ///
    /// # let score_param = Param::new("score_param");
    /// # math::sqrt!(score_param);
    /// ```
=>
    "sqrt"
);

macro_rules! create_test_for_fn_with_single_array_arg {
    ($(#[$attr:meta])* => $function_name: expr) => {
        paste::paste! {
            $(#[$attr])*
            pub fn [<$function_name _fn>](number: impl Into<ArrayLike>) -> Function {
                create_fn_with_single_array_arg(number, $function_name)
            }

            $(#[$attr])*
            #[macro_export]
            macro_rules!  [<math_ $function_name>] {
                ( $value:expr ) => {
                    $crate::functions::math::[<$function_name _fn>]($value)
                };
            }
            pub use [<math_ $function_name>] as [<$function_name>];
            #[cfg(test)]
            mod [<test_ $function_name>] {
                use super::*;
                use crate::*;
                use crate::functions::math;

                #[test]
                fn [<test_ $function_name _fn_with_field_data >] () {
                    let size_list = Field::new("size_list");
                    let result = math::[<$function_name _fn>](size_list);

                    assert_eq!(result.fine_tune_params(), format!("math::{}(size_list)", $function_name));
                    assert_eq!(result.to_raw().build(), format!("math::{}(size_list)", $function_name));
                }

                #[test]
                fn [<test_ $function_name _fn_with_number_array>]() {
                    let arr1 = array![1, 2, 3, 3.5];
                    let result = math::[<$function_name _fn>](arr1);
                    assert_eq!(result.fine_tune_params(), format!("math::{}($_param_00000001)", $function_name));
                    assert_eq!(result.to_raw().build(), format!("math::{}([1, 2, 3, 3.5f])", $function_name));
                }

                #[test]
                fn [<test_ $function_name _fn_with_mixed_array>]() {
                    let age = Field::new("age");
                    let arr = arr![1, 2, "4334", "Oyelowo", age];
                    let result = math::[<$function_name _fn>](arr);
                    assert_eq!(result.fine_tune_params(),
                        format!("math::{}([$_param_00000001, $_param_00000002, $_param_00000003, $_param_00000004, age])", $function_name));
                    assert_eq!(result.to_raw().build(), format!("math::{}([1, 2, '4334', 'Oyelowo', age])", $function_name));
                }

                // Macro version
                #[test]
                fn [<test_ $function_name _macro_with_field_data >] () {
                    let size_list = Field::new("size_list");
                    let result = math::[<$function_name>]!(size_list);

                    assert_eq!(result.fine_tune_params(), format!("math::{}(size_list)", $function_name));
                    assert_eq!(result.to_raw().build(), format!("math::{}(size_list)", $function_name));
                }

                #[test]
                fn [<test_ $function_name _macro_with_param >] () {
                    let size_list = Param::new("size_list");
                    let result = math::[<$function_name>]!(size_list);

                    assert_eq!(result.fine_tune_params(), format!("math::{}($size_list)", $function_name));
                    assert_eq!(result.to_raw().build(), format!("math::{}($size_list)", $function_name));
                }

                #[test]
                fn [<test_ $function_name _macro_with_number_array>]() {
                    let arr1 = array![1, 2, 3, 3.5];
                    let result = math::[<$function_name>]!(arr1);
                    assert_eq!(result.fine_tune_params(), format!("math::{}($_param_00000001)", $function_name));
                    assert_eq!(result.to_raw().build(), format!("math::{}([1, 2, 3, 3.5f])", $function_name));
                }

                #[test]
                fn [<test_ $function_name _macro_with_mixed_array>]() {
                    let age = Field::new("age");
                    let arr = arr![1, 2, "4334", "Oyelowo", age];
                    let result = math::[<$function_name>]!(arr);
                    assert_eq!(result.fine_tune_params(),
                        format!("math::{}([$_param_00000001, $_param_00000002, $_param_00000003, $_param_00000004, age])", $function_name));
                    assert_eq!(result.to_raw().build(), format!("math::{}([1, 2, '4334', 'Oyelowo', age])", $function_name));
                }
            }
        }
    };
}

create_test_for_fn_with_single_array_arg!(
/// The math::mean function returns the average of a set of numbers.
/// This function is aliased as `math_mean!`.
/// math::mean(number) -> number
/// The following example shows this function, and its output, when used in a select statement:
/// SELECT * FROM math::mean([26.164, 13.746189, 23, 16.4, 41.42]);
/// 24.432838
///
/// # Arguments
/// * `number` - The number to be averaged. Can be an array of numbers or field or parameter
/// representing an array of numbers.
///
/// # Example
/// ```rust
/// # use surreal_query_builder as surreal_orm;
/// # use surreal_orm::{*, functions::math};
///
/// math::mean!(vec![1, 2, 3, 4, 5]);
/// math::mean!(arr![1, 2, 3, 4, 5]);
/// math::mean!(array![1, 2, 3, 4, 5]);
/// let scores_field = Field::new("scores_field");
/// math::mean!(scores_field);
///
/// let scores_param = Param::new("scores_param");
/// math::mean!(scores_param);
/// ```
=> "mean"
);

create_test_for_fn_with_single_array_arg!(
/// The math::median function returns the median of a set of numbers.
/// This function is aliased as `math_median!`.
/// math::median(number) -> number
/// The following example shows this function, and its output, when used in a select statement:
/// SELECT * FROM math::median([26.164, 13.746189, 23, 16.4, 41.42]);
/// 23
/// # Arguments
/// * `number` - The number from which to calculate the median. Can be an array of numbers or field or parameter
/// representing an array of numbers.
///
/// # Example
/// ```rust
/// # use surreal_query_builder as surreal_orm;
/// # use surreal_orm::{*, functions::math};
///
/// math::median!(vec![1, 2, 3, 4, 5]);
/// math::median!(arr![1, 2, 3, 4, 5]);
/// math::median!(array![1, 2, 3, 4, 5]);
/// let scores_field = Field::new("scores_field");
/// math::median!(scores_field);
/// let scores_param = Param::new("scores_param");
/// math::median!(scores_param);
/// ```
=> "median"
);

create_test_for_fn_with_single_array_arg!(
/// The math::mode function returns the mode of a set of numbers.
/// This function is aliased as `math_mode!`.
/// math::mode(number) -> number
/// The following example shows this function, and its output, when used in a select statement:
/// SELECT * FROM math::mode([26.164, 13.746189, 23, 16.4, 41.42]);
/// 26.164
///
/// # Arguments
/// * `number` - The number from which to calculate the mode. Can be an array of numbers or field or parameter
/// representing an array of numbers.
///
/// # Example
/// ```rust
/// # use surreal_query_builder as surreal_orm;
/// # use surreal_orm::{*, functions::math};
///
/// math::mode!(vec![1, 2, 3, 4, 5]);
/// math::mode!(arr![1, 2, 3, 4, 5]);
/// let scores_field = Field::new("scores_field");
/// math::mode!(scores_field);
/// let scores_param = Param::new("scores_param");
/// math::mode!(scores_param);
/// ```
=> "mode"
);

create_test_for_fn_with_single_array_arg!(
/// The math::min function returns the minimum number in a set of numbers.
/// This function is aliased as `math_min!`.
/// math::min(array) -> number
/// The following example shows this function, and its output, when used in a select statement:
/// SELECT * FROM math::min([26.164, 13.746189, 23, 16.4, 41.42]);
/// 13.746189
///
/// # Arguments
/// * `array` - The array of numbers from which to calculate the minimum. Can be an array of numbers or field or parameter
/// representing an array of numbers.
///
/// # Example
/// ```rust
/// # use surreal_query_builder as surreal_orm;
/// # use surreal_orm::{*, functions::math};
///
/// math::min!(vec![1, 2, 3, 4, 5]);
/// math::min!(arr![1, 2, 3, 4, 5]);
/// let scores_field = Field::new("scores_field");
/// math::min!(scores_field);
/// let scores_param = Param::new("scores_param");
/// math::min!(scores_param);
/// ```
=> "min"
);

create_test_for_fn_with_single_array_arg!(
/// The math::max function returns the minimum number in a set of numbers.
/// This function is aliased as `math_max!`.
/// math::max(array) -> number
/// The following example shows this function, and its output, when used in a select statement:
/// SELECT * FROM math::max([26.164, 13.746189, 23, 16.4, 41.42]);
/// 13.746189
///
/// # Arguments
/// * `array` - The array of numbers from which to calculate the maximum. Can be an array of numbers or field or parameter
/// representing an array of numbers.
///
/// # Example
/// ```rust
/// # use surreal_query_builder as surreal_orm;
/// # use surreal_orm::{*, functions::math};
///
/// math::max!(vec![1, 2, 3, 4, 5]);
/// math::max!(arr![1, 2, 3, 4, 5]);
/// let scores_field = Field::new("scores_field");
/// math::max!(scores_field);
/// let scores_param = Param::new("scores_param");
/// math::max!(scores_param);
/// ```
=> "max"
);

create_test_for_fn_with_single_array_arg!(
/// The math::product function returns the product of a set of numbers.
/// This function is aliased as `math_product!`.
/// math::product(array) -> number
/// The following example shows this function, and its output, when used in a select statement:
/// SELECT * FROM math::product([26.164, 13.746189, 23, 16.4, 41.42]);
/// 5619119.004884841504
///
/// # Arguments
/// * `array` - The array of numbers from which to calculate the product. Can be an array of numbers or field or parameter
/// representing an array of numbers.
///
/// # Example
/// ```rust
/// # use surreal_query_builder as surreal_orm;
/// # use surreal_orm::{*, functions::math};
///
/// math::product!(vec![1, 2, 3, 4, 5]);
/// math::product!(arr![1, 2, 3, 4, 5]);
/// let scores_field = Field::new("scores_field");
/// math::product!(scores_field);
/// let scores_param = Param::new("scores_param");
/// math::product!(scores_param);
/// ```
=> "product"
);

// The math::sum function returns the total sum of a set of numbers.
//
// math::sum(array) -> number
// The following example shows this function, and its output, when used in a select statement:
//
// SELECT * FROM math::sum([26.164, 13.746189, 23, 16.4, 41.42]);
// 120.730189
create_test_for_fn_with_single_array_arg!(
/// The math::sum function returns the total sum of a set of numbers.
/// This function is aliased as `math_sum!`.
/// math::sum(array) -> number
/// The following example shows this function, and its output, when used in a select statement:
/// SELECT * FROM math::sum([26.164, 13.746189, 23, 16.4, 41.42]);
/// 120.730189
///
/// # Arguments
/// * `array` - The array of numbers from which to calculate the sum. Can be an array of numbers or field or parameter
/// representing an array of numbers.
///
/// # Example
/// ```rust
/// # use surreal_query_builder as surreal_orm;
/// # use surreal_orm::{*, functions::math};
///
/// math::sum!(vec![1, 2, 3, 4, 5]);
/// math::sum!(arr![1, 2, 3, 4, 5]);
/// let scores_field = Field::new("scores_field");
/// math::sum!(scores_field);
/// let scores_param = Param::new("scores_param");
/// math::sum!(scores_param);
/// ```
=> "sum"
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_fixed_fn_with_field_data() {
        let land_size = Field::new("land_size");
        let decimal_place = Field::new("decimal_place");
        let result = fixed_fn(land_size, decimal_place);

        assert_eq!(
            result.fine_tune_params(),
            "math::fixed(land_size, decimal_place)"
        );

        assert_eq!(
            result.to_raw().build(),
            "math::fixed(land_size, decimal_place)"
        );
    }

    #[test]
    fn test_fixed_fn_with_raw_numbers() {
        let result = fixed_fn(13.45423, 4);
        assert_eq!(
            result.fine_tune_params(),
            "math::fixed($_param_00000001, $_param_00000002)"
        );
        assert_eq!(result.to_raw().build(), "math::fixed(13.45423f, 4)");
    }

    #[test]
    fn test_fixed_fn_with_raw_number_with_field() {
        let land_mass = Field::new("country.land_mass");
        let result = fixed_fn(land_mass, 4);
        assert_eq!(
            result.fine_tune_params(),
            "math::fixed(country.land_mass, $_param_00000001)"
        );
        assert_eq!(result.to_raw().build(), "math::fixed(country.land_mass, 4)");
    }

    // Macro versions
    #[test]
    fn test_fixed_macro_with_field_data() {
        let land_size = Field::new("land_size");
        let decimal_place = Field::new("decimal_place");
        let result = fixed!(land_size, decimal_place);

        assert_eq!(
            result.fine_tune_params(),
            "math::fixed(land_size, decimal_place)"
        );

        assert_eq!(
            result.to_raw().build(),
            "math::fixed(land_size, decimal_place)"
        );
    }

    #[test]
    fn test_fixed_macro_with_params() {
        let land_size = Param::new("land_size");
        let decimal_place = Param::new("decimal_place");
        let result = fixed!(land_size, decimal_place);

        assert_eq!(
            result.fine_tune_params(),
            "math::fixed($land_size, $decimal_place)"
        );

        assert_eq!(
            result.to_raw().build(),
            "math::fixed($land_size, $decimal_place)"
        );
    }

    #[test]
    fn test_fixed_macro_with_raw_numbers() {
        let result = fixed!(13.45423, 4);
        assert_eq!(
            result.fine_tune_params(),
            "math::fixed($_param_00000001, $_param_00000002)"
        );
        assert_eq!(result.to_raw().build(), "math::fixed(13.45423f, 4)");
    }

    #[test]
    fn test_fixed_macro_with_raw_number_with_field() {
        let land_mass = Field::new("country.land_mass");
        let result = fixed!(land_mass, 4);
        assert_eq!(
            result.fine_tune_params(),
            "math::fixed(country.land_mass, $_param_00000001)"
        );
        assert_eq!(result.to_raw().build(), "math::fixed(country.land_mass, 4)");
    }
}
