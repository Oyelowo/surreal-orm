## Processing a Series of Items with Iterators

The iterator pattern allows you to perform some task on a sequence of items in
turn. An iterator is responsible for the logic of iterating over each item and
determining when the sequence has finished. When you use iterators, you don’t
have to reimplement that logic yourself.

In Rust, iterators are *lazy*, meaning they have no effect until you call
methods that consume the iterator to use it up. For example, the code in
Listing 13-10 creates an iterator over the items in the vector `v1` by calling
the `iter` method defined on `Vec<T>`. This code by itself doesn’t do anything
useful.

```rust
# fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();
# }
```

<span class="caption">Listing 13-10: Creating an iterator</span>

The iterator is stored in the `v1_iter` variable. Once we’ve created an
iterator, we can use it in a variety of ways. In Listing 3-5 in Chapter 3, we
iterated over an array using a `for` loop to execute some code on each of its
items. Under the hood this implicitly created and then consumed an iterator,
but we glossed over how exactly that works until now.

In the example in Listing 13-11, we separate the creation of the iterator from
the use of the iterator in the `for` loop. When the `for` loop is called using
the iterator in `v1_iter`, each element in the iterator is used in one
iteration of the loop, which prints out each value.

```rust
# fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }
# }
```

<span class="caption">Listing 13-11: Using an iterator in a `for` loop</span>

In languages that don’t have iterators provided by their standard libraries,
you would likely write this same functionality by starting a variable at index
0, using that variable to index into the vector to get a value, and
incrementing the variable value in a loop until it reached the total number of
items in the vector.

Iterators handle all that logic for you, cutting down on repetitive code you
could potentially mess up. Iterators give you more flexibility to use the same
logic with many different kinds of sequences, not just data structures you can
index into, like vectors. Let’s examine how iterators do that.

### The `Iterator` Trait and the `next` Method

All iterators implement a trait named `Iterator` that is defined in the
standard library. The definition of the trait looks like this:

```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided
}
```

Notice this definition uses some new syntax: `type Item` and `Self::Item`,
which are defining an *associated type* with this trait. We’ll talk about
associated types in depth in Chapter 19. For now, all you need to know is that
this code says implementing the `Iterator` trait requires that you also define
an `Item` type, and this `Item` type is used in the return type of the `next`
method. In other words, the `Item` type will be the type returned from the
iterator.

The `Iterator` trait only requires implementors to define one method: the
`next` method, which returns one item of the iterator at a time wrapped in
`Some` and, when iteration is over, returns `None`.

We can call the `next` method on iterators directly; Listing 13-12 demonstrates
what values are returned from repeated calls to `next` on the iterator created
from the vector.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
# #[cfg(test)]
# mod tests {
    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];

        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }
# }
```

<span class="caption">Listing 13-12: Calling the `next` method on an
iterator</span>

Note that we needed to make `v1_iter` mutable: calling the `next` method on an
iterator changes internal state that the iterator uses to keep track of where
it is in the sequence. In other words, this code *consumes*, or uses up, the
iterator. Each call to `next` eats up an item from the iterator. We didn’t need
to make `v1_iter` mutable when we used a `for` loop because the loop took
ownership of `v1_iter` and made it mutable behind the scenes.

Also note that the values we get from the calls to `next` are immutable
references to the values in the vector. The `iter` method produces an iterator
over immutable references. If we want to create an iterator that takes
ownership of `v1` and returns owned values, we can call `into_iter` instead of
`iter`. Similarly, if we want to iterate over mutable references, we can call
`iter_mut` instead of `iter`.

### Methods that Consume the Iterator

The `Iterator` trait has a number of different methods with default
implementations provided by the standard library; you can find out about these
methods by looking in the standard library API documentation for the `Iterator`
trait. Some of these methods call the `next` method in their definition, which
is why you’re required to implement the `next` method when implementing the
`Iterator` trait.

Methods that call `next` are called *consuming adaptors*, because calling them
uses up the iterator. One example is the `sum` method, which takes ownership of
the iterator and iterates through the items by repeatedly calling `next`, thus
consuming the iterator. As it iterates through, it adds each item to a running
total and returns the total when iteration is complete. Listing 13-13 has a
test illustrating a use of the `sum` method:

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
# #[cfg(test)]
# mod tests {
    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum();

        assert_eq!(total, 6);
    }
# }
```

<span class="caption">Listing 13-13: Calling the `sum` method to get the total
of all items in the iterator</span>

We aren’t allowed to use `v1_iter` after the call to `sum` because `sum` takes
ownership of the iterator we call it on.

### Methods that Produce Other Iterators

*Iterator adaptors* are methods defined on the `Iterator` trait that don’t
consume the iterator. Instead, they produce different iterators by changing
some aspect of the original iterator.

Listing 13-14 shows an example of calling the iterator adaptor method `map`,
which takes a closure to call on each item as the items are iterated through.
The `map` method returns a new iterator that produces the modified items. The
closure here creates a new iterator in which each item from the vector will be
incremented by 1:

<span class="filename">Filename: src/main.rs</span>

```rust,not_desired_behavior
# fn main() {
    let v1: Vec<i32> = vec![1, 2, 3];

    v1.iter().map(|x| x + 1);
# }
```

<span class="caption">Listing 13-14: Calling the iterator adaptor `map` to
create a new iterator</span>

However, this code produces a warning:

```console
$ cargo run
warning: unused manifest key: workspace.workspace
   Compiling iterators v0.1.0 (file:///projects/iterators)
warning: unused `Map` that must be used
 --> src/main.rs:4:5
  |
4 |     v1.iter().map(|x| x + 1);
  |     ^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: iterators are lazy and do nothing unless consumed
  = note: `#[warn(unused_must_use)]` on by default

warning: `iterators` (bin "iterators") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.47s
     Running `target/aarch64-apple-darwin/debug/iterators`
```

The code in Listing 13-14 doesn’t do anything; the closure we’ve specified
never gets called. The warning reminds us why: iterator adaptors are lazy, and
we need to consume the iterator here.

To fix this warning and consume the iterator, we’ll use the `collect` method,
which we used in Chapter 12 with `env::args` in Listing 12-1. This method
consumes the iterator and collects the resulting values into a collection data
type.

In Listing 13-15, we collect the results of iterating over the iterator that’s
returned from the call to `map` into a vector. This vector will end up
containing each item from the original vector incremented by 1.

<span class="filename">Filename: src/main.rs</span>

```rust
# fn main() {
    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
# }
```

<span class="caption">Listing 13-15: Calling the `map` method to create a new
iterator and then calling the `collect` method to consume the new iterator and
create a vector</span>

Because `map` takes a closure, we can specify any operation we want to perform
on each item. This is a great example of how closures let you customize some
behavior while reusing the iteration behavior that the `Iterator` trait
provides.

You can chain multiple calls to iterator adaptors to perform complex actions in
a readable way. But because all iterators are lazy, you have to call one of the
consuming adaptor methods to get results from calls to iterator adaptors.

### Using Closures that Capture Their Environment

Many iterator adapters take closures as arguments, and commonly the closures
we’ll specify as arguments to iterator adapters will be closures that capture
their environment.

For this example, we’ll use the `filter` method that takes a closure. The
closure gets an item from the iterator and returns a `bool`. If the closure
returns `true`, the value will be included in the iteration produced by
`filter`. If the closure returns `false`, the value won’t be included.

In Listing 13-16, we use `filter` with a closure that captures the `shoe_size`
variable from its environment to iterate over a collection of `Shoe` struct
instances. It will return only shoes that are the specified size.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}
```

<span class="caption">Listing 13-16: Using the `filter` method with a closure
that captures `shoe_size`</span>

The `shoes_in_size` function takes ownership of a vector of shoes and a shoe
size as parameters. It returns a vector containing only shoes of the specified
size.

In the body of `shoes_in_size`, we call `into_iter` to create an iterator
that takes ownership of the vector. Then we call `filter` to adapt that
iterator into a new iterator that only contains elements for which the closure
returns `true`.

The closure captures the `shoe_size` parameter from the environment and
compares the value with each shoe’s size, keeping only shoes of the size
specified. Finally, calling `collect` gathers the values returned by the
adapted iterator into a vector that’s returned by the function.

The test shows that when we call `shoes_in_size`, we get back only shoes
that have the same size as the value we specified.
