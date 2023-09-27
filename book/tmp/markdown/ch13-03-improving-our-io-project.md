## Improving Our I/O Project

With this new knowledge about iterators, we can improve the I/O project in
Chapter 12 by using iterators to make places in the code clearer and more
concise. Let’s look at how iterators can improve our implementation of the
`Config::build` function and the `search` function.

### Removing a `clone` Using an Iterator

In Listing 12-6, we added code that took a slice of `String` values and created
an instance of the `Config` struct by indexing into the slice and cloning the
values, allowing the `Config` struct to own those values. In Listing 13-17,
we’ve reproduced the implementation of the `Config::build` function as it was
in Listing 12-23:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
# use std::env;
# use std::error::Error;
# use std::fs;
# 
# pub struct Config {
#     pub query: String,
#     pub file_path: String,
#     pub ignore_case: bool,
# }
# 
impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}
# 
# pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
#     let contents = fs::read_to_string(config.file_path)?;
# 
#     let results = if config.ignore_case {
#         search_case_insensitive(&config.query, &contents)
#     } else {
#         search(&config.query, &contents)
#     };
# 
#     for line in results {
#         println!("{line}");
#     }
# 
#     Ok(())
# }
# 
# pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
#     let mut results = Vec::new();
# 
#     for line in contents.lines() {
#         if line.contains(query) {
#             results.push(line);
#         }
#     }
# 
#     results
# }
# 
# pub fn search_case_insensitive<'a>(
#     query: &str,
#     contents: &'a str,
# ) -> Vec<&'a str> {
#     let query = query.to_lowercase();
#     let mut results = Vec::new();
# 
#     for line in contents.lines() {
#         if line.to_lowercase().contains(&query) {
#             results.push(line);
#         }
#     }
# 
#     results
# }
# 
# #[cfg(test)]
# mod tests {
#     use super::*;
# 
#     #[test]
#     fn case_sensitive() {
#         let query = "duct";
#         let contents = "\
# Rust:
# safe, fast, productive.
# Pick three.
# Duct tape.";
# 
#         assert_eq!(vec!["safe, fast, productive."], search(query, contents));
#     }
# 
#     #[test]
#     fn case_insensitive() {
#         let query = "rUsT";
#         let contents = "\
# Rust:
# safe, fast, productive.
# Pick three.
# Trust me.";
# 
#         assert_eq!(
#             vec!["Rust:", "Trust me."],
#             search_case_insensitive(query, contents)
#         );
#     }
# }
```

<span class="caption">Listing 13-17: Reproduction of the `Config::build`
function from Listing 12-23</span>

At the time, we said not to worry about the inefficient `clone` calls because
we would remove them in the future. Well, that time is now!

We needed `clone` here because we have a slice with `String` elements in the
parameter `args`, but the `build` function doesn’t own `args`. To return
ownership of a `Config` instance, we had to clone the values from the `query`
and `file_path` fields of `Config` so the `Config` instance can own its values.

With our new knowledge about iterators, we can change the `build` function to
take ownership of an iterator as its argument instead of borrowing a slice.
We’ll use the iterator functionality instead of the code that checks the length
of the slice and indexes into specific locations. This will clarify what the
`Config::build` function is doing because the iterator will access the values.

Once `Config::build` takes ownership of the iterator and stops using indexing
operations that borrow, we can move the `String` values from the iterator into
`Config` rather than calling `clone` and making a new allocation.

#### Using the Returned Iterator Directly

Open your I/O project’s *src/main.rs* file, which should look like this:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
# use std::env;
# use std::process;
# 
# use minigrep::Config;
# 
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // --snip--
# 
#     if let Err(e) = minigrep::run(config) {
#         eprintln!("Application error: {e}");
#         process::exit(1);
#     }
}
```

We’ll first change the start of the `main` function that we had in Listing
12-24 to the code in Listing 13-18, which this time uses an iterator. This
won’t compile until we update `Config::build` as well.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
# use std::env;
# use std::process;
# 
# use minigrep::Config;
# 
fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // --snip--
# 
#     if let Err(e) = minigrep::run(config) {
#         eprintln!("Application error: {e}");
#         process::exit(1);
#     }
}
```

<span class="caption">Listing 13-18: Passing the return value of `env::args` to
`Config::build`</span>

The `env::args` function returns an iterator! Rather than collecting the
iterator values into a vector and then passing a slice to `Config::build`, now
we’re passing ownership of the iterator returned from `env::args` to
`Config::build` directly.

Next, we need to update the definition of `Config::build`. In your I/O
project’s *src/lib.rs* file, let’s change the signature of `Config::build` to
look like Listing 13-19. This still won’t compile because we need to update the
function body.

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore,does_not_compile
# use std::env;
# use std::error::Error;
# use std::fs;
# 
# pub struct Config {
#     pub query: String,
#     pub file_path: String,
#     pub ignore_case: bool,
# }
# 
impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        // --snip--
#         if args.len() < 3 {
#             return Err("not enough arguments");
#         }
# 
#         let query = args[1].clone();
#         let file_path = args[2].clone();
# 
#         let ignore_case = env::var("IGNORE_CASE").is_ok();
# 
#         Ok(Config {
#             query,
#             file_path,
#             ignore_case,
#         })
#     }
# }
# 
# pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
#     let contents = fs::read_to_string(config.file_path)?;
# 
#     let results = if config.ignore_case {
#         search_case_insensitive(&config.query, &contents)
#     } else {
#         search(&config.query, &contents)
#     };
# 
#     for line in results {
#         println!("{line}");
#     }
# 
#     Ok(())
# }
# 
# pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
#     let mut results = Vec::new();
# 
#     for line in contents.lines() {
#         if line.contains(query) {
#             results.push(line);
#         }
#     }
# 
#     results
# }
# 
# pub fn search_case_insensitive<'a>(
#     query: &str,
#     contents: &'a str,
# ) -> Vec<&'a str> {
#     let query = query.to_lowercase();
#     let mut results = Vec::new();
# 
#     for line in contents.lines() {
#         if line.to_lowercase().contains(&query) {
#             results.push(line);
#         }
#     }
# 
#     results
# }
# 
# #[cfg(test)]
# mod tests {
#     use super::*;
# 
#     #[test]
#     fn case_sensitive() {
#         let query = "duct";
#         let contents = "\
# Rust:
# safe, fast, productive.
# Pick three.
# Duct tape.";
# 
#         assert_eq!(vec!["safe, fast, productive."], search(query, contents));
#     }
# 
#     #[test]
#     fn case_insensitive() {
#         let query = "rUsT";
#         let contents = "\
# Rust:
# safe, fast, productive.
# Pick three.
# Trust me.";
# 
#         assert_eq!(
#             vec!["Rust:", "Trust me."],
#             search_case_insensitive(query, contents)
#         );
#     }
# }
```

<span class="caption">Listing 13-19: Updating the signature of `Config::build`
to expect an iterator</span>

The standard library documentation for the `env::args` function shows that the
type of the iterator it returns is `std::env::Args`, and that type implements
the `Iterator` trait and returns `String` values.

We’ve updated the signature of the `Config::build` function so the parameter
`args` has a generic type with the trait bounds `impl Iterator<Item = String>`
instead of `&[String]`. This usage of the `impl Trait` syntax we discussed in
the [“Traits as Parameters”][impl-trait]<!-- ignore --> section of Chapter 10
means that `args` can be any type that implements the `Iterator` type and
returns `String` items.

Because we’re taking ownership of `args` and we’ll be mutating `args` by
iterating over it, we can add the `mut` keyword into the specification of the
`args` parameter to make it mutable.

#### Using `Iterator` Trait Methods Instead of Indexing

Next, we’ll fix the body of `Config::build`. Because `args` implements the
`Iterator` trait, we know we can call the `next` method on it! Listing 13-20
updates the code from Listing 12-23 to use the `next` method:

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
# use std::env;
# use std::error::Error;
# use std::fs;
# 
# pub struct Config {
#     pub query: String,
#     pub file_path: String,
#     pub ignore_case: bool,
# }
# 
impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}
# 
# pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
#     let contents = fs::read_to_string(config.file_path)?;
# 
#     let results = if config.ignore_case {
#         search_case_insensitive(&config.query, &contents)
#     } else {
#         search(&config.query, &contents)
#     };
# 
#     for line in results {
#         println!("{line}");
#     }
# 
#     Ok(())
# }
# 
# pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
#     let mut results = Vec::new();
# 
#     for line in contents.lines() {
#         if line.contains(query) {
#             results.push(line);
#         }
#     }
# 
#     results
# }
# 
# pub fn search_case_insensitive<'a>(
#     query: &str,
#     contents: &'a str,
# ) -> Vec<&'a str> {
#     let query = query.to_lowercase();
#     let mut results = Vec::new();
# 
#     for line in contents.lines() {
#         if line.to_lowercase().contains(&query) {
#             results.push(line);
#         }
#     }
# 
#     results
# }
# 
# #[cfg(test)]
# mod tests {
#     use super::*;
# 
#     #[test]
#     fn case_sensitive() {
#         let query = "duct";
#         let contents = "\
# Rust:
# safe, fast, productive.
# Pick three.
# Duct tape.";
# 
#         assert_eq!(vec!["safe, fast, productive."], search(query, contents));
#     }
# 
#     #[test]
#     fn case_insensitive() {
#         let query = "rUsT";
#         let contents = "\
# Rust:
# safe, fast, productive.
# Pick three.
# Trust me.";
# 
#         assert_eq!(
#             vec!["Rust:", "Trust me."],
#             search_case_insensitive(query, contents)
#         );
#     }
# }
```

<span class="caption">Listing 13-20: Changing the body of `Config::build` to use
iterator methods</span>

Remember that the first value in the return value of `env::args` is the name of
the program. We want to ignore that and get to the next value, so first we call
`next` and do nothing with the return value. Second, we call `next` to get the
value we want to put in the `query` field of `Config`. If `next` returns a
`Some`, we use a `match` to extract the value. If it returns `None`, it means
not enough arguments were given and we return early with an `Err` value. We do
the same thing for the `file_path` value.

### Making Code Clearer with Iterator Adaptors

We can also take advantage of iterators in the `search` function in our I/O
project, which is reproduced here in Listing 13-21 as it was in Listing 12-19:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
# use std::error::Error;
# use std::fs;
# 
# pub struct Config {
#     pub query: String,
#     pub file_path: String,
# }
# 
# impl Config {
#     pub fn build(args: &[String]) -> Result<Config, &'static str> {
#         if args.len() < 3 {
#             return Err("not enough arguments");
#         }
# 
#         let query = args[1].clone();
#         let file_path = args[2].clone();
# 
#         Ok(Config { query, file_path })
#     }
# }
# 
# pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
#     let contents = fs::read_to_string(config.file_path)?;
# 
#     Ok(())
# }
# 
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}
# 
# #[cfg(test)]
# mod tests {
#     use super::*;
# 
#     #[test]
#     fn one_result() {
#         let query = "duct";
#         let contents = "\
# Rust:
# safe, fast, productive.
# Pick three.";
# 
#         assert_eq!(vec!["safe, fast, productive."], search(query, contents));
#     }
# }
```

<span class="caption">Listing 13-21: The implementation of the `search`
function from Listing 12-19</span>

We can write this code in a more concise way using iterator adaptor methods.
Doing so also lets us avoid having a mutable intermediate `results` vector. The
functional programming style prefers to minimize the amount of mutable state to
make code clearer. Removing the mutable state might enable a future enhancement
to make searching happen in parallel, because we wouldn’t have to manage
concurrent access to the `results` vector. Listing 13-22 shows this change:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
# use std::env;
# use std::error::Error;
# use std::fs;
# 
# pub struct Config {
#     pub query: String,
#     pub file_path: String,
#     pub ignore_case: bool,
# }
# 
# impl Config {
#     pub fn build(
#         mut args: impl Iterator<Item = String>,
#     ) -> Result<Config, &'static str> {
#         args.next();
# 
#         let query = match args.next() {
#             Some(arg) => arg,
#             None => return Err("Didn't get a query string"),
#         };
# 
#         let file_path = match args.next() {
#             Some(arg) => arg,
#             None => return Err("Didn't get a file path"),
#         };
# 
#         let ignore_case = env::var("IGNORE_CASE").is_ok();
# 
#         Ok(Config {
#             query,
#             file_path,
#             ignore_case,
#         })
#     }
# }
# 
# pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
#     let contents = fs::read_to_string(config.file_path)?;
# 
#     let results = if config.ignore_case {
#         search_case_insensitive(&config.query, &contents)
#     } else {
#         search(&config.query, &contents)
#     };
# 
#     for line in results {
#         println!("{line}");
#     }
# 
#     Ok(())
# }
# 
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}
# 
# pub fn search_case_insensitive<'a>(
#     query: &str,
#     contents: &'a str,
# ) -> Vec<&'a str> {
#     let query = query.to_lowercase();
#     let mut results = Vec::new();
# 
#     for line in contents.lines() {
#         if line.to_lowercase().contains(&query) {
#             results.push(line);
#         }
#     }
# 
#     results
# }
# 
# #[cfg(test)]
# mod tests {
#     use super::*;
# 
#     #[test]
#     fn case_sensitive() {
#         let query = "duct";
#         let contents = "\
# Rust:
# safe, fast, productive.
# Pick three.
# Duct tape.";
# 
#         assert_eq!(vec!["safe, fast, productive."], search(query, contents));
#     }
# 
#     #[test]
#     fn case_insensitive() {
#         let query = "rUsT";
#         let contents = "\
# Rust:
# safe, fast, productive.
# Pick three.
# Trust me.";
# 
#         assert_eq!(
#             vec!["Rust:", "Trust me."],
#             search_case_insensitive(query, contents)
#         );
#     }
# }
```

<span class="caption">Listing 13-22: Using iterator adaptor methods in the
implementation of the `search` function</span>

Recall that the purpose of the `search` function is to return all lines in
`contents` that contain the `query`. Similar to the `filter` example in Listing
13-16, this code uses the `filter` adaptor to keep only the lines that
`line.contains(query)` returns `true` for. We then collect the matching lines
into another vector with `collect`. Much simpler! Feel free to make the same
change to use iterator methods in the `search_case_insensitive` function as
well.

### Choosing Between Loops or Iterators

The next logical question is which style you should choose in your own code and
why: the original implementation in Listing 13-21 or the version using
iterators in Listing 13-22. Most Rust programmers prefer to use the iterator
style. It’s a bit tougher to get the hang of at first, but once you get a feel
for the various iterator adaptors and what they do, iterators can be easier to
understand. Instead of fiddling with the various bits of looping and building
new vectors, the code focuses on the high-level objective of the loop. This
abstracts away some of the commonplace code so it’s easier to see the concepts
that are unique to this code, such as the filtering condition each element in
the iterator must pass.

But are the two implementations truly equivalent? The intuitive assumption
might be that the more low-level loop will be faster. Let’s talk about
performance.

[impl-trait]: ch10-02-traits.html#traits-as-parameters
