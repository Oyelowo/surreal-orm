# HTTP Functions

This chapter introduces the http macros provided by the SurrealDB ORM. The http macros are used for performing remote HTTP requests such as HEAD, GET, POST, PUT, and PATCH.

## Table of Contents

- [http::head!()](#http-head-macro)
- [http::get!()](#http-get-macro)
- [http::delete!()](#http-delete-macro)
- [http::post!()](#http-post-macro)
- [http::put!()](#http-put-macro)
- [http::patch!()](#http-patch-macro)

## <a name="http-head-macro"></a>http::head!()

The `http::head!()` macro performs a remote HTTP HEAD request. It has the following syntax:

```rust
http::head!("https://codebreather.com");
```

The `http::head!()` macro generates the following function call:

```plaintext
http::head("https://codebreather.com", None as Option<ObjectLike>)
```

## <a name="http-get-macro"></a>http::get!()

The `http::get!()` macro performs a remote HTTP GET request. It has the following syntax:

```rust
http::get!("https://codebreather.com");
```

The `http::get!()` macro generates the following function call:

```plaintext
http::get("https://codebreather.com", None as Option<ObjectLike>)
```

## <a name="http-delete-macro"></a>http::delete!()

The `http::delete!()` macro performs a remote HTTP DELETE request. It has the following syntax:

```rust
http::delete!("https://codebreather.com");
```

The `http::delete!()` macro generates the following function call:

```plaintext
http::delete("https://codebreather.com", None as Option<ObjectLike>)
```

## <a name="http-post-macro"></a>http::post!()

The `http::post!()` macro performs a remote HTTP POST request. It has the following syntax:

```rust
http::post!("https://codebreather.com", body);
```

The `http::post!()` macro generates the following function call:

```plaintext
http::post("https://codebreather.com", body, None as Option<ObjectLike>)
```

## <a name="http-put-macro"></a>http::put!()

The `http::put!()` macro performs a remote HTTP PUT request. It has the following syntax:

```rust
http::put!("https://codebreather.com", body);
```

The `http::put!()` macro generates the following function call:

```plaintext
http::put("https://codebreather.com", body, None as Option<ObjectLike>)
```

## <a name="http-patch-macro"></a>http::patch!()

The `http::patch!()` macro performs a remote HTTP PATCH request. It has the following syntax:

```rust
http::patch!("https://codebreather.com", body);
```

The `http::patch!()` macro generates the following function call:

```plaintext
http::patch("https://codebreather.com", body, None as Option<ObjectLike>)
```
