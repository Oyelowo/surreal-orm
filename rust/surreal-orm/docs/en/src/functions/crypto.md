# Crypto Functions

This chapter introduces the crypto macros provided by SurrealDB ORM. The crypto macros are used for cryptographic operations such as password hashing and comparison.

## Table of Contents

- [argon2::compare!()](#argon2-compare-macro)
- [argon2::generate!()](#argon2-generate-macro)
- [pbkdf2::compare!()](#pbkdf2-compare-macro)
- [pbkdf2::generate!()](#pbkdf2-generate-macro)
- [scrypt::compare!()](#scrypt-compare-macro)
- [scrypt::generate!()](#scrypt-generate-macro)
- [bcrypt::compare!()](#bcrypt-compare-macro)
- [bcrypt::generate!()](#bcrypt-generate-macro)

## <a name="argon2-compare-macro"></a>argon2::compare!()

The `argon2::compare!()` macro compares two values using the Argon2 hashing algorithm. It has the following syntax:

```rust
let result = argon2::compare!("Oyelowo", "Oyedayo");
```

The `argon2::compare!()` macro generates the following SQL query:

```plaintext
crypto::argon2::compare('Oyelowo', 'Oyedayo')
```

## <a name="argon2-generate-macro"></a>argon2::generate!()

The `argon2::generate!()` macro generates a hash value using the Argon2 hashing algorithm. It has the following syntax:

```rust
let result = argon2::generate!("Oyelowo");
```

The `argon2::generate!()` macro generates the following SQL query:

```plaintext
crypto::argon2::generate('Oyelowo')
```

## <a name="pbkdf2-compare-macro"></a>pbkdf2::compare!()

The `pbkdf2::compare!()` macro compares two values using the PBKDF2 hashing algorithm. It has the following syntax:

```rust
let result = pbkdf2::compare!("hash_value", "password");
```

The `pbkdf2::compare!()` macro generates the following SQL query:

```plaintext
crypto::pbkdf2::compare('hash_value', 'password')
```

## <a name="pbkdf2-generate-macro"></a>pbkdf2::generate!()

The `pbkdf2::generate!()` macro generates a hash value using the PBKDF2 hashing algorithm. It has the following syntax:

```rust
let result = pbkdf2::generate!("password");
```

The `pbkdf2::generate!()` macro generates the following SQL query:

```plaintext
crypto::pbkdf2::generate('password')
```

## <a name="scrypt-compare-macro"></a>scrypt::compare!()

The `scrypt::compare!()` macro compares two values using the scrypt hashing algorithm. It has the following syntax:

```rust
let result = scrypt::compare!("hash_value", "password");
```

The `scrypt::compare!()` macro generates the following SQL query:

```plaintext
crypto::scrypt::compare('hash_value', 'password')
```

## <a name="scrypt-generate-macro"></a>scrypt::generate!()

The `scrypt::generate!()` macro generates a hash value using the scrypt hashing algorithm. It has the

following syntax:

```rust
let result = scrypt::generate!("password");
```

The `scrypt::generate!()` macro generates the following SQL query:

```plaintext
crypto::scrypt::generate('password')
```

## <a name="bcrypt-compare-macro"></a>bcrypt::compare!()

The `bcrypt::compare!()` macro compares two values using the bcrypt hashing algorithm. It has the following syntax:

```rust
let result = bcrypt::compare!("hash_value", "password");
```

The `bcrypt::compare!()` macro generates the following SQL query:

```plaintext
crypto::bcrypt::compare('hash_value', 'password')
```

## <a name="bcrypt-generate-macro"></a>bcrypt::generate!()

The `bcrypt::generate!()` macro generates a hash value using the bcrypt hashing algorithm. It has the following syntax:

```rust
let result = bcrypt::generate!("password");
```

The `bcrypt::generate!()` macro generates the following SQL query:
