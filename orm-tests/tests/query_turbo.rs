/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use pretty_assertions::assert_eq;
use surreal_models::{account, Account, Balance};
use surreal_orm::{
    statements::{create, create_only, select, update},
    *,
};
use surrealdb::{engine::local::Mem, Surreal};

#[test]
fn test_duplicate_variable_name_properly_chain_bound_in_query_chain() {
    let account_table = &Account::table();

    let query = query_turbo! {
        let var_name = select(All).from(account_table);
        let var_name = "Oyelowo";
        let var_name = "Oyedayo";
        select(All).from(account_table);
    };

    insta::assert_display_snapshot!(query.to_raw().build());
    insta::assert_display_snapshot!(query.fine_tune_params());
}

#[test]
fn test_duplicate_variable_name_properly_chain_bound_in_query_turbo_transaction() {
    let account_table = &Account::table();

    let query = query_turbo! {
        begin transaction;

        let var_name = select(All).from(account_table);
        let var_name = "Oyelowo";
        let var_name = "Oyedayo";
        select(All).from(account_table);

        commit transaction;
    };

    insta::assert_display_snapshot!(query.to_raw().build());
    insta::assert_display_snapshot!(query.fine_tune_params());
}

#[test]
fn test_duplicate_variable_name_properly_chain_bound_in_query_turbo_block() {
    let account_table = &Account::table();

    let query = query_turbo! {
        let var_name = "Oyelowo";
        let var_name = select(All).from(account_table);
        let var_name = "Oyedayo";
        select(All).from(account_table);

        return var_name;
    };

    insta::assert_display_snapshot!(query.to_raw().build());
    insta::assert_display_snapshot!(query.fine_tune_params());
}

#[test]
fn test_duplicate_variable_name_properly_chain_bound_in_dedicated_transaction() {
    let account_table = &Account::table();

    let query = transaction! {
        BEGIN TRANSACTION;

        let var_name = "Oyelowo";
        let var_name = "Oyedayo";
        let var_name = select(All).from(account_table);
        select(All).from(account_table);

        COMMIT TRANSACTION;
    };

    insta::assert_display_snapshot!(query.to_raw().build());
    insta::assert_display_snapshot!(query.fine_tune_params());
}

#[test]
fn test_duplicate_variable_name_properly_chain_bound_in_dedicated_block() {
    let account_table = &Account::table();

    let query = block! {
        let var_name = select(All).from(account_table);
        let var_name = "Oyedayo";
        let var_name = "Oyelowo";
        select(All).from(account_table);

        return var_name;
    };

    insta::assert_display_snapshot!(query.to_raw().build());
    insta::assert_display_snapshot!(query.fine_tune_params());
}

#[tokio::test]
async fn test_simple_standalone_for_loop() -> SurrealOrmResult<()> {
    let account::Schema { balance, .. } = Account::schema();
    let account_table = Account::table();

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

#[tokio::test]
async fn test_multiple_for_loops() -> SurrealOrmResult<()> {
    let account::Schema { balance, .. } = Account::schema();
    let account_table = &Account::table();

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
#[tokio::test]
async fn test_simple_standalone_if_else() -> SurrealOrmResult<()> {
    let account::Schema { balance, .. } = Account::schema();
    let account_table = &Account::table();

    let second_cond_from_outside = balance.lte(54);
    // let xx = {
    // };

    let query = query_turbo! {
        let first_cond = balance.gte(100);
        if first_cond {
            let first_name = "Oyelowo";
            let score = 100;
            select(All).from(account_table).where_(balance.eq(5));
        } else if second_cond_from_outside {
            let first_name = "Oyelowo";
            let score = 100;
            select(All).from(account_table).where_(balance.eq(5));
        } else if (balance.gte(100)) {
           let first_name = "Oyelowo";
            let score = 100;
            select(All).from(account_table).where_(balance.eq(5));
        } else {
            let first_name = "Oyelowo";
            let score = 100;
            select(All).from(account_table).where_(balance.eq(5));
        };
    };

    insta::assert_display_snapshot!(query.to_raw().build());
    insta::assert_display_snapshot!(query.fine_tune_params());

    Ok(())
}

#[tokio::test]
async fn test_multiple_if_else() -> SurrealOrmResult<()> {
    let account::Schema { balance, .. } = Account::schema();

    let query = query_turbo! {
        let within_turbo_cond = balance.equal(33);
        let cond_username = if  within_turbo_cond {
            let first_name = "Oyelowo";
            if balance.equal(33) {
                let username = "oye";
                if balance.equal(92) {
                    let username = "Oyedayo";
                    if within_turbo_cond {
                        let username = "codebreather";
                    };

                    for name in vec!["Oyelowo", "Oyedayo"] {
                        let first = "Oyelowo";
                        select(All).from(Account::table()).where_(balance.eq(5));
                    };

                };
            } else {
                let score = 100;
                select(All).from(Account::table()).where_(balance.eq(5));
            };
        } else if balance.less_than(100) {
            let first_name = "Oyelowo";
            let score = 100;
            select(All).from(Account::table()).where_(balance.eq(5));

        } else if (balance.gte(100)) {
           let first_name = "Oyelowo";
            let score = 100;
            return select(All).from(Account::table()).where_(balance.eq(5));
        } else {
            let first_name = "Oyelowo";
            let score = 100;
            select(All).from(Account::table()).where_(balance.eq(5));
        };
        let score = 100;

        select(All).from(Account::table()).where_(balance.eq(5));

        if (balance.greater_than(100)) {
            let first_name = "Oyelowo";
        };

        return cond_username;
    };

    insta::assert_display_snapshot!(query.to_raw().build());
    insta::assert_display_snapshot!(query.fine_tune_params());

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
    let account::Schema { balance, .. } = Account::schema();

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
            select(All).from(Account::table()).where_(acc.balance.eq(5));
        } else if balance.less_than(100) {
            let first_name = "Oyelowo";
            let score = 100;
            select(All).from(Account::table()).where_(acc.balance.eq(5));
        } else if balance.gte(100) {
            let first_name = "Oyelowo";
            let score = 100;
            select(All).from(Account::table()).where_(acc.balance.eq(5));
        } else {
            let first_name = "Oyelowo";
            let score = 100;
            select(All).from(Account::table()).where_(acc.balance.eq(5));
        };

        for name in vec!["Oyelowo", "Oyedayo"] {
            let first = "Oyelowo";
            select(All).from(Account::table()).where_(acc.balance.eq(5));

            let good_stmt = select(All).from(Account::table()).where_(acc.balance.eq(64));

            if balance.gt(50) {
                let first_name = "Oyelowo";
            };

            select(All).from(Account::table()).where_(acc.balance.eq(34));

            let numbers = vec![23, 98];

            for age in numbers {
              let score = 100;
              let first_stmt = select(All).from(Account::table()).where_(acc.balance.eq(5));

              let second_stmt = select(All).from(Account::table()).where_(acc.balance.eq(25));
              select(All).from(Account::table()).where_(acc.balance.eq(923));

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
        update::<Account>(id2).set(acc.balance.decrement_by(amount_to_transfer));
        update::<Account>(id2).set(acc.balance.decrement_by(50));

        commit transaction;
    };

    insta::assert_display_snapshot!(query_chain.to_raw().build());
    insta::assert_display_snapshot!(query_chain.fine_tune_params());

    let _result = query_chain.run(db.clone()).await?;

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
