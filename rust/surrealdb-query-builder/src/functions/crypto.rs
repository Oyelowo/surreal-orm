/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

// Crypto functions
// These functions can be used when hashing data, encrypting data, and for securely authenticating users into the database.
//
// Function	Description
// crypto::md5()	Returns the md5 hash of a value
// crypto::sha1()	Returns the sha1 hash of a value
// crypto::sha256()	Returns the sha256 hash of a value
// crypto::sha512()	Returns the sha512 hash of a value
// crypto::argon2::compare()	Compares an argon2 hash to a password
// crypto::argon2::generate()	Generates a new argon2 hashed password
// crypto::pbkdf2::compare()	Compares an pbkdf2 hash to a password
// crypto::pbkdf2::generate()	Generates a new pbkdf2 hashed password
// crypto::scrypt::compare()	Compares an scrypt hash to a password
// crypto::scrypt::generate()	Generates a new scrypt hashed password

use surrealdb::sql;

use crate::{
    sql::{Binding, Buildable, Param, ToRawStatement},
    statements::let_::let_,
    Field,
};

use super::array::Function;

pub(crate) fn create_fn_with_single_value(
    value: impl Into<sql::Value>,
    function_suffix: &str,
) -> Function {
    let value: sql::Value = value.into();
    let binding = Binding::new(value);

    Function {
        query_string: format!(
            "crypto::{function_suffix}({})",
            binding.get_param_dollarised()
        ),
        bindings: vec![binding],
    }
}

pub(crate) fn create_fn_with_two_values(
    value1: impl Into<sql::Value>,
    value2: impl Into<sql::Value>,
    function_suffix: &str,
) -> Function {
    let value1: sql::Value = value1.into();
    let value2: sql::Value = value2.into();
    let binding1 = Binding::new(value1);
    let binding2 = Binding::new(value2);

    Function {
        query_string: format!(
            "crypto::{function_suffix}({}, {})",
            binding1.get_param_dollarised(),
            binding2.get_param_dollarised()
        ),
        bindings: vec![binding1, binding2],
    }
}

macro_rules! create_fn_with_single_arg_value {
    ($function_name: expr) => {
        paste::paste! {
            pub fn [<$function_name _fn>](value: impl Into<sql::Value>) -> Function {
                create_fn_with_single_value(value, $function_name)
            }

            #[macro_export]
            macro_rules! [<cryto_ $function_name>] {
                ( $value:expr ) => {
                    crate::functions::crypto::[<$function_name _fn>]($value)
                };
            }
            pub use [<cryto_ $function_name>] as [<$function_name>];

            #[test]
            fn [<test_ $function_name>]() {
                let result = [<$function_name _fn>]("Oyelowo");
                assert_eq!(result.fine_tune_params(), format!("crypto::{}($_param_00000001)", $function_name));
                assert_eq!(result.to_raw().to_string(), format!("crypto::{}('Oyelowo')", $function_name));
            }

            #[test]
            fn [<test_ $function_name _with_macro>]() {
                let result = [<$function_name>]!("Oyelowo");
                assert_eq!(result.fine_tune_params(), format!("crypto::{}($_param_00000001)", $function_name));
                assert_eq!(result.to_raw().to_string(), format!("crypto::{}('Oyelowo')", $function_name));
            }

            #[test]
            fn [<test_ $function_name _with_field>]() {
                let title = Field::new("title");
                let result = [<$function_name _fn>](title);
                assert_eq!(result.fine_tune_params(), format!("crypto::{}($_param_00000001)", $function_name));
                assert_eq!(result.to_raw().to_string(), format!("crypto::{}(title)", $function_name));
            }

            #[test]
            fn [<test_ $function_name _macro_with_field>]() {
                let title = Field::new("title");
                let result = [<$function_name>]!(title);
                assert_eq!(result.fine_tune_params(), format!("crypto::{}($_param_00000001)", $function_name));
                assert_eq!(result.to_raw().to_string(), format!("crypto::{}(title)", $function_name));
            }
        }
    };
}
create_fn_with_single_arg_value!("md5");
create_fn_with_single_arg_value!("sha1");
create_fn_with_single_arg_value!("sha256");
create_fn_with_single_arg_value!("sha512");

pub mod argon2 {
    use surrealdb::sql;

    use super::{create_fn_with_single_value, create_fn_with_two_values};
    use crate::functions::array::Function;

    pub fn compare(value1: impl Into<sql::Value>, value2: impl Into<sql::Value>) -> Function {
        create_fn_with_two_values(value1, value2, "argon2::compare")
    }

    pub fn generate(value: impl Into<sql::Value>) -> Function {
        create_fn_with_single_value(value, "argon2::generate")
    }
}

pub mod pbkdf2 {
    use surrealdb::sql;

    use crate::functions::array::Function;

    use super::{create_fn_with_single_value, create_fn_with_two_values};
    pub fn compare(value1: impl Into<sql::Value>, value2: impl Into<sql::Value>) -> Function {
        create_fn_with_two_values(value1, value2, "pbkdf2::compare")
    }

    pub fn generate(value: impl Into<sql::Value>) -> Function {
        create_fn_with_single_value(value, "pbkdf2::generate")
    }
}

pub mod scrypt {
    use surrealdb::sql;

    use crate::functions::array::Function;

    use super::{create_fn_with_single_value, create_fn_with_two_values};

    pub fn compare(value1: impl Into<sql::Value>, value2: impl Into<sql::Value>) -> Function {
        create_fn_with_two_values(value1, value2, "scrypt::compare")
    }

    pub fn generate(value: impl Into<sql::Value>) -> Function {
        create_fn_with_single_value(value, "scrypt::generate")
    }
}

#[test]
fn test_argon2_compare() {
    let hash = Param::new("$argon2id$v=19$m=4096,t=3,p=1$pbZ6yJ2rPJKk4pyEMVwslQ$jHzpsiB+3S/H+kwFXEcr10vmOiDkBkydVCSMfRxV7CA");
    let pass = Param::new("the strongest password");
    let result = argon2::compare("Oyelowo", "Oyedayo");
    assert_eq!(
        result.fine_tune_params(),
        "crypto::argon2::compare($_param_00000001, $_param_00000002)"
    );
    assert_eq!(
        result.to_raw().to_string(),
        "crypto::argon2::compare('Oyelowo', 'Oyedayo')"
    );
}

#[test]
fn test_argon2_compare_with_param() {
    let hash = let_("hash").equal("$argon2id$v=19$m=4096,t=3,p=1$pbZ6yJ2rPJKk4pyEMVwslQ$jHzpsiB+3S/H+kwFXEcr10vmOiDkBkydVCSMfRxV7CA");
    let pass = let_("pass").equal("the strongest password");

    let result = argon2::compare(hash.get_param(), pass.get_param());
    assert_eq!(
        result.fine_tune_params(),
        "crypto::argon2::compare($_param_00000001, $_param_00000002)"
    );
    assert_eq!(
        result.to_raw().to_string(),
        "crypto::argon2::compare($hash, $pass)"
    );
}

#[test]
fn test_argon2_generate() {
    let result = argon2::generate("Oyelowo");
    assert_eq!(
        result.fine_tune_params(),
        "crypto::argon2::generate($_param_00000001)"
    );
    assert_eq!(
        result.to_raw().to_string(),
        "crypto::argon2::generate('Oyelowo')"
    );
}

#[test]
fn test_pbkdf2_compare_with_param() {
    let hash = let_("hash").equal("$argon2id$v=19$m=4096,t=3,p=1$pbZ6yJ2rPJKk4pyEMVwslQ$jHzpsiB+3S/H+kwFXEcr10vmOiDkBkydVCSMfRxV7CA");
    let pass = let_("pass").equal("the strongest password");

    let result = pbkdf2::compare(hash.get_param(), pass.get_param());
    assert_eq!(
        result.fine_tune_params(),
        "crypto::pbkdf2::compare($_param_00000001, $_param_00000002)"
    );
    assert_eq!(
        result.to_raw().to_string(),
        "crypto::pbkdf2::compare($hash, $pass)"
    );
}

#[test]
fn test_pbkdf2_generate() {
    let result = pbkdf2::generate("Oyelowo");
    assert_eq!(
        result.fine_tune_params(),
        "crypto::pbkdf2::generate($_param_00000001)"
    );
    assert_eq!(
        result.to_raw().to_string(),
        "crypto::pbkdf2::generate('Oyelowo')"
    );
}

#[test]
fn test_scrypt_compare_with_param() {
    let hash = let_("hash").equal("$argon2id$v=19$m=4096,t=3,p=1$pbZ6yJ2rPJKk4pyEMVwslQ$jHzpsiB+3S/H+kwFXEcr10vmOiDkBkydVCSMfRxV7CA");
    let pass = let_("pass").equal("the strongest password");

    let result = scrypt::compare(hash.get_param(), pass.get_param());
    assert_eq!(
        result.fine_tune_params(),
        "crypto::scrypt::compare($_param_00000001, $_param_00000002)"
    );
    assert_eq!(
        result.to_raw().to_string(),
        "crypto::scrypt::compare($hash, $pass)"
    );
}

#[test]
fn test_scrypt_generate() {
    let result = scrypt::generate("Oyelowo");
    assert_eq!(
        result.fine_tune_params(),
        "crypto::scrypt::generate($_param_00000001)"
    );
    assert_eq!(
        result.to_raw().to_string(),
        "crypto::scrypt::generate('Oyelowo')"
    );
}
