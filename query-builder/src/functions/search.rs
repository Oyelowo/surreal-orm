/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */

// These functions are used in conjunction with the 'matches' operator to either collect the relevance score or highlight the searched keywords within the content.
//
// Function	Description
// search::score()	Returns the relevance score
// search::highlight()	Highlights the matching keywords
// search::offsets()	Returns the position of the matching keywords

use crate::{Buildable, Erroneous, Function, NumberLike, Parametric, StrandLike};

fn create_single_search_arg_helper(search_arg: impl Into<StrandLike>, func_name: &str) -> Function {
    let search_arg: StrandLike = search_arg.into();
    let mut bindings = vec![];
    let mut errors = vec![];
    bindings.extend(search_arg.get_bindings());
    errors.extend(search_arg.get_errors());
    Function {
        query_string: format!("search::{}({})", func_name, search_arg.build()),
        bindings,
        errors,
    }
}

macro_rules! create_fn_with_single_search_arg {
    ($(#[$attr:meta])* => $function_name:expr, $function_path:expr) => {
        paste::paste! {
            $(#[$attr])*
            pub fn [<$function_name _fn>](search_arg: impl ::std::convert::Into<$crate::StrandLike>) -> $crate::Function {
                create_single_search_arg_helper(search_arg, $function_path)
            }

            $(#[$attr])*
            #[macro_export]
            macro_rules! [<search_ $function_name>] {
                ( $search_arg:expr ) => {
                    $crate::functions::search::[<$function_name _fn>]($search_arg)
                };
            }
            pub use [<search_ $function_name>] as [<$function_name>];

            #[cfg(test)]
            mod [<test_ $function_name>] {
                use $crate::{functions::search, *};

                #[test]
                fn [<test $function_name _fn>]() {
                    let field = Field::new("field");
                    let result = functions::search::[<$function_name _fn>](field);
                    assert_eq!(
                        result.to_raw().build(),
                        format!("search::{}(field)", $function_path)
                    );
                }

                #[test]
                fn [<test $function_name _macro>]() {
                    let field = Field::new("field");
                    let result = search::[<$function_name>]!(field);
                    assert_eq!(
                        result.to_raw().build(),
                        format!("search::{}(field)", $function_path)
                    );
                }
            }
        }
    };
}

create_fn_with_single_search_arg!(
    /// The search::score function returns the relevance score corresponding to the given 'matches' predicate reference numbers.
    ///
    /// # Arguments
    /// * `search_arg` - The search argument to compute the score of. Could be a field or a parameter that represents the value.
    ///
    /// # Example
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// use surreal_orm::{*, functions::search};
    ///
    /// let title = Field::new("title");
    /// let result = search::score!(title);
    /// assert_eq!(result.to_raw().build(), "search::score(title)");
    /// ```
    =>
    "score",
    "score"
);

create_fn_with_single_search_arg!(
    /// The search::offsets function returns the position of the matching keywords for the predicate reference number.
    ///
    /// # Arguments
    /// * `search_arg` - The search argument to find offsets for. Could be a field or a parameter that represents the value.
    ///
    /// # Example
    /// ```rust
    /// # use surreal_query_builder as surreal_orm;
    /// use surreal_orm::{*, functions::search};
    ///
    /// let title = Field::new("title");
    /// let result = search::offsets!(title);
    /// assert_eq!(result.to_raw().build(), "search::offsets(title)");
    /// ```
    =>
    "offsets",
    "offsets"
);

/// The search_highlight_fn function highlights the matching keywords for the predicate reference number.
pub fn highlight_fn(
    str1: impl Into<StrandLike>,
    str2: impl Into<StrandLike>,
    predicate_ref_number: impl Into<NumberLike>,
) -> Function {
    let str1: StrandLike = str1.into();
    let str2: StrandLike = str2.into();
    let predicate_ref_number: NumberLike = predicate_ref_number.into();
    let mut bindings = vec![];
    let mut errors = vec![];
    bindings.extend(str1.get_bindings());
    bindings.extend(str2.get_bindings());
    bindings.extend(predicate_ref_number.get_bindings());
    errors.extend(str1.get_errors());
    errors.extend(str2.get_errors());
    errors.extend(predicate_ref_number.get_errors());

    Function {
        query_string: format!(
            "search::highlight({}, {}, {})",
            str1.build(),
            str2.build(),
            predicate_ref_number.build()
        ),
        bindings,
        errors,
    }
}

/// The search::highlight function highlights the matching keywords for the predicate reference number.
///
/// # Arguments
///
/// * `str1` - The string to use as the start of the highlight.
/// * `str2` - The string to use as the end of the highlight.
/// * `predicate_ref_number` - The predicate reference number to highlight.
///     All arguments could be a field or a parameter that represents the value.
///
/// # Example
/// ```rust
/// # use surreal_query_builder as surreal_orm;
/// use surreal_orm::{*, functions::search};
/// let result = search::highlight!("<b>", "</b>", 1);
/// assert_eq!(result.to_raw().build(), "search::highlight('<b>', '</b>', 1)");
/// ```
#[macro_export]
macro_rules! search_highlight {
    ( $str1:expr, $str2:expr, $predicate_ref_number:expr ) => {
        $crate::functions::search::highlight_fn($str1, $str2, $predicate_ref_number)
    };
}
pub use search_highlight as highlight;

#[cfg(test)]
mod test_search_highlight {
    use crate::{functions::search, *};

    // seach::highlight('<b>', '</b>', 1);

    #[test]
    fn test_search_highlight_fn_with_strands() {
        let str1 = "<b>";
        let str2 = "</b>";
        let predicate_ref_number = 1;
        let result = search::highlight_fn(str1, str2, predicate_ref_number);
        assert_eq!(
            result.to_raw().build(),
            "search::highlight('<b>', '</b>', 1)"
        );
    }

    #[test]
    fn test_search_highlight_macro_with_strands() {
        let str1 = "<b>";
        let str2 = "</b>";
        let predicate_ref_number = 1;
        let result = search::highlight!(str1, str2, predicate_ref_number);
        assert_eq!(
            result.to_raw().build(),
            "search::highlight('<b>', '</b>', 1)"
        );
    }

    #[test]
    fn test_search_highlight_fn_with_fields() {
        let str1 = Field::new("str1");
        let str2 = Field::new("str2");
        let predicate_ref_number = Field::new("predicate_ref_number");
        let result = search::highlight_fn(str1, str2, predicate_ref_number);
        assert_eq!(
            result.to_raw().build(),
            "search::highlight(str1, str2, predicate_ref_number)"
        );
    }

    #[test]
    fn test_search_highlight_macro() {
        let str1 = Field::new("str1");
        let str2 = Field::new("str2");
        let predicate_ref_number = Field::new("predicate_ref_number");
        let result = search::highlight!(str1, str2, predicate_ref_number);
        assert_eq!(
            result.to_raw().build(),
            "search::highlight(str1, str2, predicate_ref_number)"
        );
    }

    #[test]
    fn test_search_highlight_fn_with_parameters() {
        let str1 = Param::new("str1");
        let str2 = Param::new("str2");
        let predicate_ref_number = Param::new("predicate_ref_number");
        let result = search::highlight_fn(str1, str2, predicate_ref_number);
        assert_eq!(
            result.to_raw().build(),
            "search::highlight($str1, $str2, $predicate_ref_number)"
        );
    }

    #[test]
    fn test_search_highlight_macro_with_parameters() {
        let str1 = Param::new("str1");
        let str2 = Param::new("str2");
        let predicate_ref_number = Param::new("predicate_ref_number");
        let result = search::highlight!(str1, str2, predicate_ref_number);
        assert_eq!(
            result.to_raw().build(),
            "search::highlight($str1, $str2, $predicate_ref_number)"
        );
    }
}
