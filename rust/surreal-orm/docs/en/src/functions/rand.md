# Rand Functions

## Table of Contents

- [rand!()](#rand-macro)
- [rand::bool!()](#bool-macro)
- [rand::uuid!()](#uuid-macro)
- [rand::uuid::v4!()](#uuid-v4-macro)
- [rand::uuid::v7!()](#uuid-v7-macro)
- [rand::enum!()](#enum-macro)
- [rand::string!()](#string-macro)
- [rand::guid!()](#guid-macro)
- [rand::float!()](#float-macro)
- [rand::int!()](#int-macro)
- [rand::time!()](#time-macro)

## rand!() <a name="rand-macro"></a>

The `rand!()` macro generates a random number.

**Example:**

```rust
use surreal_orm::functions::rand;

let result = rand!();
```

## rand::bool!() <a name="bool-macro"></a>

The `rand::bool!()` macro generates a random boolean value.

**Example:**

```rust
use surreal_orm::functions::rand;

let result = rand::bool!();
```

## rand::uuid!() <a name="uuid-macro"></a>

The `rand::uuid!()` macro generates a random UUID.

**Example:**

```rust
use surreal_orm::functions::rand;

let result = rand::uuid!();
```

## rand::uuid::v4!() <a name="uuid-v4-macro"></a>

The `rand::uuid::v4!()` macro generates a random UUID v4.

**Example:**

```rust
use surreal_orm::functions::rand;

let result = rand::uuid::v4!();
```

## rand::uuid::v7!() <a name="uuid-v7-macro"></a>

The `rand::uuid::v7!()` macro generates a random UUID v7.

**Example:**

```rust
use surreal_orm::functions::rand;

let result = rand::uuid::v7!();
```

## rand::enum!() <a name="enum-macro"></a>

The `rand::enum!()` macro generates a random value from a list of options.

**Example:**

```rust
use surreal_orm::functions::rand;
use surreal_orm::functions::rand::arr;

let result = rand::enum!(arr!["one", "two", 3, 4.15385, "five", true]);
```

## rand::string!() <a name="string-macro"></a>

The `rand::string!()` macro generates a random string.

**Example:**

```rust
use surreal_orm::functions::rand;

let result = rand::string!();
```

## rand::guid!() <a name="guid-macro"></a>

The `rand::guid!()` macro generates a random GUID.

**Example:**

```rust
use surreal_orm::functions::rand;

let result = rand::guid!();
```

## rand::float!() <a name="float-macro"></a>

The `rand::float!()` macro generates a random floating-point number.

**Example:**

```rust
use surreal_orm::functions::rand;

let result = rand::float!();
```

## rand::int!() <a name="int-macro"></a>

The `rand::int!()` macro generates a random integer.

**Example:**

```rust
use surreal_orm::functions::rand;

let result = rand::int!();
```

## rand::time!() <a name="time-macro"></a>

The `rand::time!()` macro generates a random time value.

**Example:**

```rust
use surreal_orm::functions::rand;

let result = rand::time!();
```
