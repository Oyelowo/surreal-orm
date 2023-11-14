/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use pretty_assertions::assert_eq;
use surreal_models::{Account, Balance};
use surreal_orm::{
    statements::{begin_transaction, create, select, update},
    *,
};
use surrealdb::{engine::local::Mem, Surreal};

#[tokio::test]
async fn test_transaction_with_block_macro() -> SurrealOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let id1 = &Account::create_id("one".to_string());
    let id2 = &Account::create_id("two".to_string());
    let amount_to_transfer = 300.00;

    let acc = Account::schema();
    let xxxx = {
        #[allow(non_snake_case)]
        let ref balance1 =
            surreal_orm::statements::let_("balance1").equal_to(create().content(Balance {
                id: Balance::create_id("balance1".to_string()),
                amount: amount_to_transfer,
            }));
        let _surreal_orm__private__internal_variable_prefix__e6dab861d67247bf9143a127f81a0eba =
            create().content(Balance {
                id: Balance::create_id("balance1".to_string()),
                amount: amount_to_transfer,
            });
        let _surreal_orm__private__internal_variable_prefix__6710e39e970f4b25b9d16eff46c9025d = {
            let name = surreal_orm::Param::new("name");
            let ref first = surreal_orm::statements::let_("first").equal_to("Oyelowo");
            let _surreal_orm__private__internal_variable_prefix__cc7896b084aa43028a3fbb970ca13cc9 =
                select(All)
                    .from(Account::table_name())
                    .where_(acc.balance.eq(5));
            let ref good_stmt = surreal_orm::statements::let_("good_stmt").equal_to(
                select(All)
                    .from(Account::table_name())
                    .where_(acc.balance.eq(64)),
            );
            let _surreal_orm__private__internal_variable_prefix__e1073ae4b43a4ecd84d96fa5fd7ed96d =
                select(All)
                    .from(Account::table_name())
                    .where_(acc.balance.eq(34));
            surreal_orm::statements::for_(name).in_(
                vec!["Oyelowo","Oyedayo"]).block(surreal_orm::chain(first)
                    .chain(_surreal_orm__private__internal_variable_prefix__cc7896b084aa43028a3fbb970ca13cc9)
                    .chain(good_stmt).chain(_surreal_orm__private__internal_variable_prefix__e1073ae4b43a4ecd84d96fa5fd7ed96d).parenthesized())
        };
        let ref balance3 =
            surreal_orm::statements::let_("balance3").equal_to(create().content(Balance {
                id: Balance::create_id("balance1".into()),
                amount: amount_to_transfer,
            }));
        let ref accounts =
            surreal_orm::statements::let_("accounts").equal_to(select(All).from(id1..=id2));
        let ref updated1 = surreal_orm::statements::let_("updated1").equal_to(
            update::<Account>(id1).set(
                acc.balance
                    .increment_by(balance1.with_path::<Balance>(E).amount),
            ),
        );
        let ref update2 = surreal_orm::statements::let_("update2")
            .equal_to(update::<Account>(id2).set(acc.balance.decrement_by(amount_to_transfer)));
        surreal_orm::chain(balance1)
            .chain(
                _surreal_orm__private__internal_variable_prefix__e6dab861d67247bf9143a127f81a0eba,
            )
            .chain(
                _surreal_orm__private__internal_variable_prefix__6710e39e970f4b25b9d16eff46c9025d,
            )
            .chain(balance3)
            .chain(accounts)
            .chain(updated1)
            .chain(update2)
    };

    let query_chain = query_turbo! {

         let balance1 = create().content(Balance {
                id: Balance::create_id("balance1".to_string()),
                amount: amount_to_transfer,
            });

         create().content(Balance {
                id: Balance::create_id("balance1".to_string()),
                amount: amount_to_transfer,
            });

        for name in vec!["Oyelowo", "Oyedayo"] {
            let first = "Oyelowo";

            select(All).from(Account::table_name()).where_(acc.balance.eq(5));

            let good_stmt = select(All).from(Account::table_name()).where_(acc.balance.eq(64));

            select(All).from(Account::table_name()).where_(acc.balance.eq(34));

            for age in vec![23, 98] {
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

    };

    let result = query_chain
        .run(db.clone())
        .await?
        .take::<Vec<Account>>(3)
        .unwrap();

    // .run(db.clone())
    // .await?;
    //
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
    //
    Ok(())
}
