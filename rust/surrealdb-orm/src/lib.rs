/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

pub use surrealdb_derive::*;
pub use surrealdb_query_builder::*;

#[test]
fn test_orm_basic() {
    let email = Field::new("email");
    let select_statement = statements::select(All)
        .where_(cond(email.like("@oyelowo")).and(email.is("Oyedayo")))
        .group_by(email)
        .parallel()
        .to_raw();

    insta::assert_display_snapshot!(select_statement);
}
