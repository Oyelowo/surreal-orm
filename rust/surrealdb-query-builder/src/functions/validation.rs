/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

// Validation functions
// These functions can be used when checking and validating the format of fields and values.
//
// Function	Description
// is::alphanum()	Checks whether a value has only alphanumeric characters
// is::alpha()	Checks whether a value has only alpha characters
// is::ascii()	Checks whether a value has only ascii characters
// is::domain()	Checks whether a value is a domain
// is::email()	Checks whether a value is an email
// is::hexadecimal()	Checks whether a value is hexadecimal
// is::latitude()	Checks whether a value is a latitude value
// is::longitude()	Checks whether a value is a longitude value
// is::numeric()	Checks whether a value has only numeric characters
// is::semver()	Checks whether a value matches a semver version
// is::uuid()	Checks whether a value is a UUID
//

use surrealdb::sql;

use crate::{
    sql::{Binding, Buildable, Name, ToRawStatement},
    Field,
};

use super::array::Function;

pub fn alphanum(value: impl Into<sql::Value>) -> Function {
    let binding = Binding::new(value);

    Function {
        query_string: format!("is::alphanum({})", binding.get_param_dollarised()),
        bindings: vec![binding],
    }
}

#[test]
fn test_alphanum_with_field() {
    let username = Field::new("username");
    let result = alphanum(username);

    assert_eq!(result.fine_tune_params(), "is::alphanum($_param_00000001)");
    assert_eq!(result.to_raw().to_string(), "is::alphanum(username)");
}
