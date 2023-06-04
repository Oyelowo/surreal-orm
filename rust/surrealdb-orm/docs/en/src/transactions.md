# Transaction Management in SurrealDB ORM

SurrealDB ORM provides transaction management capabilities to ensure the integrity and consistency of database operations. This allows you to group multiple database operations into a single atomic unit that can be committed or canceled as a whole. This documentation covers the Begin Transaction, Commit Statement, and Cancel Transaction features in SurrealDB ORM.

## Table of Contents

- [Begin Transaction](#begin-transaction)
- [Commit Statement](#commit-statement)
  - [Recommended Approaches](#recommended-approaches)
    - [Using `block!` Macro with Commit Statement also Within Block for Chaining Multiple Statements](#using-block-macro-with-commit-statement-also-within-block-for-chaining-multiple-statements)
    - [Using `block!` Macro for Chaining Multiple Statements](#using-block-macro-for-chaining-multiple-statements)
  - [Less Recommended Approach](#less-recommended-approach)
    - [Chaining Multiple Statements Directly](#chaining-multiple-statements-directly)
- [Cancel Transaction](#cancel-transaction)
- [Handling Transactions with Database Operations](#handling-transactions-with-database-operations)

## Begin Transaction

The `begin_transaction` statement in SurrealDB ORM marks the beginning of a transaction. It sets the context for a series of database operations that should be treated as a single atomic unit. By starting a transaction, you can ensure the integrity and consistency of your database operations.

To begin a transaction, you can use the `begin_transaction` statement. Let's see an example:

```rust
let db = Surreal::new::<Mem>(()).await.unwrap();
db.use_ns("test").use_db("test").await.unwrap();

begin_transaction();

// Perform database operations within the transaction

Ok(())
```

In the code snippet above, the `begin_transaction` statement is used to start a transaction. This sets the context for the subsequent database operations.

## Commit Statement

The `commit` statement in SurrealDB ORM is used to commit a transaction and save the changes made within the transaction. It ensures that the changes are durable and permanent in the database.

### Recommended Approaches

#### Using `block!` Macro with Commit Statement also Within Block for Chaining Multiple Statements

To perform a transaction and commit the changes, you can use the `block!` macro to chain multiple statements together. The `commit_transaction` statement is used within the `block!` macro to explicitly indicate the commitment of the transaction. Let's take a look at an example:

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

    COMMIT TRANSACTION;
};

Ok(())
```

In the code snippet above, the `block!` macro is used to define a transaction with multiple statements. The `LET` statement is used to bind variables `

acc1`, `acc2`, `updated1`, and `update2`to the respective statements. The`BEGIN TRANSACTION`statement marks the start of the transaction, and the`COMMIT TRANSACTION` statement explicitly commits the transaction.

Using the `block!` macro with the `commit_transaction` statement within the block provides a clear and concise way to define a transaction and commit the changes.

#### Using `block!` Macro for Chaining Multiple Statements

Another recommended approach is to use the `block!` macro to chain multiple statements together within a transaction. The `commit_transaction` statement is called separately after the `block!` macro to explicitly commit the transaction. Let's see an example:

```rust
let db = Surreal::new::<Mem>(()).await.unwrap();
db.use_ns("test").use_db("test").await.unwrap();

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
    .commit_transaction();

transaction_query.run(db.clone()).await?;

Ok(())
```

In this approach, the `block!` macro is used to define a transaction block that includes multiple statements. The `BEGIN TRANSACTION` and `COMMIT TRANSACTION` statements mark the start and end of the transaction, respectively. The `LET` statement is used to bind variables to the statements within the block.

Using the `block!` macro for chaining multiple statements and explicitly committing the transaction provides a more structured and organized way to handle complex transactions.

### Less Recommended Approach

The less recommended approach involves chaining multiple statements directly without using the `block!` macro. Although functional, this approach may feel less ergonomic, especially when there is a need to bind and share variables within the statements.

#### Chaining Multiple Statements Directly

Here's an example of chaining multiple statements directly without using the `block!` macro:

```rust
#[tokio::test]
async fn test_transaction_commit_increment_and_decrement_update() -> SurrealdbOrmResult<()> {
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
        .commit_transaction()
        .run(db.clone())
        .await?;

    // Assertions and other code...

    Ok(())
}
```

In this approach, multiple statements are chained directly within the transaction. The `create` and `update` statements

are used to perform operations on the `Account` table.

The less recommended approach of chaining multiple statements directly can be less ergonomic, especially when dealing with complex transactions that require variable bindings and subqueries.

It is generally recommended to use the recommended approaches with the `block!` macro for better readability, automation of variable bindings, and subquery handling.

## Cancel Transaction

The `cancel` transaction feature in SurrealDB ORM allows you to roll back a transaction and discard the changes made within the transaction. It is useful when you want to undo a series of database operations within a transaction.

To cancel a transaction, you can use the `cancel_transaction` statement. Let's see an example:

```rust
let db = Surreal::new::<Mem>(()).await.unwrap();
db.use_ns("test").use_db("test").await.unwrap();

let transaction_query = begin_transaction()
    .query(create().content(Account {
        id: Account::create_id("one".into()),
        balance: 135_605.16,
    }))
    .cancel_transaction();

transaction_query.run(db.clone()).await?;

Ok(())
```

In the code snippet above, the `cancel_transaction` statement is used to cancel the ongoing transaction. This ensures that any changes made within the transaction are discarded, and the database state remains unchanged.

## Handling Transactions with Database Operations

When performing database operations within a transaction, it is important to ensure that the operations are executed as a single atomic unit. SurrealDB ORM provides transaction management features to facilitate this.

To handle transactions with database operations, you can follow these steps:

1. Begin the transaction using the `begin_transaction` statement.
2. Chain the necessary database operations using the appropriate ORM statements.
3. Use the recommended approaches described earlier to define and commit the transaction.
4. If needed, use the `cancel_transaction` statement to cancel the transaction and discard any changes.

By following these steps, you can ensure the integrity and consistency of your database operations and handle transactions effectively.

That concludes the documentation for the Begin Transaction, Commit Statement, and Cancel Transaction features in SurrealDB ORM. Use the recommended approaches to perform transactions, commit changes, handle cancellations, and manage your database operations effectively.
