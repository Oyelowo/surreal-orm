/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */

// Meta functions
// These functions can be used to retrieve specific metadata from a SurrealDB Record ID.

use crate::{Buildable, Erroneous, Function, Parametric, ThingLike};

/// Extracts and returns the table id from a SurrealDB Record ID
pub fn id_fn(record_id: impl Into<ThingLike>) -> crate::Function {
    let record_id: ThingLike = record_id.into();
    let query_string = format!("meta::id({})", record_id.build());

    Function {
        query_string,
        bindings: record_id.get_bindings(),
        errors: record_id.get_errors(),
    }
}

/**
Extracts and returns the table id from a SurrealDB Record ID
Also aliased as `meta_id!`

# Arguments
* `record_id` - The record id to extract the table name from. Can also be a field or a parameter
    representing the record id.

# Example
```rust
# use surreal_query_builder as surreal_orm;
use surreal_orm::{*, functions::meta, statements::let_};
use surrealdb::sql;

let record_id = sql::Thing::from(("person", "oyelowo"));
let result = meta::id!(record_id);
assert_eq!(result.to_raw().build(), format!("meta::id(person:oyelowo)"));

let id_field = Field::new("id_field");
let result = meta::id!(id_field);
assert_eq!(result.to_raw().build(), format!("meta::id(id_field)"));

let id_param = let_("id_param").equal_to("person:oyelowo").get_param();
let result = meta::id!(id_param);
assert_eq!(result.to_raw().build(), format!("meta::id($id_param)"));
```
*/
#[macro_export]
macro_rules! meta_id {
    ($record_id: expr) => {
        $crate::functions::meta::id_fn($record_id)
    };
}

pub use meta_id as id;

/// Extracts and returns the table name from a SurrelDB Record ID
pub fn tb_fn(record_id: impl Into<ThingLike>) -> crate::Function {
    let record_id: ThingLike = record_id.into();
    let query_string = format!("meta::tb({})", record_id.build());

    Function {
        query_string,
        bindings: record_id.get_bindings(),
        errors: record_id.get_errors(),
    }
}

/**
Extracts and returns the table name from a SurrelDB Record ID
Also aliased as `meta_tb!`
# Arguments
* `record_id` - The record id to extract the table name from. Can also be a field or a parameter
    representing the record id.

# Example
```rust
# use surreal_query_builder as surreal_orm;
use surreal_orm::{*, functions::meta, statements::let_};
use surrealdb::sql;

let record_id = sql::Thing::from(("person", "oyelowo"));
let result = meta::tb!(record_id);
assert_eq!(result.to_raw().build(), format!("meta::tb(person:oyelowo)"));

let id_field = Field::new("id_field");
let result = meta::tb!(id_field);
assert_eq!(result.to_raw().build(), format!("meta::tb(id_field)"));

let id_param = let_("id_param").equal_to("person:oyelowo").get_param();
let result = meta::tb!(id_param);
assert_eq!(result.to_raw().build(), format!("meta::tb($id_param)"));
```
*/
#[macro_export]
macro_rules! meta_tb {
    ($record_id: expr) => {
        $crate::functions::meta::tb_fn($record_id)
    };
}

pub use meta_tb as tb;

#[cfg(test)]
mod tests {
    use surrealdb::sql;

    use super::*;
    use crate::{statements::let_, Field, ToRaw};

    #[test]
    fn test_id() {
        let record_id = sql::Thing::from(("person", "oyelowo"));
        let result = id!(record_id);
        assert_eq!(result.to_raw().build(), format!("meta::id(person:oyelowo)"));

        let id_field = Field::new("id_field");
        let result = id!(id_field);
        assert_eq!(result.to_raw().build(), format!("meta::id(id_field)"));

        let id_param = let_("id_param").equal_to("person:oyelowo").get_param();
        let result = id!(id_param);
        assert_eq!(result.to_raw().build(), format!("meta::id($id_param)"));
    }

    #[test]
    fn test_tb() {
        let record_id = sql::Thing::from(("person", "oyelowo"));
        let result = tb!(record_id);
        assert_eq!(result.to_raw().build(), format!("meta::tb(person:oyelowo)"));

        let id_field = Field::new("id_field");
        let result = tb!(id_field);
        assert_eq!(result.to_raw().build(), format!("meta::tb(id_field)"));

        let id_param = let_("id_param").equal_to("person:oyelowo").get_param();
        let result = tb!(id_param);
        assert_eq!(result.to_raw().build(), format!("meta::tb($id_param)"));
    }
}
