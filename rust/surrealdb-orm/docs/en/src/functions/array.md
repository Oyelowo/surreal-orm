# Array Functions

SurrealDB ORM provides a set of array functions that allow you to manipulate and perform operations on arrays. These functions are designed to work with arrays of various types, including vectors, fields, and parameters. This documentation covers the usage and examples of the array functions available in SurrealDB ORM.

## Table of Contents

<!--toc:start-->

- [Array Functions](#array-functions)
  - [Table of Contents](#table-of-contents)
  - [Append](#append)
  - [Combine](#combine)
  - [Concat](#concat)
  - [Union](#union)
  - [Difference](#difference)
  - [Intersect](#intersect)
  - [Complement](#complement)
  - [Distinct](#distinct)
  - [Flatten](#flatten)
  - [Group](#group)
  - [Insert](#insert)
  - [Len](#len)
  - [Pop](#pop)
  - [Prepend](#prepend)
  - [Push](#push)
  - [Remove](#remove)
  - [Reverse](#reverse)
  - [Sort](#sort)
  - [Asc and Desc](#asc-and-desc)
  <!--toc:end-->

## Append

The `append` function appends a value to the end of an array.

```rust
use surrealdb_orm::{functions::array, *};

let result = array::append!(vec![1, 2, 3, 4, 5], 6);
```

You can use the `append` function to add values to an existing array.

## Combine

The `combine` function combines all values from two arrays together, returning an array of arrays.

```rust
use surrealdb_orm::{functions::array, *};

let result = array::combine!(vec![1, 2, 3], vec![4, 5, 6]);
```

The `combine` function provides the same functionality as the `append` function but can work with two arrays instead of appending a single value.

## Concat

The `concat` function merges two arrays together, returning an array that may contain duplicate values.

```rust
use surrealdb_orm::{functions::array, *};

let result = array::concat!(vec![1, 2, 3], vec![4, 5, 6]);
```

The `concat` function provides the same functionality as the `combine` function but does not remove duplicate values from the resulting array.

## Union

The `union` function combines two arrays together, removing duplicate values, and returning a single array.

```rust
use surrealdb_orm::{functions::array, *};

let result = array::union!(vec![1, 2, 3], vec![4, 5, 6]);
```

The `union` function provides the same functionality as the `concat` function but removes duplicate values from the resulting array.

## Difference

The `difference` function determines the difference between two arrays, returning a single array containing items that are not in both arrays.

```rust
use surrealdb_orm::{functions::array, *};

let result = array::difference!(vec![1, 2, 3], vec![4, 5, 6]);
```

The `difference` function provides the same functionality

as the previous functions but returns only the unique values that are present in one array but not in the other.

## Intersect

The `intersect` function calculates the values that intersect two arrays, returning a single array containing the values present in both arrays.

```rust
use surrealdb_orm::{functions::array, *};

let result = array::intersect!(vec![1, 2, 3], vec![4, 5, 6]);
```

The `intersect` function provides the same functionality as the previous functions but returns only the values that are common between the two arrays.

## Complement

The `complement` function returns the complement of two arrays, returning a single array containing items that are not in the second array.

```rust
use surrealdb_orm::{functions::array, *};

let result = array::complement!(vec![1, 2, 3, 4], vec![3, 4, 5, 6]);
```

The `complement` function provides the same functionality as the previous functions but returns only the values that are present in the first array but not in the second array.

## Distinct

The `distinct` function calculates the unique values in an array, returning a single array.

```rust
use surrealdb_orm::{functions::array, *};

let result = array::distinct!(vec![1, 2, 3]);
```

You can use the `distinct` function to obtain unique values from an array.

## Flatten

The `flatten` function flattens an array of arrays, returning a new array with all sub-array elements concatenated into it.

```rust
use surrealdb_orm::{functions::array, *};

let result = array::flatten!(array![vec![1, 2], vec![3, 4], "SurrealDB", vec![5, 6]]);
```

The `flatten` function provides the same functionality as the previous functions but flattens an array of arrays.

## Group

The `group` function flattens and returns the unique items in an array.

```rust
use surrealdb_orm::{functions::array, *};

let result = array::group!(array![1, 2, 3, 4, array![3, 5, 6], vec![2, 4, 5, 6], 7, 8, 8, 9]);
```

The `group` function provides the same functionality as the previous functions but returns only the unique items in the array.

## Insert

The `insert` function inserts a value into an array at a specific position.

```rust
use surrealdb_orm::{functions::array, *};

let result = array::insert!(vec![1, 2, 3, 4], 5, 2);
```

The `insert` function allows you to insert a value into an array at a specified index.

## Len

The `len` function calculates the length of an array, returning a number. This function includes all items when counting the number of items in the array.

```rust
use surrealdb_orm::{functions::array, *};

let result = array::len!(vec![1, 2, 3]);
```

You can use the `len` function to calculate the length of an array.

## Pop

The `pop` function removes a value from the end of an array and returns it.

```rust
use surrealdb_orm::{functions::array, *};

let result = array::pop!(vec![1, 2, 3, 4]);
```

You can use the `pop` function to remove the last value from an array.

## Prepend

The `prepend` function prepends a value to the end of an array.

```rust
use surrealdb_orm::{functions::array, *};

let result = array::prepend!(vec![1, 2, 3, 4], 5);
```

You can use the `prepend` function to add a value to the beginning of an array.

## Push

The `push` function appends a value to the end of an array.

```rust
use surrealdb_orm::{functions::array, *};

let result = array::push!(vec![1, 2, 3, 4], 5);
```

The `push` function provides the same functionality as the `prepend` function but appends the value to the end of the array instead of the beginning.

## Remove

The `remove` function removes an item from a specific position in an array.

```rust
use surrealdb_orm::{functions::array, *};

let result = array::remove!(vec![1, 2, 3, 4, 5], 2);
```

You can use the `remove` function to delete an item from a specific position in an array.

## Reverse

The `reverse` function reverses the order of the elements in an array.

```rust
use surrealdb_orm::{functions::array, *};

let result = array::reverse!(vec![1, 2, 3, 4, 5]);
```

You can use the `reverse` function to reverse the order of elements in an array.

## Sort

The `sort` function sorts an array in ascending or descending order.

```rust
use surrealdb_orm::{functions::array, *};

let result = array::sort!(vec![3, 1, 2], "asc");
```

You can use the `sort` function to sort an array in ascending or descending order.

The `sort` function also provides the following ordering options:

- `"asc"`: Sorts the array in ascending order.
- `"desc"`: Sorts the array in descending order.
- `false`: Does not sort the array.

You can use the `sort` function with different ordering options to sort an array accordingly.

## Asc and Desc

The `asc` and `desc` functions are shorthand convenience functions for the `sort` function. They sort values in an array in ascending or descending order, respectively.

```rust
use surrealdb_orm::{functions::array, *};

let result_asc = array::sort::asc!(vec![3, 1, 2]);
let result_desc = array::sort::desc!(vec![3, 1, 2]);
```

The `asc` and `desc` functions provide the same functionality as the `sort` function but with a more concise syntax.

These are the array functions available in SurrealDB ORM. Use them to perform various operations on arrays and manipulate array data effectively.

That concludes the documentation for the array functions in SurrealDB ORM.
Refer to this documentation whenever you need to use array functions in your code.
