# Let Statement

The `let` statement in SurrealDB ORM allows you to bind variables within a code block. It simplifies complex queries and enables parameter handling.

## Table of Contents

- [Recommended Approach](#recommended-approach)
  - [Using `let` or `LET` Statement/keyword within `block!` Macro](#using-let-or-let-statement-within-block-macro)
- [Less Recommended Approach](#less-recommended-approach)
  - [Using `let_!` Macro](#using-let-macro)
- [Least Recommended Approach](#least-recommended-approach)
  - [Using `let` Statements with `let_` Function](#using-let-statements-with-let-function)

## Recommended Approach

In the recommended approach, you can use the `let` statement within the `block!` macro. This approach provides a natural syntax that handles variable bindings and parameter references automatically.

### Using `let` or `LET` Statement within `block!` Macro

To define variables and bind them within a code block, you can use the `let` statement (or `LET` statement) within the `block!` macro. This approach offers simplicity and automation in handling variable bindings and parameter references. Let's take a look at an example:

```rust
let alien = Table::new("alien");
let metrics = Table::new("metrics");
let strength = Field::new("strength");

let code_block = block! {
    let strengths = select_value(strength).from(alien);
    let total = math::sum!(strengths);
    let count = count!(strengths);
    let name = "Oyelowo";
};

// This is equivalent to the above. Note: This is not to be confused with actual Rust's native `let` keyword.

let code_block = block! {
    LET strengths = select_value(strength).from(alien);
    LET total = math::sum!(strengths);
    LET count = count!(strengths);
    LET name = "Oyelowo";
};
```

In the code snippet above, the `let` (or `LET`) statements bind the variables `strengths`, `total`, `count`, and `name` within the code block. These variables are automatically handled by the ORM, simplifying the query construction process.

The generated SQL query for this code block would look like:

```sql
LET $strengths = (SELECT VALUE strength FROM alien);

LET $total = math::sum($strengths);

LET $count = count($strengths);

LET $name = 'Oyelowo';
```

The recommended approach using the `let` statement (or `LET` statement) within the `block!` macro is preferred because it provides a clean and concise syntax, handles variable bindings and parameter referencing automatically, and promotes code readability.

## Less Recommended Approach

The less recommended approach involves using the `let_!` macro to bind variables manually within a code block. Although it provides flexibility, it requires more manual handling of parameters and can be error-prone.

<a name="using-let-macro"></a>

### Using `let_!` Macro

Here's an example of using the `let_!` macro to define variables within a code block:

```rust
let_!(strengths = select_value(strength).from(alien));
let_!(total = math::sum!(strengths));
let_!(count = count!(strengths));
let_!(name = "Oyelowo");
chain(strengths).chain(total).chain(count).chain(name)
```

In the code snippet above, the `let_!`macro is used to bind variables`strengths`, `total`, `count`, and `name`within the code block. The variables are manually defined and then chained together using the`chain` function.

The generated SQL query for this code block would look like:

```sql
LET $strengths = (SELECT VALUE strength FROM alien);

LET $total = math::sum($strengths);

LET $count = count($strengths);

LET $name = 'Oyelowo';
```

The less recommended approach using the `let_!` macro requires explicit definition and chaining of variables, making the code more complex and error-prone compared to the recommended approach.

## Least Recommended Approach

The least recommended approach involves using the `let` statements with the `let_` function to bind variables manually within a code block. This approach requires even more manual handling of parameters and is prone to errors.

<a name="using-let-statements-with-let-function"></a>

### Using `let` Statements with `let_` Function

Here's another example of using the `let` statements with the `let_` function to bind variables within a code block:

```rust
let strengths = let_("strengths").equal_to(select_value(strength).from(alien));
let total = let_("total").equal_to(math::sum!(strengths));
let count = let_("count").equal_to(count!(strengths));
let name = let_("name").equal_to("Oyelowo");
chain(strengths).chain(total).chain(count).chain(name);
```

In this example, the `let_` function is used to define variables `strengths`, `total`, `count`, and `name` within the code block. The variables are manually defined and then chained together using the `chain` function.

The generated SQL query for this code block would look like:

```sql
LET $strengths = (SELECT VALUE strength FROM alien);

LET $total = math::sum($strengths);

LET $count = count($strengths);

LET $name = 'Oyelowo';
```

Similar to the previous approach, the use of `let` statements with the `let_` function in the least recommended approach requires explicit variable definition and chaining, making the code more complex and error-prone.

It is generally recommended to use the recommended approach with the `let` statement (or `LET` statement) within the `block!` macro for better readability, automation of variable bindings, and parameter handling.

That concludes the documentation for the `let` statement in SurrealDB ORM. Use the recommended approach to simplify complex queries and handle variable bindings effortlessly.
