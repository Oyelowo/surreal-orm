use std::fmt::Display;

#[derive(serde::Serialize, Debug, Default)]
pub struct DbField(String);

impl DbField {
    pub fn push_str(&mut self, string: &str) {
        self.0.push_str(string)
    }

    pub fn __as__(&self, alias: impl std::fmt::Display) -> String {
        format!("{self} AS {alias}")
    }
}

impl From<String> for DbField {
    fn from(value: String) -> Self {
        Self(value.into())
    }
}
impl From<&str> for DbField {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}
impl From<DbField> for String {
    fn from(value: DbField) -> Self {
        value.0
    }
}

impl std::fmt::Display for DbField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.0))
    }
}

/* impl std::fmt::Debug for DbField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.0))
    }
} */

#[derive(Debug, Clone)]
pub struct DbQuery {
    query_string: String,
}

impl DbQuery {
    pub fn new(query_string: String) -> Self {
        Self { query_string }
    }

    pub fn get_query_string(&self) -> &str {
        &self.query_string
    }
}

impl DbField {
    pub fn new(field_name: &str) -> Self {
        Self(field_name.to_owned())
    }

    pub fn equals<T: Display>(&self, value: T) -> DbQuery {
        DbQuery::new(format!("{} = {}", self.0, value))
    }

    pub fn not_equals<T: Display>(&self, value: T) -> DbQuery {
        DbQuery::new(format!("{} != {}", self.0, value))
    }

    pub fn greater_than<T: Display>(&self, value: T) -> DbQuery {
        DbQuery::new(format!("{} > {}", self.0, value))
    }

    pub fn greater_than_or_equals<T: Display>(&self, value: T) -> DbQuery {
        DbQuery::new(format!("{} >= {}", self.0, value))
    }

    pub fn less_than<T: Display>(&self, value: T) -> DbQuery {
        DbQuery::new(format!("{} < {}", self.0, value))
    }

    pub fn less_than_or_equals<T: Display>(&self, value: T) -> DbQuery {
        DbQuery::new(format!("{} <= {}", self.0, value))
    }

    pub fn between<T: Display, U: Display>(&self, lower_bound: T, upper_bound: U) -> DbQuery {
        DbQuery::new(format!(
            "{} BETWEEN {} AND {}",
            self.0, lower_bound, upper_bound
        ))
    }

    pub fn like(&self, pattern: &str) -> DbQuery {
        DbQuery::new(format!("{} LIKE '{}'", self.0, pattern))
    }

    pub fn not_like(&self, pattern: &str) -> DbQuery {
        DbQuery::new(format!("{} NOT LIKE '{}'", self.0, pattern))
    }

    pub fn is_null(&self) -> DbQuery {
        DbQuery::new(format!("{} IS NULL", self.0))
    }

    pub fn is_not_null(&self) -> DbQuery {
        DbQuery::new(format!("{} IS NOT NULL", self.0))
    }

    pub fn equal<T: Display>(&self, value: T) -> DbQuery {
        DbQuery::new(format!("{} = {}", self.0, value))
    }

    pub fn not_equal<T: Display>(&self, value: T) -> DbQuery {
        DbQuery::new(format!("{} != {}", self.0, value))
    }

    pub fn exactly_equal<T: Display>(&self, value: T) -> DbQuery {
        DbQuery::new(format!("{} == {}", self.0, value))
    }

    pub fn any_equal<T: Display>(&self, values: &[T]) -> DbQuery {
        let values_str = values
            .iter()
            .map(|value| format!("{}", value))
            .collect::<Vec<String>>()
            .join(", ");
        DbQuery::new(format!("{} ?= ({})", self.0, values_str))
    }

    pub fn all_equal<T: Display>(&self, values: &[T]) -> DbQuery {
        let values_str = values
            .iter()
            .map(|value| format!("{}", value))
            .collect::<Vec<String>>()
            .join(", ");
        DbQuery::new(format!("{} *= ({})", self.0, values_str))
    }

    pub fn fuzzy_equal<T: Display>(&self, value: T) -> DbQuery {
        DbQuery::new(format!("{} ~ {}", self.0, value))
    }

    pub fn fuzzy_not_equal<T: Display>(&self, value: T) -> DbQuery {
        DbQuery::new(format!("{} !~ {}", self.0, value))
    }

    pub fn any_fuzzy_equal<T: Display>(&self, values: &[T]) -> DbQuery {
        let values_str = values
            .iter()
            .map(|value| format!("{}", value))
            .collect::<Vec<String>>()
            .join(", ");
        DbQuery::new(format!("{} ?~ ({})", self.0, values_str))
    }

    pub fn all_fuzzy_equal<T: Display>(&self, values: &[T]) -> DbQuery {
        let values_str = values
            .iter()
            .map(|value| format!("{}", value))
            .collect::<Vec<String>>()
            .join(", ");
        DbQuery::new(format!("{} *~ ({})", self.0, values_str))
    }

    pub fn less_than_or_equal<T: Display>(&self, value: T) -> DbQuery {
        DbQuery::new(format!("{} <= {}", self.0, value))
    }

    pub fn greater_than_or_equal<T: Display>(&self, value: T) -> DbQuery {
        DbQuery::new(format!("{} >= {}", self.0, value))
    }

    pub fn add<T: Display>(&self, value: T) -> DbQuery {
        DbQuery::new(format!("{} + {}", self.0, value))
    }

    pub fn contains<T: Display>(&self, value: T) -> DbQuery {
        DbQuery::new(format!("{} CONTAINS {}", self.0, value))
    }

    pub fn contains_not<T: Display>(&self, value: T) -> DbQuery {
        DbQuery::new(format!("{} CONTAINSNOT {}", self.0, value))
    }

    pub fn contains_all<T: Display>(&self, values: &[T]) -> DbQuery {
        let values_str = values
            .iter()
            .map(|value| format!("{}", value))
            .collect::<Vec<String>>()
            .join(",");
        DbQuery::new(format!("{} CONTAINSALL ({})", self.0, values_str))
    }

    pub fn contains_any<T: Display>(&self, values: &[T]) -> DbQuery {
        let values_str = values
            .iter()
            .map(|value| format!("{}", value))
            .collect::<Vec<String>>()
            .join(",");
        DbQuery::new(format!("{} CONTAINSANY ({})", self.0, values_str))
    }

    pub fn contains_none<T: Display>(&self, values: &[T]) -> DbQuery {
        let values_str = values
            .iter()
            .map(|value| format!("{}", value))
            .collect::<Vec<String>>()
            .join(",");
        DbQuery::new(format!("{} CONTAINSNONE ({})", self.0, values_str))
    }

    pub fn inside<T: Display>(&self, value: T) -> DbQuery {
        DbQuery::new(format!("{} INSIDE {}", self.0, value))
    }

    pub fn not_inside<T: Display>(&self, value: T) -> DbQuery {
        DbQuery::new(format!("{} NOTINSIDE {}", self.0, value))
    }

    pub fn all_inside<T: Display>(&self, values: &[T]) -> DbQuery {
        let values_str = values
            .iter()
            .map(|value| format!("{}", value))
            .collect::<Vec<String>>()
            .join(",");
        DbQuery::new(format!("{} ALLINSIDE ({})", self.0, values_str))
    }

    pub fn any_inside<T: Display>(&self, values: &[T]) -> DbQuery {
        let values_str = values
            .iter()
            .map(|value| format!("{}", value))
            .collect::<Vec<String>>()
            .join(",");
        DbQuery::new(format!("{} ANYINSIDE ({})", self.0, values_str))
    }

    pub fn none_inside<T: Display>(&self, values: &[T]) -> DbQuery {
        let values_str = values
            .iter()
            .map(|value| format!("{}", value))
            .collect::<Vec<String>>()
            .join(",");
        DbQuery::new(format!("{} NONEINSIDE ({})", self.0, values_str))
    }

    pub fn outside<T: Display>(&self, value: T) -> DbQuery {
        DbQuery::new(format!("{} OUTSIDE {}", self.0, value))
    }

    pub fn intersects<T: Display>(&self, value: T) -> DbQuery {
        DbQuery::new(format!("{} INTERSECTS {}", self.0, value))
    }

    pub fn any_in_set<T: Display>(&self, values: &[T]) -> DbQuery {
        let value_str = values
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        DbQuery::new(format!("{} ?= ({})", self.0, value_str))
    }

    pub fn all_in_set<T: Display>(&self, values: &[T]) -> DbQuery {
        let value_str = values
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        DbQuery::new(format!("{} *= ({})", self.0, value_str))
    }

    pub fn subtract<T: Display>(&self, value: T) -> DbQuery {
        DbQuery::new(format!("{} - {}", self.0, value))
    }

    pub fn multiply<T: Display>(&self, value: T) -> DbQuery {
        DbQuery::new(format!("{} * {}", self.0, value))
    }

    pub fn divide<T: Display>(&self, value: T) -> DbQuery {
        DbQuery::new(format!("{} / {}", self.0, value))
    }

    pub fn is_truthy(&self) -> DbQuery {
        DbQuery::new(format!("{} && true", self.0))
    }

    pub fn truthy_and<T: Display>(&self, value: T) -> DbQuery {
        DbQuery::new(format!("{} && {}", self.0, value))
    }

    pub fn truthy_or<T: Display>(&self, value: T) -> DbQuery {
        DbQuery::new(format!("{} || {}", self.0, value))
    }

    pub fn is<T: Display>(&self, value: T) -> DbQuery {
        DbQuery::new(format!("{} IS {}", self.0, value))
    }

    pub fn is_not<T: Display>(&self, value: T) -> DbQuery {
        DbQuery::new(format!("{} IS NOT {}", self.0, value))
    }

    pub fn set_equal<T: Display>(&self, values: &[T]) -> DbQuery {
        let joined_values = values
            .iter()
            .map(|v| format!("{}", v))
            .collect::<Vec<String>>()
            .join(", ");
        DbQuery::new(format!("{} ?= {{{}}}", self.0, joined_values))
    }

    pub fn set_all_equal<T: Display>(&self, values: &[T]) -> DbQuery {
        let joined_values = values
            .iter()
            .map(|v| format!("{}", v))
            .collect::<Vec<String>>()
            .join(", ");
        DbQuery::new(format!("{} *= {{{}}}", self.0, joined_values))
    }

    pub fn and(&self, other: &DbField) -> DbQuery {
        DbQuery::new(format!("{} AND {}", self.0, other.0))
    }

    pub fn or(&self, other: &DbField) -> DbQuery {
        DbQuery::new(format!("{} OR {}", self.0, other.0))
    }
}

// #[derive(serde::Serialize, Debug, Default)]
// pub struct DbFieldTest(String);
// // Check whether two values are equal
// // Check whether two values are equal
// fn equal<T>(field: &str, value: T) -> Self where T: Into<FieldValue>;
//
// // Check whether two values are not equal
// fn not_equal<T>(field: &str, value: T) -> Self where T: Into<FieldValue>;
//
// // Check whether two values are exactly equal
// fn exactly_equal<T>(field: &str, value: T) -> Self where T: Into<FieldValue>;
//
// // Check whether any value in a set is equal to a value
// fn any_in_set<T>(field: &str, values: Vec<T>) -> Self where T: Into<FieldValue>;
//
// // Check whether all values in a set are equal to a value
// fn all_in_set<T>(field: &str, values: Vec<T>) -> Self where T: Into<FieldValue>;
//
// // Compare two values for equality using fuzzy matching
// fn fuzzy_equal<T>(field: &str, value: T) -> Self where T: Into<FieldValue>;
//
// // Compare two values for inequality using fuzzy matching
// fn fuzzy_not_equal<T>(field: &str, value: T) -> Self where T: Into<FieldValue>;
//
// // Check whether any value in a set is equal to a value using fuzzy matching
// fn any_fuzzy_equal<T>(field: &str, values: Vec<T>) -> Self where T: Into<FieldValue>;
//
// // Check whether all values in a set are equal to a value using fuzzy matching
// fn all_fuzzy_equal<T>(field: &str, values: Vec<T>) -> Self where T: Into<FieldValue>;
//
// // Check whether a value is less than another value
// fn less_than<T>(field: &str, value: T) -> Self where T: Into<FieldValue>;
//
// // Check whether a value is less than or equal to another value
// fn less_than_or_equal<T>(field: &str, value: T) -> Self where T: Into<FieldValue>;
//
// // Check whether a value is greater than another value
// fn greater_than<T>(field: &str, value: T) -> Self where T: Into<FieldValue>;
//
// // Check whether a value is greater than or equal to another value
// fn greater_than_or_equal<T>(field: &str, value: T) -> Self where T: Into<FieldValue>;
//
// // Add two values together
// fn add<T>(field: &str, value: T) -> Self where T: Into<FieldValue>;
//
// // Subtract a value from another value
// fn subtract<T>(field: &str, value: T) -> Self where T: Into<FieldValue>;
//
// // Multiply two values together
// fn multiply<T>(field: &str, value: T) -> Self where T: Into<FieldValue>;
//
// // Divide a value by another value
// fn divide<T>(field: &str, value: T) -> Self where T: Into<FieldValue>;
//
// // Checks whether two values are truthy
// fn and<T>(field: &str, value: T) -> Self where T: Into<FieldValue>;
//
// // Checks whether either of two values is truthy
// fn or<T>(field: &str, value: T) -> Self where T: Into<FieldValue>;
//
// // Checks whether two values are truthy
// fn truthy<T>(field: &str, value: T) -> Self where T: Into<FieldValue>;
//
// // Checks whether either of two values is truthy
// fn either_truthy<T>(field: &str, value: T) -> Self where T: Into<FieldValue>;
//
// // Check whether two values are equal
// fn is<T>(field: &str, value: T) -> Self where T:
// fn equal<T>(field: &str, value: T) -> Self where T: Into<FieldValue>;
//
// // Check whether two values are not equal
// fn not_equal<T>(field: &str, value: T) -> Self where T: Into<FieldValue>;
//
// // Check whether two values are exactly equal
// fn exactly_equal<T>(field: &str, value: T) -> Self where T: Into<FieldValue>;
//
// // Check whether any value in a set is equal to a value
// fn any_in_set<T>(field: &str, values: Vec<T>) -> Self where T: Into<FieldValue>;
//
// // Check whether all values in a set are equal to a value
// fn all_in_set<T>(field: &str, values: Vec<T>) -> Self where T: Into<FieldValue>;
//
// // Compare two values for equality using fuzzy matching
// fn fuzzy_equal<T>(field: &str, value: T) -> Self where T: Into<FieldValue>;
//
// // Compare two values for inequality using fuzzy matching
// fn fuzzy_not_equal<T>(field: &str, value: T) -> Self where T: Into<FieldValue>;
//
// // Check whether any value in a set is equal to a value using fuzzy matching
// fn any_fuzzy_equal<T>(field: &str, values: Vec<T>) -> Self where T: Into<FieldValue>;
//
// // Check whether all values in a set are equal to a value using fuzzy matching
// fn all_fuzzy_equal<T>(field: &str, values: Vec<T>) -> Self where T: Into<FieldValue>;
//
// // Check whether a value is less than another value
// fn less_than<T>(field: &str, value: T) -> Self where T: Into<FieldValue>;
//
// // Check whether a value is less than or equal to another value
// fn less_than_or_equal<T>(field: &str, value: T) -> Self where T: Into<FieldValue>;
//
// // Check whether a value is greater than another value
// fn greater_than<T>(field: &str, value: T) -> Self where T: Into<FieldValue>;
//
// // Check whether a value is greater than or equal to another value
// fn greater_than_or_equal<T>(field: &str, value: T) -> Self where T: Into<FieldValue>;
//
// // Add two values together
// fn add<T>(field: &str, value: T) -> Self where T: Into<FieldValue>;
//
// // Subtract a value from another value
// fn subtract<T>(field: &str, value: T) -> Self where T: Into<FieldValue>;
//
// // Multiply two values together
// fn multiply<T>(field: &str, value: T) -> Self where T: Into<FieldValue>;
//
// // Divide a value by another value
// fn divide<T>(field: &str, value: T) -> Self where T: Into<FieldValue>;
//
// // Checks whether two values are truthy
// fn and<T>(field: &str, value: T) -> Self where T: Into<FieldValue>;
//
// // Checks whether either of two values is truthy
// fn or<T>(field: &str, value: T) -> Self where T: Into<FieldValue>;
//
// // Checks whether two values are truthy
// fn truthy<T>(field: &str, value: T) -> Self where T: Into<FieldValue>;
//
// // Checks whether either of two values is truthy
// fn either_truthy<T>(field: &str, value: T) -> Self where T: Into<FieldValue>;
//
// // Check whether two values are equal
// fn is<T>(field: &str, value: T) -> Self where T:
// // Check whether a value contains another value
// pub fn contains<T>(field: &str, value: T) -> Self where T: Into<FieldValue> {
//     Self::new().add_condition(Condition::new(field, Operator::Contains, value.into()))
// }
//
// // Checks whether a value does not contain another value
// pub fn contains_not<T>(field: &str, value: T) -> Self where T: Into<FieldValue> {
//     Self::new().add_condition(Condition::new(field, Operator::ContainsNot, value.into()))
// }
//
// // Checks whether a value contains all other values
// pub fn contains_all<T>(field: &str, values: Vec<T>) -> Self where T: Into<FieldValue> {
//     Self::new().add_condition(Condition::new(field, Operator::ContainsAll, values.into_iter().map(|v| v.into()).collect()))
// }
//
// // Checks whether a value contains any other value
// pub fn contains_any<T>(field: &str, values: Vec<T>) -> Self where T: Into<FieldValue> {
//     Self::new().add_condition(Condition::new(field, Operator::ContainsAny, values.into_iter().map(|v| v.into()).collect()))
// }
//
// // Checks whether a value contains none of the following values
// pub fn contains_none<T>(field: &str, values: Vec<T>) -> Self where T: Into<FieldValue> {
//     Self::new().add_condition(Condition::new(field, Operator::ContainsNone, values.into_iter().map(|v| v.into()).collect()))
// }
//
// // Checks whether a value is contained within another value
// pub fn inside<T>(field: &str, value: T) -> Self where T: Into<FieldValue> {
//     Self::new().add_condition(Condition::new(field, Operator::Inside, value.into()))
// }
//
// // Checks whether a value is not contained within another value
// pub fn not_inside<T>(field: &str, value: T) -> Self where T: Into<FieldValue> {
//     Self::new().add_condition(Condition::new(field, Operator::NotInside, value.into()))
// }
//
// // Checks whether all values are contained within other values
// pub fn all_inside<T>(field: &str, values: Vec<T>) -> Self where T: Into<FieldValue> {
//     Self::new().add_condition(Condition::new(field, Operator::AllInside, values.into_iter().map(|v| v.into()).collect()))
// }
//
// // Checks whether any value is contained within other values
// pub fn any_inside<T>(field: &str, values: Vec<T>) -> Self where T: Into<FieldValue> {
//     Self::new().add_condition(Condition::new(field, Operator::AnyInside, values.into_iter().map(|v| v.into()).collect()))
// }
//
// // Checks whether no value is contained within other values
// pub fn none_inside<T>(field: &str, values: Vec<T>) -> Self where T: Into<FieldValue> {
//     Self::new().add_condition(Condition::new(field, Operator::NoneInside, values.into_iter().map(|v| v.into()).collect()))
// }
//
// // Checks whether a geometry type is outside of another geometry type
// pub fn outside<T>(field: &str, value: T) -> Self where T: Into<FieldValue> {
//     Self::new().add_condition(Condition::new(field, Operator::Outside, value.into()))
// }
//
// // Checks whether a geometry type intersects another geometry type
// pub fn intersects<T>(field: &str, value: T) -> Self where T: Into<FieldValue> {
// l   Self::new().add_condition(Condition::new(field, Operator::
