# Parse Functions

## Table of Contents

- [parse::email::host()](#parse-email-host-macro)
- [parse::email::user()](#parse-email-user-macro)
- [parse::url::domain()](#parse-url-domain-macro)
- [parse::url::fragment()](#parse-url-fragment-macro)
- [parse::url::host()](#parse-url-host-macro)
- [parse::url::path()](#parse-url-path-macro)
- [parse::url::port()](#parse-url-port-macro)
- [parse::url::query()](#parse-url-query-macro)

---

## parse::email::host() <a name="parse-email-host-macro"></a>

The `parse::email::host` function parses and returns the email host from a valid email address. This function is also aliased as `parse_email_host!`.

Function signature: `parse::email::host(string) -> value`

**Example:**

```rust
parse::email::host!("oyelowo@codebreather.com");
```

---

## parse::email::user() <a name="parse-email-user-macro"></a>

The `parse::email::user` function parses and returns the email username from a valid email address. This function is also aliased as `parse_email_user!`.

Function signature: `parse::email::user(string) -> value`

**Example:**

```rust
parse::email::user!("oyelowo@codebreather.com");
```

---

## parse::url::domain() <a name="parse-url-domain-macro"></a>

The `parse::url::domain` function parses and returns the domain from a valid URL. This function is also aliased as `parse_url_domain!`.

Function signature: `parse::url::domain(string) -> value`

**Example:**

```rust
parse::url::domain!("https://codebreather.com:443/topics?arg=value#fragment");
```

---

## parse::url::fragment() <a name="parse-url-fragment-macro"></a>

The `parse::url::fragment` function parses and returns the fragment from a valid URL. This function is also aliased as `parse_url_fragment!`.

Function signature: `parse::url::fragment(string) -> value`

**Example:**

```rust
parse::url::fragment!("https://codebreather.com:443/topics?arg=value#fragment");
```

---

## parse::url::host() <a name="parse-url-host-macro"></a>

The `parse::url::host` function parses and returns the hostname from a valid URL. This function is also aliased as `parse_url_host!`.

Function signature: `parse::url::host(string) -> value`

**Example:**

```rust
parse::url::host!("https://codebreather.com:443/topics?arg=value#fragment");
```

---

## parse::url::path() <a name="parse-url-path-macro"></a>

The `parse::url::path` function parses and returns the path from a valid URL. This function is also aliased as `parse_url_path!`.

Function signature: `parse::url::path(string) -> value`

**Example:**

```rust
parse::url::path!("https://codebreather.com:443/topics?arg=value#fragment");
```

---

## parse::url::port() <a name="parse-url-port-macro"></a>

The `parse::url::port` function parses and returns the port from a valid URL. This function is also aliased as `parse_url_port!`.

Function signature: `parse::url

::port(string) -> value`

**Example:**

```rust
parse::url::port!("https://codebreather.com:443/topics?arg=value#fragment");
```

---

## parse::url::query() <a name="parse-url-query-macro"></a>

The `parse::url::query` function parses and returns the query from a valid URL. This function is also aliased as `parse_url_query!`.

Function signature: `parse::url::query(string) -> value`

**Example:**

```rust
parse::url::query!("https://codebreather.com:443/topics?arg=value#fragment");
```

---

That concludes the documentation for the parse macros.
