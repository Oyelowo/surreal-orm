/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */

use pretty_assertions::assert_eq;
use surreal_models::Organization;
use surreal_orm::{Buildable, Field, SchemaGetter, SetterAssignable, ToRaw};

#[test]
fn test_setter() {
    let org = Organization::schema();

    org.age.equal_to(34);
    org.age.equal_to(Field::new("age"));
    let org = org.time().connected.equal_to(chrono::Utc::now());

    assert_eq!(org.fine_tune_params(), "time.connected = $_param_00000001");
    assert!(org.to_raw().build().len() > 40,);
}
