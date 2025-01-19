# Casting

Casting is an indispensable tool in data management, allowing developers to
convert values from one type to another. This chapter provides an in-depth look
into the casting functionality provided by `surreal_orm`, illuminating its
power, elegance, and strict adherence to the SurrealDB specifications.

## Table of Contents

1. [Introduction to Casting](#introduction-to-casting)
2. [The `Cast` Structure in `surreal_orm`](#the-cast-structure-in-surreal_orm)
3. [Cast Functions](#cast-functions)
   - [Casting to Boolean](#casting-to-boolean)
   - [Casting to Integer](#casting-to-integer)
   - [Casting to Float](#casting-to-float)
   - [Casting to String](#casting-to-string)
   - [Casting to Number](#casting-to-number)
   - [Casting to Decimal](#casting-to-decimal)
   - [Casting to DateTime](#casting-to-datetime)
   - [Casting to Duration](#casting-to-duration)
4. [Conclusion](#conclusion)

---

## Introduction to Casting

In programming, casting is the practice of converting variables from one type to
another, enabling more flexible data manipulation. Whether receiving input from
a user, reading data from a file, or interfacing with databases, casting becomes
a pivotal component.

### Casting to Boolean

This function converts a value into a boolean. In raw queries, it's represented
as `<bool>`.

```rust
let result = bool("true");
assert_eq!(result.build(), "<bool> true");
```

### Casting to Integer

Convert a value into an integer. In raw queries, it's represented by `<int>`.

```rust
let result = int(13.572948467293847293841093845679289);
assert_eq!(result.build(), "<int> 13");
```

### Casting to Float

Convert a value into a floating point number. In raw queries, it's represented
by `<float>`.

```rust
let result = float(13.572948467293847293841093845679289);
assert_eq!(result.build(), "<float> 13.572948467293847");
```

### Casting to String

Convert a value into a string. In raw queries, it's represented by `<string>`.

```rust
let result = string(true);
assert_eq!(result.build(), "<string> true");
```

### Casting to Number

Convert a value into an infinite precision decimal number. In raw queries, it's
represented by `<number>`.

```rust
let result = number(13.572948467293847293841093845679289);
assert_eq!(result.build(), "<number> 13.572948467293847293841093845679289");
```

### Casting to Decimal

Convert a value into an infinite precision decimal number. In raw queries, it's
represented by `<decimal>`.

```rust
let result = decimal(13.572948467293847293841093845679289);
assert_eq!(result.build(), "<decimal> 13.572948467293847293841093845679289");
```

### Casting to DateTime

Convert a value into a datetime. In raw queries, it's represented by
`<datetime>`.

```rust
let result = datetime("2022-06-07 will be parsed");
assert_eq!(result.build(), "<datetime> 2022-06-07");
```

### Casting to Duration

Convert a value into a duration. In raw queries, it's represented by
`<duration>`.

```rust
let result = duration("1h30m will be parsed");
assert_eq!(result.build(), "<duration> 1h30m");
```

---

## Conclusion

Surreal Orm presents a powerful and user-friendly approach to casting, adhering
closely to SurrealDB standards. Whether you're an experienced Rust developer or
just starting, surreal_orm provides the tools for precise and effortless data
manipulation.
