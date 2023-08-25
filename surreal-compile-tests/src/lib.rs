/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

mod node_field_attributes_tests;

pub use surreal_orm::{
    serde::{Deserialize, Serialize},
    statements::{DefineFieldStatement, Permissions},
    *,
};

pub fn field_define_fn() -> DefineFieldStatement {
    unimplemented!()
}

pub fn permissions_fn() -> Permissions {
    unimplemented!()
}

pub fn assert_fn() -> Filter {
    unimplemented!()
}

pub fn value_fn() -> u8 {
    unimplemented!()
}
