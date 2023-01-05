# Surreal simple querybuilder
A simple query-builder for the Surreal Query Language, for [SurrealDB](https://surrealdb.com/).
Aims at being simple to use and not too verbose first.

# Summary
- [Surreal simple querybuilder](#surreal-simple-querybuilder)
- [Summary](#summary)
- [Why a query-builder](#why-a-query-builder)
- [SQL injections](#sql-injections)
- [Compiler requirements/features](#compiler-requirementsfeatures)
- [Examples](#examples)
  - [The `model` macro](#the-model-macro)
    - [public \& private fields in models](#public--private-fields-in-models)
    - [Relations between your models](#relations-between-your-models)
  - [The `NodeBuilder` traits](#the-nodebuilder-traits)
  - [The `QueryBuilder` type](#the-querybuilder-type)
  - [The `ForeignKey` and `Foreign` types](#the-foreignkey-and-foreign-types)
    - [`ForeignKey` and loaded data during serialization](#foreignkey-and-loaded-data-during-serialization)

# Why a query-builder
Query builders allow you to dynamically build your queries with some compile time
checks to ensure they result in valid SQL queries. Unlike ORMs, query-builders are
built to be lightweight and easy to use, meaning you decide when and where to use
one. You could stick to hard coded string for the simple queries but use a builder
for complex ones that require parameters & variables and may change based on these
variables for example.

While the crate is first meant as a query-building utility, it also comes with
macros and generic types that may help you while managing you SQL models in your rust code.
Refer to the [node macro](#the-node-macro) and the [Foreign type](#the-foreignkey-and-foreign-type) example

# SQL injections
The strings you pass to the query builder are not sanitized in any way. Please use
parameters in your queries like `SET username = $username` with surrealdb parameters to avoid injection issues.
However the crate comes with utility functions to easily create parameterized fields, refer to the [`NodeBuilder`](src/node_builder.rs) trait.

# Compiler requirements/features
The crate uses const expressions for its [model creation macros](#the-model-macro)
in order to use stack based arrays with sizes deduced by the compiler. For this reason
any program using the crate has to add the following at the root of the main file:
```
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
```

# Examples
 - A series of [examples are available](/examples/) to offer a **guided introduction** to the core features of the crate
 - An all-in-one exapmle can be found in the [`tests project`](/tests/src/querybuilder.rs).
 - For an explanation of what each component in the crate does, refer to the chapters below.
## The `model` macro
The `model` macro allows you to quickly create structs (aka models) with fields
that match the nodes of your database.

<details>
  <summary>example</summary>

  ```rust
  use surreal_simple_querybuilder::prelude::*;

  struct Account {
    id: Option<String>,
    handle: String,
    password: String,
    email: String,
    friends: Foreign<Vec<Account>>
  }

  model!(Account {
    id,
    handle,
    password,
    friends<Vec<Account>>
  });

  fn main() {
    // the schema module is created by the macro
    use schema::model as account;

    let query = format!("select {} from {account}", account.handle);
    assert_eq!("select handle from Account", query);
  }
  ```
</details>


This allows you to have compile time checked constants for your fields, allowing
you to reference them while building your queries without fearing of making a typo
or using a field you renamed long time ago.

### public & private fields in models

The QueryBuilder type offers a series of methods to quickly list the fields of your
models in SET or UPDATE statements so you don't have to write the fields and the
variable names one by one. Since you may not want to serialize some of the fields
like the `id` for example the model macro has the `pub` keyword to mark a field
as serializable. Any field without the `pub` keyword in front of it will not
be serialized by these methods.

```rs
model!(Project {
  id, // <- won't be serialized
  pub name, // <- will be serialized
})

fn example() {
  use schema::model as project;

  let query = QueryBuilder::new()
    .set_model(project)
    .build();

  assert_eq!(query, "SET name = $name");
}
```


### Relations between your models
If you wish to include relations (aka edges) in your models, the `model` macro
has a special syntax for them:

```rust
mod account {
  use surreal_simple_querybuilder::prelude::*;
  use super::project::schema::Project;

  model!(Account {
    id,

    ->manage->Project as managed_projects
  });
}

mod project {
  use surreal_simple_querybuilder::prelude::*;
  use super::project::schema::Project;

  model!(Project {
    id,
    name,

    <-manage<-Account as authors
  });
}

fn main() {
    use account::schema::model as account;

    let query = format!("select {} from {account}", account.managed_projects);
    assert_eq!("select ->manage->Project from Account");

    let query = format!("select {} from {account}", account.managed_projects().name.as_alias("project_names"))
    assert_eq!("select ->manage->Project.name as project_names from Account", query);
  }
```

## The `NodeBuilder` traits
These traits add a few utility functions to the `String` and `str` types that can
be used alongside the querybuilder for even more flexibility.

```rust
use surreal_simple_querybuilder::prelude::*;

let my_label = "John".as_named_label("Account");
assert_eq!("Account:John", &my_label);

let my_relation = my_label
  .with("FRIEND")
  .with("Mark".as_named_label("Account"));

assert_eq!("Account:John->FRIEND->Account:Mark", my_relation);
```


## The `QueryBuilder` type
It allows you to dynamically build complex or simple queries out of _segments_ and easy to use
methods.
<details>
  <summary>Simple example</summary>

  ```rust
  use surreal_simple_querybuilder::prelude::*;

  let query = QueryBuilder::new()
    .select("*")
    .from("Account")
    .build();

  assert_eq!("SELECT * FROM Account", &query);
  ```
</details>

<details>
  <summary>Complex example</summary>

  ```rust
  use surreal_simple_querybuilder::prelude::*;

  let should_fetch_authors = false;
  let query = QueryBuilder::new()
    .select("*")
    .from("File")
    .if_then(should_fetch_authors, |q| q.fetch("author"))
    .build();

  assert_eq!("SELECT * FROM Account", &query);

  let should_fetch_authors = true;
  let query = QueryBuilder::new()
    .select("*")
    .from("File")
    .if_then(should_fetch_authors, |q| q.fetch("author"))
    .build();

  assert_eq!("SELECT * FROM Account FETCH author", &query);
  ```
</details>


## The `ForeignKey` and `Foreign` types
SurrealDB has the ability to fetch the data out of foreign keys. For example:
```sql
create Author:JussiAdlerOlsen set name = "Jussi Adler-Olsen";
create File set name = "Journal 64", author = Author:JussiAdlerOlsen;

select * from File;
select * from File fetch author;
```
which gives us
```json
// without FETCH author
{
  "author": "Author:JussiAdlerOlsen",
  "id":"File:rg30uybsmrhsf7o6guvi",
  "name":"Journal 64"
}

// with FETCH author
{
  "author": {
    "id":"Author:JussiAdlerOlsen",
    "name":"Jussi Adler-Olsen"
  },
  "id":"File:rg30uybsmrhsf7o6guvi",
  "name":"Journal 64"
}
```

The "issue" with this functionality is that our results may either contain an ID
to the author, no value, or the fully fetched author with its data depending on
the query and whether it includes `fetch` or not.

The `ForeignKey` types comes to the rescue. It is an enum with 3 variants:
 - The loaded data for when it was fetched
 - The key data for when it was just an ID
 - The unloaded data when it was null (if you wish to support missing data you must use the `#serde(default)` attribute to the field)

The type comes with an implementation of the Deserialize and Serialize serde traits
so that it can fallback to whatever data it finds or needs. However any type that
is referenced by a `ForeignKey` must implement the `IntoKey` trait that allows it
to safely serialize it into an ID during serialization.

<details>
  <summary>example</summary>

  ```rust
  /// For the tests, and as an example we are creating what could be an Account in
  /// a simple database.
  #[derive(Debug, Serialize, Deserialize, Default)]
  struct Account {
    id: Option<String>,
    handle: String,
    password: String,
    email: String,
  }

  impl IntoKey<String> for Account {
    fn into_key<E>(&self) -> Result<String, E>
    where
      E: serde::ser::Error,
    {
      self
        .id
        .as_ref()
        .map(String::clone)
        .ok_or(serde::ser::Error::custom("The account has no ID"))
    }
  }

  #[derive(Debug, Serialize, Deserialize)]
  struct File {
    name: String,

    /// And now we can set the field as a Foreign node
    author: Foreign<Account>,
  }

  fn main() {
    // ...imagine `query` is a function to send a query and get the first result...
    let file: File = query("SELECT * from File FETCH author");

    if let Some(user) = file.author.value() {
      // the file had an author and it was loaded
      dbg!(&user);
    }

    // now we could also support cases where we do not want to fetch the authors
    // for performance reasons...
    let file: File = query("SELECT * from File");

    if let Some(user_id) = file.author.key() {
      // the file had an author ID, but it wasn't fetched
      dbg!(&user_id);
    }

    // we can also handle the cases where the field was missing
    if file.author.is_unloaded {
      panic!("Author missing in file {file}");
    }
  }
  ```
</details>

### `ForeignKey` and loaded data during serialization

A `ForeignKey` always tries to serialize itself into an ID by default. Meaning that
if the foreign-key holds a value and not an ID, it will call the `IntoKey` trait on
the value in order to get an ID to serialize.

There are cases where this may pose a problem, for example in an API where you wish
to serialize a struct with `ForeignKey` fields so the users can get all the data
they need in a single request.

By default if you were to serialize a `File` (from the example above) struct
with a fetched `author`, it would automatically be converted into the author's id.

The `ForeignKey` struct offers two methods to control this behaviour:
```rust
// ...imagine `query` is a function to send a query and get the first result...
let file: File = query("SELECT * from File FETCH author");

file.author.allow_value_serialize();

// ... serializing `file` will now serialize its author field as-is.

// to go back to the default behaviour
file.author.disallow_value_serialize();
```

You may note that mutability is not needed, the methods use interior mutability
to work even on immutable ForeignKeys if needed.