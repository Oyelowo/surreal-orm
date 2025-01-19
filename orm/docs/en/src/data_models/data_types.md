# Data Types

# Data Model in `surreal_orm`

In the `surreal_orm`, developers are provided with a comprehensive data model
that mirrors the specifications laid out by the SurrealDB documentation. This
ensures seamless integration with SurrealDB while also extending the
capabilities to cater to more advanced use cases, such as supporting diverse
value types in one unified representation.

## Table of Contents

1. [Overview](#overview)
2. [Record IDs](#record-ids)
3. [Basic Types](#basic-types)
   - [Strings](#strings)
   - [Numbers](#numbers)
   - [Datetimes](#datetimes)
   - [Objects](#objects)
   - [Arrays](#arrays)
   - [Geometries](#geometries)
4. [Record Links](#record-links)

---

## Overview

The data model in `surreal_orm` allows for a flexible representation of
different data types. By utilizing structures such as `ValueType`, the ORM can
represent a wide array of types from basic values, fields, parameters, to
complex operations and statements.

## Record IDs

While the official SurrealDB documentation might detail how unique identifiers
are managed for records, the ORM's handling of this might be implicit or handled
in a way that abstracts the details away from the developer. You can read more
on a dedicated chapter to `Surreal Id` where an abstraction is created to make
it a easier, more intuitive and consistent to work with record ids in surrealdb.

## Basic Types

### Strings

In `surreal_orm`, strings are represented using the `StrandLike` structure:

```rust
pub struct StrandLike(..);
```

This struct can be used to represent a string value, field, or parameter,
allowing it to be seamlessly integrated into various parts of a query.

### Numbers

Numbers are represented using the `NumberLike` structure:

```rust
pub struct NumberLike(..);
```

Like `StrandLike`, it can be used to represent a numeric value, field, or
parameter in a query.

### Datetimes

Datetimes are encapsulated using the `DatetimeLike` structure:

```rust
pub struct DatetimeLike(..);
```

This allows for a clear representation of date and time values within the ORM.

### Objects

Objects are complex data types that encapsulate key-value pairs. They are
represented in `surreal_orm` using the `ObjectLike` structure:

```rust
pub struct ObjectLike(..);
```

### Arrays

Arrays, which can contain multiple items of the same type, are represented using
the `ArrayLike` structure:

```rust
pub struct ArrayLike(..);
```

And for function arguments, the `ArgsList` structure is used:

```rust
pub struct ArgsList(..);
```

### Geometries

Geometries, which might represent spatial data, are encapsulated in the
`GeometryLike` structure:

```rust
pub struct GeometryLike(..);
```

## Record Links

While the provided code does not show explicit handling for record links, it can
be inferred that such links could be represented using `SurrealId` types.

---

This is a foundational overview of the data model in `surreal_orm`, with the aim
of mirroring the SurrealDB specifications. The ORM extends the basic data types
to provide a richer experience, supporting various operations and query
constructs seamlessly.
