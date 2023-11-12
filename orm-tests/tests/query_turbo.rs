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

    let query_chain = query_turbo! {
         let balance1 = create().content(Balance {
                id: Balance::create_id("balance1".into()),
                amount: amount_to_transfer,
            });

         create().content(Balance {
                id: Balance::create_id("balance1".into()),
                amount: amount_to_transfer,
            });

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
