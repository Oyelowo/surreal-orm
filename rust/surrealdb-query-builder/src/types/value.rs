struct ValueCustom(sql::Value);

impl From<sql::Value> for ValueCustom {
    fn from(value: sql::Value) -> Operator {
        ValueCustom(value)
    }
}
