# Math Functions

## Table of Contents

- [math::abs!()](#math-abs-macro)
- [math::ceil!()](#math-ceil-macro)
- [math::floor!()](#math-floor-macro)
- [math::round!()](#math-round-macro)
- [math::sqrt!()](#math-sqrt-macro)
- [math::mean!()](#math-mean-macro)
- [math::median!()](#math-median-macro)
- [math::mode!()](#math-mode-macro)
- [math::min!()](#math-min-macro)
- [math::product!()](#math-product-macro)
- [math::sum!()](#math-sum-macro)

---

## math::abs!() <a name="math-abs-macro"></a>

The math::abs function returns the absolute value of a number.

Function signature: `math::abs(number) -> number`

**Example:**

```rust
math::abs!(45.23);
```

---

## math::ceil!() <a name="math-ceil-macro"></a>

The math::ceil function rounds a number up to the next largest integer.

Function signature: `math::ceil(number) -> number`

**Example:**

```rust
math::ceil!(45.23);
```

---

## math::floor!() <a name="math-floor-macro"></a>

The math::floor function rounds a number down to the next largest integer.

Function signature: `math::floor(number) -> number`

**Example:**

```rust
math::floor!(45.23);
```

---

## math::round!() <a name="math-round-macro"></a>

The math::round function rounds a number up or down to the nearest integer.

Function signature: `math::round(number) -> number`

**Example:**

```rust
math::round!(45.23);
```

---

## math::sqrt!() <a name="math-sqrt-macro"></a>

The math::sqrt function returns the square root of a number.

Function signature: `math::sqrt(number) -> number`

**Example:**

```rust
math::sqrt!(45.23);
```

---

## math::mean!() <a name="math-mean-macro"></a>

The math::mean function returns the average of a set of numbers.

Function signature: `math::mean(array) -> number`

**Example:**

```rust
math::mean!(vec![1, 2, 3, 4, 5]);
```

---

## math::median!() <a name="math-median-macro"></a>

The math::median function returns the median of a set of numbers.

Function signature: `math::median(array) -> number`

**Example:**

```rust
math::median!(vec![1, 2, 3, 4, 5]);
```

---

## math::mode!() <a name="math-mode-macro"></a>

The math::mode function returns the mode of a set of numbers.

Function signature: `math::mode(array) -> number`

**Example:**

```rust
math::mode!(vec![1, 2, 3, 4, 5]);
```

---

## math::min!() <a name="math-min-macro"></a>

The math::min function returns the minimum number in a set of numbers.

Function signature: `math::min(array) -> number`

**Example:**

```rust
math::min!(vec![1, 2, 3, 4, 5]);
```

---

## math::product!() <a name="math-product-macro"></a>

The math::product function returns the product of a set of numbers.

Function signature: `math::product(array) -> number`

**Example:**

```rust
math::product!(vec![1, 2, 3, 4, 5]);
```

---

## math::sum!() <a name="math-sum-macro"></a>

The math::sum function returns the total sum of a set of numbers.

Function signature: `math::sum(array) -> number`

**Example:**

```rust
math::sum!(vec![1, 2, 3, 4, 5]);
```

---

That concludes the documentation for the math macros.
