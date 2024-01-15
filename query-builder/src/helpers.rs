use surrealdb::sql;

/// A macro to create a surrealdb::sql::Value from a value
pub fn val(val: impl Into<sql::Value>) -> sql::Value {
    val.into()
}

/// A macro to create a vector of heterogeous/diverse surrealdb value
#[macro_export]
macro_rules! array {
    ($( $val:expr ),*) => {{
        vec![
            $( surrealdb::sql::Value::from($val) ),*
        ]
    }};
}
