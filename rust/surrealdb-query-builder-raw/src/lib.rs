/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

// pub use surrealdb_derive::*;
pub use surrealdb_query_builder::{
    field, filter, json, links, model_id, sql, utils, BindingsList, Clause, Erroneous, ErrorList,
    Field, Operatable, Parametric, RecordId, Schemaful, SurrealdbEdge, SurrealdbModel,
    SurrealdbNode, Table, Value,
};
use surrealdb_query_builder::{sql::All, statements::statements::select, utils::cond};
// use surrealdb_query_builder
// pub mod statements {
//     pub use surrealdb_query_builder_param::statements::*;
//     pub use surrealdb_query_builder_raw::*;
// }

// pub mod prelude {
//     pub use super::statements::*;
//     pub use surrealdb_query_builder::{
//         field, filter, json, links, model_id, sql, utils, BindingsList, Clause, Erroneous,
//         ErrorList, Field, Operatable, Parametric, RecordId, Schemaful, SurrealdbEdge,
//         SurrealdbModel, SurrealdbNode, Table, Value,
//     };
// }

// use statements::select_raw;
// use surrealdb_query_builder_param::statements::select;
// use surrealdb_query_builder_param::utils::cond;

#[test]
fn mananana() {
    let email = Field::new("email");
    let xx = select(All)
        .where_(cond(email.like("@oyelowo")).and(email.is("Oyedayo")))
        .group_by(email)
        .parallel();
    assert_eq!(xx.to_string(), "poe".to_string());
}
