use crate::{
    Binding, Buildable, Empty, Field, Filter, Function, Operation, Param, Parametric, Valuex,
};
use surrealdb::sql;
// Count functions
// These functions can be used when counting field values and expressions.

#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub enum CountArg {
    Empty,
    Field(Field),
    Param(Param),
    Operation(Operation),
    Filter(Filter),
    Array(sql::Array),
    MixedArray(Vec<Valuex>),
}

impl From<Empty> for CountArg {
    fn from(value: Empty) -> Self {
        CountArg::Empty
    }
}

impl From<Field> for CountArg {
    fn from(value: Field) -> Self {
        CountArg::Field(value)
    }
}

impl From<Operation> for CountArg {
    fn from(value: Operation) -> Self {
        CountArg::Operation(value)
    }
}

impl From<Filter> for CountArg {
    fn from(value: Filter) -> Self {
        CountArg::Filter(value)
    }
}

impl<T: Into<sql::Array>> From<T> for CountArg {
    fn from(value: T) -> Self {
        Self::Array(value.into())
    }
}

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
pub fn count_fn(countable: impl Into<CountArg>) -> Function {
    let countable: CountArg = countable.into();
    let mut bindings = vec![];

    let string = match countable {
        CountArg::Empty => format!(""),
        CountArg::Param(param) => {
            bindings = param.get_bindings();
            format!("{}", param)
        }
        CountArg::Field(field) => {
            bindings = field.get_bindings();
            format!("{}", field)
        }
        CountArg::Filter(filter) => {
            bindings = filter.get_bindings();
            format!("{}", filter)
        }
        CountArg::Array(array) => {
            let array: sql::Value = sql::Value::from(array);
            let array_binding = Binding::new(array);
            let param = format!("{}", array_binding.get_param_dollarised());
            bindings = vec![array_binding];
            param
        }
        CountArg::Operation(op) => {
            bindings = op.get_bindings();
            // format!("{}", filter)
            op.build()
        }
        CountArg::MixedArray(ma) => ma
            .into_iter()
            .map(|m| {
                let b = m.get_bindings();
                bindings.extend(b);
                m.build()
            })
            .collect::<Vec<_>>()
            .join(", "),
    };
    Function {
        query_string: format!("count({})", &string),
        bindings,
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
/// # use surrealdv_query_builder as surrealdb_orm;
/// # use surrealdb_orm::{*, functions::count};
/// count!();
///
/// count!(email.greater_than(15));
///
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
/// # let email_param = Param::new("email_param");
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
        let email = Field::new("email");
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
