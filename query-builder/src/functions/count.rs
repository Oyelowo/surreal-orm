/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */

use crate::{Buildable, Erroneous, Function, Parametric, ValueLike};
// Count functions
// These functions can be used when counting field values and expressions.

/// count()
/// Counts a row, or whether a given value is truthy
/// count
/// The count function counts the number of times that the function is called. This is useful for returning the total number of rows in a SELECT statement with a GROUP BY clause.
///
/// count() -> 1
/// If a value is given as the first argument, then this function checks whether a given value is truthy. This is useful for returning the total number of rows, which match a certain condition, in a SELECT statement, with a GROUP BY clause.
/// count(value) -> number
/// If an array is given, this function counts the number of items in the array which are truthy. If, instead, you want to count the total number of items in the given array, then use the array::len() function.
///
/// count(array) -> number
/// The following examples show this function, and its output, when used in a select statement:
pub fn count_fn(countable: impl Into<ValueLike>) -> Function {
    let countable: ValueLike = countable.into();

    Function {
        query_string: format!("count({})", countable.build()),
        bindings: countable.get_bindings(),
        errors: countable.get_errors(),
    }
}

/// Counts a row, or whether a given value is truthy
/// `count() -> 1`
/// If a value is given as the first argument, then this function checks whether a given value is truthy. This is useful for returning the total number of rows, which match a certain condition, in a SELECT statement, with a GROUP BY clause.
/// `count(value) -> number`
///
/// # Arguments
/// * `countable` - The value to count. Can be a field, a param, an operation, a filter, an array or a mixed array
///
/// # Examples
/// ```rust
/// # use surreal_query_builder as surreal_orm;
/// # use surreal_orm::{*, functions::count, statements::let_};
/// count!();
///
/// # let email = Field::new("email");
/// count!(email.greater_than(15));
///
/// # let email = Field::new("email");
/// count!(cond(email.greater_than(15)).and(email.less_than(20)));
///
/// # let email = Field::new("email");
/// # let age = Field::new("age");
/// # let student_count = AliasName::new("student_count");
/// count!(cond(age.greater_than(15)).and(email.like("oyelowo@example.com")))
///     .__as__(student_count);
///
/// count!(1);
///
/// # let head_count = AliasName::new("head_count");
/// count!().__as__(head_count);
///
/// # let email_field = Field::new("email_field");
/// count!(email_field);
///
/// let email_param = let_("email_param").equal_to("oyelowo@codebreather.com").get_param();
/// count!(email_param);
#[macro_export]
macro_rules! count {
    ( $countable:expr ) => {
        $crate::functions::count_fn($countable)
    };
    () => {
        $crate::functions::count_fn($crate::Empty)
    };
}
pub use count;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_count_withoout_arguments() {
        let result = count_fn(Empty);
        assert_eq!(result.fine_tune_params(), "count()");
        assert_eq!(result.to_raw().to_string(), "count()");
    }

    #[test]
    fn test_count_with_db_field() {
        let email = Field::new("email");
        let result = count_fn(email);
        assert_eq!(result.fine_tune_params(), "count(email)");
        assert_eq!(result.to_raw().to_string(), "count(email)");
    }

    #[test]
    fn test_count_with_simple_field_filter_operation() {
        let email = Field::new("email");
        let result = count_fn(email.greater_than(15));
        assert_eq!(result.fine_tune_params(), "count(email > $_param_00000001)");
        assert_eq!(result.to_raw().to_string(), "count(email > 15)");

        let email = Field::new("email");
        let result = count_fn(email.greater_than(15).or(true));
        assert_eq!(
            result.fine_tune_params(),
            "count(email > $_param_00000001 OR $_param_00000002)"
        );
        assert_eq!(result.to_raw().to_string(), "count(email > 15 OR true)");
    }

    #[test]
    fn test_count_with_complex_field_filter_operation() {
        let email = Field::new("email");
        let age = Field::new("age");
        let result = count_fn(cond(age.greater_than(15)).and(email.like("oyelowo@example.com")));
        assert_eq!(
            result.fine_tune_params(),
            "count((age > $_param_00000001) AND (email ~ $_param_00000002))"
        );
        assert_eq!(
            result.to_raw().to_string(),
            "count((age > 15) AND (email ~ 'oyelowo@example.com'))"
        );
    }

    #[test]
    fn test_count_with_array() {
        let result = count_fn(array![1, 2, 3, 4, 5, "4334", "Oyelowo"]);
        assert_eq!(result.fine_tune_params(), "count($_param_00000001)");
        assert_eq!(
            result.to_raw().to_string(),
            "count([1, 2, 3, 4, 5, '4334', 'Oyelowo'])"
        );
    }

    #[test]
    fn test_count_macro_withoout_arguments() {
        let result = count!();
        assert_eq!(result.fine_tune_params(), "count()");
        assert_eq!(result.to_raw().to_string(), "count()");
    }

    #[test]
    fn test_count_macro_withoout_arguments_aliased() {
        let head_count = AliasName::new("head_count");
        let result = count!().__as__(head_count);

        assert_eq!(result.fine_tune_params(), "count() AS head_count");
        assert_eq!(result.to_raw().to_string(), "count() AS head_count");
    }

    #[test]
    fn test_count_macro_with_db_field() {
        let email = Field::new("email");
        let result = count!(email);
        assert_eq!(result.fine_tune_params(), "count(email)");
        assert_eq!(result.to_raw().to_string(), "count(email)");
    }

    #[test]
    fn test_count_macro_with_simple_field_filter_operation() {
        let email = Field::new("email");
        let result = count!(email.greater_than(15));
        assert_eq!(result.fine_tune_params(), "count(email > $_param_00000001)");
        assert_eq!(result.to_raw().to_string(), "count(email > 15)");

        let email = Field::new("email");
        let result = count!(email.greater_than(15).or(true));
        assert_eq!(
            result.fine_tune_params(),
            "count(email > $_param_00000001 OR $_param_00000002)"
        );
        assert_eq!(result.to_raw().to_string(), "count(email > 15 OR true)");
    }

    #[test]
    fn test_count_macro_with_complex_field_filter_operation() {
        let email = Field::new("email");
        let age = Field::new("age");
        let result = count!(cond(age.greater_than(15)).and(email.like("oyelowo@example.com")));
        assert_eq!(
            result.fine_tune_params(),
            "count((age > $_param_00000001) AND (email ~ $_param_00000002))"
        );
        assert_eq!(
            result.to_raw().to_string(),
            "count((age > 15) AND (email ~ 'oyelowo@example.com'))"
        );
    }

    #[test]
    fn test_count_macro_with_complex_field_filter_operation_aliased() {
        let email = Field::new("email");
        let age = Field::new("age");
        let student_count = AliasName::new("student_count");
        let result = count!(cond(age.greater_than(15)).and(email.like("oyelowo@example.com")))
            .__as__(student_count);
        assert_eq!(
            result.fine_tune_params(),
            "count((age > $_param_00000001) AND (email ~ $_param_00000002)) AS student_count"
        );
        assert_eq!(
            result.to_raw().to_string(),
            "count((age > 15) AND (email ~ 'oyelowo@example.com')) AS student_count"
        );
    }
}
