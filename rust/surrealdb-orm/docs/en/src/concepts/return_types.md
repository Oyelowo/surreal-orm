# Return Types

Return types in the Surreal ORM define how database operations result in
returned data. They allow the specification of the format and structure of the
data returned after running a database operation. In this chapter, we will
discuss some of the "return" types that are part of the `ReturnableStandard`
trait.

## `return_one`

The `return_one` function runs a statement against the database and returns a
single result. If the result contains more than one record, it throws an error.

Consider a scenario where you want to fetch a single user from your database.
You can use `return_one` to get the `User` record:

```rust
let user = User::find(1).return_one(db).await.unwrap();
```

## `return_many`

The `return_many` function runs a statement against the database and returns
multiple results.

For instance, if you want to fetch all users from your database, you can use
`return_many` to get the `User` records:

```rust
let users = User::all().return_many(db).await.unwrap();
```

## `return_none`

The `return_none` function runs a statement against the database and returns no
result.

This is particularly useful when you perform operations that don't require a
return value. For example, deleting a user:

```rust
User::delete(1).return_none(db).await.unwrap();
```

## `return_first`

The `return_first` function runs a statement against the database and returns
the first result.

For example, to get the first user in the database:

```rust
let user = User::all().return_first(db).await.unwrap();
```

## `return_many_before`

The `return_many_before` function runs a statement against the database and
returns the many results before the change.

This is useful when you want to compare the state of the records before and
after a database operation. For instance, updating a user's profile:

```rust
let users_before_update = User::all().return_many_before(db).await.unwrap();
User::update(1, new_profile_data).return_none(db).await.unwrap();
```

In conclusion, return types provide a way to control the data returned by
database operations. Whether you want a single record, multiple records, the
first record, or no record at all, return types allow you to specify the
outcome.
