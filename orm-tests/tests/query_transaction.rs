/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use pretty_assertions::assert_eq;
use surreal_models::{Account, Balance};
use surreal_orm::{
    statements::{begin_transaction, create_only, select, update},
    *,
};
use surrealdb::{engine::local::Mem, Surreal};
// CREATE account:one SET balance = 135,605.16;
// CREATE account:two SET balance = 91,031.31;
// -- Move money
// UPDATE account:one SET balance += 300.00;
// UPDATE account:two SET balance -= 300.00;
// test Increment update and decrement update

#[tokio::test]
async fn test_transaction_with_surreal_queries_macro() -> SurrealOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let id1 = &Account::create_id("one".into());
    let id2 = &Account::create_id("two".into());
    let acc = Account::schema();

    let amount_to_transfer = 300.00;
    let transaction_query = begin_transaction()
        .query(query_turbo!(
            let balance = create_only().content(Balance {
                id: Balance::create_id("balance1".into()),
                amount: amount_to_transfer,
            });

            create_only().content(Account {
                id: id1.clone(),
                balance: 135_605.16,
            });

            create_only().content(Account {
                id: id2.clone(),
                balance: 91_031.31,
            });

        // You can reference the balance object by using the $balance variable and pass the amount
        // as a parameter to the decrement_by function. i.e $balance.amount
            update::<Account>(id1).set(acc.balance.increment_by(balance.with_path::<Balance>(E).amount));
        // You can also pass the amount directly to the decrement_by function. i.e 300.00
            update::<Account>(id2).set(acc.balance.decrement_by(amount_to_transfer));
        ))
        .commit_transaction();

    transaction_query.run(db.clone()).await?;

    let accounts = select(All)
        .from(id1..=id2)
        .return_many::<Account>(db.clone())
        .await?;

    assert_eq!(
        transaction_query.to_raw().build(),
        "BEGIN TRANSACTION;\n\n\
            LET $balance = (CREATE ONLY balance CONTENT { amount: 300f, id: balance:balance1 });\n\n\
            CREATE ONLY account CONTENT { balance: 135605.16f, id: account:one };\n\n\
            CREATE ONLY account CONTENT { balance: 91031.31f, id: account:two };\n\n\
            UPDATE account:one SET balance += $balance.amount;\n\n\
            UPDATE account:two SET balance -= 300f;\n\n\
            COMMIT TRANSACTION;\n\t"
    );

    assert_eq!(
        transaction_query.fine_tune_params(),
        "BEGIN TRANSACTION;\n\n\
            LET $balance = $_param_00000001;\n\n\
            CREATE ONLY account CONTENT $_param_00000002;\n\n\
            CREATE ONLY account CONTENT $_param_00000003;\n\n\
            UPDATE $_param_00000004 SET balance += $balance.amount;\n\n\
            UPDATE $_param_00000005 SET balance -= $_param_00000006;\n\n\
            COMMIT TRANSACTION;\n\t"
    );

    insta::assert_display_snapshot!(transaction_query.fine_tune_params());
    insta::assert_display_snapshot!(transaction_query.to_raw().build());

    assert_eq!(accounts.len(), 2);
    assert_eq!(accounts[0].balance, 135_905.16);
    assert_eq!(accounts[1].balance, 90_731.31);
    assert_eq!(accounts[0].id.to_string(), "account:one");
    assert_eq!(accounts[1].id.to_string(), "account:two");

    Ok(())
}
#[tokio::test]
async fn test_transaction_with_block_macro() -> SurrealOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let id1 = &Account::create_id("one".to_string());
    let id2 = &Account::create_id("two".to_string());
    let amount_to_transfer = 300.00;

    let acc = Account::schema();

    transaction! {
        BEGIN TRANSACTION;

        let balance = create_only().content(Balance {
                id: Balance::create_id("balance1".into()),
                amount: amount_to_transfer,
            });

         create_only().content(Account {
            id: id1.clone(),
            balance: 135_605.16,
        });
        
        create_only().content(Account {
            id: id2.clone(),
            balance: 91_031.31,
        });

        // You can reference the balance object by using the $balance variable and pass the amount
        // as a parameter to the decrement_by function. i.e $balance.amount
        update::<Account>(id1).set(acc.balance.increment_by(balance.with_path::<Balance>(E).amount));

        // You can also pass the amount directly to the decrement_by function. i.e 300.00
        update::<Account>(id2).set(acc.balance.decrement_by(amount_to_transfer));

        COMMIT TRANSACTION;
    }
    .run(db.clone())
    .await?;

    let accounts = select(All)
        .from(id1..=id2)
        .return_many::<Account>(db.clone())
        .await?;

    assert_eq!(accounts.len(), 2);
    assert_eq!(accounts[0].balance, 135_905.16);
    assert_eq!(accounts[1].balance, 90_731.31);
    assert_eq!(accounts[0].id.to_string(), "account:one");
    assert_eq!(accounts[1].id.to_string(), "account:two");

    Ok(())
}

#[tokio::test]
async fn test_transaction_commit_increment_and_decrement_update() -> SurrealOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let id1 = &Account::create_id("one".into());
    let id2 = &Account::create_id("two".into());
    let amount_to_transfer = 300.00;

    let acc = Account::schema();

    begin_transaction()
        .query(create_only().content(Account {
            id: id1.clone(),
            balance: 135_605.16,
        }))
        .query(create_only().content(Account {
            id: id2.clone(),
            balance: 91_031.31,
        }))
        .query(update::<Account>(id1).set(acc.balance.increment_by(amount_to_transfer)))
        .query(update::<Account>(id2).set(acc.balance.decrement_by(amount_to_transfer)))
        .commit_transaction()
        .run(db.clone())
        .await?;

    let accounts = select(All)
        .from(id1..=id2)
        .return_many::<Account>(db.clone())
        .await?;

    assert_eq!(accounts.len(), 2);
    assert_eq!(accounts[0].balance, 135_905.16);
    assert_eq!(accounts[1].balance, 90_731.31);
    assert_eq!(accounts[0].id.to_string(), "account:one");
    assert_eq!(accounts[1].id.to_string(), "account:two");

    Ok(())
}

#[tokio::test]
async fn test_transaction_cancellation_increment_and_decrement_update() -> SurrealOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let id1 = &Account::create_id("one".into());
    let id2 = &Account::create_id("two".into());

    let acc = Account::schema();

    begin_transaction()
        .query(create_only().content(Account {
            id: id1.clone(),
            balance: 135_605.16,
        }))
        .query(create_only().content(Account {
            id: id2.clone(),
            balance: 91_031.31,
        }))
        .query(update::<Account>(id1).set(acc.balance.increment_by(300.00)))
        .query(update::<Account>(id2).set(acc.balance.decrement_by(300.00)))
        .cancel_transaction()
        .run(db.clone())
        .await?;

    let accounts = select(All)
        .from(id1..=id2)
        .return_many::<Account>(db.clone())
        .await?;

    assert_eq!(accounts.len(), 0);
    Ok(())
}
