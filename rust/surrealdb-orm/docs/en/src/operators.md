## Operators

# SQL Query Builder: Field Operators

All these operators can also be chained together to create more complex conditions. For instance:

```rust
age.outside(18, 65).and(age.not_equal(99));
p1.outside(18, 65).and(p1.not_equal(99));
```

The chaining of these operators is quite flexible and allows for the construction of complex query logic in a clear and concise manner.

## Table of Contents

- [Introduction](#introduction)
- [Comparison Operators](#comparison-operators)
  - [`equal`, `eq`](#equal-eq)
  - [`not_equal`, `neq`](#not_equal-neq)
  - [`exactly_equal`](#exactly_equal)
  - [`any_equal`](#any_equal)
  - [`all_equal`](#all_equal)
- [String Operators](#string-operators)
  - [`like`](#like)
  - [`not_like`](#not_like)
  - [`any_like`](#any_like)
  - [`all_like`](#all_like)
- [Relational Operators](#relational-operators)
  - [`less_than`, `lt`](#less_than-lt)
  - [`less_than_or_equal`, `lte`](#less_than_or_equal-lte)
  - [`greater_than`, `gt`](#greater_than-gt)
  - [`greater_than_or_equal`, `gte`](#greater_than_or_equal-gte)
- [Arithmetic Operators](#arithmetic-operators)
  - [`add`, `plus`](#add-plus)
  - [`subtract`, `minus`](#subtract-minus)
  - [`multiply`, `mul`](#multiply-mul)
  - [`divide`, `div`](#divide-div)
  - [`power`, `pow`](#power-pow)
- [Logical Operators](#logical-operators)
  - [`truthy_and`](#truthy_and)
  - [`truthy_or`](#truthy_or)
  - [`and`](#and)
  - [`or`](#or)
  - [`is`](#is)
  - [`is_not`](#is_not)
- [Membership Operators](#membership-operators)
  - [`contains`](#contains)
  - [`contains_not`](#contains_not)
  - [`contains_all`](#contains_all)
  - [`contains_any`](#contains_any)
  - [`contains_none`](#contains_none)
  - [`inside`, `in_`](#inside_in)
  - [`not_inside`](#not_inside)
  - [`all_inside`](#all_inside)
  - [`any_inside`](#any_inside)
  - [`none_inside`](#none_inside)
  - [`outside`](#outside)
  - [`intersects`](#intersects)
- [Conclusion](#conclusion)

## Introduction

This document provides an overview of the different SQL query field operators available in our SQL Query Builder. For each operator, a brief description, usage, and examples are provided.

## Comparison Operators

### `equal`, `eq`

The `equal` operator checks if the given field is equal to the provided value. It returns true if the condition is met, and false otherwise.

The `eq` operator is an alias for `equal`.

Usage:

````rust
# use surreal_orm::*;
# let field = Field::new("field");
field.equal(5);
field.eq(9);

### `not_equal`, `neq`

The `not_equal` operator checks if the given field is not equal to the provided value. It returns true if the condition is met, and false otherwise.

The `neq` operator is an alias for `not_equal`.

Usage:

```rust
# use surreal_orm::*;
# let ref price = Field::new("price");
price.not_equal(100);
price.neq(100);
````

### `exactly_equal`

The `exactly_equal` operator checks if the given field is exactly equal to the provided value. It is generally used for fields that are binary or have specific precision requirements.

Usage:

```rust
# use surreal_orm::*;
# let name = Field::new("name");
# let p1 = Field::new("p1");
name.exactly_equal("Oyelowo");
p1.exactly_equal(3.14);
```

### `any_equal`

The `any_equal` operator checks if any value in a list of values is equal to the given field.

Usage:

```rust
# use surreal_orm::*;
# let status = Field::new("status");
# let p1 = Field::new("p1");
status.any_equal(vec!["ACTIVE", "IN PROGRESS"]);
p1.any_equal(vec!["APPLE", "BANANA"]);
```

### `all_equal`

The `all_equal` operator checks if all values in a list are equal to the given field. It's a niche operator and has limited use.

Usage:

```rust
# use surreal_orm::*;
# let id = Field::new("id");
# let p1 = Field::new("p1");
id.all_equal([1, 3, 5]);
p1.all_equal([2, 2, 2]);
```

### `like`

The `like` operator checks if the given field matches the provided pattern. `%` is used as a wildcard character.

Usage:

```rust
# use surreal_orm::*;
# let name = Field::new("name");
# let p1 = Field::new("p1");
name.like("Jo");
p1.like("son");
```

### `not_like`

The `not_like` operator checks if the given field does not match the provided pattern. `%` is used as a wildcard character.

Usage:

```rust
# use surreal_orm::*;
# let name = Field::new("name");
# let p1 = Field::new("p1");
name.not_like("Jo%");
p1.not_like("%son");
```

### `any_like`

The `any_like` operator checks if any value in a list of values matches the given field.

Usage:

```rust
# use surreal_orm::*;
# let status = Field::new("status");
# let p1 = Field::new("p1");
status.any_like(vec!["ACTIVE", "IN PROGRESS"]);
p1.any_like(vec!["APPLE", "BANANA"]);
```

### `all_like`

The `all_like` operator checks if all values in a list match the given field. It's a niche operator and has limited use.

Usage:

```rust
# use surreal_orm::*;
# let id = Field::new("id");
# let p1 = Field::new("p1");
id.all_like(vec!["1", "2", "4"]);
p1.all_like(vec!["2", "2", "2"]);
```

### `less_than`, `lt`

The `less_than` operator checks if the given field is less than the provided value.

Usage:

```rust
# use surreal_orm::*;
# let age = Field::new("age");
# let p1 = Field::new("p1");
age.less_than(18);
p1.lt(100);
```

### `less_than_or_equal`, `lte`

The `less_than_or_equal` operator checks if the given field is less than or equal to the provided value.

Usage:

```rust
# use surreal_orm::*;
# let age = Field::new("age");
# let p1 = Field::new("p1");
age.less_than_or_equal(18);
p1.lte(100);
```

### `greater_than`, `gt`

The `greater_than` operator checks if the given field is greater than the provided value.

Usage:

```rust
# use surreal_orm::*;
# let age = Field::new("age");
# let p1 = Field::new("p1");
age.greater_than(18);
p1.gt(100);
```

### `greater_than_or_equal`, `gte`

The `greater_than_or_equal` operator checks if the given field is greater than or equal to the provided value.

Usage:

```rust
# use surreal_orm::*;
# let age = Field::new("age");
# let p1 = Field::new("p1");
age.greater_than_or_equal(18);
p1.gte(100);
```

<h2 id="add-plus">add, plus, +</h2>

The `add` operator adds a value to the given field.

Usage:

```rust
# use surreal_orm::*;
# let salary = Field::new("salary");
# let age = Field::new("age");
# let p1 = Field::new("p1");
salary.add(500);
age + 500;
p1.plus(200);
```

<h2 id="subtract-minus">subtract, minus, -</h2>

The `subtract` operator subtracts a value from the given field.

Usage:

```rust
# use surreal_orm::*;
# let salary = Field::new("salary");
# let age = Field::new("age");
# let p1 = Field::new("p1");
salary.subtract(500);
age - 500;
p1.subtract(200);
```

<h2 id="multiply-mul">multiply, mul, *</h2>

The `multiply` operator multiplies the given field by the provided value.

Usage:

```rust
# use surreal_orm::*;
# let salary = Field::new("salary");
# let quantity = Field::new("quantity");
# let p1 = Field::new("p1");
quantity.multiply(price);
salary * 434;
p1.multiply(10);
```

This will generate the following SQL statement:

```sql
quantity * price
p1 * 10
```

<h2 id="divide-div">divide, div, /</h2>

The `divide` operator divides the given field by the provided value.

Usage:

```rust
# use surreal_orm::*;
# let salary = Field::new("salary");
# let quantity = Field::new("quantity");
# let count = Field::new("count");
# let param = Param::new("param");
salary.divide(343);
quantity / count;
param.div(2);
```

### `power`, `pow`

The `power` operator raises the given field to the power of the provided value.

Usage:

```rust
# use surreal_orm::*;
# let length = Field::new("length");
# let p1 = Field::new("p1");
length.power(2);
p1.power(3);
```

### `trthy_and`

The `truthy_and` operator performs a logical AND operation between the field and the provided value.

Usage:

```rust
# use surreal_orm::*;
# let is_active = Field::new("is_active");
is_active.truthy_and(true);
```

### `truthy_or`

The `truthy_or` operator performs a logical OR operation between the field and the provided value.

Usage:

```rust
# use surreal_orm::*;
# let is_active = Field::new("is_active");
is_active.truthy_or(is_paid);
```

This will generate the following SQL statement:

```sql
is_active OR is_paid
p1 OR p2
```

### `and`

The `and` operator is used to combine multiple conditions in a WHERE clause to create more complex conditions. It returns true if all conditions are true.

Usage:

```rust
# use surreal_orm::*;
# let price = Field::new("price");
price.and(54).and(92);
```

### `or`

The `or` operator is used to combine multiple conditions in a WHERE clause to create more complex conditions. It returns true if at least one of the conditions is true.

Usage:

```rust
# use surreal_orm::*;
# let  = Field::new("price");
is_active.or(is_paid);
p1.or(p2);
```

This will generate the following SQL statement:

### `is`

The `is` operator compares if a field is equal to a specific value.

Usage:

```rust
# use surreal_orm::*;
# let age = Field::new("age");
# let p1 = Field::new("p1");
age.is(21);
p1.is("John");
```

### `is_not`

The `is_not` operator compares if a field is not equal to a specific value.

Usage:

```rust
# use surreal_orm::*;
# let age = Field::new("age");
age.is_not(21);
p1.is_not("John");
```

### `contains`

The `contains` operator checks if a field contains a specific value.

Usage:

```rust
# use surreal_orm::*;
# let names = Field::new("names");
names.contains("John");
```

### `contains_not`

The `contains_not` operator checks if a field does not contain a specific value.

Usage:

```rust
# use surreal_orm::*;
# let names = Field::new("names");
names.contains_not("John");
```

### `contains_all`

The `contains_all` operator checks if a field contains all specified values.

Usage:

```rust
# use surreal_orm::*;
# let tags = Field::new("tags");
tags.contains_all(vec!["novel", "adventure"]);
```

### `contains_any`

The `contains_any` operator checks if a field contains any of the specified values.

Usage:

```rust
# use surreal_orm::*;
# let tags = Field::new("tags");
tags.contains_any(vec!["novel", "adventure"]);
```

### `contains_none`

The `contains_none` operator checks if a field does not contain any of the specified values.

Usage:

```rust
# use surreal_orm::*;
# let tags = Field::new("tags");
tags.contains_none(vec!["novel", "adventure"]);
```

### `inside` and `in_`

The `inside` and `in_` operators check if a field's value is within a specified array of values.

Usage:

```rust
# use surreal_orm::*;
# let scores = Field::new("scores");
# let p1 = Field::new("p1");
scores.inside(vec![20, 30, 40]);
p1.inside(vec![20, 30, 40]);
```

### `not_inside`

The `not_inside` operator checks if a field's value is not within a specified array of values.

Usage:

```rust
# use surreal_orm::*;
# let scores = Field::new("scores");
scores.not_inside(vec![20, 30, 40]);
```

### `all_inside`

The `all_inside` operator checks if all values in a field are within a specified array of values.

Usage:

```rust
# use surreal_orm::*;
# let tags = Field::new("tags");
tags.all_inside(["novel", "adventure", "mystery"]);
```

### `any_inside`

The `any_inside` operator checks if any value in a field is within a specified array of values.

Usage:

```rust
# use surreal_orm::*;
# let tags = Field::new("tags");
tags.any_inside(vec!["novel", "adventure", "mystery"]);
```

### `none_inside`

The `none_inside` operator checks if none of the values in a field are within a specified array of values.

Usage:

```rust
# use surreal_orm::*;
# let tags = Field::new("tags");
tags.none_inside(["novel", "adventure", "mystery"]);
```

### `outside`

The `outside` operator checks whether a geometry value is outside another geometry value.

Usage:

```rust
# use surreal_orm::*;
# let point = Field::new("point");
# let area = Param::new("area");
point.outside(area);
```

### `intersects`

The `intersects` operator checks whether a geometry value intersects annother geometry value.

Usage:

```rust
# use surreal_orm::*;
# let area1 = Field::new("area1");
# let area2 = Field::new("area2");
area1.intersects(area2);
```

Also, note the distinction between `Field` and `Param` in the usage and examples.
A `Field` represents a column in a database table, while a `Param` represents a parameter
that could be used in the for value assignment. These are interchangeable in the context of these operators,
meaning that you can apply the same operators whether you are comparing fields or parameters.

## Conclusion

This document covers the complete list of SQL Query Builder field operators.
Using these operators will help you build complex and robust SQL queries.
Always ensure that you use the correct operator for your specific needs to prevent unexpected results or errors.
