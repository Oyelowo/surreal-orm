use surrealdb::sql;

pub fn val(val: impl Into<sql::Value>) -> sql::Value {
    val.into()
}

#[macro_export]
macro_rules! array {
    ($( $val:expr ),*) => {{
        vec![
            $( crate::helpers::val($val) ),*
        ]
    }};
}
