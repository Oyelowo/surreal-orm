# Validation Functions

This documentation provides an overview of the validation functions in the codebase. The `is` macros are used to create validation functions for checking various conditions on values.

## Table of Contents

- [Alphanum](#alphanum)
- [Alpha](#alpha)
- [ASCII](#ascii)
- [Domain](#domain)
- [Email](#email)
- [Hexadecimal](#hexadecimal)
- [Latitude](#latitude)
- [Longitude](#longitude)
- [Numeric](#numeric)
- [Semver](#semver)
- [UUID](#uuid)
- [Datetime](#datetime)

## Alphanum

The `is::alphanum` function checks whether a value has only alphanumeric characters. It is also aliased as `is_alphanum!`.

### Arguments

- `value` - The value to check. It could be a field or a parameter that represents the value.

### Example

```rust
use surreal_orm::{*, functions::is, statements::let_};

let result = is::alphanum!("oyelowo1234");
assert_eq!(result.to_raw().build(), "is::alphanum('oyelowo1234')");

let alphanum_field = Field::new("alphanum_field");
let result = is::alphanum!(alphanum_field);
assert_eq!(result.to_raw().build(), "is::alphanum(alphanum_field)");

block!{
    LET alphanum_param = "oyelowo1234";
    LET result = is::alphanum!(alphanum_param);
};
```

## Alpha

The `is::alpha` function checks whether a value has only alpha characters. It is also aliased as `is_alpha!`.

### Arguments

- `value` - The value to check. It could be a field or a parameter that represents the value.

### Example

```rust
use surreal_orm::{*, functions::is, statements::let_};

let result = is::alpha!("oyelowo");
assert_eq!(result.to_raw().build(), "is::alpha('oyelowo')");

let alpha_field = Field::new("alpha_field");
let result = is::alpha!(alpha_field);
assert_eq!(result.to_raw().build(), "is::alpha(alpha_field)");
```

## ASCII

The `is::ascii` function checks whether a value has only ASCII characters. It is also aliased as `is_ascii!`.

### Arguments

- `value` - The value to check. It could be a field or a parameter that represents the value.

### Example

```rust
use surreal_orm::{*, functions::is, statements::let_};

let result = is::ascii!("oyelowo");
assert_eq!(result.to_raw().build(), "is::ascii('oyelowo')");

let ascii_field = Field::new("ascii_field");
let result = is::ascii!(ascii_field);
assert_eq!(result.to_raw().build(), "is::ascii(ascii_field)");
```

## Domain

The `is::domain` function checks whether a value is a domain. It is also aliased as `is_domain!`.

### Arguments

- `value` - The value to check. It could be a field or a parameter that represents the value.

### Example

```rust
use surreal_orm::{*, functions::is, statements::let_};

let result = is::domain!("oyelowo.com");
assert_eq!(result.to_raw().build(), "is::domain('oyelowo.com')");

let domain_field = Field::new("domain_field");
let result = is::domain!(domain_field);
assert_eq!(result.to_raw().build(), "is::domain(domain_field)");
```

## Email

The `is::email` function checks whether a value is an email. It is also aliased as `is_email!`.

### Arguments

- `value` - The value to check. It could be a field or a parameter that represents the value.

### Example

```rust
use surreal_orm::{*, functions::is, statements::let_};

let result = is::email!("oyelowo@codebreather.com");
assert_eq!(result.to_raw().to_string(), "is::email('oyelowo@codebreather.com')");

let email_field = Field::new("email_field");
let result = is::email!(email_field);
assert_eq!(result.to_raw().to_string(), "is::email(email_field)");
```

## Hexadecimal

The `is::hexadecimal` function checks whether a value is hexadecimal. It is also aliased as `is_hexadecimal!`.

### Arguments

- `value` - The value to check. It could be a field or a parameter that represents the value.

### Example

```rust
use surreal_orm::{*, functions::is, statements::let_};

let result = is::hexadecimal!("oyelowo");
assert_eq!(result.to_raw().to_string(), "is::hexadecimal('oyelowo')");

let hexadecimal_field = Field::new("hexadecimal_field");
let result = is::hexadecimal!(hexadecimal_field);
assert_eq!(result.to_raw().to_string(), "is::hexadecimal(hexadecimal_field)");

let!(hexadecimal_param = "oyelowo");
let result = is::hexadecimal!(hexadecimal_param);
assert_eq!(result.fine_tune_params(), "is::hexadecimal($hexadecimal_param)");
```

## Latitude

The `is::latitude` function checks whether a value is a latitude value. It is also aliased as `is_latitude!`.

### Arguments

- `value` - The value to check. It could be a field or a parameter that represents the value.

### Example

```rust
use surreal_orm::{*, functions::is, statements::let_};

let result = is::latitude!("-0.118092");
assert_eq!(result.to_raw().build(), "is::latitude('-0.118092')");

let latitude_field = Field::new("latitude_field");
let result = is::latitude!(latitude_field);
assert_eq!(

result.to_raw().build(), "is::latitude(latitude_field)");
```

## Longitude

The `is::longitude` function checks whether a value is a longitude value. It is also aliased as `is_longitude!`.

### Arguments

- `value` - The value to check. It could be a field or a parameter that represents the value.

### Example

```rust
use surreal_orm::{*, functions::is, statements::let_};

let result = is::longitude!("51.509865");
assert_eq!(result.to_raw().build(), "is::longitude('51.509865')");

let longitude_field = Field::new("longitude_field");
let result = is::longitude!(longitude_field);
assert_eq!(result.to_raw().build(), "is::longitude(longitude_field)");
```

## Numeric

The `is::numeric` function checks whether a value has only numeric characters. It is also aliased as `is_numeric!`.

### Arguments

- `value` - The value to check. It could be a field or a parameter that represents the value.

### Example

```rust
use surreal_orm::{*, functions::is, statements::let_};

let result = is::numeric!("oyelowo");
assert_eq!(result.to_raw().build(), "is::numeric('oyelowo')");

let numeric_field = Field::new("numeric_field");
let result = is::numeric!(numeric_field);
assert_eq!(result.to_raw().build(), "is::numeric(numeric_field)");
```

## Semver

The `is::semver` function checks whether a value matches a semver version. It is also aliased as `is_semver!`.

### Arguments

- `value` - The value to check. It could be a field or a parameter that represents the value.

### Example

```rust
use surreal_orm::{*, functions::is, statements::let_};

let result = is::semver!("oyelowo");
assert_eq!(result.to_raw().build(), "is::semver('oyelowo')");

let semver_field = Field::new("semver_field");
let result = is::semver!(semver_field);
assert_eq!(result.to_raw().build(), "is::semver(semver_field)");
```

## UUID

The `is::uuid` function checks whether a value is a UUID. It is also aliased as `is_uuid!`.

### Arguments

- `value` - The value to check. It could be a field or a parameter that represents the value.

### Example

```rust
use surreal_orm::{*, functions::is, statements::let_};

let result = is::uuid!("oyelowo");
assert_eq!(result.to_raw().build(), "

is::uuid('oyelowo')");

let uuid_field = Field::new("uuid_field");
let result = is::uuid!(uuid_field);
assert_eq!(result.to_raw().build(), "is::uuid(uuid_field)");
```

## Datetime

The `is::datetime` function checks whether a value matches a datetime format. It is also aliased as `is_datetime!`.

### Arguments

- `value` - The value to check. It could be a field or a parameter that represents the value.

### Example

```rust
use surreal_orm::{*, functions::is, statements::let_};

let result = is::datetime!("oyelowo");
assert_eq!(result.to_raw().build(), "is::datetime('oyelowo')");

let datetime_field = Field::new("datetime_field");
let result = is::datetime!(datetime_field);
assert_eq!(result.to_raw().build(), "is::datetime(datetime_field)");
```
