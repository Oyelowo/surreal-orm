/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use pretty_assertions::assert_eq;
use surreal_models::{account, Account, Balance};
use surreal_orm::{
    statements::{begin_transaction, create, create_only, let_, select, update},
    *,
};
use surrealdb::{engine::local::Mem, Surreal};

#[tokio::test]
async fn test_simple_standalone_for_loop() -> SurrealOrmResult<()> {
    let account::Schema { balance, .. } = Account::schema();
    let account_table = Account::table_name();

    let query = query_turbo! {
        for name in vec!["Oyelowo", "Oyedayo"] {
            let new_bal = 5;
            select(All).from(account_table).where_(balance.eq(new_bal));
        };
    };

    insta::assert_display_snapshot!(query.to_raw().build());
    insta::assert_display_snapshot!(query.fine_tune_params());

    Ok(())
}

// with multiple for loops
#[tokio::test]
async fn test_multiple_for_loops() -> SurrealOrmResult<()> {
    let account::Schema { balance, .. } = Account::schema();
    let account_table = &Account::table_name();

    let names_closure = || vec!["Oyelowo", "Oyedayo"];

    let query = query_turbo! {
        for name in vec!["Oyelowo", "Oyedayo"] {
            let new_bal = 5;
            select(All).from(account_table).where_(balance.eq(new_bal));

            let names = vec!["Oyelowo", "Oyedayo"];
            for name in names {
                let amount_to_use = 999;
                create_only::<Balance>().set(object!(Balance {
                    id: Balance::create_id("balance1".to_string()),
                    amount: amount_to_use,
                }));
            };
        };

        for name in  names_closure() {
            let new_bal = 5;
            select(All).from(account_table).where_(balance.eq(new_bal));
        };
    };

    insta::assert_display_snapshot!(query.to_raw().build());
    insta::assert_display_snapshot!(query.fine_tune_params());

    Ok(())
}

// with simple standalone if else statement
//
// with multiple if else statements
//
// with for loop inside if else statement
//
// with if else statement inside for loop
//
// with for loop inside for loop
// with if else statement inside if else statement
//
// with nested for loop inside if else statement
// with nested if else statement inside for loop
//
// with mixed for loop and if else statement
// with mixed multiple various statements

#[tokio::test]
async fn test_transaction_with_block_macro() -> SurrealOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let id1 = &Account::create_id("one".to_string());
    let id2 = &Account::create_id("two".to_string());
    let amount_to_transfer = 300.00;

    let acc = Account::schema();
    let account::Schema { balance, .. } = Account::schema();

    let names = || vec!["Oyelowo", "Oyedayo"];
    let colors = vec!["red", "blue"];
    let xx = query_turbo! {
        // for (name in  vec!["Oyelowo", "Oyedayo"]) {
        // for name in  vec!["Oyelowo", "Oyedayo"] {
        for name in  names() {
            let first = "Oyelowo";
            select(All).from(Account::table_name()).where_(acc.balance.eq(5));

            for color in colors {
                let something_else = "testing_tested";
                select(All).from(Account::table_name()).where_(acc.balance.eq(5));
            };
        };
    };

    let outside_turbo_cond = balance.greater_than(100);
    let query_chain = query_turbo! {
        begin transaction;

        let within_turbo_cond = balance.equal(33);
        if  within_turbo_cond {
            let first_name = "Oyelowo";
            if balance.equal(33) {
                let username = "Oyelowo";
                if balance.equal(33) {
                    let username = "Oyelowo";
                    if within_turbo_cond {
                        let username = "Oyelowo";
                    };

                    for name in vec!["Oyelowo", "Oyedayo"] {
                        let first = "Oyelowo";
                        select(All).from(Account::table_name()).where_(acc.balance.eq(5));
                    };

                };
            } else {
                let score = 100;
                select(All).from(Account::table_name()).where_(acc.balance.eq(5));
            };
        } else if balance.less_than(100) {
            let first_name = "Oyelowo";
            let score = 100;
            select(All).from(Account::table_name()).where_(acc.balance.eq(5));

        } else if (balance.gte(100)) {
           let first_name = "Oyelowo";
            let score = 100;
            return select(All).from(Account::table_name()).where_(acc.balance.eq(5));
        } else {
            let first_name = "Oyelowo";
            let score = 100;
            select(All).from(Account::table_name()).where_(acc.balance.eq(5));
        };
        let score = 100;

        select(All).from(Account::table_name()).where_(acc.balance.eq(5));

        if (balance.greater_than(100)) {
            let first_name = "Oyelowo";
        };
        commit transaction;
    };

    let query_chain = query_turbo! {
        begin transaction;
         let balance1 = create_only().content(Balance {
                id: Balance::create_id("balance1".to_string()),
                amount: amount_to_transfer,
            });

         create_only().content(Balance {
                id: Balance::create_id("balance2".to_string()),
                amount: amount_to_transfer,
            });

        if balance.greater_than(100) {
            let first_name = "Oyelowo";
            let score = 100;
            select(All).from(Account::table_name()).where_(acc.balance.eq(5));
        } else if balance.less_than(100) {
            let first_name = "Oyelowo";
            let score = 100;
            select(All).from(Account::table_name()).where_(acc.balance.eq(5));
        } else if balance.gte(100) {
            let first_name = "Oyelowo";
            let score = 100;
            select(All).from(Account::table_name()).where_(acc.balance.eq(5));
        } else {
            let first_name = "Oyelowo";
            let score = 100;
            select(All).from(Account::table_name()).where_(acc.balance.eq(5));
        };

        // You can also use parenthesis for the iteration if you want
        // for (name in vec!["Oyelowo", "Oyedayo"]) {
        // let names = vec!["Oyelowo", "Oyedayo"];
        // for name in  names {
        for name in vec!["Oyelowo", "Oyedayo"] {
            let first = "Oyelowo";
            select(All).from(Account::table_name()).where_(acc.balance.eq(5));

            let good_stmt = select(All).from(Account::table_name()).where_(acc.balance.eq(64));

            if balance.gt(50) {
                let first_name = "Oyelowo";
            };

            select(All).from(Account::table_name()).where_(acc.balance.eq(34));

            let numbers = vec![23, 98];

            for age in numbers {
              let score = 100;
              let first_stmt = select(All).from(Account::table_name()).where_(acc.balance.eq(5));

              let second_stmt = select(All).from(Account::table_name()).where_(acc.balance.eq(25));
              select(All).from(Account::table_name()).where_(acc.balance.eq(923));

            };
        };

         let  balance3 = create().content(Balance {
                id: Balance::create_id("balance3".into()),
                amount: amount_to_transfer,
            });

        let accounts = select(All)
            .from(id1..=id2);


        // You can reference the balance object by using the $balance variable and pass the amount
        // as a parameter to the decrement_by function. i.e $balance.amount
        let updated1 = update::<Account>(id1).set(acc.balance.increment_by(balance1.with_path::<Balance>(E).amount));
        update::<Account>(id1).set(acc.balance.increment_by(balance1.with_path::<Balance>(E).amount));
        update::<Account>(id1).set(acc.balance.increment_by(45.3));

        // You can also pass the amount directly to the decrement_by function. i.e 300.00
        let update2 = update::<Account>(id2).set(acc.balance.decrement_by(amount_to_transfer));
        // FIX: Scenario where the same variable is used twice, only the latest let statement value
        // is duplicated because we are using the bound variable name in the chaining rather than
        // the value itself. So, either probably consider suffixing the variable name with a random
        // id or using the values directly in the binding.
        // Or probably doing the chaining immediately after declaration and aggregating that after
        // the entire chained queries.
        let update3 = update::<Account>(id2).set(acc.balance.decrement_by(50));

        commit transaction;
    };

    insta::assert_display_snapshot!(query_chain.to_raw().build());
    insta::assert_display_snapshot!(query_chain.fine_tune_params());

    let result = query_chain.run(db.clone()).await?;

    let accounts = select(All)
        .from(id1..=id2)
        .return_many::<Account>(db.clone())
        .await?;

    assert_eq!(accounts.len(), 2);
    assert_eq!(accounts[0].balance, 645.3);
    assert_eq!(accounts[1].balance, -350.0);
    assert_eq!(accounts[0].id.to_string(), "account:one");
    assert_eq!(accounts[1].id.to_string(), "account:two");
    Ok(())
}
