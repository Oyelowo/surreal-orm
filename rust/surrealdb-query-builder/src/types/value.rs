use surrealdb::sql;

struct Value(sql::Value);

impl From<sql::Value> for Value {
    fn from(value: sql::Value) -> Self {
        Value(value)
    }
}
