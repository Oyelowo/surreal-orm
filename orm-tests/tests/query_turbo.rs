/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use pretty_assertions::assert_eq;
use surreal_models::{account, Account, Balance};
use surreal_orm::{
    statements::{begin_transaction, create, select, update},
    *,
};
use surrealdb::{engine::local::Mem, Surreal};

// with simple standalone for loop
// with simple standalone if else statement
//
// with multiple for loops
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
         let balance1 = create().content(Balance {
                id: Balance::create_id("balance1".to_string()),
                amount: amount_to_transfer,
            });

         create().content(Balance {
                id: Balance::create_id("balance1".to_string()),
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
                id: Balance::create_id("balance1".into()),
                amount: amount_to_transfer,
            });

        let accounts = select(All)
            .from(id1..=id2);


        // You can reference the balance object by using the $balance variable and pass the amount
        // as a parameter to the decrement_by function. i.e $balance.amount
        let updated1 = update::<Account>(id1).set(acc.balance.increment_by(balance1.with_path::<Balance>(E).amount));

        // You can also pass the amount directly to the decrement_by function. i.e 300.00
        let update2 = update::<Account>(id2).set(acc.balance.decrement_by(amount_to_transfer));
        commit transaction;
    };

    insta::assert_display_snapshot!(query_chain.to_raw().build());
    insta::assert_display_snapshot!(query_chain.fine_tune_params());

    // TODO: Update db engine and also figure out why this is not being executed.
    // Got this error:
    // called `Result::unwrap()` on an `Err` value: Db(QueryNotExecuted)
    // let result = query_chain
    //     .run(db.clone())
    //     .await?
    //     .take::<Vec<Account>>(0)
    //     .unwrap();
    //
    // // assert_eq!(result.len(), 3);
    // // assert_eq!(result[0].balance, 35_605.16);
    // // assert_eq!(result[1].balance, 90_731.31);
    // //
    // let accounts = select(All)
    //     .from(id1..=id2)
    //     .return_many::<Account>(db.clone())
    //     .await?;
    //
    // assert_eq!(accounts.len(), 2);
    // assert_eq!(accounts[0].balance, 135_605.16);
    // assert_eq!(accounts[1].balance, 90_731.31);
    // assert_eq!(accounts[0].id.to_string(), "account:one");
    // assert_eq!(accounts[1].id.to_string(), "account:two");
    Ok(())
}
