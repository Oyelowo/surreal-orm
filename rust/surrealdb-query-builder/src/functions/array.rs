/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

// array::combine()	Combines all values from two arrays together
// array::concat()	Returns the merged values from two arrays
// array::difference()	Returns the difference between two arrays
// array::distinct()	Returns the unique items in an array
// array::intersect()	Returns the values which intersect two arrays
// array::len()	Returns the length of an array
// array::sort()	Sorts the values in an array in ascending or descending order
// array::sort::asc()	Sorts the values in an array in ascending order
// array::sort::desc()	Sorts the values in an array in descending order
// array::union()
// struct Function(String);

use std::fmt::Display;

use crate::{ArrayLike, Field, Function};
use crate::{Binding, BindingsList, Buildable, Parametric, ToRaw};

fn create_array_helper(
    arr1: impl Into<ArrayLike>,
    arr2: impl Into<ArrayLike>,
    func_name: &str,
) -> Function {
    let arr1: ArrayLike = arr1.into();
    let arr2: ArrayLike = arr2.into();
    let mut bindings = vec![];
    bindings.extend(arr1.get_bindings());
    bindings.extend(arr2.get_bindings());
    Function {
        query_string: format!("array::{func_name}({}, {})", arr1.build(), arr2.build()),
        bindings,
    }
}

macro_rules! create_fn_with_two_array_args {
    ($(#[$attr:meta])* => $function_name:expr) => {
        paste::paste! {
            $(#[$attr])*
            pub fn [<$function_name _fn>](arr1: impl Into<$crate::ArrayLike>, arr2: impl Into<$crate::ArrayLike>) -> $crate::Function {
                create_array_helper(arr1, arr2, $function_name)
            }

            $(#[$attr])*
            #[macro_export]
            macro_rules! [<array_ $function_name>] {
                ( $arr1:expr, $arr2:expr ) => {
                    $crate::functions::array::[<$function_name _fn>]($arr1, $arr2)
                };
            }
            pub use [<array_ $function_name>] as [<$function_name>];

            #[cfg(test)]
            mod [<test_ $function_name>] {
                use super::*;

                #[test]
                fn [<test $function_name fn_on_array_macro_on_diverse_array>]() {
                    let age = Field::new("age");
                    let arr1 = $crate::arr![1, "Oyelowo", age];
                    let arr2 = $crate::arr![4, "dayo", 6];
                    let result = $crate::functions::array::[<$function_name _fn>](arr1, arr2);
                    assert_eq!(
                        result.fine_tune_params(),
                        format!("array::{}([$_param_00000001, $_param_00000002, age], [$_param_00000003, $_param_00000004, $_param_00000005])", $function_name)
                    );
                    assert_eq!(
                        result.to_raw().build(),
                        format!("array::{}([1, 'Oyelowo', age], [4, 'dayo', 6])", $function_name)
                    );
                }

                #[test]
                fn [<test $function_name _fn_on_same_element_types>]() {
                    let arr1 = $crate::arr![1, 2, 3];
                    let arr2 = $crate::arr![4, 5, 6];
                    let result = $crate::functions::array::[<$function_name _fn>](arr1, arr2);
                    assert_eq!(
                        result.fine_tune_params(),
                        format!("array::{}([$_param_00000001, $_param_00000002, $_param_00000003], \
                            [$_param_00000004, $_param_00000005, $_param_00000006])", $function_name)
                    );

                    assert_eq!(
                        result.to_raw().build(),
                        format!("array::{}([1, 2, 3], [4, 5, 6])", $function_name)
                    );
                }

                #[test]
                fn [<test $function_name _macro_on_array_macro_on_diverse_array>]() {
                    let age = Field::new("age");
                    let arr1 = $crate::arr![1, "Oyelowo", age];
                    let arr2 = $crate::arr![4, "dayo", 6];
                    let result = $crate::functions::array::[<$function_name>]!(arr1, arr2);
                    assert_eq!(
                        result.fine_tune_params(),
                        format!("array::{}([$_param_00000001, $_param_00000002, age], \
                            [$_param_00000003, $_param_00000004, $_param_00000005])", $function_name)
                    );
                    assert_eq!(
                        result.to_raw().build(),
                        format!("array::{}([1, 'Oyelowo', age], [4, 'dayo', 6])", $function_name)
                    );
                }

                #[test]
                fn [<test $function_name _macro_on_same_element_types>]() {
                    let arr1 = $crate::arr![1, 2, 3];
                    let arr2 = $crate::arr![4, 5, 6];
                    let result = $crate::functions::array::[<$function_name>]!(arr1, arr2);
                    assert_eq!(
                        result.fine_tune_params(),
                        format!("array::{}([$_param_00000001, $_param_00000002, $_param_00000003], \
                            [$_param_00000004, $_param_00000005, $_param_00000006])", $function_name)
                    );

                    assert_eq!(
                        result.to_raw().build(),
                        format!("array::{}([1, 2, 3], [4, 5, 6])", $function_name)
                    );
                }

                #[test]
                fn [<test $function_name _macro_on_fields>]() {
                    let students_ages = Field::new("students_ages");
                    let teachers_ages = Field::new("teachers_ages");
                    let result = $crate::functions::array::[<$function_name>]!(students_ages, teachers_ages);
                    assert_eq!(
                        result.fine_tune_params(),
                        format!("array::{}(students_ages, teachers_ages)", $function_name)
                    );

                    assert_eq!(
                        result.to_raw().build(),
                        format!("array::{}(students_ages, teachers_ages)", $function_name)
                    );
                }
        }
        }
    };
}

/// Creates a function that returns the elements of an array that are not present in another array.
create_fn_with_two_array_args!(
    /// "The array::combine function combines all values from two arrays together, returning an array of arrays.
    ///
    ///   # Arguments
    ///   * `arr1` - The first array to combine. A vector, field or param.
    ///   * `arr2` - The second array to combine. A vector, field or param.
    ///
    /// # Examples
    /// ```rust
    /// # use surrealdb_query_builder as  surrealdb_orm;
    /// use surrealdb_orm::{*, functions::array};
    /// let own_goals = Field::new("own_goals");
    /// let goals = Param::new("goals");
    ///
    /// array::combine!(vec![1, 2, 3], vec![4, 5, 6]);
    /// array::combine!(own_goals, goals);
    /// array::combine!(&[1, 2, 3], &[4, 5, 6]);
    ///
    /// // It is also aliased as array_combine;
    /// array_combine!(&[1, 2, 3], &[4, 5, 6]);
    /// ```
=>
    "combine"
);

create_fn_with_two_array_args!(
    /// The array::concat function merges two arrays together, returning an array which may contain duplicate values. If you want to remove duplicate values from the resulting array, then use the array::union() function
    ///
    ///   # Arguments
    ///   * `arr1` - The first array to concat. A vector, field or param.
    ///   * `arr2` - The second array to concat. A vector, field or param.
    /// # Examples
    /// ```rust
    /// # use surrealdb_query_builder as  surrealdb_orm;
    /// use surrealdb_orm::{*, functions::array};
    /// let own_goals = Field::new("own_goals");
    /// let goals = Param::new("goals");
    ///
    /// array::concat!(vec![1, 2, 3], vec![4, 5, 6]);
    /// array::concat!(own_goals, goals);
    /// array::concat!(&[1, 2, 3], &[4, 5, 6]);
    ///
    /// // It is also aliased as array_concat;
    /// array_concat!(&[1, 2, 3], &[4, 5, 6]);
    /// ```
    =>
    "concat"
);

create_fn_with_two_array_args!(
    /// The array::union function combines two arrays together, removing duplicate values, and returning a single array.
    /// # Examples
    ///
    /// # Arguments
    ///  * `arr1` -  A vector, field or param.
    ///  * `arr2` -  A vector, field or param.
    ///
    /// ```rust
    /// # use surrealdb_query_builder as  surrealdb_orm;
    /// use surrealdb_orm::{*, functions::array};
    /// let own_goals = Field::new("own_goals");
    /// let goals = Param::new("goals");
    ///
    /// array::union!(vec![1, 2, 3], vec![4, 5, 6]);
    /// array::union!(own_goals, goals);
    /// array::union!(&[1, 2, 3], &[4, 5, 6]);
    /// // It is also aliased as array_union;
    /// array_union!(&[1, 2, 3], &[4, 5, 6]);
    /// ```
    =>
    "union"
);

create_fn_with_two_array_args!(
    /// The array::difference determines the difference between two arrays, returning a single array
    /// containing items which are not in both arrays.
    ///
    /// # Arguments
    /// * `arr1` -  A vector, field or param.
    /// * `arr2` -  A vector, field or param.
    ///
    /// # Examples
    /// ```rust
    /// # use surrealdb_query_builder as  surrealdb_orm;
    /// use surrealdb_orm::{*, functions::array};
    /// let own_goals = Field::new("own_goals");
    /// let goals = Param::new("goals");
    ///
    /// array::difference!(vec![1, 2, 3], vec![4, 5, 6]);
    /// array::difference!(own_goals, goals);
    /// array::difference!(&[1, 2, 3], &[4, 5, 6]);
    ///
    /// // It is also aliased as array_difference;
    /// array_difference!(&[1, 2, 3], &[4, 5, 6]);
    /// ```
    =>
    "difference"
);

create_fn_with_two_array_args!(
    /// The array::intersect function calculates the values which intersect two arrays, returning a
    /// single array containing the values which are in both arrays.
    ///
    /// # Arguments
    /// * `arr1` -  A vector, field or param.
    /// * `arr2` -  A vector, field or param.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use surrealdb_query_builder as  surrealdb_orm;
    /// use surrealdb_orm::{*, functions::array};
    /// let own_goals = Field::new("own_goals");
    /// let goals = Param::new("goals");
    ///
    /// array::intersect!(vec![1, 2, 3], vec![4, 5, 6]);
    /// array::intersect!(own_goals, goals);
    /// array::intersect!(&[1, 2, 3], &[4, 5, 6]);
    ///
    /// // It is also aliased as array_intersect;
    /// array_intersect!(&[1, 2, 3], &[4, 5, 6]);
    /// ```
    =>
    "intersect"
);

/// The array::distinct function calculates the unique values in an array, returning a single array.
pub fn distinct_fn(arr: impl Into<ArrayLike>) -> Function {
    let arr: ArrayLike = arr.into();

    Function {
        query_string: format!("array::distinct({})", arr.build()),
        bindings: arr.get_bindings(),
    }
}

/// The array::distinct function calculates the unique values in an array, returning a single
/// array.
/// # Arguments
/// * `arr` -  A vector, field or param.
///
/// # Examples
/// ```rust
/// # use surrealdb_query_builder as  surrealdb_orm;
/// use surrealdb_orm::{*, functions::array};
/// let own_goals = Field::new("own_goals");
/// let goals = Param::new("goals");
///
/// array::distinct!(vec![1, 2, 3]);
/// array::distinct!(&[1, 2, 3]);
/// array::distinct!(own_goals);
/// array::distinct!(goals);
///
/// // It is also aliased as array_distinct;
/// array_distinct!(vec![1, 2, 3]);
/// ```
#[macro_export]
macro_rules! array_distinct {
    ( $arr:expr ) => {
        $crate::functions::array::distinct_fn($arr)
    };
}
pub use array_distinct as distinct;

/// The array::len function calculates the length of an array, returning a number. This function
/// includes all items when counting the number of items in the array. If you want to only count
/// truthy values, then use the count() function.
pub fn len_fn(arr: impl Into<ArrayLike>) -> Function {
    let arr: ArrayLike = arr.into();

    Function {
        query_string: format!("array::len({})", arr.build()),
        bindings: arr.get_bindings(),
    }
}

/// The array::len function calculates the length of an array, returning a number.
/// This function includes all items when counting the number of items in the array.
/// If you want to only count truthy values, then use the count() function.
///
/// # Arguments
/// * `arr` -  A vector, field or param.
///
/// # Examples
/// ```rust
/// # use surrealdb_query_builder as  surrealdb_orm;
/// use surrealdb_orm::{*, functions::array};
/// let own_goals = Field::new("own_goals");
/// let goals = Param::new("goals");
///
/// array::len!(vec![1, 2, 3]);
/// array::len!(&[1, 2, 3]);
/// array::len!(own_goals);
/// array::len!(goals);
/// // It is also aliased as array_len;
/// array_len!(vec![1, 2, 3]);
/// ```
#[macro_export]
macro_rules! array_len {
    ( $arr:expr ) => {
        $crate::functions::array::len_fn($arr)
    };
}
pub use array_len as len;

/// The ordering of the array.
pub enum Ordering {
    /// Sort the array in ascending order.
    Asc,
    /// Sort the array in descending order.
    Desc,
    /// Do not sort the array.
    False,
    /// Do not sort the array.
    Empty,
}

impl Display for Ordering {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Ordering::Asc => "'asc'",
                Ordering::Desc => "'desc'",
                Ordering::False => "false",
                Ordering::Empty => "",
            }
        )
    }
}

/// The array::sort function calculates the length of an array, returning a number. This function
/// includes all items when counting the number of items in the array. If you want to only count
/// truthy values, then use the count() function.
pub fn sort_fn(arr: impl Into<ArrayLike>, ordering: Ordering) -> Function {
    let arr: ArrayLike = arr.into();
    let query_string = match ordering {
        Ordering::Empty => format!("array::sort({})", arr.build()),
        _ => format!("array::sort({}, {ordering})", arr.build()),
    };
    Function {
        query_string,
        bindings: arr.get_bindings(),
    }
}
/// The array::sort function calculates the length of an array, returning a number. This function
/// includes all items when counting the number of items in the array. If you want to only count
/// truthy values, then use the count() function.
///
/// # Arguments
/// * `arr` -  A vector, field or param.
/// * `ordering` -  The ordering of the array.
/// # Examples
/// ```rust
/// # use surrealdb_query_builder as  surrealdb_orm;
/// use surrealdb_orm::{*, functions::array};
/// let own_goals = Field::new("own_goals");
/// let goals = Param::new("goals");
///
/// array::sort!(vec![1, 2, 3], "asc");
/// array::sort!(&[1, 2, 3], "desc");
/// array::sort!(own_goals, false);
/// array::sort!(goals, array::Ordering::Empty);
/// // It is also aliased as array_sort;
/// array_sort!(vec![1, 2, 3], "asc");
/// ```
#[macro_export]
macro_rules! array_sort {
    ( $arr:expr, "asc" ) => {
        $crate::functions::array::sort_fn($arr, $crate::functions::array::Ordering::Asc)
    };
    ( $arr:expr, "desc" ) => {
        $crate::functions::array::sort_fn($arr, $crate::functions::array::Ordering::Desc)
    };
    ( $arr:expr, false ) => {
        $crate::functions::array::sort_fn($arr, $crate::functions::array::Ordering::False)
    };
    ( $arr:expr, $ordering:expr ) => {
        $crate::functions::array::sort_fn($arr, $ordering)
    };
    ( $arr:expr ) => {
        $crate::functions::array::sort_fn($arr, $crate::functions::array::Ordering::Empty)
    };
}
pub use array_sort as sort;

pub mod sort {
    use surrealdb::sql;

    use crate::{traits::Binding, types::ArrayLike, Buildable, Parametric};

    use super::Function;

    /// The array::sort::asc function is a shorthand convenience function for the array::sort function, to sort values in an array in ascending order.
    pub fn asc_fn(arr: impl Into<ArrayLike>) -> Function {
        let arr: ArrayLike = arr.into();

        Function {
            query_string: format!("array::sort::asc({})", arr.build()),
            bindings: arr.get_bindings(),
        }
    }

    /// The array::sort::asc function is a shorthand convenience function for the array::sort function, to sort values in an array in ascending order.
    /// # Arguments
    /// * `arr` -  A vector, field or param.
    /// # Examples
    /// ```rust
    /// # use surrealdb_query_builder as  surrealdb_orm;
    /// use surrealdb_orm::{*, functions::array};
    /// let own_goals = Field::new("own_goals");
    /// let goals = Param::new("goals");
    /// array::sort::asc!(vec![1, 2, 3]);
    /// array::sort::asc!(&[1, 2, 3]);
    /// array::sort::asc!(own_goals);
    /// array::sort::asc!(goals);
    ///
    /// // It is also aliased as array_sort_asc;
    /// array_sort_asc!(vec![1, 2, 3]);
    /// ```
    #[macro_export]
    macro_rules! array_sort_asc {
        ( $arr:expr ) => {
            $crate::functions::array::sort::asc_fn($arr)
        };
    }
    pub use array_sort_asc as asc;

    /// The array::sort::desc function is a shorthand convenience function for the array::sort
    /// function, to sort values in an array in descending order.
    pub fn desc_fn(arr: impl Into<ArrayLike>) -> Function {
        let arr: ArrayLike = arr.into();

        Function {
            query_string: format!("array::sort::desc({})", arr.build()),
            bindings: arr.get_bindings(),
        }
    }

    /// The array::sort::desc function is a shorthand convenience function for the array::sort function, to sort values in an array in descending order.
    /// # Arguments
    /// * `arr` -  A vector, field or param.
    /// # Examples
    /// ```rust
    /// # use surrealdb_query_builder as  surrealdb_orm;
    /// use surrealdb_orm::{*, functions::array};
    /// let own_goals = Field::new("own_goals");
    /// let goals = Param::new("goals");
    /// array::sort::desc!(vec![1, 2, 3]);
    /// array::sort::desc!(&[1, 2, 3]);
    /// array::sort::desc!(own_goals);
    /// array::sort::desc!(goals);
    /// // It is also aliased as array_sort_desc;
    /// array_sort_desc!(vec![1, 2, 3]);
    /// ```
    #[macro_export]
    macro_rules! array_sort_desc {
        ( $arr:expr ) => {
            $crate::functions::array::sort::desc_fn($arr)
        };
    }
    pub use array_sort_desc as desc;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::arr;

    #[test]
    fn test_distinct() {
        let arr = arr![1, 2, 31];
        let result = distinct_fn(arr);

        assert_eq!(
            result.fine_tune_params(),
            "array::distinct([$_param_00000001, $_param_00000002, $_param_00000003])"
        );
        assert_eq!(result.to_raw().build(), "array::distinct([1, 2, 31])");
    }

    #[test]
    fn test_distinct_macro_with_vec_parameterization() {
        let arr = vec![1, 2, 3, 3, 2, 1];
        let result = distinct!(arr);
        assert_eq!(
            result.fine_tune_params(),
            "array::distinct($_param_00000001)"
        );
        assert_eq!(
            result.to_raw().to_string(),
            "array::distinct([1, 2, 3, 3, 2, 1])"
        );
    }

    #[test]
    fn test_distinct_macro_with_mixed_array_parametization() {
        let age = Field::new("age");
        let arr = arr![1, 2, 3, age];
        let result = distinct!(arr);
        assert_eq!(
            result.fine_tune_params(),
            "array::distinct([$_param_00000001, $_param_00000002, $_param_00000003, age])"
        );
        assert_eq!(
            result.to_raw().to_string(),
            "array::distinct([1, 2, 3, age])"
        );
    }

    #[test]
    fn test_len_on_diverse_array_custom_array_function() {
        let email = Field::new("email");
        let arr = arr![1, 21, "4334", "Oyelowo", email];
        let result = len_fn(arr);
        assert_eq!(
        result.fine_tune_params(),
        "array::len([$_param_00000001, $_param_00000002, $_param_00000003, $_param_00000004, email])"
    );
        assert_eq!(
            result.to_raw().build(),
            "array::len([1, 21, '4334', 'Oyelowo', email])"
        );
    }

    #[test]
    fn test_len_macro_on_diverse_array_custom_array_function() {
        let email = Field::new("email");
        let arr = arr![1, 21, "4334", "Oyelowo", email];
        let result = len!(arr);
        assert_eq!(result.fine_tune_params(),
        "array::len([$_param_00000001, $_param_00000002, $_param_00000003, $_param_00000004, email])");
        assert_eq!(
            result.to_raw().to_string(),
            "array::len([1, 21, '4334', 'Oyelowo', email])"
        );
    }

    #[test]
    fn test_sort() {
        // Mono vector should use a single param
        let arr = vec![3, 2, 1];
        let result = sort_fn(arr.clone(), Ordering::Asc);
        assert_eq!(
            result.fine_tune_params(),
            "array::sort($_param_00000001, 'asc')"
        );
        assert_eq!(result.to_raw().build(), "array::sort([3, 2, 1], 'asc')");

        let result = sort_fn(arr.clone(), Ordering::Desc);
        assert_eq!(
            result.fine_tune_params(),
            "array::sort($_param_00000001, 'desc')"
        );
        assert_eq!(result.to_raw().build(), "array::sort([3, 2, 1], 'desc')");

        let result = sort_fn(arr.clone(), Ordering::Empty);
        assert_eq!(result.fine_tune_params(), "array::sort($_param_00000001)");
        assert_eq!(result.to_raw().build(), "array::sort([3, 2, 1])");

        let result = sort_fn(arr.clone(), Ordering::False);
        assert_eq!(
            result.fine_tune_params(),
            "array::sort($_param_00000001, false)"
        );
        assert_eq!(result.to_raw().build(), "array::sort([3, 2, 1], false)");
    }

    #[test]
    fn test_sort_macro_ordering_type() {
        let arr = vec![3, 2, 1];
        let result = sort!(arr.clone(), Ordering::Asc);
        assert_eq!(
            result.fine_tune_params(),
            "array::sort($_param_00000001, 'asc')"
        );
        assert_eq!(result.to_raw().build(), "array::sort([3, 2, 1], 'asc')");

        let result = sort!(arr.clone(), Ordering::Desc);
        assert_eq!(
            result.fine_tune_params(),
            "array::sort($_param_00000001, 'desc')"
        );
        assert_eq!(result.to_raw().build(), "array::sort([3, 2, 1], 'desc')");

        let result = sort!(arr.clone(), Ordering::Empty);
        assert_eq!(result.fine_tune_params(), "array::sort($_param_00000001)");
        assert_eq!(result.to_raw().build(), "array::sort([3, 2, 1])");

        let result = sort!(arr.clone(), Ordering::False);
        assert_eq!(
            result.fine_tune_params(),
            "array::sort($_param_00000001, false)"
        );
        assert_eq!(result.to_raw().build(), "array::sort([3, 2, 1], false)");
    }

    #[test]
    fn test_sort_macro() {
        let arr = vec![3, 2, 1];
        let result = sort!(arr.clone(), "asc");
        assert_eq!(
            result.fine_tune_params(),
            "array::sort($_param_00000001, 'asc')"
        );
        assert_eq!(result.to_raw().to_string(), "array::sort([3, 2, 1], 'asc')");

        let result = sort!(arr.clone(), "desc");
        assert_eq!(
            result.fine_tune_params(),
            "array::sort($_param_00000001, 'desc')"
        );
        assert_eq!(result.to_raw().build(), "array::sort([3, 2, 1], 'desc')");

        // Without ordering
        let result = sort!(arr.clone());
        assert_eq!(result.fine_tune_params(), "array::sort($_param_00000001)");
        assert_eq!(result.to_raw().build(), "array::sort([3, 2, 1])");

        let result = sort!(arr.clone(), false);
        assert_eq!(
            result.fine_tune_params(),
            "array::sort($_param_00000001, false)"
        );
        assert_eq!(result.to_raw().build(), "array::sort([3, 2, 1], false)");
    }

    #[test]
    fn test_sort_asc() {
        let arr = vec![3, 2, 1];
        let result = sort::asc_fn(arr);
        assert_eq!(
            result.fine_tune_params(),
            "array::sort::asc($_param_00000001)"
        );
        assert_eq!(result.to_raw().build(), "array::sort::asc([3, 2, 1])");
    }

    #[test]
    fn test_sort_asc_macro() {
        let arr = arr![3, 2, 1];
        let result = sort::asc!(arr);
        assert_eq!(
            result.fine_tune_params(),
            "array::sort::asc([$_param_00000001, $_param_00000002, $_param_00000003])"
        );
        assert_eq!(result.to_raw().build(), "array::sort::asc([3, 2, 1])");
    }

    #[test]
    fn test_sort_desc() {
        let arr = vec![3, 2, 1];
        let result = sort::desc_fn(arr);
        assert_eq!(
            result.fine_tune_params(),
            "array::sort::desc($_param_00000001)"
        );
        assert_eq!(result.to_raw().build(), "array::sort::desc([3, 2, 1])");
    }

    #[test]
    fn test_sort_desc_macro() {
        let arr = arr![3, 2, 1];
        let result = sort::desc!(arr);
        assert_eq!(
            result.fine_tune_params(),
            "array::sort::desc([$_param_00000001, $_param_00000002, $_param_00000003])"
        );
        assert_eq!(result.to_raw().build(), "array::sort::desc([3, 2, 1])");
    }
}
