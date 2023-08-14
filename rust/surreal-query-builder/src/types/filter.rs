/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
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
            query_string: query.build(),
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

        let ref filter = filter.build();
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

        let ref filter = filter.build();
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
    pub fn bracketed(&self) -> Self {
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
    use crate::{Field, Operatable, ToRaw};

    use super::*;

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
    fn test_filter_bracketed() {
        let age = Field::new("age");
        let title = Field::new("title");

        let filter = cond(age.greater_than(18))
            .or(title.equal("Professor"))
            .and(age.less_than(100));

        let bracketed_filter = filter.bracketed();
        assert_eq!(
            bracketed_filter.to_raw().build(),
            "((age > 18) OR (title = 'Professor') AND (age < 100))"
        );
    }
}
