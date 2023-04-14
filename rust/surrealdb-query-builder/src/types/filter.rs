/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::{borrow::Cow, fmt::Display};

use crate::traits::{
    Binding, BindingsList, Buildable, Conditional, Erroneous, Operatable, Operation, Parametric,
};

use super::Empty;

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
///
/// # Examples
///
/// ```
/// use crate::query::filter::{Filter, cond};
///
/// let filter1 = Filter::new("name = 'John'".to_string());
/// let filter2 = Filter::new("age > 18".to_string());
///
/// // Combine two filters using the 'AND' operator
/// let combined_filter = filter1.and(filter2);
///
/// assert_eq!(combined_filter.to_string(), "(name = 'John') AND (age > 18)");
///
/// // Create a filter from a string
/// let filter3 = Filter::new("name like '%Doe%'".to_string());
///
/// // Combine multiple filters using the 'OR' operator
/// let all_filters = cond(filter1).or(filter2).or(filter3);
///
/// assert_eq!(all_filters.to_string(), "(name = 'John') OR (age > 18) OR (name like '%Doe%')");
/// ```
#[derive(Debug, Clone, Default)]
pub struct Filter {
    query_string: String,
    bindings: BindingsList,
}

impl Buildable for Filter {
    fn build(&self) -> String {
        self.query_string.to_string()
    }
}

impl Conditional for Filter {}

impl From<Empty> for Filter {
    fn from(value: Empty) -> Self {
        Filter::new(Empty)
    }
}

impl Parametric for Filter {
    fn get_bindings(&self) -> BindingsList {
        self.bindings.to_vec()
    }
}
///
/// Creates a new filter from a given `filterable` input.
///
/// # Arguments
///
/// * `filterable` - A value that can be converted into a `Filter`.
///
/// # Example
///
/// ```
/// use crate::query::filter::{Filter, cond};
///
/// let filter = Filter::new("name = 'John'".to_string());
///
/// let combined_filter = cond(filter).and("age > 18");
///
/// assert_eq!(combined_filter.to_string(), "(name = 'John') AND (age > 18)");
/// ```
pub fn cond(filterable: impl Conditional) -> Filter {
    Filter::new(filterable)
}

// /// Creates an empty filter.
// ///
// /// # Example
// ///
// /// ```
// /// use crate::query::filter::Filter;
// ///
// /// let empty_filter = Filter::empty();
// ///
// /// assert_eq!(empty_filter.to_string(), "");
// ///
// pub fn empty() -> Empty {
//     Empty
// }

impl Erroneous for Filter {
    fn get_errors(&self) -> Vec<String> {
        vec![]
    }
}

impl Filter {
    /// Creates a new `Filter` instance.
    ///
    /// # Arguments
    ///
    /// * `query_string` - The query string used to initialize the filter.
    ///
    /// # Example
    ///
    /// ```
    /// use crate::query::filter::Filter;
    ///
    /// let filter = Filter::new("name = 'John'".to_string());
    ///
    /// assert_eq!(filter.to_string(), "name = 'John'");
    /// ```
    // pub fn new(query_string: String) -> Self {
    pub fn new(query: impl Conditional) -> Self {
        let query_string = format!("{}", query.build());
        // let query_string = if query_string.is_empty() {
        //     "".into()
        // } else {
        //     format!("({query_string})")
        // };

        Self {
            query_string,
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
    /// ```
    /// use crate::query::filter::{Filter, cond};
    ///
    /// let filter = cond(Filter::new("name = 'John'".to_string())).or(
    ///     cond(Filter::new("age > 30".to_string()))
    /// );
    ///
    /// assert_eq!(filter.to_string(), "(name = 'John') OR (age > 30)");
    /// ```
    pub fn or(self, filter: Operation) -> Self {
        let precendence = self._______bracket_if_not_already();
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
    /// ```
    /// use crate::query::filter::{Filter, cond};
    ///
    /// let filter1 = cond(Filter::new("name = 'John'"));
    /// let filter2 = cond(Filter::new("age > 30"));
    /// let combined = filter1.and(filter2);
    ///
    /// assert_eq!(combined.to_string(), "(name = 'John') AND (age > 30)");
    /// ```
    pub fn and(self, filter: impl Buildable + Parametric) -> Self {
        let precendence = self._______bracket_if_not_already();
        let new_params = self.___update_bindings(&filter);

        let ref filter = filter.build();
        let query_string = format!("{precendence} AND ({filter})");

        Filter {
            query_string,
            bindings: new_params,
        }
    }

    pub(crate) fn ___update_bindings(self, filter: &impl Parametric) -> Vec<Binding> {
        // let new_params = self
        //     .params
        //     .to_owned()
        //     .into_iter()
        //     .chain(filter.get_params().into_iter())
        //     .collect::<Vec<_>>(); // Consumed
        // let mut new_bindings = vec![];
        // new_bindings.extend(self.bindings);
        // new_bindings.extend(filter.get_bindings());
        // new_bindings
        [self.bindings.as_slice(), filter.get_bindings().as_slice()].concat()
    }

    /// Wraps this `Filter` instance in a set of brackets.
    ///
    /// # Example
    ///
    /// ```
    /// use crate::query::filter::{Filter, cond};
    ///
    /// let filter = cond(Filter::new("name = 'John'")).or(cond(Filter::new("age > 30")));
    /// let wrapped = filter.bracketed();
    ///
    /// assert_eq!(wrapped.to_string(), "((name = 'John') OR (age > 30))");
    /// ```
    pub fn bracketed(&self) -> Self {
        Filter {
            query_string: format!("({self})"),
            bindings: self.bindings.to_vec(),
        }
    }

    /// Wraps this `Filter` instance in a set of brackets if it isn't already wrapped.
    fn _______bracket_if_not_already(&self) -> impl Display {
        let filter = self.to_string();
        match (filter.starts_with('('), filter.ends_with(')')) {
            (true, true) => format!("{self}"),
            _ => format!("({self})"),
        }
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

impl std::fmt::Display for Filter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.query_string))
    }
}
