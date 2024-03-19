# Model Schema

This guide covers the `SchemaGetter` trait in SurrealDB, a Rust crate, and
provides examples on how to use it.

### The SchemaGetter Trait

This trait is defined as follows:

```rust
pub trait SchemaGetter {
    type Schema;
    fn schema() -> Self::Schema;
    fn schema_prefixed(prefix: impl Into<ValueLike>) -> Self::Schema;
}
```

This trait is used for defining schemas for different entities in your database.
It contains two associated functions:

1. `schema()`: Returns a schema for an entity. This is used for defining the
   structure and constraints of the entity.
2. `schema_prefixed(prefix: impl Into<ValueLike>)`: This is similar to
   `schema()`, but it allows the schema to be prefixed with a custom value. This
   can be useful when working with entities that may share similar fields but
   have different schemas.

The `SchemaGetter` trait's primary use is to allow types to be used as a
'Schema' - a representation of the structure of the data you're storing or
retrieving from the database. It's particularly useful in constructing complex
queries with strong type safety.

### Examples

The examples below demonstrate the different methods you can utilize in
SurrealDB, leveraging the SchemaGetter trait:

#### Creating and Retrieving Entities:

This piece of code uses the `schema()` function of `SchemaGetter` to create and
retrieve entities in the database:

```rust
let _simple_relation = Student::schema()
    .writes__(Empty)
    .book(Book::schema().id.equal(Thing::from(("book", "blaze"))))
    .title;
```

This creates a relation between the `Student` and `Book` entities. It uses the
`writes__` method to create a relation indicating the `Student` writes a `Book`.
The `book` call then specifies that the book's id equals a specific `Thing`
entity.

#### Pattern Selection:

SurrealDB also allows the pattern-like selection of entities:

```rust
let student_id = Student::create_id("oyelowo");
let book_id = Book::create_id("2");
let likes = StudentLiksBook::table();
let writes = StudentWritesBook::table();
let writes::Schema { timeWritten, .. } = StudentWritesBook::schema();

let aliased_connection = Student::with(student_id)
    .writes__(Empty)
    .writes__(Empty)
    .writes__(any_other_edges(&[writes, likes]).where_(timeWritten.less_than_or_equal(50)))
    .book(book_id)
    .__as__(Student::aliases().writtenBooks);
```

In this case, we are selecting all the books that a specific student wrote where
the `timeWritten` is less than or equal to 50. This query is an example of how
you can combine different methods and concepts provided by SurrealDB to form
complex, yet understandable, queries.

#### Modifying and Updating Entities:

The following example illustrates how to modify and update entities:

```rust
let ref id = created_weapon.clone().id;
let weapon::Schema { strength, .. } = Weapon::schema();

update::<Weapon>(id)
    .set(strength.increment_by(5u64))
    .run(db.clone())
    .await?;

let updated = update::<Weapon>(id)
    .set(strength.decrement_by(2u64))
    .return_one(db.clone())
    .await?;

let selected: Option<Weapon> = select(All)
    .from(Weapon::table())
    .return_one(db.clone())
    .await?;
assert_eq!(updated.unwrap().strength, 8);
assert_eq!(selected.unwrap().strength, 8);
```
