# Future

In `surrealdb`, futures provide a powerful mechanism to compute dynamic values
when data is selected and returned to the client. Essentially, a future is a
type of cast function that enables values to be dynamically evaluated upon
retrieval.

## Table of Contents

1. [Introduction to Futures](#introduction-to-futures)
2. [Simple Futures](#simple-futures)
3. [Futures Depending on Other Fields](#futures-depending-on-other-fields)
4. [Advanced Usage of Futures](#advanced-usage-of-futures)

---

## Introduction to Futures

Futures are a unique feature of SurrealDB that allows for dynamic computation of
values. Instead of storing a static value within a record, futures compute the
value dynamically whenever the record is accessed. This ensures that you always
get the most recent and relevant data.

## Simple Futures

Any value or expression can be wrapped inside a future, ensuring it's evaluated
upon every access.

** Example **

```rust
let result = create().set(object!(Person {
    accessed_date: future(time::now!())
});
assert_eq!(result.build(), "CREATE person SET accessed_date = <future> { time::now() }");
```

## Futures Depending on Other Fields

Futures can also compute values based on other fields in the record. This allows
for dynamic calculations that reflect the latest state of the record.

** Example **

```rust
let birthday = Person::schema().birthday;
let eighteen_years = Duration::from_secs(60 * 60 * 24 * 7 * 365 * 18);
let date_of_birth = chrono::Date::MIN_UTC;

let can_drive = future("time::now() > birthday + 18y");
let result = create().set(object!(Person {
    birthday: date_of_birth,
    can_drive: future(time::now!().gt(birthday).plus(eighteen_years))
}));
assert_eq!(result.build(), "CREATE person SET birthday = 2007-06-22, can_drive = <future> { time::now() > birthday + 18y }");
```

## Advanced Usage of Futures

Futures offer much more than just simple dynamic calculations. They can
dynamically access remote records, execute subqueries, and even traverse graphs.

** Example **

```rust
let friends = Person::schema().friends;
let id1 = Person::create_id("dayo");
let id2 = Person::create_id("yelow");
let friends = Person::schema().friends;

let result = create().set(object!(Person {
    name: String::from("Oyelowo"),
    friends: vec![id1, id2],
    adult_friends: future(friends(cond(age.gt(18))).name),
}));
assert_eq!(result.build(), "CREATE person SET name = 'Oyelowo', friends = [person:dayo, person:yelow], adult_friends = <future> { friends[WHERE age > 18].name }");
```

---

Utilizing futures in `surreal_orm` provides a dynamic layer to your data,
ensuring that you always receive the most up-to-date calculations and
evaluations when querying your records. Whether you're calculating age, fetching
related records, or even performing complex graph operations, futures have got
you covered.
