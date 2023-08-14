# Session functions

## Table of Contents

- [session::db!()](#db-macro)
- [session::id!()](#id-macro)
- [session::ip!()](#ip-macro)
- [session::ns!()](#ns-macro)
- [session::origin!()](#origin-macro)
- [session::sc!()](#sc-macro)

## session::db!() <a name="db-macro"></a>

The `session::db!()` macro returns the currently selected database.

**Example:**

```rust
use surreal_orm::functions::session;

session::db!();
```

## session::id!() <a name="id-macro"></a>

The `session::id!()` macro returns the current user's session ID.

**Example:**

```rust
use surreal_orm::functions::session;

session::id!();
```

## session::ip!() <a name="ip-macro"></a>

The `session::ip!()` macro returns the current user's session IP address.

**Example:**

```rust
use surreal_orm::functions::session;

session::ip!();
```

## session::ns!() <a name="ns-macro"></a>

The `session::ns!()` macro returns the currently selected namespace.

**Example:**

```rust
use surreal_orm::functions::session;

session::ns!();
```

## session::origin!() <a name="origin-macro"></a>

The `session::origin!()` macro returns the current user's HTTP origin.

**Example:**

```rust
use surreal_orm::functions::session;

session::origin!();
```

## session::sc!() <a name="sc-macro"></a>

The `session::sc!()` macro returns the current user's authentication scope.

**Example:**

```rust
use surreal_orm::functions::session;

session::sc!();
```
