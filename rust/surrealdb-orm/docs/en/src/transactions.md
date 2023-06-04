# Transactions in SurrealDB ORM

Transactions play a crucial role in ensuring data integrity and consistency when working with databases. SurrealDB ORM provides convenient methods to handle transactions effectively. This documentation explains how to use transactions in SurrealDB ORM and the available options for committing or canceling transactions.

## Table of Contents

- [Introduction to Transactions](#introduction-to-transactions)
- [Begin Transaction](#begin-transaction)
- [Commit Transaction](#commit-transaction)
- [Cancel Transaction](#cancel-transaction)

## Introduction to Transactions

A transaction is a logical unit of work that consists of multiple database operations. It allows you to group these operations together, ensuring that they are executed as a single, atomic unit. In SurrealDB ORM, you can perform transactions using the provided methods and macros.

## Begin Transaction

To start a transaction, you can use the `begin_transaction` method. This method creates a new transaction and sets the database connection in the transaction mode. Here's an example:

```rust
let db = Surreal::new::<Mem>(()).await.unwrap();
db.use_ns("test").use_db("test").await.unwrap();

let transaction = begin_transaction().run(db.clone()).await?;
```

In the code snippet above, the `begin_transaction` method is called to initiate a new transaction. The `run` method is used to execute the transaction on the specified database connection. The `await?` operator is used to handle any potential errors that may occur during the transaction execution.

## Commit Transaction

To commit a transaction and save the changes made within the transaction, you can use the `commit_transaction` method. This method finalizes the transaction and makes the changes permanent in the database. Here's an example:

### Recommended Approaches

There are two recommended approaches for performing transactions and committing changes using SurrealDB ORM.

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

In the code snippet above, the `block!` macro is used to define a transaction with multiple statements. The `LET` statement is used to bind variables `acc1`, `acc2`, `updated1`, and `update2` to the respective statements. The `BEGIN TRANSACTION` statement marks the start of the transaction, and the `COMMIT TRANSACTION` statement explicitly commits the transaction.

The generated SQL query for this code block would look like:

```sql
BEGIN TRANSACTION;

LET acc1 = CREATE account CONTENT { balance: 135605.16, id: account:one };
LET

 acc2 = CREATE account CONTENT { balance: 91031.31, id: account:two };
LET updated1 = UPDATE account:one SET balance += 300.0;
LET update2 = UPDATE account:two SET balance -= 300.0;

COMMIT TRANSACTION;
```

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

The generated SQL query for this code block would be the same as the previous approach.

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

    // Assertions and other code

...

    Ok(())
}
```

In this approach, multiple statements are chained directly within the transaction. The `create` and `update` statements are used to perform operations on the `Account` table.

The generated SQL query for this code block would be the same as the previous approaches.

The less recommended approach of chaining multiple statements directly can be less ergonomic, especially when dealing with complex transactions that require variable bindings and subqueries.

It is generally recommended to use the recommended approaches with the `block!` macro for better readability, automation of variable bindings, and subquery handling.

## Cancel Transaction

In some cases, you may need to cancel a transaction and discard the changes made within it. SurrealDB ORM provides the `cancel_transaction` method for this purpose. This method cancels the ongoing transaction and rolls back any modifications made. Here's an example:

```rust
let db = Surreal::new::<Mem>(()).await.unwrap();
db.use_ns("test").use_db("test").await.unwrap();

let transaction_query = begin_transaction()
    .query(/* Database operations within the transaction */)
    .cancel_transaction();

transaction_query.run(db.clone()).await?;
```

In the code snippet above, the `cancel_transaction` method is called after the database operations within the transaction are defined. The `run` method is used to execute the transaction and cancel it, discarding any changes made within it.

## Handling Transactions with Database Operations

To perform database operations within a transaction, SurrealDB ORM provides various methods and macros. You can use the `query` method or the `block!` macro to chain multiple statements together within a transaction. Here's an example using the `block!` macro:

```rust
let db = Surreal::new::<Mem>(()).await.unwrap();
db.use_ns("test").use_db("test").await.unwrap();

let transaction_query = begin_transaction()
    .query(block! {
        BEGIN TRANSACTION;

        // Database operation 1
        create().content(/* ... */);

        // Database operation 2
        update(/* ... */);
    })
    .commit_transaction();

transaction_query.run(db.clone()).await?;
```

In this example, the `block!` macro is used to define a transaction block that includes multiple statements. The `BEGIN TRANSACTION` and `COMMIT TRANSACTION` statements mark the start and end of the transaction, respectively. Database operations like `create` and `update` are performed within the transaction block.

## Conclusion

Transactions are an essential feature of SurrealDB ORM for maintaining data integrity and consistency. They allow you to group multiple database operations as a single atomic unit. SurrealDB ORM provides convenient methods and macros for handling transactions effectively. It is recommended to use the `block!` macro for chaining multiple statements within a transaction and explicitly committing or canceling the transaction using the provided methods.

Choose the recommended approaches for better readability, automated variable bindings, and subquery handling in your transactions.
