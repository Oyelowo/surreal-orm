# Field Traversal

The `surreal_orm` library equips developers with powerful field traversal
capabilities, allowing for seamless querying and navigation through the
`surrealdb` graph database. This chapter provides an in-depth exploration into
the different traversal methods available and how to harness them effectively.

## Basics of Field Traversal

Field traversal is the mechanism used to navigate through a data structure,
pinpointing specific fields or relationships. The design of the `surreal_orm`
makes traversal not only intuitive but also direct, offering methods to navigate
fields, relationships, and even to apply specific conditions.

To get started, let's set up our environment:

```rust
use pretty_assertions::assert_eq;
use surreal_models::{student, Student};
use surreal_orm::{index, this, where_, All, Buildable, Operatable, SchemaGetter, ToRaw, E};
```

## Root Object: The Starting Point

Every traversal starts with the root object. The `this()` function is your
gateway, representing the current node or object you're working on.

```rust
# fn basic() {
let param_with_path = this();
assert_eq!(param_with_path.to_raw().build(), "$this");
# }
```

In the code snippet above, the `this()` function signifies a reference to the
root object or the primary context of the traversal. When executed, this will
produce `"$this"`.

## Traversing the Path

Navigating relationships between nodes is where the real power of a graph
database shines. The `with_path::<T>(index_or_clause)` method allows you to
specify this path. Here `T` is the type of node you're targeting, while
`index_or_clause` can either be an index or a clause, such as `WHERE age > 18`
or `E` (an alias for `Empty`).

For instance, to get the `firstName` of a `Student` at index `2`:

```rust
let param_with_path = this().with_path::<Student>([2]).firstName;
```

This traversal, when executed, will output `"$this[2].firstName"`.

### Direct Field Access within an Object

Sometimes, all you want is to directly access a field within an object. Here's
how you can achieve that:

```rust
# fn test_param_simple_clause() {
let param_with_path = this().with_path::<Student>(E).lastName;
assert_eq!(param_with_path.to_raw().build(), "$this.lastName");
# }
```

In this example, the alias `E` (standing for `Empty`) is employed to directly
traverse to the `lastName` field of the `Student` object.

### Direct Field Access within an Array

At other times, you might want to directly access a field within an array:

```rust
# fn test_param_with_path_simple() {
let param_with_path = this().with_path::<Student>([2]).firstName;
assert_eq!(param_with_path.to_raw().build(), "$this[2].firstName");
# }
```

Here, the code fetches the `firstName` of the `Student` located at index `2`.

### Deep Relationship Traversal

The true essence of a graph database is revealed when traversing deep
relationships. Consider this test:

```rust
# fn test_param_with_path() {
let param_with_path = this()
    .with_path::<Student>([2])
    .bestFriend()
    .bestFriend()
    .course()
    .title;
assert_eq!(param_with_path.to_raw().build(), "$this[2].bestFriend.bestFriend.course.title");
# }
```

This test showcases how to navigate through a `Student`'s best friend's best
friend's course title.

### Index function for Indexing

An alternate to square bracket notation `[2]` is the `index` helper function
e.g(`index(2)`):

```rust
# fn test_param_with_path_with_index_square_bracket_variation() {
let param_with_path = this()
    .with_path::<Student>(index(2))
    .bestFriend()
    .bestFriend()
    .course()
    .title;
assert_eq!(param_with_path.to_raw().build(), "$this[2].bestFriend.bestFriend.course.title");
# }
```

### Traversal with Conditional Clauses

You can also traverse paths with conditions, allowing for more refined querying:

```rust
# fn test_param_with_path_with_clause() {
let student::Schema { age, .. } = Student::schema();
let param_with_path = this()
    .with_path::<Student>(where_(age.greater_than(18)))
    .bestFriend()
    .allSemesterCourses([5])
    .title;
assert_eq!(param_with_path.to_raw().build(), "$this[WHERE age > 18].bestFriend.allSemesterCourses[5].title");
# }
```

This traversal fetches the title of the fifth semester course of the best
friends of students older than 18.

### Using the All Wildcard

For scenarios where you want to traverse all items or elements of a certain
relationship or field, the `All` wildcard is invaluable:

```rust
# fn test_param_with_path_with_all_wildcard() {
let param_with_path = this()
    .with_path::<Student>(All)
    .bestFriend()
    .allSemesterCourses([5])
    .title;
assert_eq!(param_with_path.to_raw().build(), "$this[*].bestFriend.allSemesterCourses[5].title");
# }
```

In the traversal above, `All` is a wildcard that represents every instance of
the `Student` type. The traversal then specifies the fifth course title of all
students' best friends.

### Multiple Indexes in Path

There are scenarios where traversing multiple indexed fields or relationships
becomes necessary:

```rust
# fn test_param_with_path_multiple_indexes() {
let param_with_path = this()
    .with_path::<Student>([2])
    .bestFriend()
    .allSemesterCourses([5])
    .title;
assert_eq!(param_with_path.to_raw().build(), "$this[2].bestFriend.allSemesterCourses[5].title");
# }
```

Here, the traversal first targets the `Student` at index 2 and then fetches the
title of the fifth semester course of that student's best friend.

---

## Conclusion

Field traversal in `surreal_orm` equips developers with a versatile and powerful
toolset, enabling effective navigation and querying within
