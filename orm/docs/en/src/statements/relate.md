# Relate Statement

The `relate` statement is used to create relationships between different
entities in SurrealDB. It allows you to establish connections and associate data
between tables. Here are some examples and usage scenarios for the `relate`
statement.

## Table of Contents

- [Getting Relations](#getting-relations)
- [Valid ID Usage](#valid-id-usage)
- [Invalid ID Usage](#invalid-id-usage)
- [Relate Subquery to Subquery](#relate-subquery-to-subquery)
- [Any Edge Filter](#any-edge-filter)
- [Recursive Edge-to-Edge Connection](#recursive-edge-to-edge-connection)
- [Relate Query](#relate-query)
- [Relate Query with Subquery](#relate-query-with-subquery)
- [Using `set` Method with `object!` Macro in the `relate` Statement](#using-set-method-with-object-macro-in-the-relate-statement)
  - [Example: Using `object!` Macro with `set` in `relate`](#example-using-object-macro-with-set-in-relate)
- [The Importance of the `object!` Macro in the `relate` Statement](#the-importance-of-the-object-macro-in-the-relate-statement)

## Getting Relations

You can retrieve the relations and aliases for a specific field in a struct
using the `get_fields_relations_aliased` method. This example demonstrates how
to retrieve the relations and aliases for the `Student` struct:

```rust
let relations_aliases = Student::get_fields_relations_aliased();
```

## Valid ID Usage

To create a relationship between entities using valid IDs, you can use the
`relate` statement. Here's an example of how to relate a student to a book:

```rust
let student_id = Student::create_id("1");
let book_id = Book::create_id("2");

let write_book = StudentWritesBook {
    time_written: Duration::from_secs(343),
    // other fields...
};

let relation = relate(Student::with(&student_id).writes__(Empty).book(&book_id))
    .content(write_book)
    .parallel();
```

## Invalid ID Usage

When using invalid IDs in the `relate` statement, errors will be generated.
Here's an example of relating entities with invalid IDs:

```rust
let student_id = Student::create_id("oye");
let book_id = Book::create_id("mars");

let write = StudentWritesBook {
    time_written: Duration::from_secs(343),
    // other fields...
};

let relate_statement = relate(Student::with(&book_id).writes__(Empty).book(&student_id))
    .content(write.clone())
    .return_type(ReturnType::Before)
    .parallel();
```

## Relate Subquery to Subquery

You can also use subqueries in the `relate` statement to establish relationships
between subquery results. Here's an example:

```rust
let write = StudentWritesBook {
    time_written: Duration::from_secs(52),
    // other fields...
};

let statement = relate(
    Student::with(select(All).from(Student::get_table()))
        .writes__(E)
        .book(
            select(All).from(Book::get_table()),
        ),
)
.content(write.clone());
```

## Any Edge Filter

The `any_other_edges` function allows you to filter relationships based on
multiple edge types. Here's an example:

```rust
let aliased_connection = Student::with(student_id)
    .writes__(any_other_edges([visits, likes]).where_(timeWritten.less_than_or_equal(50)))
    .book(book_id)
    .__as__(Student::aliases().writtenBooks);
```

## Recursive Edge-to-Edge Connection

You can create recursive edge-to-edge connections using the `relate` statement.
This allows you to select and relate entities at multiple levels. Here's an
example:

```rust
let aliased_connection = Student::with(student_id)
    .writes__(Empty)
    .writes__(Empty)
    .writes__(any_other_edges(&[writes, likes]).where_(timeWritten.less_than_or_equal(50)))
    .book(book_id)
    .__as__(Student::aliases().writtenBooks);
```

## Relate Query

The `relate` statement can be used to execute a query and return the result.
Here's an example:

```rust
let relate_simple = relate(Student::with(student_id).writes__(E).book(book_id)).content(write);
let relate_simple_object = relate_simple.return_one(db.clone()).await?;
let relate_simple_array = relate_simple.return_many(db.clone()).await?;
```

## Relate Query with Subquery

You can also use subqueries in the `relate` statement to execute more complex
queries. Here's an example:

```rust
let statement = relate(
    Student::with(select(All).from(Student::get_table()))
        .writes__(E)
        .book(
            select(All).from(Book::get_table()),
        ),
)
.content(write.clone());
```

### Using `set` Method with `object!` Macro in the `relate` Statement

The `relate` statement supports the use of the `set` method, serving as an
alternative to the `content` method for specifying data when creating
relationships between entities.

The `set` method, when combined with the `object!` macro, offers a concise,
type-safe, and robust way to define the fields to be set during the relation
creation. Using the `object!` macro ensures that all fields are present, which
is crucial for avoiding serialization/deserialization issues arising from
missing fields or schema mismatches.

#### Example: Using `object!` Macro with `set` in `relate`

```rust
let student_id = Student::create_id("1");
let book_id = Book::create_id("2");

relate(Student::with(&student_id).writes__(Empty).book(&book_id))
    .set(object!(StudentWritesBook {
        time_written: Duration::from_secs(343),
        // other fields...
    }))
    .parallel();
```

### The Importance of the `object!` Macro in the `relate` Statement

In the context of the `relate` statement, the `object!` macro provides
significant advantages:

1. **Type Safety:** The `object!` macro ensures type safety, drastically
   reducing the risk of type mismatches during compile-time.
2. **Full Field Coverage:** Ensures that all fields are present, protecting
   against potential issues during serialization/deserialization due to missing
   fields or schema mismatches.
3. **Readability and Clarity:** Using the `object!` macro leads to cleaner code.
   By explicitly defining fields and their corresponding values, the code
   becomes more understandable.
4. **Parameterized Fields:** Supports the inclusion of parameters and fields,
   making it especially valuable in transactional contexts within the `block!`
   macro.

Given these benefits, it's strongly recommended to utilize the `object!` macro
in the `relate` statement:

```rust
let relation_with_macro = relate(Student::with(&student_id).writes__(Empty).book(&book_id))
    .set(object!({
        timeWritten: Utc::now(),
        someOtherField: "Some Value",
        anotherField: "Another Value"
    }))
    .parallel();
```

Prioritizing the use of the `object!` macro ensures a combination of safety,
clarity, and robustness in your development process.
