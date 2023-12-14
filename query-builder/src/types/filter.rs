/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::{borrow::Cow, fmt::Display};

use crate::{Binding, BindingsList, Buildable, Conditional, Erroneous, Parametric};

use super::Empty;

/// Creates a new filter from a given `filterable` input.
///
/// # Arguments
///
/// * `filterable` - A value that can be converted into a `Filter`.
///
/// # Example
///
/// ```rust
/// # use surreal_query_builder as surreal_orm;
/// # use surreal_orm::*;
/// # let age = Field::new("age");
/// # let name = Field::new("name");
/// # let title = Field::new("title");
///
/// let filter = cond(age.greater_than(18))
///                 .and(name.like("%Oyelowo%"))
///                 .or(title.equal("Professor"));
/// ```
pub fn cond(filterable: impl Conditional) -> Filter {
    Filter::new(filterable)
}

/// Creates a new filter from a given `filterable` input.
/// This is a macro version of the `cond` function.
/// It is useful for creating filters.
/// It is also useful for creating compound conditions.
///
/// # Arguments
/// * `filterable` - A value that can be converted into a `Filter`.
///
/// # Example
///
/// ```rust
/// # use surreal_query_builder as surreal_orm;
/// # use surreal_orm::*;
/// # let age = Field::new("age");
/// # let name = Field::new("name");
/// # let title = Field::new("title");
///
/// let filter = cond!((age > 18) AND (name ~ "%Oyelowo%") OR (title == "Professor"));
/// let filter_simple = cond!(age > 18);
/// let filter_mixed = cond!((age.or(4).or(545).or(232)) OR (title = "Professor") AND (age < 100));
/// ```
#[macro_export]
macro_rules! cond {
    // Base patterns
    ($field:ident IS $value:expr) => {
        $field.is($value)
    };
    ($field:ident is $value:expr) => {
        $field.is($value)
    };
    ($field:ident = $value:expr) => {
        $field.equal($value)
    };
    ($field:ident != $value:expr) => {
        $field.not_equal($value)
    };
    ($field:ident == $value:expr) => {
        $field.exactly_equal($value)
    };
    ($field:ident ?= $value:expr) => {
        $field.any_equal($value)
    };
    ($field:ident *= $value:expr) => {
        $field.all_equal($value)
    };
    ($field:ident ~ $value:expr) => {
        $field.like($value)
    };
    ($field:ident !~ $value:expr) => {
        $field.not_like($value)
    };
    ($field:ident ?~ $value:expr) => {
        $field.any_like($value)
    };
    ($field:ident *~ $value:expr) => {
        $field.all_like($value)
    };
    ($field:ident > $value:expr) => {
        $field.greater_than($value)
    };
    ($field:ident >= $value:expr) => {
        $field.greater_than_or_equal($value)
    };
    ($field:ident < $value:expr) => {
        $field.less_than($value)
    };
    ($field:ident <= $value:expr) => {
        $field.less_than_or_equal($value)
    };
    ($field:ident + $value:expr) => {
        $field.add($value)
    };
    ($field:ident - $value:expr) => {
        $field.minus($value)
    };
    ($field:ident * $value:expr) => {
        $field.multiply($value)
    };
    ($field:ident / $value:expr) => {
        $field.divide($value)
    };
    ($field:ident ** $value:expr) => {
        $field.power($value)
    };
    ($field:ident && $value:expr) => {
        $field.truthy_and($value)
    };
    ($field:ident || $value:expr) => {
        $field.truthy_or($value)
    };
    ($field:ident and $value:expr) => {
        $field.and($value)
    };
    ($field:ident or $value:expr) => {
        $field.or($value)
    };
    ($field:ident AND $value:expr) => {
        $field.and($value)
    };
    ($field:ident OR $value:expr) => {
        $field.or($value)
    };
    ($field:ident is not $value:expr) => {
        $field.is_not($value)
    };
    ($field:ident IS NOT $value:expr) => {
        $field.is_not($value)
    };
    ($field:ident CONTAINS $value:expr) => {
        $field.contains($value)
    };

    ($field:ident contains $value:expr) => {
        $field.contains($value)
    };

    ($field:ident CONTAINSNOT $value:expr) => {
        $field.contains_not($value)
    };

    ($field:ident containsnot $value:expr) => {
        $field.contains_not($value)
    };

    ($field:ident CONTAINSALL $value:expr) => {
        $field.contains_all($value)
    };

    ($field:ident containsall $value:expr) => {
        $field.contains_all($value)
    };

    ($field:ident CONTAINSANY $value:expr) => {
        $field.contains_any($value)
    };

    ($field:ident containsany $value:expr) => {
        $field.contains_any($value)
    };

    ($field:ident CONTAINSNONE $value:expr) => {
        $field.contains_none($value)
    };

    ($field:ident containsnone $value:expr) => {
        $field.contains_none($value)
    };

    ($field:ident INSIDE $value:expr) => {
        $field.inside($value)
    };

    ($field:ident inside $value:expr) => {
        $field.inside($value)
    };

    ($field:ident IN $value:expr) => {
        $field.in_($value)
    };

    ($field:ident in $value:expr) => {
        $field.in_($value)
    };

    ($field:ident NOTINSIDE $value:expr) => {
        $field.not_inside($value)
    };

    ($field:ident notinside $value:expr) => {
        $field.not_inside($value)
    };

    ($field:ident ALLINSIDE $value:expr) => {
        $field.all_inside($value)
    };

    ($field:ident allinside $value:expr) => {
        $field.all_inside($value)
    };

    ($field:ident ANYINSIDE $value:expr) => {
        $field.any_inside($value)
    };

    ($field:ident anyinside $value:expr) => {
        $field.any_inside($value)
    };

    ($field:ident NONEINSIDE $value:expr) => {
        $field.none_inside($value)
    };

    ($field:ident noneinside $value:expr) => {
        $field.none_inside($value)
    };

    ($field:ident OUTSIDE $value:expr) => {
        $field.outside($value)
    };

    ($field:ident outside $value:expr) => {
        $field.outside($value)
    };

    ($field:ident INTERSECTS $value:expr) => {
        $field.intersects($value)
    };

    ($field:ident intersects $value:expr) => {
        $field.intersects($value)
    };

    // Base patterns
    (($field:ident IS $value:expr)) => {
        $field.is($value)
    };
    (($field:ident = $value:expr)) => {
        $field.equal($value)
    };
    (($field:ident != $value:expr)) => {
        $field.not_equal($value)
    };
    (($field:ident == $value:expr)) => {
        $field.exactly_equal($value)
    };
    (($field:ident ?= $value:expr)) => {
        $field.any_equal($value)
    };
    (($field:ident *= $value:expr)) => {
        $field.all_equal($value)
    };
    (($field:ident ~ $value:expr)) => {
        $field.like($value)
    };
    (($field:ident !~ $value:expr)) => {
        $field.not_like($value)
    };
    (($field:ident ?~ $value:expr)) => {
        $field.any_like($value)
    };
    (($field:ident *~ $value:expr)) => {
        $field.all_like($value)
    };
    (($field:ident > $value:expr)) => {
        $field.greater_than($value)
    };
    (($field:ident >= $value:expr)) => {
        $field.greater_than_or_equal($value)
    };
    (($field:ident < $value:expr)) => {
        $field.less_than($value)
    };
    (($field:ident <= $value:expr)) => {
        $field.less_than_or_equal($value)
    };
    (($field:ident + $value:expr)) => {
        $field.add($value)
    };
    (($field:ident - $value:expr)) => {
        $field.minus($value)
    };
    (($field:ident * $value:expr)) => {
        $field.multiply($value)
    };
    (($field:ident / $value:expr)) => {
        $field.divide($value)
    };
    (($field:ident ** $value:expr)) => {
        $field.power($value)
    };
    (($field:ident && $value:expr)) => {
        $field.truthy_and($value)
    };
    (($field:ident || $value:expr)) => {
        $field.truthy_or($value)
    };
    (($field:ident AND $value:expr)) => {
        $field.and($value)
    };
    (($field:ident and $value:expr)) => {
        $field.and($value)
    };
    (($field:ident OR $value:expr)) => {
        $field.or($value)
    };
    (($field:ident or $value:expr)) => {
        $field.or($value)
    };
    (($field:ident IS NOT $value:expr)) => {
        $field.is_not($value)
    };

    (($field:ident is not $value:expr)) => {
        $field.is_not($value)
    };

    (($field:ident CONTAINS $value:expr)) => {
        $field.contains($value)
    };

    (($field:ident contains $value:expr)) => {
        $field.contains($value)
    };

    (($field:ident CONTAINSNOT $value:expr)) => {
        $field.contains_not($value)
    };

    (($field:ident containsnot $value:expr)) => {
        $field.contains_not($value)
    };

    (($field:ident CONTAINSALL $value:expr)) => {
        $field.contains_all($value)
    };

    (($field:ident containsall $value:expr)) => {
        $field.contains_all($value)
    };

    (($field:ident CONTAINSANY $value:expr)) => {
        $field.contains_any($value)
    };

    (($field:ident containsany $value:expr)) => {
        $field.contains_any($value)
    };

    (($field:ident CONTAINSNONE $value:expr)) => {
        $field.contains_none($value)
    };


    (($field:ident containsnone $value:expr)) => {
        $field.contains_none($value)
    };


    (($field:ident INSIDE $value:expr)) => {
        $field.inside($value)
    };


    (($field:ident inside $value:expr)) => {
        $field.inside($value)
    };


    (($field:ident IN $value:expr)) => {
        $field.in_($value)
    };

    (($field:ident in $value:expr)) => {
        $field.in_($value)
    };


    (($field:ident NOTINSIDE $value:expr)) => {
        $field.not_inside($value)
    };

    (($field:ident notinside $value:expr)) => {
        $field.not_inside($value)
    };


    (($field:ident ALLINSIDE $value:expr)) => {
        $field.all_inside($value)
    };

    (($field:ident allinside $value:expr)) => {
        $field.all_inside($value)
    };

    (($field:ident ANYINSIDE $value:expr)) => {
        $field.any_inside($value)
    };

    (($field:ident anyinside $value:expr)) => {
        $field.any_inside($value)
    };

    (($field:ident NONEINSIDE $value:expr)) => {
        $field.none_inside($value)
    };

    (($field:ident noneinside $value:expr)) => {
        $field.none_inside($value)
    };

    (($field:ident OUTSIDE $value:expr)) => {
        $field.outside($value)
    };

    (($field:ident outside $value:expr)) => {
        $field.outside($value)
    };

    (($field:ident INTERSECTS $value:expr)) => {
        $field.intersects($value)
    };

    (($field:ident intersects $value:expr)) => {
        $field.intersects($value)
    };


    ($left:tt AND $right:tt) => {
        $crate::cond(cond!($left)).and(cond!($right))
    };
    ($left:tt OR $right:tt) => {
        $crate::cond(cond!($left)).or(cond!($right))
    };


    // Handling recursive connectors
    ($left:tt AND $middle:tt AND $($tail:tt)*) => {
        $crate::cond!($left AND $middle).and($crate::cond!($($tail)*))
    };
    ($left:tt OR $middle:tt OR $($tail:tt)*) => {
        $crate::cond!($left OR $middle).or($crate::cond!($($tail)*))
    };
    ($left:tt AND $middle:tt OR $($tail:tt)*) => {
        $crate::cond!($left AND $middle).or($crate::cond!($($tail)*))
    };
    ($left:tt OR $middle:tt AND $($tail:tt)*) => {
        $crate::cond!($left OR $middle).and($crate::cond!($($tail)*))
    };

    ($left:tt and $middle:tt and $($tail:tt)*) => {
        $crate::cond!($left AND $middle).and($crate::cond!($($tail)*))
    };
    ($left:tt or $middle:tt or $($tail:tt)*) => {
        $crate::cond!($left OR $middle).or($crate::cond!($($tail)*))
    };
    ($left:tt and $middle:tt or $($tail:tt)*) => {
        $crate::cond!($left AND $middle).or($crate::cond!($($tail)*))
    };
    ($left:tt or $middle:tt and $($tail:tt)*) => {
        $crate::cond!($left OR $middle).and($crate::cond!($($tail)*))
    };

    // Base condition (a catch-all at the end)
    ($base:tt) => {
        $base
    };
}

/// This module provides functionality for building complex filters for database queries.
///
/// A `Filter` struct represents a filter that can be composed of subfilters using logical
/// operators like `AND` and `OR`. Filters can be created using the `empty` function or by
/// converting a string using `Filter::new`.
///
/// The `cond` function is used to create a new filter from a given `filterable` input, which
/// can be of type `Filter`.
///
/// Methods on a `Filter` instance are used to combine filters with logical operators or to
/// modify the filter using methods like `bracketed`.
/// ```
#[derive(Debug, Clone, Default)]
pub struct Filter {
    query_string: String,
    bindings: BindingsList,
}

impl Filter {
    /// Creates a new `Filter` instance.
    pub fn new(query: impl Conditional) -> Self {
        Self {
            query_string: query.get_condition_query_string(),
            bindings: query.get_bindings(),
        }
    }

    /// Combines the current filter with another filter using a logical OR operator.
    ///
    /// # Arguments
    ///
    /// * `filter` - The filter to be combined with the current filter.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// # use surreal_orm::*;
    /// # let age = Field::new("age");
    /// # let title = Field::new("title");
    /// cond(age.greater_than(18))
    ///     .or(title.equal("Professor"));
    /// ```
    pub fn or(self, filter: impl Buildable + Parametric) -> Self {
        let precendence = self.bracket_if_not_already();
        let new_params = self.___update_bindings(&filter);

        let filter = &filter.build();
        let query_string = format!("{precendence} OR ({filter})");

        Filter {
            query_string,
            bindings: new_params,
        }
    }

    /// Combines this `Filter` instance with another using the `AND` operator.
    ///
    /// # Arguments
    ///
    /// * `filter` - The `Filter` instance to combine with this one.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// # use surreal_orm::*;
    /// # let age = Field::new("age");
    /// # let title = Field::new("title");
    /// cond(age.greater_than(18))
    ///     .and(title.equal("Professor"));
    /// ```
    pub fn and(self, filter: impl Buildable + Parametric) -> Self {
        let precendence = self.bracket_if_not_already();
        let new_params = self.___update_bindings(&filter);

        let filter = &filter.build();
        let query_string = format!("{precendence} AND ({filter})");

        Filter {
            query_string,
            bindings: new_params,
        }
    }

    /// Wraps this `Filter` instance in a set of brackets.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// # use surreal_orm::*;
    /// # let age = Field::new("age");
    /// # let title = Field::new("title");
    ///
    /// let filter = cond(age.greater_than(18))
    ///     .or(title.equal("Professor"));
    /// let bracketed_filter = filter.bracketed();
    /// assert_eq!(bracketed_filter.to_raw().build(), "((age > 18) OR (title = 'Professor'))");
    /// ```
    pub fn parenthesized(&self) -> Self {
        Filter {
            query_string: format!("({self})"),
            bindings: self.bindings.to_vec(),
        }
    }

    /// Wraps this `Filter` instance in a set of brackets if it isn't already wrapped.
    fn bracket_if_not_already(&self) -> impl Display {
        let filter = self.to_string();
        match (filter.starts_with('('), filter.ends_with(')')) {
            (true, true) => format!("{self}"),
            _ => format!("({self})"),
        }
    }

    pub(crate) fn ___update_bindings(self, filter: &impl Parametric) -> Vec<Binding> {
        [self.bindings.as_slice(), filter.get_bindings().as_slice()].concat()
    }
}

impl<'a> From<Cow<'a, Filter>> for Filter {
    fn from(value: Cow<'a, Filter>) -> Self {
        match value {
            Cow::Borrowed(v) => v.clone(),
            Cow::Owned(v) => v,
        }
    }
}

impl From<Option<Self>> for Filter {
    fn from(value: Option<Filter>) -> Self {
        match value {
            Some(v) => v,
            None => Empty.into(),
        }
    }
}

impl From<String> for Filter {
    fn from(value: String) -> Self {
        Self {
            query_string: value,
            bindings: vec![],
        }
    }
}

impl Buildable for Filter {
    fn build(&self) -> String {
        self.query_string.to_string()
    }
}

impl Conditional for Filter {}

impl Conditional for &Filter {}
impl Buildable for &Filter {
    fn build(&self) -> String {
        self.query_string.to_string()
    }
}
impl Erroneous for &Filter {
    fn get_errors(&self) -> Vec<String> {
        vec![]
    }
}

impl Parametric for &Filter {
    fn get_bindings(&self) -> BindingsList {
        self.bindings.to_vec()
    }
}

impl From<Empty> for Filter {
    fn from(_value: Empty) -> Self {
        Filter::new(Empty)
    }
}

impl Erroneous for Filter {
    fn get_errors(&self) -> Vec<String> {
        vec![]
    }
}

impl Parametric for Filter {
    fn get_bindings(&self) -> BindingsList {
        self.bindings.to_vec()
    }
}

impl std::fmt::Display for Filter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.build())
    }
}

#[cfg(test)]
mod tests {
    use crate::{cond, Field, Operatable, ToRaw};

    use super::*;

    #[test]
    fn test_filter_simple() {
        let age = Field::new("age");

        let filter = age.greater_than(18);

        assert_eq!(filter.to_raw().build(), "age > 18");
    }

    #[test]
    fn test_filter_simple_with_cond_macro() {
        let age = Field::new("age");
        let filter = cond!(age > 18);

        assert_eq!(filter.to_raw().build(), "age > 18");
    }

    #[test]
    fn test_filter() {
        let age = Field::new("age");
        let title = Field::new("title");

        let filter = cond(age.greater_than(18))
            .or(title.equal("Professor"))
            .and(age.less_than(100));

        assert_eq!(
            filter.to_raw().build(),
            "(age > 18) OR (title = 'Professor') AND (age < 100)"
        );
    }

    #[test]
    fn test_filter_with_cond_macro() {
        let age = Field::new("age");
        let title = Field::new("title");

        let filter = cond!((age > 18) OR (title = "Professor") AND (age < 100));

        assert_eq!(
            filter.to_raw().build(),
            "(age > 18) OR (title = 'Professor') AND (age < 100)"
        );
    }

    #[test]
    fn test_filter_bracketed() {
        let age = Field::new("age");
        let title = Field::new("title");

        let filter = cond(age.greater_than(18))
            .or(title.equal("Professor"))
            .and(age.less_than(100));

        let bracketed_filter = filter.parenthesized();
        assert_eq!(
            bracketed_filter.to_raw().build(),
            "((age > 18) OR (title = 'Professor') AND (age < 100))"
        );
    }

    #[test]
    fn test_filter_bracketed_with_cond_macro() {
        let age = Field::new("age");
        let title = Field::new("title");

        let filter = cond!((age > 18) OR (title = "Professor") AND (age < 100));

        let bracketed_filter = filter.parenthesized();
        assert_eq!(
            bracketed_filter.fine_tune_params(),
            "((age > $_param_00000001) OR (title = $_param_00000002) AND (age < $_param_00000003))"
        );
        assert_eq!(
            bracketed_filter.to_raw().build(),
            "((age > 18) OR (title = 'Professor') AND (age < 100))"
        );
    }
    #[test]
    fn test_filter_bracketed_with_cond_macro_nested() {
        let age = Field::new("age");
        let title = Field::new("title");

        let filter = cond!((age OR cond!(age >= 18)) OR (title = "Professor") AND (age < 100));

        let bracketed_filter = filter.parenthesized();
        assert_eq!(
            bracketed_filter.to_raw().build(),
            "((age OR age >= 18) OR (title = 'Professor') AND (age < 100))"
        );
    }

    #[test]
    fn test_filter_bracketed_with_cond_macro_mixed() {
        let age = Field::new("age");
        let title = Field::new("title");

        let filter = cond!((age.or(4).or(545).or(232)) OR (title = "Professor") AND (age < 100));

        let bracketed_filter = filter.parenthesized();
        assert_eq!(
            bracketed_filter.fine_tune_params(),
            "((age OR $_param_00000001 OR $_param_00000002 OR $_param_00000003) OR (title = $_param_00000004) AND (age < $_param_00000005))"
        );

        assert_eq!(
            bracketed_filter.to_raw().build(),
            "((age OR 4 OR 545 OR 232) OR (title = 'Professor') AND (age < 100))"
        );
    }
}
