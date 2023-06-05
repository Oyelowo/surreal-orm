# String functions

## Table of Contents

- [string::concat!()](#concat-macro)
- [string::join!()](#join-macro)
- [string::ends_with!()](#ends-with-macro)
- [string::starts_with!()](#starts-with-macro)
- [string::split!()](#split-macro)
- [string::len!()](#length-macro)
- [string::reverse!()](#reverse-macro)
- [string::trim!()](#trim-macro)
- [string::slug!()](#slug-macro)
- [string::lowercase!()](#lowercase-macro)
- [string::uppercase!()](#uppercase-macro)
- [string::words!()](#words-macro)
- [string::repeat!()](#repeat-macro)
- [string::replace!()](#replace-macro)
- [string::slice!()](#slice-macro)

## string::concat!() <a name="concat-macro"></a>

The `string::concat!()` macro allows you to concatenate multiple values into a string.

**Examples:**

```rust
use crate::functions::string;
use crate::*;

let title = Field::new("title");
let result = string::concat!(title, "one", 3, 4.15385, "  ", true);
assert_eq!(result.fine_tune_params(), "string::concat(title, $_param_00000001, $_param_00000002, $_param_00000003, $_param_00000004, $_param_00000005)");
assert_eq!(
    result.to_raw().build(),
    "string::concat(title, 'one', 3, 4.15385, '  ', true)"
);

let result = string::concat!(arr!["one", "two", 3, 4.15385, "five", true]);
assert_eq!(result.fine_tune_params(), "string::concat($_param_00000001, $_param_00000002, $_param_00000003, $_param_00000004, $_param_00000005, $_param_00000006)");
assert_eq!(
    result.to_raw().build(),
    "string::concat('one', 'two', 3, 4.15385, 'five', true)"
);
```

## string::join!() <a name="join-macro"></a>

The `string::join!()` macro allows you to join multiple values into a string using a delimiter.

**Examples:**

```rust
use crate::functions::string;
use crate::*;

let title = Field::new("title");
let result = string::join!(title, "one", 3, 4.15385, "  ", true);
assert_eq!(result.fine_tune_params(), "string::join(title, $_param_00000001, $_param_00000002, $_param_00000003, $_param_00000004, $_param_00000005)");
assert_eq!(
    result.to_raw().build(),
    "string::join(title, 'one', 3, 4.15385, '  ', true)"
);

let result = string::join!(arr!["one", "two", 3, 4.15385, "five", true]);
assert_eq!(result.fine_tune_params(), "string::join($_param_00000001, $_param_00000002, $_param_00000003, $_param_00000004

, $_param_00000005, $_param_00000006)");
assert_eq!(
    result.to_raw().build(),
    "string::join('one', 'two', 3, 4.15385, 'five', true)"
);
```

## string::ends_with!() <a name="ends-with-macro"></a>

The `string::ends_with!()` macro allows you to check if a string ends with a specified substring.

**Examples:**

```rust
use crate::functions::string;
use crate::*;

let name = Field::new("name");
let result = string::ends_with!(name, "lowo");
assert_eq!(
    result.fine_tune_params(),
    "string::ends_with(name, $_param_00000001)"
);
assert_eq!(result.to_raw().build(), "string::ends_with(name, 'lowo')");

let result = string::ends_with!("Oyelowo", "lowo");
assert_eq!(
    result.fine_tune_params(),
    "string::ends_with($_param_00000001, $_param_00000002)"
);
assert_eq!(
    result.to_raw().build(),
    "string::ends_with('Oyelowo', 'lowo')"
);
```

## string::starts_with!() <a name="starts-with-macro"></a>

The `string::starts_with!()` macro allows you to check if a string starts with a specified substring.

**Examples:**

```rust
use crate::functions::string;
use crate::*;

let name = Field::new("name");
let result = string::starts_with!(name, "lowo");
assert_eq!(
    result.fine_tune_params(),
    "string::starts_with(name, $_param_00000001)"
);
assert_eq!(result.to_raw().build(), "string::starts_with(name, 'lowo')");

let result = string::starts_with!("Oyelowo", "Oye");
assert_eq!(
    result.fine_tune_params(),
    "string::starts_with($_param_00000001, $_param_00000002)"
);
assert_eq!(
    result.to_raw().build(),
    "string::starts_with('Oyelowo', 'Oye')"
);
```

## string::split!() <a name="split-macro"></a>

The `string::split!()` macro allows you to split a string into multiple substrings based on a delimiter.

**Examples:**

```rust
use crate::functions::string;
use crate::*;

let phrase = Field::new("phrase");
let result = string::split!(phrase, ", ");
assert_eq!(
    result.fine_tune_params(),
    "string::split(phrase, $_param_00000001)"
);
assert_eq!(result.to_raw().build(), "string::split(phrase, ', ')");

let result = string::split!(
    "With great power, comes great responsibility",
    ", "
);
assert_eq!(
    result.fine_tune_params(),
    "string::split($_param_00000001, $_param_00000002)"
);
assert_eq!(
    result.to_raw().build(),
    "string::split('With great power, comes great responsibility', ', ')"
);
```

## string::len!() <a name="length-macro"></a>

The `string::len!()` macro allows you to get the length of a string.

**Examples:**

```rust
use crate::functions::string;
use crate::*;

let name = Field::new("name");
let result = string::len!(name);
assert_eq!(result.fine_tune_params(), "string::length(name)");
assert_eq!(result.to_raw().build(), "

string::length(name)");

let result = string::len!("toronto");
assert_eq!(
    result.fine_tune_params(),
    "string::length($_param_00000001)"
);
assert_eq!(result.to_raw().build(), "string::length('toronto')");
```

## string::reverse!() <a name="reverse-macro"></a>

The `string::reverse!()` macro allows you to reverse a string.

**Examples:**

```rust
use crate::functions::string;
use crate::*;

let name = Field::new("name");
let result = string::reverse!(name);
assert_eq!(result.fine_tune_params(), "string::reverse(name)");
assert_eq!(result.to_raw().build(), "string::reverse(name)");

let result = string::reverse!("oyelowo");
assert_eq!(
    result.fine_tune_params(),
    "string::reverse($_param_00000001)"
);
assert_eq!(result.to_raw().build(), "string::reverse('oyelowo')");
```

## string::trim!() <a name="trim-macro"></a>

The `string::trim!()` macro allows you to remove leading and trailing whitespace from a string.

**Examples:**

```rust
use crate::functions::string;
use crate::*;

let name = Field::new("name");
let result = string::trim!(name);
assert_eq!(result.fine_tune_params(), "string::trim(name)");
assert_eq!(result.to_raw().build(), "string::trim(name)");

let result = string::trim!("oyelowo");
assert_eq!(result.fine_tune_params(), "string::trim($_param_00000001)");
assert_eq!(result.to_raw().build(), "string::trim('oyelowo')");
```

## string::slug!() <a name="slug-macro"></a>

The `string::slug!()` macro allows you to convert a string into a slug.

**Examples:**

```rust
use crate::functions::string;
use crate::*;

let name = Field::new("name");
let result = string::slug!(name);
assert_eq!(result.fine_tune_params(), "string::slug(name)");
assert_eq!(result.to_raw().build(), "string::slug(name)");

let result = string::slug!("Codebreather is from #Jupiter");
assert_eq!(result.fine_tune_params(), "string::slug($_param_00000001)");
assert_eq!(
    result.to_raw().build(),
    "string::slug('Codebreather is from #Jupiter')"
);
```

## string::lowercase!() <a name="lowercase-macro"></a>

The `string::lowercase!()` macro allows you to convert a string to lowercase.

**Examples:**

```rust
use crate::functions::string;
use crate::*;

let name = Field::new("name");
let result = string::lowercase!(name);
assert_eq!(result.fine_tune_params(), "string::lowercase(name)");
assert_eq!(result.to_raw().build(), "string::lowercase(name)");

let result = string::lowercase!("OYELOWO");
assert_eq!(
    result.fine_tune_params(),
    "string::lowercase($_param_00000001)"
);
assert_eq!(result.to_raw().build(), "string::lowercase('OYELOWO')");
```

## string::uppercase!() <a name="uppercase-macro"></a>

The `string::uppercase!()` macro allows you to convert a string to uppercase.

**Examples:**

```rust
use crate::functions::string;
use crate::*;

let name = Field::new("name");
let result = string::uppercase!(name);
assert

_eq!(result.fine_tune_params(), "string::uppercase(name)");
assert_eq!(result.to_raw().build(), "string::uppercase(name)");

let result = string::uppercase!("oyelowo");
assert_eq!(
    result.fine_tune_params(),
    "string::uppercase($_param_00000001)"
);
assert_eq!(result.to_raw().build(), "string::uppercase('oyelowo')");
```

## string::words!() <a name="words-macro"></a>

The `string::words!()` macro allows you to split a string into individual words.

**Examples:**

```rust
use crate::functions::string;
use crate::*;

let sentence = Field::new("sentence");
let result = string::words!(sentence);
assert_eq!(result.fine_tune_params(), "string::words(sentence)");
assert_eq!(result.to_raw().build(), "string::words(sentence)");

let result = string::words!("The quick brown fox");
assert_eq!(
    result.fine_tune_params(),
    "string::words($_param_00000001)"
);
assert_eq!(result.to_raw().build(), "string::words('The quick brown fox')");
```

## string::repeat!() <a name="repeat-macro"></a>

The `string::repeat!()` macro allows you to repeat a string multiple times.

**Examples:**

```rust
use crate::functions::string;
use crate::*;

let word = Field::new("word");
let result = string::repeat!(word, 3);
assert_eq!(result.fine_tune_params(), "string::repeat(word, $_param_00000001)");
assert_eq!(result.to_raw().build(), "string::repeat(word, 3)");

let result = string::repeat!("hello", 5);
assert_eq!(result.fine_tune_params(), "string::repeat($_param_00000001, $_param_00000002)");
assert_eq!(result.to_raw().build(), "string::repeat('hello', 5)");
```

## string::replace!() <a name="replace-macro"></a>

The `string::replace!()` macro allows you to replace occurrences of a substring in a string with another substring.

**Examples:**

```rust
use crate::functions::string;
use crate::*;

let phrase = Field::new("phrase");
let result = string::replace!(phrase, "world", "Universe");
assert_eq!(
    result.fine_tune_params(),
    "string::replace(phrase, $_param_00000001, $_param_00000002)"
);
assert_eq!(
    result.to_raw().build(),
    "string::replace(phrase, 'world', 'Universe')"
);

let result = string::replace!("Hello, world!", "world", "Universe");
assert_eq!(
    result.fine_tune_params(),
    "string::replace($_param_00000001, $_param_00000002, $_param_00000003)"
);
assert_eq!(
    result.to_raw().build(),
    "string::replace('Hello, world!', 'world', 'Universe')"
);
```

## string::slice!() <a name="slice-macro"></a>

The `string::slice!()` macro allows you to extract a portion of a string.

**Examples:**

```rust
use crate::functions::string;
use crate::*;

let phrase = Field::new("phrase");
let result = string::slice!(phrase, 6, 11);
assert_eq!(
    result.fine_tune_params(),
    "string::slice(phrase, $_param_00000001, $_param_00000002)"
);
assert_eq!(result.to_raw().build(), "string::slice

(phrase, 6, 11)");

let result = string::slice!("Hello, world!", 7, 12);
assert_eq!(
    result.fine_tune_params(),
    "string::slice($_param_00000001, $_param_00000002, $_param_00000003)"
);
assert_eq!(
    result.to_raw().build(),
    "string::slice('Hello, world!', 7, 12)"
);
```

## string::concat!() <a name="concat-macro"></a>

The `string::concat!()` macro allows you to concatenate multiple strings.

**Examples:**

```rust
use crate::functions::string;
use crate::*;

let word1 = Field::new("word1");
let word2 = Field::new("word2");
let result = string::concat!(word1, " ", word2);
assert_eq!(
    result.fine_tune_params(),
    "string::concat(word1, $_param_00000001, word2)"
);
assert_eq!(result.to_raw().build(), "string::concat(word1, ' ', word2)");

let result = string::concat!("Hello", ", ", "world!");
assert_eq!(
    result.fine_tune_params(),
    "string::concat($_param_00000001, $_param_00000002, $_param_00000003)"
);
assert_eq!(
    result.to_raw().build(),
    "string::concat('Hello', ', ', 'world!')"
);
```

## string::to_string!() <a name="to-string-macro"></a>

The `string::to_string!()` macro allows you to convert a value to a string.

**Examples:**

```rust
use crate::functions::string;
use crate::*;

let number = Field::new("number");
let result = string::to_string!(number);
assert_eq!(
    result.fine_tune_params(),
    "string::to_string(number)"
);
assert_eq!(result.to_raw().build(), "string::to_string(number)");

let result = string::to_string!(42);
assert_eq!(
    result.fine_tune_params(),
    "string::to_string($_param_00000001)"
);
assert_eq!(result.to_raw().build(), "string::to_string(42)");
```
