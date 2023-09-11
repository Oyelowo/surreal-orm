# Type functions

## Table of Contents

- [type\_::bool!()](#bool-macro)
- [type\_::datetime!()](#datetime-macro)
- [type\_::decimal!()](#decimal-macro)
- [type\_::duration!()](#duration-macro)
- [type\_::float!()](#float-macro)
- [type\_::int!()](#int-macro)
- [type\_::number!()](#number-macro)
- [type\_::point!()](#point-macro)
- [type\_::regex!()](#regex-macro)
- [type\_::string!()](#string-macro)
- [type\_::table!()](#table-macro)
- [type\_::thing!()](#thing-macro)



## type\_::bool!() <a name="bool-macro"></a>

The `type_::bool!()` macro allows you to convert a value into a boolean.

**Examples:**

```rust
use surreal_orm::macros::type_;

let result = type_::bool!(43545);
assert_eq!(result.to_raw().build(), "type::bool(43545)");

let bool_field = Field::new("bool_field");
let result = type_::bool!(bool_field);
assert_eq!(result.to_raw().build(), "type::bool(bool_field)");

let bool_param = Param::new("bool_param");
let result = type_::bool!(bool_param);
assert_eq!(result.to_raw().build(), "type::bool($bool_param)");
```



## type\_::datetime!() <a name="datetime-macro"></a>

The `type_::datetime!()` macro allows you to convert a value into a datetime.

**Examples:**

```rust
use surreal_orm::macros::type_;
use chrono::DateTime;
use chrono::Utc;

let value = DateTime::<Utc>::from_utc(chrono::NaiveDateTime::from_timestamp_opt(61, 0).unwrap(), Utc);
let result = type_::datetime!(value);
assert_eq!(result.to_raw().build(), "type::datetime('1970-01-01T00:01:01Z')");

let datetime_field = Field::new("datetime_field");
let result = type_::datetime!(datetime_field);
assert_eq!(result.to_raw().build(), "type::datetime(datetime_field)");

let datetime_param = Param::new("datetime_param");
let result = type_::datetime!(datetime_param);
assert_eq!(result.to_raw().build(), "type::datetime($datetime_param)");
```



## type\_::decimal!() <a name="decimal-macro"></a>

The `type_::decimal!()` macro allows you to convert a value into a decimal.

**Examples:**

```rust
use surreal_orm::macros::type_;

let result = type_::decimal!(1234.56);
assert_eq!(result.to_raw().build(), "type::decimal(1234.56)");

let decimal_field = Field::new("decimal_field");
let result = type_::decimal!(decimal_field);
assert_eq!(result.to_raw().build(), "type::decimal(decimal_field)");

let decimal_param = Param::new("decimal_param");
let result = type_::decimal!(decimal_param);
assert_eq!(result.to_raw().build(), "type::decimal($decimal_param)");
```



## type\_::duration!() <a name="duration-macro"></a>

The `type_::duration!()` macro allows you to convert a value into a duration.

**Examples:**

```rust


use surreal_orm::macros::type_;
use std::time::Duration;

let result = type_::duration!(Duration::from_secs(24 * 60 * 60 * 7));
assert_eq!(result.to_raw().build(), "type::duration(1w)");

let duration_field = Field::new("duration_field");
let result = type_::duration!(duration_field);
assert_eq!(result.to_raw().build(), "type::duration(duration_field)");

let duration_param = Param::new("duration_param");
let result = type_::duration!(duration_param);
assert_eq!(result.to_raw().build(), "type::duration($duration_param)");
```



## type\_::float!() <a name="float-macro"></a>

The `type_::float!()` macro allows you to convert a value into a floating point number.

**Examples:**

```rust
use surreal_orm::macros::type_;

let result = type_::float!(43.5);
assert_eq!(result.to_raw().build(), "type::float(43.5)");

let float_field = Field::new("float_field");
let result = type_::float!(float_field);
assert_eq!(result.to_raw().build(), "type::float(float_field)");

let float_param = Param::new("float_param");
let result = type_::float!(float_param);
assert_eq!(result.to_raw().build(), "type::float($float_param)");
```



## type\_::int!() <a name="int-macro"></a>

The `type_::int!()` macro allows you to convert a value into an integer.

**Examples:**

```rust
use surreal_orm::macros::type_;

let result = type_::int!(99);
assert_eq!(result.to_raw().build(), "type::int(99)");

let int_field = Field::new("int_field");
let result = type_::int!(int_field);
assert_eq!(result.to_raw().build(), "type::int(int_field)");

let int_param = Param::new("int_param");
let result = type_::int!(int_param);
assert_eq!(result.to_raw().build(), "type::int($int_param)");
```



## type\_::number!() <a name="number-macro"></a>

The `type_::number!()` macro allows you to convert a value into a number.

**Examples:**

```rust
use surreal_orm::macros::type_;

let result = type_::number!(5);
assert_eq!(result.to_raw().build(), "type::number(5)");

let number_field = Field::new("number_field");
let result = type_::number!(number_field);
assert_eq!(result.to_raw().build(), "type::number(number_field)");

let number_param = Param::new("number_param");
let result = type_::number!(number_param);
assert_eq!(result.to_raw().build(), "type::number($number_param)");
```



## type\_::point!() <a name="point-macro"></a>

The `type_::point!()` macro allows you to convert a value into a geometry point.

**Examples:**

```rust
use surreal_orm::macros::type_;

let result = type_::point!(51.509865, -0.118092);
assert_eq!(result.to_raw().build(), "type::point(51.509865, -0.118092)");

let point_field = Field::new("point_field");
let result = type_::point!(point_field);
assert_eq!(result.to_raw().build(), "type::point(point_field)");

let point_param = Param::new("point_param");
let result = type

_::point!(point_param);
assert_eq!(result.to_raw().build(), "type::point($point_param)");
```



## type\_::regex!() <a name="regex-macro"></a>

The `type_::regex!()` macro allows you to convert a value into a regular expression.

**Examples:**

```rust
use surreal_orm::macros::type_;

let result = type_::regex!("/[A-Z]{3}/");
assert_eq!(result.to_raw().build(), "type::regex('/[A-Z]{3}/')");

let regex_field = Field::new("regex_field");
let result = type_::regex!(regex_field);
assert_eq!(result.to_raw().build(), "type::regex(regex_field)");

let regex_param = Param::new("regex_param");
let result = type_::regex!(regex_param);
assert_eq!(result.to_raw().build(), "type::regex($regex_param)");
```



## type\_::string!() <a name="string-macro"></a>

The `type_::string!()` macro allows you to convert a value into a string.

**Examples:**

```rust
use surreal_orm::macros::type_;

let result = type_::string!(5454);
assert_eq!(result.to_raw().build(), "type::string(5454)");

let string_field = Field::new("string_field");
let result = type_::string!(string_field);
assert_eq!(result.to_raw().build(), "type::string(string_field)");

let string_param = Param::new("string_param");
let result = type_::string!(string_param);
assert_eq!(result.to_raw().build(), "type::string($string_param)");
```



## type\_::table!() <a name="table-macro"></a>

The `type_::table!()` macro allows you to convert a value into a table definition.

**Examples:**

```rust
use surreal_orm::macros::type_;
use surreal_orm::statements::let_;

let result = type_::table!("user");
assert_eq!(result.to_raw().build(), "type::table(user)");

let table_field = Field::new("table_field");
let result = type_::table!(table_field);
assert_eq!(result.to_raw().build(), "type::table(table_field)");

let table_param = let_("table_param").equal_to("user").get_param();
let result = type_::table!(table_param);
assert_eq!(result.to_raw().build(), "type::table($table_param)");
```



## type\_::thing!() <a name="thing-macro"></a>

The `type_::thing!()` macro allows you to convert a value into a record pointer.

**Examples:**

```rust
use surreal_orm::macros::type_;
use surreal_orm::Table;

let user = Table::from("user");
let id = "oyelowo";
let result = type_::thing!(user, id);
assert_eq!(result.to_raw().build(), "type::thing(user, 'oyelowo')");

let table = Table::new("table");
let id = Field::new("id");
let result = type_::thing!(table, id);
assert_eq!(result.to_raw().build(), "type::thing(table, id)");
```
