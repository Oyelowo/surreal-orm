# Time Functions

- [time::day()](#day-function)
- [time::floor()](#floor-function)
- [time::format()](#format-function)
- [time::group()](#group-function)
- [time::hour()](#hour-function)
- [time::minute()](#minute-function)
- [time::month()](#month-function)
- [time::nano()](#nano-function)
- [time::now()](#now-function)
- [time::round()](#round-function)
- [time::second()](#second-function)
- [time::timezone()](#timezone-function)
- [time::unix()](#unix-function)
- [time::wday()](#wday-function)
- [time::week()](#week-function)
- [time::yday()](#yday-function)
- [time::year()](#year-function)

## time::day() <a name="day-function"></a>

The `time::day()` function extracts the day as a number from a datetime.

**Usage:**

```rust
use surrealdb_orm::{*, functions::time};

let dt = chrono::DateTime::<chrono::Utc>::from_utc(
    chrono::NaiveDateTime::from_timestamp_opt(61, 0).unwrap(),
    chrono::Utc,
);

let result = time::day(dt);
assert_eq!(
    result.to_raw().build(),
    "time::day('1970-01-01T00:01:01Z')"
);

let rebirth_date = Field::new("rebirth_date");
let result = time::day(rebirth_date);
assert_eq!(result.to_raw().build(), "time::day(rebirth_date)");

let param = Param::new("rebirth_date");
let result = time::day(param);
assert_eq!(result.to_raw().build(), "time::day($rebirth_date)");
```

**Example:**

```rust
use surrealdb_orm::{*, functions::time};

let dt = chrono::DateTime::<chrono::Utc>::from_utc(
    chrono::NaiveDateTime::from_timestamp_opt(61, 0).unwrap(),
    chrono::Utc,
);

let result = time::day(dt);
assert_eq!(
    result.to_raw().build(),
    "time::day('1970-01-01T00:01:01Z')"
);

let rebirth_date = Field::new("rebirth_date");
let result = time::day(rebirth_date);
assert_eq!(result.to_raw().build(), "time::day(rebirth_date)");

let param = Param::new("rebirth_date");
let result = time::day(param);
assert_eq!(result.to_raw().build(), "time::day($rebirth_date)");
```

## time::floor() <a name="floor-function"></a>

The `time::floor()` function rounds a datetime down by a specific duration.

**Usage:**

```rust
use surrealdb_orm::{*, functions::time};

let rebirth_date = Field::new("rebirth_date");
let duration = Field::new("duration");
let result = time::floor(rebirth_date, duration);

assert_eq!(
    result.to_raw().build(),
    "time::floor(rebirth_date, duration)"
);
```

**Example:**

```rust
use surrealdb_orm::{*, functions::time};

let rebirth_date = Field::new("rebirth_date");
let duration = Field::new("duration");
let result = time::floor(rebirth_date, duration);

assert_eq!(
    result.to_raw().

build(),
    "time::floor(rebirth_date, duration)"
);
```

## time::format() <a name="format-function"></a>

The `time::format()` function outputs a datetime according to a specific format.

**Usage:**

```rust
use surrealdb_orm::{*, functions::time};

let dt = chrono::DateTime::<chrono::Utc>::from_utc(
    chrono::NaiveDateTime::from_timestamp_opt(61, 0).unwrap(),
    chrono::Utc,
);
let format_str = "'Year: 'yyyy-MM-dd";

let result = time::format(dt, format_str);
assert_eq!(
    result.to_raw().build(),
    "time::format('1970-01-01T00:01:01Z', 'Year: 'yyyy-MM-dd)"
);
```

**Example:**

```rust
use surrealdb_orm::{*, functions::time};

let dt = chrono::DateTime::<chrono::Utc>::from_utc(
    chrono::NaiveDateTime::from_timestamp_opt(61, 0).unwrap(),
    chrono::Utc,
);
let format_str = "'Year: 'yyyy-MM-dd";

let result = time::format(dt, format_str);
assert_eq!(
    result.to_raw().build(),
    "time::format('1970-01-01T00:01:01Z', 'Year: 'yyyy-MM-dd)"
);
```

## time::group() <a name="group-function"></a>

The `time::group()` function groups a datetime by a particular time interval.

**Usage:**

```rust
use surrealdb_orm::{*, functions::time};

let dt = chrono::DateTime::<chrono::Utc>::from_utc(
    chrono::NaiveDateTime::from_timestamp_opt(61, 0).unwrap(),
    chrono::Utc,
);
let interval = "month";

let result = time::group(dt, interval);
assert_eq!(
    result.to_raw().build(),
    "time::group('1970-01-01T00:01:01Z', 'month')"
);

let rebirth_date = Field::new("rebirth_date");
let interval_field = Field::new("interval");
let result = time::group(rebirth_date, interval_field);
assert_eq!(
    result.to_raw().build(),
    "time::group(rebirth_date, interval)"
);

let param = Param::new("rebirth_date");
let result = time::group(param, interval);
assert_eq!(
    result.to_raw().build(),
    "time::group($rebirth_date, 'month')"
);
```

**Example:**

```rust
use surrealdb_orm::{*, functions::time};

let dt = chrono::DateTime::<chrono::Utc>::from_utc(
    chrono::NaiveDateTime::from_timestamp_opt(61, 0).unwrap(),
    chrono::Utc,
);
let interval = "month";

let result = time::group(dt, interval);
assert_eq!(
    result.to_raw().build(),
    "time::group('1970-01-01T00:01:01Z', 'month')"
);

let rebirth_date = Field::new("rebirth_date");
let interval_field = Field::new("interval");
let result = time::group(rebirth_date, interval_field);
assert_eq!(
    result.to_raw().build(),
    "time::group(rebirth_date, interval)"
);

let param = Param::new("rebirth_date");
let result = time::group(param, interval);
assert_eq!(
    result.to_raw().build(),
    "time::group($rebirth_date, 'month')"
);
```

## time::hour() <a name="hour-function"></a>

The `time::hour()` function extracts the hour as a number

from a datetime.

**Usage:**

```rust
use surrealdb_orm::{*, functions::time};

let dt = chrono::DateTime::<chrono::Utc>::from_utc(
    chrono::NaiveDateTime::from_timestamp_opt(61, 0).unwrap(),
    chrono::Utc,
);

let result = time::hour(dt);
assert_eq!(
    result.to_raw().build(),
    "time::hour('1970-01-01T00:01:01Z')"
);

let rebirth_date = Field::new("rebirth_date");
let result = time::hour(rebirth_date);
assert_eq!(result.to_raw().build(), "time::hour(rebirth_date)");

let param = Param::new("rebirth_date");
let result = time::hour(param);
assert_eq!(result.to_raw().build(), "time::hour($rebirth_date)");
```

**Example:**

```rust
use surrealdb_orm::{*, functions::time};

let dt = chrono::DateTime::<chrono::Utc>::from_utc(
    chrono::NaiveDateTime::from_timestamp_opt(61, 0).unwrap(),
    chrono::Utc,
);

let result = time::hour(dt);
assert_eq!(
    result.to_raw().build(),
    "time::hour('1970-01-01T00:01:01Z')"
);

let rebirth_date = Field::new("rebirth_date");
let result = time::hour(rebirth_date);
assert_eq!(result.to_raw().build(), "time::hour(rebirth_date)");

let param = Param::new("rebirth_date");
let result = time::hour(param);
assert_eq!(result.to_raw().build(), "time::hour($rebirth_date)");
```

## time::minute() <a name="minute-function"></a>

The `time::minute()` function extracts the minutes as a number from a datetime.

**Usage:**

```rust
use surrealdb_orm::{*, functions::time};

let dt = chrono::DateTime::<chrono::Utc>::from_utc(
    chrono::NaiveDateTime::from_timestamp_opt(61, 0).unwrap(),
    chrono::Utc,
);

let result = time::minute(dt);
assert_eq!(
    result.to_raw().build(),
    "time::minute('1970-01-01T00:01:01Z')"
);

let rebirth_date = Field::new("rebirth_date");
let result = time::minute(rebirth_date);
assert_eq!(result.to_raw().build(), "time::minute(rebirth_date)");

let param = Param::new("rebirth_date");
let result = time::minute(param);
assert_eq!(result.to_raw().build(), "time::minute($rebirth_date)");
```

**Example:**

```rust
use surrealdb_orm::{*, functions::time};

let dt = chrono::DateTime::<chrono::Utc>::from_utc(
    chrono::NaiveDateTime::from_timestamp_opt(61, 0).unwrap(),
    chrono::Utc,
);

let result = time::minute(dt);
assert_eq!(
    result.to_raw().build(),
    "time::minute('1970-01-01T00:01:01Z')"
);

let rebirth_date = Field::new("rebirth_date");
let result = time::minute(rebirth_date);
assert_eq!(result.to_raw().build(), "time::minute(rebirth_date)");

let param = Param::new("rebirth_date");
let result = time::minute(param);
assert_eq!(result.to_raw().build(), "time::minute($rebirth_date)");
```

## time::month() <a name="month-function"></a>

The `time::month()` function extracts the month as a number from a datetime.

**Usage:**

```rust
use surrealdb_orm::{*, functions::time};

let dt = chrono::DateTime::<chrono::Utc>::from_utc(
    chrono::NaiveDateTime::from_timestamp_opt(61, 0).unwrap(),
    chrono::Utc,
);

let result = time::month(dt);
assert_eq!(
    result.to_raw().build(),
    "time::month('1970-01-01T00:01:01Z')"
);

let rebirth_date = Field::new("rebirth_date");
let result = time::month(rebirth_date);
assert_eq!(result.to_raw().build(), "time::month(rebirth_date)");

let param = Param::new("rebirth_date");
let result = time::month(param);
assert_eq!(result.to_raw().build(), "time::month($rebirth_date)");
```

**Example:**

```rust
use surrealdb_orm::{*, functions::time};

let dt = chrono::DateTime::<chrono::Utc>::from_utc(
    chrono::NaiveDateTime::from_timestamp_opt(61, 0).unwrap(),
    chrono::Utc,
);

let result = time::month(dt);
assert_eq!(
    result.to_raw().build(),
    "time::month('1970-01-01T00:01:01Z')"
);

let rebirth_date = Field::new("rebirth_date");
let result = time::month(rebirth_date);
assert_eq!(result.to_raw().build(), "time::month(rebirth_date)");

let param = Param::new("rebirth_date");
let result = time::month(param);
assert_eq!(result.to_raw().build(), "time::month($rebirth_date)");
```

## time::nano() <a name="nano-function"></a>

The `time::nano()` function returns the number of nanoseconds since the UNIX epoch.

**Usage:**

```rust
use surrealdb_orm::{*, functions::time};

let result = time::nano();
assert_eq!(result.to_raw().build(), "time::nano()");
```

**Example:**

```rust
use surrealdb_orm::{*, functions::time};

let result = time::nano();
assert_eq!(result.to_raw().build(), "time::nano()");
```

## time::now() <a name="now-function"></a>

The `time::now()` function returns the current datetime.

**Usage:**

```rust
use surrealdb_orm::{*, functions::time};

let result = time::now();
assert_eq!(result.to_raw().build(), "time::now()");
```

**Example:**

```rust
use surrealdb_orm::{*, functions::time};

let result = time::now();
assert_eq!(result.to_raw().build(), "time::now()");
```

## time::round() <a name="round-function"></a>

The `time::round()` function rounds a datetime to the nearest multiple of a specific duration.

**Usage:**

```rust
use surrealdb_orm::{*, functions::time};

let rebirth_date = Field::new("rebirth_date");
let duration = Field::new("duration");
let result = time::round(rebirth_date, duration);

assert_eq!(
    result.to_raw().build(),
    "time::round(rebirth_date, duration)"
);
```

**Example:**

```rust
use surrealdb_orm::{*, functions::time};

let rebirth_date = Field::new("rebirth_date");
let duration = Field::new("duration");
let result = time::round(rebirth_date, duration);

assert_eq!(
    result.to_raw().build(),
    "time::round(rebirth_date, duration)"
);
```

## time::second() <a name="second-function"></a

>

The `time::second()` function extracts the second as a number from a datetime.

**Usage:**

```rust
use surrealdb_orm::{*, functions::time};

let dt = chrono::DateTime::<chrono::Utc>::from_utc(
    chrono::NaiveDateTime::from_timestamp_opt(61, 0).unwrap(),
    chrono::Utc,
);

let result = time::second(dt);
assert_eq!(
    result.to_raw().build(),
    "time::second('1970-01-01T00:01:01Z')"
);

let rebirth_date = Field::new("rebirth_date");
let result = time::second(rebirth_date);
assert_eq!(result.to_raw().build(), "time::second(rebirth_date)");

let param = Param::new("rebirth_date");
let result = time::second(param);
assert_eq!(result.to_raw().build(), "time::second($rebirth_date)");
```

**Example:**

```rust
use surrealdb_orm::{*, functions::time};

let dt = chrono::DateTime::<chrono::Utc>::from_utc(
    chrono::NaiveDateTime::from_timestamp_opt(61, 0).unwrap(),
    chrono::Utc,
);

let result = time::second(dt);
assert_eq!(
    result.to_raw().build(),
    "time::second('1970-01-01T00:01:01Z')"
);

let rebirth_date = Field::new("rebirth_date");
let result = time::second(rebirth_date);
assert_eq!(result.to_raw().build(), "time::second(rebirth_date)");

let param = Param::new("rebirth_date");
let result = time::second(param);
assert_eq!(result.to_raw().build(), "time::second($rebirth_date)");
```

## time::timezone() <a name="timezone-function"></a>

The `time::timezone()` function returns the current local timezone offset in hours.

**Usage:**

```rust
use surrealdb_orm::{*, functions::time};

let result = time::timezone();
assert_eq!(result.to_raw().build(), "time::timezone()");
```

**Example:**

```rust
use surrealdb_orm::{*, functions::time};

let result = time::timezone();
assert_eq!(result.to_raw().build(), "time::timezone()");
```

## time::unix() <a name="unix-function"></a>

The `time::unix()` function returns the number of seconds since the UNIX epoch.

**Usage:**

```rust
use surrealdb_orm::{*, functions::time};

let result = time::unix();
assert_eq!(result.to_raw().build(), "time::unix()");
```

**Example:**

```rust
use surrealdb_orm::{*, functions::time};

let result = time::unix();
assert_eq!(result.to_raw().build(), "time::unix()");
```

## time::wday() <a name="wday-function"></a>

The `time::wday()` function extracts the week day as a number from a datetime.

**Usage:**

```rust
use surrealdb_orm::{*, functions::time};

let dt = chrono::DateTime::<chrono::Utc>::from_utc(
    chrono::NaiveDateTime::from_timestamp_opt(61, 0).unwrap(),
    chrono::Utc,
);

let result = time::wday(dt);
assert_eq!(
    result.to_raw().build(),
    "time::wday('1970-01-01T00:01:01Z')"
);

let rebirth_date = Field::new("rebirth_date");
let result = time::wday(rebirth_date);
assert_eq!(result.to_raw().build(), "time::wday(rebirth_date)");



let param = Param::new("rebirth_date");
let result = time::wday(param);
assert_eq!(result.to_raw().build(), "time::wday($rebirth_date)");
```

**Example:**

```rust
use surrealdb_orm::{*, functions::time};

let dt = chrono::DateTime::<chrono::Utc>::from_utc(
    chrono::NaiveDateTime::from_timestamp_opt(61, 0).unwrap(),
    chrono::Utc,
);

let result = time::wday(dt);
assert_eq!(
    result.to_raw().build(),
    "time::wday('1970-01-01T00:01:01Z')"
);

let rebirth_date = Field::new("rebirth_date");
let result = time::wday(rebirth_date);
assert_eq!(result.to_raw().build(), "time::wday(rebirth_date)");

let param = Param::new("rebirth_date");
let result = time::wday(param);
assert_eq!(result.to_raw().build(), "time::wday($rebirth_date)");
```

## time::week() <a name="week-function"></a>

The `time::week()` function extracts the week as a number from a datetime.

**Usage:**

```rust
use surrealdb_orm::{*, functions::time};

let dt = chrono::DateTime::<chrono::Utc>::from_utc(
    chrono::NaiveDateTime::from_timestamp_opt(61, 0).unwrap(),
    chrono::Utc,
);

let result = time::week(dt);
assert_eq!(
    result.to_raw().build(),
    "time::week('1970-01-01T00:01:01Z')"
);

let rebirth_date = Field::new("rebirth_date");
let result = time::week(rebirth_date);
assert_eq!(result.to_raw().build(), "time::week(rebirth_date)");

let param = Param::new("rebirth_date");
let result = time::week(param);
assert_eq!(result.to_raw().build(), "time::week($rebirth_date)");
```

**Example:**

```rust
use surrealdb_orm::{*, functions::time};

let dt = chrono::DateTime::<chrono::Utc>::from_utc(
    chrono::NaiveDateTime::from_timestamp_opt(61, 0).unwrap(),
    chrono::Utc,
);

let result = time::week(dt);
assert_eq!(
    result.to_raw().build(),
    "time::week('1970-01-01T00:01:01Z')"
);

let rebirth_date = Field::new("rebirth_date");
let result = time::week(rebirth_date);
assert_eq!(result.to_raw().build(), "time::week(rebirth_date)");

let param = Param::new("rebirth_date");
let result = time::week(param);
assert_eq!(result.to_raw().build(), "time::week($rebirth_date)");
```

## time::yday() <a name="yday-function"></a>

The `time::yday()` function extracts the yday as a number from a datetime.

**Usage:**

```rust
use surrealdb_orm::{*, functions::time};

let dt = chrono::DateTime::<chrono::Utc>::from_utc(
    chrono::NaiveDateTime::from_timestamp_opt(61, 0).unwrap(),
    chrono::Utc,
);

let result = time::yday(dt);
assert_eq!(
    result.to_raw().build(),
    "time::yday('1970-01-01T00:01:01Z')"
);

let rebirth_date = Field::new("rebirth_date");
let result = time::yday(rebirth_date);
assert_eq!(result.to_raw().build(), "

time::yday(rebirth_date)");

let param = Param::new("rebirth_date");
let result = time::yday(param);
assert_eq!(result.to_raw().build(), "time::yday($rebirth_date)");
```

**Example:**

```rust
use surrealdb_orm::{*, functions::time};

let dt = chrono::DateTime::<chrono::Utc>::from_utc(
    chrono::NaiveDateTime::from_timestamp_opt(61, 0).unwrap(),
    chrono::Utc,
);

let result = time::yday(dt);
assert_eq!(
    result.to_raw().build(),
    "time::yday('1970-01-01T00:01:01Z')"
);

let rebirth_date = Field::new("rebirth_date");
let result = time::yday(rebirth_date);
assert_eq!(result.to_raw().build(), "time::yday(rebirth_date)");

let param = Param::new("rebirth_date");
let result = time::yday(param);
assert_eq!(result.to_raw().build(), "time::yday($rebirth_date)");
```

## time::year() <a name="year-function"></a>

The `time::year()` function extracts the year as a number from a datetime.

**Usage:**

```rust
use surrealdb_orm::{*, functions::time};

let dt = chrono::DateTime::<chrono::Utc>::from_utc(
    chrono::NaiveDateTime::from_timestamp_opt(61, 0).unwrap(),
    chrono::Utc,
);

let result = time::year(dt);
assert_eq!(
    result.to_raw().build(),
    "time::year('1970-01-01T00:01:01Z')"
);

let rebirth_date = Field::new("rebirth_date");
let result = time::year(rebirth_date);
assert_eq!(result.to_raw().build(), "time::year(rebirth_date)");

let param = Param::new("rebirth_date");
let result = time::year(param);
assert_eq!(result.to_raw().build(), "time::year($rebirth_date)");
```

**Example:**

```rust
use surrealdb_orm::{*, functions::time};

let dt = chrono::DateTime::<chrono::Utc>::from_utc(
    chrono::NaiveDateTime::from_timestamp_opt(61, 0).unwrap(),
    chrono::Utc,
);

let result = time::year(dt);
assert_eq!(
    result.to_raw().build(),
    "time::year('1970-01-01T00:01:01Z')"
);

let rebirth_date = Field::new("rebirth_date");
let result = time::year(rebirth_date);
assert_eq!(result.to_raw().build(), "time::year(rebirth_date)");

let param = Param::new("rebirth_date");
let result = time::year(param);
assert_eq!(result.to_raw().build(), "time::year($rebirth_date)");
```
