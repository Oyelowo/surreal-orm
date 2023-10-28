/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

// TODOs:
// Check within macro that:
// two fields do not have same old_name value
// old_name value is not same with any of the field names
// old name value is currently in migration directory/live db state, which means it has not yet
// been removed, therefore, still valid to be used as an annotation. The old_name attribute is
// meant to be used temporarily to help with migrations. Once the migration is done, the old_name
// attribute should be removed.
mod database;
mod error;
// mod models;
mod resources;

pub use database::*;
pub use error::*;
// pub use models::*;
pub use resources::*;

#[macro_export]
macro_rules! embed_migrations {
    ($dir:expr) => {
        concat!(
            $(
                include_str!(concat!($dir, "/", $file)),
            )*
        )
    };
}
#[cfg(test)]
mod tests {

    #[allow(dead_code)]
    fn test_remove_statement_generation_for_define_user_on_namespace() {
        // let stmt = generate_removal_statement(
        //     &"DEFINE USER Oyelowo ON NAMESPACE PASSWORD 'mapleleaf' ROLES OWNER".to_string(),
        //     "Oyelowo".into(),
        //     None,
        // );
        // assert_eq!(stmt, "REMOVE USER Oyelowo ON NAMESPACE".to_string());
    }

    #[allow(dead_code)]
    fn test_remove_statement_generation_for_define_user_on_database() {
        // let stmt = generate_removal_statement(
        //     &"DEFINE USER Oyelowo ON DATABASE PASSWORD 'mapleleaf' ROLES OWNER".to_string(),
        //     "Oyelowo".into(),
        //     None,
        // );
        // assert_eq!(stmt, "REMOVE USER Oyelowo ON DATABASE".to_string());
    }
}
