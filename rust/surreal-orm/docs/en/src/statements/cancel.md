# Cancel Statement

The `cancel` statement in SurrealDB ORM is used to cancel and rollback a
transaction, discarding any changes made within the transaction. It ensures that
the database remains unaffected by the transaction.

## Table of Contents

- [Recommended Approaches](#recommended-approaches)
  - [Using `block!` Macro with Cancel Statement also Within Block for Chaining Multiple Statements](#using-block-macro-with-cancel-statement-also-within-block-for-chaining-multiple-statements)
  - [Using `block!` Macro for Chaining Multiple Statements](#using-block-macro-for-chaining-multiple-statements)
- [Less Recommended Approach](#less-recommended-approach)
  - [Chaining Multiple Statements Directly](#chaining-multiple-statements-directly)

## Recommended Approaches

### Using `block!` Macro with Cancel Statement also Within Block for Chaining Multiple Statements

To perform a transaction and cancel it, discarding any changes made within the
transaction, you can use the `block!` macro to chain multiple statements
together. The `cancel_transaction` statement is used within the `block!` macro
to explicitly indicate the cancellation of the transaction. Let's take a look at
an example:

```rust
let db = Surreal::new::<Mem>(()).await.unwrap();
db.use_ns("test").use_db("test").await.unwrap();

let ref id1 = Account::create_id("one".into());
let ref id2 = Account::create_id("two".into());
let acc = Account::schema();

let amount_to_transfer = 300.00;

block! {
    BEGIN TRANSACTION;

    LET acc1 = create().content(Account {
        id: id1.clone(),
        balance: 135_605.16,
    });
    LET acc2 = create().content(Account {
        id: id2.clone(),
        balance: 91_031.31,
    });

    LET updated1 = update::<Account>(id1).set(acc.balance.increment_by(amount_to_transfer));
    LET update2 = update::<Account>(id2).set(acc.balance.decrement_by(amount_to_transfer));

    CANCEL TRANSACTION;
};

Ok(())
```

In the code snippet above, the `block!` macro is used to define a transaction
with multiple statements. The `LET` statement is used to bind variables `acc1`,
`acc2`, `updated1`, and `update2` to the respective statements. The
`BEGIN TRANSACTION` statement marks the start of the transaction, and the
`CANCEL TRANSACTION` statement explicitly cancels the transaction.

The generated SQL query for this code block would look like:

```sql
BEGIN TRANSACTION;

LET acc1 = CREATE account CONTENT { balance: 135605.16, id: account:one };
LET acc2 = CREATE account CONTENT { balance: 91031.31, id: account:two };
LET updated1 = UPDATE account:one SET balance += 300.0;
LET update2 = UPDATE account:two SET balance -= 300.0;

CANCEL TRANSACTION;
```

Using the `block!` macro with the `cancel_transaction` statement within the
block provides a clear and concise way to define a transaction and cancel it.

### Using `block!` Macro for Chaining Multiple Statements

Another recommended approach is to use the `block!` macro to chain multiple
statements together within a transaction. The `cancel_transaction` statement is
called separately after the `block!` macro to explicitly cancel the transaction.
Let's see an example:

```rust
let db = Surreal::new::<Mem>(()).await.unwrap();
db.use_ns("test").use_db

("test").await.unwrap();

let ref id1 = Account::create_id("one".into());
let ref id2 = Account::create_id("two".into());
let acc = Account::schema();

let amount_to_transfer = 300.00;

let transaction_query = begin_transaction()
    .query(block! {
        LET acc1 = create().content(Account {
            id: id1.clone(),
            balance: 135_605.16,
        });
        LET acc2 = create().content(Account {
            id: id2.clone(),
            balance: 91_031.31,
        });

        LET updated1 = update::<Account>(id1).set(acc.balance.increment_by(amount_to_transfer));
        LET update2 = update::<Account>(id2).set(acc.balance.decrement_by(amount_to_transfer));
    })
    .cancel_transaction();

transaction_query.run(db.clone()).await?;

// Assertions and other code...

Ok(())
```

In this approach, the `block!` macro is used to define a transaction with
multiple statements. The `LET` statement is used to bind variables to the
statements within the block. After the `block!` macro, the `cancel_transaction`
statement is called separately to cancel the transaction.

The generated SQL query for this code block would be the same as the previous
approach.

Using the `block!` macro for chaining multiple statements and explicitly
canceling the transaction provides a structured and organized way to handle
complex transactions.

## Less Recommended Approach

The less recommended approach involves chaining multiple statements directly
without using the `block!` macro. Although functional, this approach may feel
less ergonomic, especially when there is a need to bind and share variables
within the statements.

### Chaining Multiple Statements Directly

Here's an example of chaining multiple statements directly without using the
`block!` macro:

```rust
#[tokio::test]
async fn test_transaction_cancel_increment_and_decrement_update() -> SurrealOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let ref id1 = Account::create_id("one".into());
    let ref id2 = Account::create_id("two".into());
    let amount_to_transfer = 300.00;

    let acc = Account::schema();

    begin_transaction()
        .query(create().content(Account {
            id: id1.clone(),
            balance: 135_605.16,
        }))
        .query(create().content(Account {
            id: id2.clone(),
            balance: 91_031.31,
        }))
        .query(update::<Account>(id1).set(acc.balance.increment_by(amount_to_transfer)))
        .query(update::<Account>(id2).set(acc.balance.decrement_by(amount_to_transfer)))
        .cancel_transaction()
        .run(db.clone())
        .await?;

    // Assertions and other code...

    Ok(())
}
```

In this approach, multiple statements are chained directly within the
transaction. The `create` and `update` statements are used to perform operations
on the `Account` table.

The generated SQL query for this code block would be the same as the previous
approaches.

The less recommended approach of chaining multiple statements directly can be
less ergonomic, especially when dealing with complex transactions that require
variable bindings and subqueries.

It is generally recommended to use the recommended approaches with the `block!`
macro for better readability, automation of variable bindings, and subquery
handling.

That concludes the documentation for the `cancel` statement in SurrealDB ORM.
Use the recommended approaches to perform transaction cancellation effectively.
