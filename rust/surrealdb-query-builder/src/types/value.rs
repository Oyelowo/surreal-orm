struct Value(sql::Value);

impl From<sql::Value> for Value {
    fn from(value: sql::Value) -> Operator {
        Value(value)
    }
}
