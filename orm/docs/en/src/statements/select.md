# Surreal ORM Documentation

## Table of Contents

1. [Introduction](#introduction)
2. [Defining Your Data](#defining-data)
3. [Advanced Schema Definitions](#advanced-schema)
4. [Select Statements](#select-statements)
   - [Using the cond! Macro](#using-the-cond-macro)
5. [Advanced Select Queries](#advanced-select-queries)
6. [Select Value Statements](#select-value-statements)
7. [Advanced Select Value Queries](#advanced-select-value-queries)
8. [Running Select Statements](#running-select-statements)
9. [Running and Returning from a Select Statement](#running-and-returning-from-a-select-statement)

<a name="introduction"></a>

## 1. Introduction

This document focuses on defining models and using `select` and `select_value`
statements for data retrieval.

<a name="defining-data"></a>

## 2. Defining Your Data

Start by defining a `User` struct representing a user in your application.

```rust
extern crate surreal_orm;
use surreal_orm::*;

#[derive(Node, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table = "user")]
pub struct User {
    pub id: SurrealSimpleId<Self>,
    pub account: String,
    pub friend: String,
}
```

<a name="advanced-schema"></a>

## 3. Advanced Schema Definitions

Surreal ORM supports more complex data types including links between different
models. Here's a detailed example using a `Student` and a `Book`:

```rust
#[derive(Node, Serialize, Deserialize)]
#[surreal_orm(table = "student")]
pub struct Student {
    id: SurrealSimpleId<Self>,
    first_name: String,
    last_name: String,
    age: u8,

    #[surreal_orm(link_self = "Student")]
    best_friend: LinkSelf<Student>,

    #[surreal_orm(link_one = "Book")]
    fav_book: LinkOne<Book>,

    #[surreal_orm(link_one = "Book")]
    course: LinkOne<Book>,

    #[surreal_orm(link_many = "Book")]
    sem_courses: LinkMany<Book>,
}

#[derive(Node, Serialize, Deserialize)]
#[surreal_orm(table = "book")]
pub struct Book {
    id: SurrealSimpleId<Self>,
    content: String,
}
```

<a name="select-statements"></a>

## 4. Select Statements

`select` allows you to construct a SELECT statement to fetch records.

```rust
use surreal_orm::{*, statements::{order, select}};

let student::Schema {
    id,
    first_name,
    last_name,
    best_friend,
    uno_book,
    course,
    sem_courses,
    ref age,
    ..
} = &Student::schema();

let book::Schema { ref content, .. } = Book::schema();

let mut statement = select(arr![age, last_name, content])
    .from(Book::table())
    .where_(
        cond(content.like("lowo"))
            .and(age.greater_than_or_equal(600))
            .or(first_name.equal("Oyelowo"))
            .and(last_name.equal("Oyedayo")),
    )
    .order_by(last_name.desc()
    .limit(50)
    .start(20)
    .timeout(Duration::from_secs(9))
    .parallel();

let is_lowo = true;
if is_lowo {
    statement = statement.limit(55).order_by(age.desc());
}
```

<a name="using-the-cond-macro"></a>

### Using the cond! Macro

In Surreal ORM, while the `cond` function provides an elegant way to construct
filters, there's also a macro alternative called `cond!`. This macro can offer
more concise and readable representations, especially for complex conditions.

The `cond!` macro provides a convenient syntax for constructing filters, similar
to standard Rust conditional expressions. It can handle various operations like
equalities, inequalities, and logical combinations.

Here's a simple example:

```rust
# use surreal_query_builder as surreal_orm;
# use surreal_orm::*;
# let age = Field::new("age");
# let name = Field::new("name");
# let title = Field::new("title");

let filter_simple = cond!(age > 18);
let filter_compound = cond!((age > 18) AND (name ~ "%Oyelowo%") OR (title == "Professor"));
let filter_mixed = cond!((age.or(4).or(545).or(232)) OR (title = "Professor") AND (age < 100));
```

This macro provides a more intuitive way of writing conditions, especially when
compared to chaining methods. The full definition and capabilities of the
`cond!` macro are documented within the Surreal ORM codebase.

<a name="advanced-select-queries"></a>

## 5. Advanced Select Queries

You can perform complex queries including nested select statements and
conditional query generation. Here is an example:

```rust
use surreal_orm::{*, statements::{order, select}};

let student::Schema {
    id,
    firstName,
    lastName,
    bestFriend,
    unoBook,
    course,
    semCoures,
    ref age,
    ..
} = &Student::schema();

let book::Schema { ref content, .. } = Book::schema();
let ref student_table = Student::get_table();
let ref book_table = Book::get_table();
let ref book_id = thing("book:1").unwrap();

let mut query1 = select(arr![age, lastName, content])
    .from(Book::get_table())
    .where_(
        cond(content.like("lowo"))
            .and(age.greater_than_or_equal(600))
            .or(firstName.equal("Oyelowo"))
            .and(lastName.equal("Oyedayo")),
    )
    .order_by(lastName.desc())
    .limit(50)
    .start(20)
    .timeout(Duration::from_secs(9))
    .parallel();

let statement = select(All)
    .from(student_table)
    // .from(&[student_table, book_table])
    // .from(book_id)
    // .from(query1)
    .where_(
        cond(
            (((age + 5) - 6) * 10).greater_then(5) // You can even use raw mathematical operators directly.
        )
        .and(bestFriend.exactly_equal("Oyelowo"))
        .or(firstName.equal("Oyedayo"))
        .and(age.greater_than_or_equal(150)),
    )
    .order_by(firstName.rand().desc())
    // .order_by(lastName.collate().asc())
    // .order_by(id.numeric().desc())
    // .group_by(course)
    // .group_by(firstName)
    // .group_by(arr![lastName, unoBook])
    .start(5)
    .limit(400)
    .fetch(firstName)
    // .fetch(lastName)
    // .fetch(arr![age, unoBook])
    .split(lastName)
    // .split(firstName)
    // .split(arr![firstName, semCoures])
    .timeout(Duration::from_secs(8))
    .parallel();

let is_oyelowo = true;
if is_oyelowo {
    query = query.group_by(arr![age, bestFriend, &Field::new("dayo")]);
}
```

<a name="select-value-statements"></a>

## 6. Select Value Statements

`select_value` is similar to `select` but it only returns the first column from
the result. Here is a basic usage:

```rust
let statement = select_value(account)
    .from(user)
    .where_(account.is("abc"));
```

<a name="advanced-select-value-queries"></a>

## 7. Advanced Select Value Queries

You can perform complex value queries as well. Here is an example:

```rust
let statement = select_value(account)
    .from(user)
    .where_(
        and(
            account.is("abc"),
            or(
                friend.is("xyz"),
                friend.is("lmn"),
            ),
        ),
    );

let statement = select_value(account)
    .from(user)
    .where_(
        not(account.is("def")),
    );
```

<a name="running-select-statements"></a>

## 8. Running Select Statements

Executing a select statement is straightforward. Here's an example that uses
`return_many`:

```rust
extern crate surreal_orm;
use surreal_orm::{*, statements::{select, insert}};

#[derive(Node, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table = "weapon")]
pub struct Weapon {
    pub name: String,
    pub strength: i32,
    pub created: chrono::DateTime<chrono::Utc>,
}

let db = Surreal::new::<Mem>(()).await.unwrap();
db.use_ns("test").use_db("test").await.unwrap();

let generated_weapons = (1..=10)
    .map(|i| Weapon {
        name: format!("Weapon {}", i),
        strength: i * 10,
        created: chrono::Utc::now(),
        ..Default::default()
    })
    .collect::<Vec<_>>();
insert(generated_weapons.clone()).run(db.clone()).await?;

let ref weapon = Weapon::table();
let weapon::Schema { ref strength, .. } = &Weapon::schema();

let statement = select(All)
    .from(weapon)
    .where_(
        strength.inside(
            select_value(strength)
                .from(weapon)
                .order_by(strength.asc())
                .limit(6),
        ),
    )
    .order_by(strength.desc())
    .start(2)
    .limit(10);

assert_eq!(
    statement.to_raw().build(),
    "SELECT * FROM weapon WHERE strength INSIDE \
        (SELECT VALUE strength FROM weapon ORDER BY strength LIMIT 6) \
        ORDER BY strength DESC LIMIT 10 START AT 2;"
);
let result = statement.return_many::<Weapon>(db.clone()).await?;

assert_eq!(&result[0].name, "Weapon 4");
assert_eq!(&result[1].name, "Weapon 3");
assert_eq!(&result[2].name, "Weapon 2");
assert_eq!(&result[3].name, "Weapon 1");

assert_eq!(result.len(), 4);
assert!(result[0].id.to_string().starts_with("weapon:"));
Ok(())
```

This example first inserts generated weapon data into the database. Then it
constructs a `select` statement and retrieves the weapons whose `strength` is in
the top 6, ordered by `strength` in descending order, and returns the results
from the third entry. The `return_many` function is used to run the statement
and get the result.

<a name="running-and-returning-from-a-select-statement"></a>

## 9. Running and Returning from a Select Statement

The Surreal ORM package provides the `ReturnableSelect` trait that defines
several functions to run a select statement and return results in different
ways. These functions include `return_none`, `return_first`, `return_one`,
`return_one_unchecked`, and `return_many`.

All these functions run the statement against the SurrealDB database and return
results:

- `return_none`: Returns no result.
- `return_first`: Returns the first result.
- `return_one`: Returns one result.
- `return_one_unchecked`: Returns one result without checking if it's
  successful.
- `return_many`: Returns many results.
- `run`: Runs the query and provide more flexible deserialization just like
  surrealdb native drive e.g `.run(db).take::<T>(0)`.
