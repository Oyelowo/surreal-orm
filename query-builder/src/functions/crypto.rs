/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
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
// crypto::bcrypt::compare() Compares an bcrypt hash to a password
// crypto::bcrypt::generate() Generates a new bcrypt hashed password
// crypto::pbkdf2::compare()	Compares an pbkdf2 hash to a password
// crypto::pbkdf2::generate()	Generates a new pbkdf2 hashed password
// crypto::scrypt::compare()	Compares an scrypt hash to a password
// crypto::scrypt::generate()	Generates a new scrypt hashed password

use crate::{Buildable, Erroneous, Function, Parametric, StrandLike};

pub(crate) fn create_fn_with_single_value(
    value: impl Into<StrandLike>,
    function_suffix: &str,
) -> Function {
    let value: StrandLike = value.into();

    Function {
        query_string: format!("crypto::{function_suffix}({})", value.build()),
        bindings: value.get_bindings(),
        errors: value.get_errors(),
    }
}

pub(crate) fn create_fn_with_two_values(
    value1: impl Into<StrandLike>,
    value2: impl Into<StrandLike>,
    function_suffix: &str,
) -> Function {
    let value1: StrandLike = value1.into();
    let value2: StrandLike = value2.into();
    let mut bindings = vec![];
    let mut errors = vec![];
    bindings.extend(value1.get_bindings());
    bindings.extend(value2.get_bindings());
    errors.extend(value1.get_errors());
    errors.extend(value2.get_errors());

    Function {
        query_string: format!(
            "crypto::{function_suffix}({}, {})",
            value1.build(),
            value2.build()
        ),
        bindings,
        errors,
    }
}

macro_rules! create_fn_with_single_arg_value {
    ($(#[$attr:meta])* => $function_name: expr) => {
        paste::paste! {
            $(#[$attr])*
            pub fn [<$function_name _fn>](value: impl Into<$crate::StrandLike>) -> $crate::Function {
                create_fn_with_single_value(value, $function_name)
            }

            $(#[$attr])*
            #[macro_export]
            macro_rules! [<cryto_ $function_name>] {
                ( $value:expr ) => {
                    $crate::functions::crypto::[<$function_name _fn>]($value)
                };
            }
            pub use [<cryto_ $function_name>] as [<$function_name>];

            #[cfg(test)]
            mod [<test_ $function_name>] {
                use super::*;
                use crate::*;

                #[test]
                fn [<test_ $function_name _with_str>]() {
                    let result = [<$function_name _fn>]("Oyelowo");
                    assert_eq!(result.fine_tune_params(), format!("crypto::{}($_param_00000001)", $function_name));
                    assert_eq!(result.to_raw().build(), format!("crypto::{}('Oyelowo')", $function_name));
                }

                #[test]
                fn [<test_ $function_name _with_macro>]() {
                    let result = [<$function_name>]!("Oyelowo");
                    assert_eq!(result.fine_tune_params(), format!("crypto::{}($_param_00000001)", $function_name));
                    assert_eq!(result.to_raw().build(), format!("crypto::{}('Oyelowo')", $function_name));
                }

                #[test]
                fn [<test_ $function_name _with_field>]() {
                    let title = $crate::Field::new("title");
                    let result = [<$function_name _fn>](title);
                    assert_eq!(result.fine_tune_params(), format!("crypto::{}(title)", $function_name));
                    assert_eq!(result.to_raw().build(), format!("crypto::{}(title)", $function_name));
                }

                #[test]
                fn [<test_ $function_name _macro_with_field>]() {
                    let title = $crate::Field::new("title");
                    let result = [<$function_name>]!(title);
                    assert_eq!(result.fine_tune_params(), format!("crypto::{}(title)", $function_name));
                    assert_eq!(result.to_raw().build(), format!("crypto::{}(title)", $function_name));
                }
            }
        }
    };
}

create_fn_with_single_arg_value!(
    /// The crypto::md5 function returns the md5 hash of the input valuee.
    ///
    /// # Arguments
    /// * `value` - The value to be hashed. Can also be a field or a param.
    ///
    /// # Example
    /// ```rust
    /// # use surreal_query_builder as  surreal_orm;
    /// use surreal_orm::{*, functions::crypto};
    ///
    /// let result = crypto::md5!("Oyelowo");
    /// assert_eq!(result.fine_tune_params(), "crypto::md5($_param_00000001)");
    /// assert_eq!(result.to_raw().build(), "crypto::md5('Oyelowo')");
    /// ```
    => "md5"
);

create_fn_with_single_arg_value!(
    /// The crypto::sha1 function returns the sha1 hash of the input value.
    ///
    /// # Arguments
    /// * `value` - The value to be hashed. Can also be a field or a param.
    ///
    /// # Example
    /// ```rust
    /// # use surreal_query_builder as  surreal_orm;
    /// use surreal_orm::{*, functions::crypto};
    ///
    /// let result = crypto::sha1!("Oyelowo");
    /// assert_eq!(result.fine_tune_params(), "crypto::sha1($_param_00000001)");
    /// assert_eq!(result.to_raw().build(), "crypto::sha1('Oyelowo')");
    /// ```
    => "sha1"
);

create_fn_with_single_arg_value!(
    /// The crypto::sha256 function returns the sha256 hash of the input value.
    ///
    /// # Arguments
    /// * `value` - The value to be hashed. Can also be a field or a param.
    ///
    /// # Example
    /// ```rust
    /// # use surreal_query_builder as  surreal_orm;
    /// use surreal_orm::{*, functions::crypto};
    ///
    /// let result = crypto::sha256!("Oyelowo");
    /// assert_eq!(result.fine_tune_params(), "crypto::sha256($_param_00000001)");
    /// assert_eq!(result.to_raw().build(), "crypto::sha256('Oyelowo')");
    /// ```
    => "sha256"
);

create_fn_with_single_arg_value!(
    /// The crypto::sha512 function returns the sha512 hash of the input value.
    ///
    /// # Arguments
    /// * `value` - The value to be hashed. Can also be a field or a param.
    ///
    /// # Example
    /// ```rust
    /// # use surreal_query_builder as  surreal_orm;
    /// use surreal_orm::{*, functions::crypto};
    ///
    /// let result = crypto::sha512!("Oyelowo");
    /// assert_eq!(result.fine_tune_params(), "crypto::sha512($_param_00000001)");
    /// assert_eq!(result.to_raw().build(), "crypto::sha512('Oyelowo')");
    /// ```
    => "sha512"
);

/// This module contains functions for working with the argon2 hashing algorithm.
pub mod argon2 {
    use super::{create_fn_with_single_value, create_fn_with_two_values};
    use crate::{Function, StrandLike};

    /// The crypto::argon2::compare function compares a hashed-and-salted argon2 password value with an unhashed password value.
    pub fn compare_fn(hash: impl Into<StrandLike>, pass: impl Into<StrandLike>) -> Function {
        create_fn_with_two_values(hash, pass, "argon2::compare")
    }

    /// The crypto::argon2::compare function compares a hashed-and-salted argon2 password value with an unhashed password value.
    /// Also aliased as `crypto_argon2_compare!`.
    ///
    /// # Arguments
    /// * `hash` - The hashed password value. Can also be a field or a param.
    /// * `pass` - The unhashed password value. Can also be a field or a param.
    ///
    /// # Example
    /// ```rust
    /// # use surreal_query_builder as  surreal_orm;
    /// use surreal_orm::{*, statements::let_, functions::crypto};
    /// let hash = let_("hash").equal_to("$argon2id$v=19$m=4096,t=3,p=1$pbZ6yJ2rPJKk4pyEMVwslQ$jHzpsiB+3S/H+kwFXEcr10vmOiDkBkydVCSMfRxV7CA");
    /// let pass = let_("pass").equal_to("this is a strong password");
    ///
    /// let result = crypto::argon2::compare!(hash.get_param(), pass.get_param());
    /// assert_eq!(result.fine_tune_params(), "crypto::argon2::compare($hash, $pass)");
    /// assert_eq!(result.to_raw().build(), "crypto::argon2::compare($hash, $pass)");
    ///
    /// let hash_field = Field::new("hash_field");
    /// let result = crypto::argon2::compare!(hash_field, "Oyelowo");
    /// assert_eq!(result.fine_tune_params(), "crypto::argon2::compare(hash_field, $_param_00000001)");
    /// assert_eq!(result.to_raw().build(), "crypto::argon2::compare(hash_field, 'Oyelowo')");
    ///
    /// let hash_field = Field::new("hash_field");
    /// let pass = let_("pass").equal_to("Oyelowo");
    /// let result = crypto::argon2::compare!(hash_field, pass.get_param());
    /// assert_eq!(result.fine_tune_params(), "crypto::argon2::compare(hash_field, $pass)");
    /// assert_eq!(result.to_raw().build(), "crypto::argon2::compare(hash_field, $pass)");
    /// ```
    #[macro_export]
    macro_rules! crypto_argon2_compare {
        ( $value1:expr,  $value2:expr ) => {
            $crate::functions::crypto::argon2::compare_fn($value1, $value2)
        };
    }
    pub use crypto_argon2_compare as compare;

    /// The crypto::argon2::generate function hashes and salts a password using the argon2 hashing
    /// algorithm.
    pub fn generate_fn(value: impl Into<StrandLike>) -> Function {
        create_fn_with_single_value(value, "argon2::generate")
    }

    /// The crypto::argon2::generate function hashes and salts a password using the argon2 hashing
    /// algorithm. Also aliased as `crypto_argon2_generate!`.
    /// # Arguments
    ///
    /// * `value` - The password value to be hashed and salted. Can also be a field or a param.
    /// # Example
    /// ```rust
    /// # use surreal_query_builder as  surreal_orm;
    /// use surreal_orm::{*, statements::let_, functions::crypto};
    /// crypto::argon2::generate!("password from jupiter");
    ///
    /// let pass = let_("pass").equal_to("this is a strong password");
    /// let result = crypto::argon2::generate!(pass.get_param());
    /// assert_eq!(result.fine_tune_params(), "crypto::argon2::generate($pass)");
    /// assert_eq!(result.to_raw().build(), "crypto::argon2::generate($pass)");
    #[macro_export]
    macro_rules! crypto_argon2_generate {
        ( $value1:expr) => {
            $crate::functions::crypto::argon2::generate_fn($value1)
        };
    }
    pub use crypto_argon2_generate as generate;
}

/// This module contains functions for working with the bcrypt hashing algorithm.
pub mod bcrypt {
    use super::{create_fn_with_single_value, create_fn_with_two_values};
    use crate::{Function, StrandLike};

    /// The crypto::bcrypt::compare function compares a hashed-and-salted bcrypt password value with an unhashed password value.
    pub fn compare_fn(hash: impl Into<StrandLike>, pass: impl Into<StrandLike>) -> Function {
        create_fn_with_two_values(hash, pass, "bcrypt::compare")
    }

    /// The crypto::bcrypt::compare function compares a hashed-and-salted bcrypt password value with an unhashed password value.
    /// Also aliased as `crypto_bcrypt_compare!`.
    ///
    /// # Arguments
    /// * `hash` - The hashed password value. Can also be a field or a param.
    /// * `pass` - The unhashed password value. Can also be a field or a param.
    ///
    /// # Example
    /// ```rust
    /// # use surreal_query_builder as  surreal_orm;
    /// use surreal_orm::{*, statements::let_, functions::crypto};
    /// let hash = let_("hash").equal_to("bcrypt_hash");
    /// let pass = let_("pass").equal_to("this is a strong password");
    /// let result = crypto::bcrypt::compare!(hash.get_param(), pass.get_param());
    /// assert_eq!(result.fine_tune_params(), "crypto::bcrypt::compare($hash, $pass)");
    /// assert_eq!(result.to_raw().build(), "crypto::bcrypt::compare($hash, $pass)");
    ///
    /// let hash_field = Field::new("hash_field");
    /// let result = crypto::bcrypt::compare!(hash_field, "Oyelowo");
    /// assert_eq!(result.fine_tune_params(), "crypto::bcrypt::compare(hash_field, $_param_00000001)");
    /// assert_eq!(result.to_raw().build(), "crypto::bcrypt::compare(hash_field, 'Oyelowo')");
    /// ```
    #[macro_export]
    macro_rules! crypto_bcrypt_compare {
        ( $value1:expr,  $value2:expr ) => {
            $crate::functions::crypto::bcrypt::compare_fn($value1, $value2)
        };
    }
    pub use crypto_bcrypt_compare as compare;

    /// The crypto::bcrypt::generate function hashes and salts a password using the bcrypt hashing algorithm.
    pub fn generate_fn(value: impl Into<StrandLike>) -> Function {
        create_fn_with_single_value(value, "bcrypt::generate")
    }

    /// The crypto::bcrypt::generate function hashes and salts a password using the bcrypt hashing algorithm.
    /// Also aliased as `crypto_bcrypt_generate!`.
    /// # Arguments
    /// * `value` - The password value to be hashed and salted. Can also be a field or a param.
    /// # Example
    /// ```rust
    /// # use surreal_query_builder as  surreal_orm;
    /// use surreal_orm::{*, statements::let_, functions::crypto};
    /// crypto::bcrypt::generate!("password from jupiter");
    ///
    /// let pass = let_("pass").equal_to("this is a strong password");
    /// let result = crypto::bcrypt::generate!(pass.get_param());
    /// assert_eq!(result.fine_tune_params(), "crypto::bcrypt::generate($pass)");
    /// assert_eq!(result.to_raw().build(), "crypto::bcrypt::generate($pass)");
    /// ```
    #[macro_export]
    macro_rules! crypto_bcrypt_generate {
        ( $value1:expr) => {
            $crate::functions::crypto::bcrypt::generate_fn($value1)
        };
    }
    pub use crypto_bcrypt_generate as generate;
}

/// This module contains functions for working with the pbkdf2 hashing algorithm.
pub mod pbkdf2 {
    use super::{create_fn_with_single_value, create_fn_with_two_values};
    use crate::{Function, StrandLike};

    /// The crypto::pbkdf2::compare function compares a hashed-and-salted pbkdf2 password value
    /// with an unhashed password value.
    pub fn compare_fn(hash: impl Into<StrandLike>, pass: impl Into<StrandLike>) -> Function {
        create_fn_with_two_values(hash, pass, "pbkdf2::compare")
    }

    /// The crypto::pbkdf2::compare function compares a hashed-and-salted pbkdf2 password value
    /// with an unhashed password value. Also aliased as `crypto_pbkdf2_compare!`.
    /// # Arguments
    /// * `hash` - The hashed password value. Can also be a field or a param.
    /// * `pass` - The unhashed password value. Can also be a field or a param.
    ///
    /// # Example
    /// ```rust
    /// # use surreal_query_builder as  surreal_orm;
    /// use surreal_orm::{*, statements::let_, functions::crypto};
    /// let hash = let_("hash").equal_to("pbkdf2$sha256$1000$ZG9uZQ==$MjAxOS0wNC0xMCAxMzowMzowMA==$c2FsdA==");
    /// let pass = let_("pass").equal_to("this is a strong password");
    ///
    /// let result = crypto::pbkdf2::compare!(hash.get_param(), pass.get_param());
    /// assert_eq!(result.fine_tune_params(), "crypto::pbkdf2::compare($hash, $pass)");
    /// assert_eq!(result.to_raw().build(), "crypto::pbkdf2::compare($hash, $pass)");
    ///
    /// let hash_field = Field::new("hash_field");
    /// let result = crypto::pbkdf2::compare!(hash_field, "Oyelowo");
    /// assert_eq!(result.fine_tune_params(), "crypto::pbkdf2::compare(hash_field, $_param_00000001)");
    /// assert_eq!(result.to_raw().build(), "crypto::pbkdf2::compare(hash_field, 'Oyelowo')");
    ///
    /// let hash_field = Field::new("hash_field");
    /// let pass = let_("pass").equal_to("Oyelowo");
    /// let result = crypto::pbkdf2::compare!(hash_field, pass.get_param());
    /// assert_eq!(result.fine_tune_params(), "crypto::pbkdf2::compare(hash_field, $pass)");
    /// assert_eq!(result.to_raw().build(), "crypto::pbkdf2::compare(hash_field, $pass)");
    /// ```
    #[macro_export]
    macro_rules! crypto_pbkdf2_compare {
        ( $value1:expr,  $value2:expr ) => {
            $crate::functions::crypto::pbkdf2::compare_fn($value1, $value2)
        };
    }
    pub use crypto_pbkdf2_compare as compare;

    /// The crypto::pbkdf2::generate function hashes and salts a password using the pbkdf2 hashing algorithm.
    pub fn generate_fn(value: impl Into<StrandLike>) -> Function {
        create_fn_with_single_value(value, "pbkdf2::generate")
    }

    /// The crypto::pbkdf2::generate function hashes and salts a password using the pbkdf2 hashing algorithm. Also aliased as `crypto_pbkdf2_generate!`.
    /// # Arguments
    ///
    /// * `value` - The password value to be hashed and salted. Can also be a field or a param.
    ///
    /// # Example
    /// ```rust
    /// # use surreal_query_builder as  surreal_orm;
    /// use surreal_orm::{*, statements::let_, functions::crypto};
    /// crypto::pbkdf2::generate!("password from jupiter");
    ///
    /// let pass = let_("pass").equal_to("this is a strong password");
    /// let result = crypto::pbkdf2::generate!(pass.get_param());
    /// assert_eq!(result.fine_tune_params(), "crypto::pbkdf2::generate($pass)");
    /// assert_eq!(result.to_raw().build(), "crypto::pbkdf2::generate($pass)");
    /// ```
    #[macro_export]
    macro_rules! crypto_pbkdf2_generate {
        ( $value1:expr) => {
            $crate::functions::crypto::pbkdf2::generate_fn($value1)
        };
    }
    pub use crypto_pbkdf2_generate as generate;
}

/// This module contains functions for working with the scrypt hashing algorithm.
pub mod scrypt {
    use super::{create_fn_with_single_value, create_fn_with_two_values};
    use crate::{Function, StrandLike};

    /// The crypto::scrypt::compare function compares a hashed-and-salted scrypt password value
    /// with an unhashed password value.
    pub fn compare_fn(hash: impl Into<StrandLike>, pass: impl Into<StrandLike>) -> Function {
        create_fn_with_two_values(hash, pass, "scrypt::compare")
    }

    /// The crypto::scrypt::compare function compares a hashed-and-salted scrypt password value
    /// with an unhashed password value. Also aliased as `crypto_scrypt_compare!`.
    /// # Arguments
    ///
    /// * `hash` - The hashed password value. Can also be a field or a param.
    /// * `pass` - The unhashed password value. Can also be a field or a param.
    ///
    /// # Example
    /// ```rust
    /// # use surreal_query_builder as  surreal_orm;
    /// use surreal_orm::{*, statements::let_, functions::crypto};
    /// let hash = let_("hash").equal_to("pbkdf2$sha256$1000$ZG9uZQ==$MjAxOS0wNC0xMCAxMzowMzowMA==$c2FsdA==");
    /// let pass = let_("pass").equal_to("this is a strong password");
    /// crypto::scrypt::compare!(hash.get_param(), pass.get_param());
    ///
    /// let hash_field = Field::new("hash_field");
    /// let result = crypto::scrypt::compare!(hash_field, "Oyelowo");
    /// assert_eq!(result.fine_tune_params(), "crypto::scrypt::compare(hash_field, $_param_00000001)");
    /// assert_eq!(result.to_raw().build(), "crypto::scrypt::compare(hash_field, 'Oyelowo')");
    /// ```
    #[macro_export]
    macro_rules! crypto_scrypt_compare {
        ( $value1:expr,  $value2:expr ) => {
            $crate::functions::crypto::scrypt::compare_fn($value1, $value2)
        };
    }
    pub use crypto_scrypt_compare as compare;

    ///  The crypto::scrypt::generate function hashes and salts a password using the scrypt hashing algorithm.
    pub fn generate_fn(value: impl Into<StrandLike>) -> Function {
        create_fn_with_single_value(value, "scrypt::generate")
    }

    /// The crypto::scrypt::generate function hashes and salts a password using the scrypt hashing algorithm. Also aliased as `crypto_scrypt_generate!`.
    /// # Arguments
    ///
    /// * `value` - The password value to be hashed and salted. Can also be a field or a param.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use surreal_query_builder as  surreal_orm;
    /// use surreal_orm::{*, statements::let_, functions::crypto};
    /// crypto::scrypt::generate!("password from jupiter");
    ///
    /// let pass = let_("pass").equal_to("this is a strong password");
    /// let result = crypto::scrypt::generate!(pass.get_param());
    /// assert_eq!(result.fine_tune_params(), "crypto::scrypt::generate($pass)");
    /// assert_eq!(result.to_raw().build(), "crypto::scrypt::generate($pass)");
    /// ```
    #[macro_export]
    macro_rules! crypto_scrypt_generate {
        ( $value1:expr) => {
            $crate::functions::crypto::scrypt::generate_fn($value1)
        };
    }
    pub use crypto_scrypt_generate as generate;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{statements::let_, *};

    #[test]
    fn test_argon2_compare() {
        let result = argon2::compare_fn("Oyelowo", "Oyedayo");
        assert_eq!(
            result.fine_tune_params(),
            "crypto::argon2::compare($_param_00000001, $_param_00000002)"
        );
        assert_eq!(
            result.to_raw().build(),
            "crypto::argon2::compare('Oyelowo', 'Oyedayo')"
        );
    }

    #[test]
    fn test_argon2_compare_with_param() {
        let hash = let_("hash").equal_to("$argon2id$v=19$m=4096,t=3,p=1$pbZ6yJ2rPJKk4pyEMVwslQ$jHzpsiB+3S/H+kwFXEcr10vmOiDkBkydVCSMfRxV7CA");
        let pass = let_("pass").equal_to("the strongest password");

        let result = argon2::compare_fn(hash.get_param(), pass.get_param());
        assert_eq!(
            result.fine_tune_params(),
            "crypto::argon2::compare($hash, $pass)"
        );
        assert_eq!(
            result.to_raw().build(),
            "crypto::argon2::compare($hash, $pass)"
        );
    }

    #[test]
    fn test_argon2_generate() {
        let result = argon2::generate_fn("Oyelowo");
        assert_eq!(
            result.fine_tune_params(),
            "crypto::argon2::generate($_param_00000001)"
        );
        assert_eq!(
            result.to_raw().build(),
            "crypto::argon2::generate('Oyelowo')"
        );
    }

    #[test]
    fn test_argon2_compare_macro_with_raw_values() {
        let result = argon2::compare!("$argon2id$v=19$m=4096,t=3,p=1$pbZ6yJ2rPJKk4pyEMVwslQ$jHzpsiB+3S/H+kwFXEcr10vmOiDkBkydVCSMfRxV7CA", "the strongest password");
        assert_eq!(
            result.fine_tune_params(),
            "crypto::argon2::compare($_param_00000001, $_param_00000002)"
        );
        assert_eq!(
        result.to_raw().build(),
        "crypto::argon2::compare('$argon2id$v=19$m=4096,t=3,p=1$pbZ6yJ2rPJKk4pyEMVwslQ$jHzpsiB+3S/H+kwFXEcr10vmOiDkBkydVCSMfRxV7CA', 'the strongest password')"
    );
    }

    #[test]
    fn test_argon2_compare_macro_with_fields() {
        let hash = Field::new("hash");
        let pass = Field::new("pass");

        let result = argon2::compare!(hash, pass);
        assert_eq!(
            result.fine_tune_params(),
            "crypto::argon2::compare(hash, pass)"
        );
        assert_eq!(
            result.to_raw().build(),
            "crypto::argon2::compare(hash, pass)"
        );
    }

    #[test]
    fn test_argon2_compare_macro_with_param() {
        let hash = let_("hash").equal_to("$argon2id$v=19$m=4096,t=3,p=1$pbZ6yJ2rPJKk4pyEMVwslQ$jHzpsiB+3S/H+kwFXEcr10vmOiDkBkydVCSMfRxV7CA");
        let pass = let_("pass").equal_to("the strongest password");

        let result = argon2::compare!(hash.get_param(), pass.get_param());
        assert_eq!(
            result.fine_tune_params(),
            "crypto::argon2::compare($hash, $pass)"
        );
        assert_eq!(
            result.to_raw().build(),
            "crypto::argon2::compare($hash, $pass)"
        );
    }

    #[test]
    fn test_argon2_generate_fn() {
        let result = argon2::generate!("Oyelowo");
        assert_eq!(
            result.fine_tune_params(),
            "crypto::argon2::generate($_param_00000001)"
        );
        assert_eq!(
            result.to_raw().build(),
            "crypto::argon2::generate('Oyelowo')"
        );
    }

    // pbkdf2
    #[test]
    fn test_pbkdf2_compare_with_param() {
        let hash = let_("hash").equal_to("$argon2id$v=19$m=4096,t=3,p=1$pbZ6yJ2rPJKk4pyEMVwslQ$jHzpsiB+3S/H+kwFXEcr10vmOiDkBkydVCSMfRxV7CA");
        let pass = let_("pass").equal_to("the strongest password");

        let result = pbkdf2::compare_fn(hash.get_param(), pass.get_param());
        assert_eq!(
            result.fine_tune_params(),
            "crypto::pbkdf2::compare($hash, $pass)"
        );
        assert_eq!(
            result.to_raw().build(),
            "crypto::pbkdf2::compare($hash, $pass)"
        );
    }

    #[test]
    fn test_pbkdf2_generate() {
        let result = pbkdf2::generate_fn("Oyelowo");
        assert_eq!(
            result.fine_tune_params(),
            "crypto::pbkdf2::generate($_param_00000001)"
        );
        assert_eq!(
            result.to_raw().build(),
            "crypto::pbkdf2::generate('Oyelowo')"
        );
    }

    #[test]
    fn test_pbkdf2_compare_macro_with_raw_values() {
        let result = pbkdf2::compare!("$argon2id$v=19$m=4096,t=3,p=1$pbZ6yJ2rPJKk4pyEMVwslQ$jHzpsiB+3S/H+kwFXEcr10vmOiDkBkydVCSMfRxV7CA", "the strongest password");
        assert_eq!(
            result.fine_tune_params(),
            "crypto::pbkdf2::compare($_param_00000001, $_param_00000002)"
        );
        assert_eq!(
        result.to_raw().build(),
        "crypto::pbkdf2::compare('$argon2id$v=19$m=4096,t=3,p=1$pbZ6yJ2rPJKk4pyEMVwslQ$jHzpsiB+3S/H+kwFXEcr10vmOiDkBkydVCSMfRxV7CA', 'the strongest password')"
    );
    }

    #[test]
    fn test_pbkdf2_compare_macro_with_fields() {
        let hash = Field::new("hash");
        let pass = Field::new("pass");

        let result = pbkdf2::compare!(hash, pass);
        assert_eq!(
            result.fine_tune_params(),
            "crypto::pbkdf2::compare(hash, pass)"
        );
        assert_eq!(
            result.to_raw().build(),
            "crypto::pbkdf2::compare(hash, pass)"
        );
    }

    #[test]
    fn test_pbkdf2_compare_macro_with_param() {
        let hash = let_("hash").equal_to("$argon2id$v=19$m=4096,t=3,p=1$pbZ6yJ2rPJKk4pyEMVwslQ$jHzpsiB+3S/H+kwFXEcr10vmOiDkBkydVCSMfRxV7CA");
        let pass = let_("pass").equal_to("the strongest password");

        let result = pbkdf2::compare!(hash.get_param(), pass.get_param());
        assert_eq!(
            result.fine_tune_params(),
            "crypto::pbkdf2::compare($hash, $pass)"
        );
        assert_eq!(
            result.to_raw().build(),
            "crypto::pbkdf2::compare($hash, $pass)"
        );
    }

    #[test]
    fn test_pbkdf2_generate_fn() {
        let result = pbkdf2::generate!("Oyelowo");
        assert_eq!(
            result.fine_tune_params(),
            "crypto::pbkdf2::generate($_param_00000001)"
        );
        assert_eq!(
            result.to_raw().build(),
            "crypto::pbkdf2::generate('Oyelowo')"
        );
    }

    #[test]
    fn test_scrypt_compare_with_param() {
        let hash = let_("hash").equal_to("$argon2id$v=19$m=4096,t=3,p=1$pbZ6yJ2rPJKk4pyEMVwslQ$jHzpsiB+3S/H+kwFXEcr10vmOiDkBkydVCSMfRxV7CA");
        let pass = let_("pass").equal_to("the strongest password");

        let result = scrypt::compare_fn(hash.get_param(), pass.get_param());
        assert_eq!(
            result.fine_tune_params(),
            "crypto::scrypt::compare($hash, $pass)"
        );
        assert_eq!(
            result.to_raw().build(),
            "crypto::scrypt::compare($hash, $pass)"
        );
    }

    #[test]
    fn test_scrypt_generate() {
        let result = scrypt::generate_fn("Oyelowo");
        assert_eq!(
            result.fine_tune_params(),
            "crypto::scrypt::generate($_param_00000001)"
        );
        assert_eq!(
            result.to_raw().build(),
            "crypto::scrypt::generate('Oyelowo')"
        );
    }

    #[test]
    fn test_scrypt_compare_macro_with_raw_values() {
        let result = scrypt::compare!("$argon2id$v=19$m=4096,t=3,p=1$pbZ6yJ2rPJKk4pyEMVwslQ$jHzpsiB+3S/H+kwFXEcr10vmOiDkBkydVCSMfRxV7CA", "the strongest password");
        assert_eq!(
            result.fine_tune_params(),
            "crypto::scrypt::compare($_param_00000001, $_param_00000002)"
        );
        assert_eq!(
        result.to_raw().build(),
        "crypto::scrypt::compare('$argon2id$v=19$m=4096,t=3,p=1$pbZ6yJ2rPJKk4pyEMVwslQ$jHzpsiB+3S/H+kwFXEcr10vmOiDkBkydVCSMfRxV7CA', 'the strongest password')"
    );
    }

    #[test]
    fn test_scrypt_compare_macro_with_fields() {
        let hash = Field::new("hash");
        let pass = Field::new("pass");

        let result = scrypt::compare!(hash, pass);
        assert_eq!(
            result.fine_tune_params(),
            "crypto::scrypt::compare(hash, pass)"
        );
        assert_eq!(
            result.to_raw().build(),
            "crypto::scrypt::compare(hash, pass)"
        );
    }

    #[test]
    fn test_scrypt_compare_macro_with_param() {
        let hash = let_("hash").equal_to("$argon2id$v=19$m=4096,t=3,p=1$pbZ6yJ2rPJKk4pyEMVwslQ$jHzpsiB+3S/H+kwFXEcr10vmOiDkBkydVCSMfRxV7CA");
        let pass = let_("pass").equal_to("the strongest password");

        let result = scrypt::compare!(hash.get_param(), pass.get_param());
        assert_eq!(
            result.fine_tune_params(),
            "crypto::scrypt::compare($hash, $pass)"
        );
        assert_eq!(
            result.to_raw().build(),
            "crypto::scrypt::compare($hash, $pass)"
        );
    }

    #[test]
    fn test_scrypt_generate_fn() {
        let result = scrypt::generate!("Oyelowo");
        assert_eq!(
            result.fine_tune_params(),
            "crypto::scrypt::generate($_param_00000001)"
        );
        assert_eq!(
            result.to_raw().build(),
            "crypto::scrypt::generate('Oyelowo')"
        );
    }

    // bcrypt
    // Create all tests for bcrypt
    #[test]
    fn test_bcrypt_compare_with_param() {
        let hash = let_("hash").equal_to("$argon2id$v=19$m=4096,t=3,p=1$pbZ6yJ2rPJKk4pyEMVwslQ$jHzpsiB+3S/H+kwFXEcr10vmOiDkBkydVCSMfRxV7CA");
        let pass = let_("pass").equal_to("the strongest password");

        let result = bcrypt::compare_fn(hash.get_param(), pass.get_param());
        assert_eq!(
            result.fine_tune_params(),
            "crypto::bcrypt::compare($hash, $pass)"
        );
        assert_eq!(
            result.to_raw().build(),
            "crypto::bcrypt::compare($hash, $pass)"
        );
    }

    #[test]
    fn test_bcrypt_generate() {
        let result = bcrypt::generate_fn("Oyelowo");
        assert_eq!(
            result.fine_tune_params(),
            "crypto::bcrypt::generate($_param_00000001)"
        );
        assert_eq!(
            result.to_raw().build(),
            "crypto::bcrypt::generate('Oyelowo')"
        );
    }

    #[test]
    fn test_bcrypt_compare_macro_with_raw_values() {
        let result = bcrypt::compare!("$argon2id$v=19$m=4096,t=3,p=1$pbZ6yJ2rPJKk4pyEMVwslQ$jHzpsiB+3S/H+kwFXEcr10vmOiDkBkydVCSMfRxV7CA", "the strongest password");
        assert_eq!(
            result.fine_tune_params(),
            "crypto::bcrypt::compare($_param_00000001, $_param_00000002)"
        );
        assert_eq!(
        result.to_raw().build(),
        "crypto::bcrypt::compare('$argon2id$v=19$m=4096,t=3,p=1$pbZ6yJ2rPJKk4pyEMVwslQ$jHzpsiB+3S/H+kwFXEcr10vmOiDkBkydVCSMfRxV7CA', 'the strongest password')"
    );
    }

    #[test]
    fn test_bcrypt_compare_macro_with_fields() {
        let hash = Field::new("hash");
        let pass = Field::new("pass");

        let result = bcrypt::compare!(hash, pass);
        assert_eq!(
            result.fine_tune_params(),
            "crypto::bcrypt::compare(hash, pass)"
        );
        assert_eq!(
            result.to_raw().build(),
            "crypto::bcrypt::compare(hash, pass)"
        );
    }

    #[test]
    fn test_bcrypt_compare_macro_with_param() {
        let hash = let_("hash").equal_to("$argon2id$v=19$m=4096,t=3,p=1$pbZ6yJ2rPJKk4pyEMVwslQ$jHzpsiB+3S/H+kwFXEcr10vmOiDkBkydVCSMfRxV7CA");
        let pass = let_("pass").equal_to("the strongest password");

        let result = bcrypt::compare!(hash.get_param(), pass.get_param());
        assert_eq!(
            result.fine_tune_params(),
            "crypto::bcrypt::compare($hash, $pass)"
        );
        assert_eq!(
            result.to_raw().build(),
            "crypto::bcrypt::compare($hash, $pass)"
        );
    }

    #[test]
    fn test_bcrypt_generate_fn() {
        let result = bcrypt::generate!("Oyelowo");
        assert_eq!(
            result.fine_tune_params(),
            "crypto::bcrypt::generate($_param_00000001)"
        );
        assert_eq!(
            result.to_raw().build(),
            "crypto::bcrypt::generate('Oyelowo')"
        );
    }
}
