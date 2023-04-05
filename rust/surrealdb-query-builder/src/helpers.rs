use surrealdb::sql;

pub fn val(val: impl Into<sql::Value>) -> sql::Value {
    val.into()
}

#[macro_export]
macro_rules! array {
    ($( $val:expr ),*) => {{
        vec![
            $( surrealdb::sql::Value::from($val) ),*
        ]
    }};
}

pub use array;
