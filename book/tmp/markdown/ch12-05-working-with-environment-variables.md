## Working with Environment Variables

We’ll improve `minigrep` by adding an extra feature: an option for
case-insensitive searching that the user can turn on via an environment
variable. We could make this feature a command line option and require that
users enter it each time they want it to apply, but by instead making it an
environment variable, we allow our users to set the environment variable once
and have all their searches be case insensitive in that terminal session.

### Writing a Failing Test for the Case-Insensitive `search` Function

We first add a new `search_case_insensitive` function that will be called when
the environment variable has a value. We’ll continue to follow the TDD process,
so the first step is again to write a failing test. We’ll add a new test for
the new `search_case_insensitive` function and rename our old test from
`one_result` to `case_sensitive` to clarify the differences between the two
tests, as shown in Listing 12-20.

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore,does_not_compile
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
#     for line in search(&config.query, &contents) {
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
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
```

<span class="caption">Listing 12-20: Adding a new failing test for the
case-insensitive function we’re about to add</span>

Note that we’ve edited the old test’s `contents` too. We’ve added a new line
with the text `"Duct tape."` using a capital D that shouldn’t match the query
`"duct"` when we’re searching in a case-sensitive manner. Changing the old test
in this way helps ensure that we don’t accidentally break the case-sensitive
search functionality that we’ve already implemented. This test should pass now
and should continue to pass as we work on the case-insensitive search.

The new test for the case-*insensitive* search uses `"rUsT"` as its query. In
the `search_case_insensitive` function we’re about to add, the query `"rUsT"`
should match the line containing `"Rust:"` with a capital R and match the line
`"Trust me."` even though both have different casing from the query. This is
our failing test, and it will fail to compile because we haven’t yet defined
the `search_case_insensitive` function. Feel free to add a skeleton
implementation that always returns an empty vector, similar to the way we did
for the `search` function in Listing 12-16 to see the test compile and fail.

### Implementing the `search_case_insensitive` Function

The `search_case_insensitive` function, shown in Listing 12-21, will be almost
the same as the `search` function. The only difference is that we’ll lowercase
the `query` and each `line` so whatever the case of the input arguments,
they’ll be the same case when we check whether the line contains the query.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
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
#     for line in search(&config.query, &contents) {
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
pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
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

<span class="caption">Listing 12-21: Defining the `search_case_insensitive`
function to lowercase the query and the line before comparing them</span>

First, we lowercase the `query` string and store it in a shadowed variable with
the same name. Calling `to_lowercase` on the query is necessary so no
matter whether the user’s query is `"rust"`, `"RUST"`, `"Rust"`, or `"rUsT"`,
we’ll treat the query as if it were `"rust"` and be insensitive to the case.
While `to_lowercase` will handle basic Unicode, it won’t be 100% accurate. If
we were writing a real application, we’d want to do a bit more work here, but
this section is about environment variables, not Unicode, so we’ll leave it at
that here.

Note that `query` is now a `String` rather than a string slice, because calling
`to_lowercase` creates new data rather than referencing existing data. Say the
query is `"rUsT"`, as an example: that string slice doesn’t contain a lowercase
`u` or `t` for us to use, so we have to allocate a new `String` containing
`"rust"`. When we pass `query` as an argument to the `contains` method now, we
need to add an ampersand because the signature of `contains` is defined to take
a string slice.

Next, we add a call to `to_lowercase` on each `line` to lowercase all
characters. Now that we’ve converted `line` and `query` to lowercase, we’ll
find matches no matter what the case of the query is.

Let’s see if this implementation passes the tests:

```console
$ cargo test
warning: unused manifest key: workspace.workspace
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished test [unoptimized + debuginfo] target(s) in 1.33s
     Running unittests src/lib.rs (target/aarch64-apple-darwin/debug/deps/minigrep-34585fafb4f80268)

running 2 tests
test tests::case_insensitive ... ok
test tests::case_sensitive ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/aarch64-apple-darwin/debug/deps/minigrep-d930d9b5385055b2)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests minigrep

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

Great! They passed. Now, let’s call the new `search_case_insensitive` function
from the `run` function. First, we’ll add a configuration option to the
`Config` struct to switch between case-sensitive and case-insensitive search.
Adding this field will cause compiler errors because we aren’t initializing
this field anywhere yet:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore,does_not_compile
# use std::error::Error;
# use std::fs;
# 
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}
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

We added the `ignore_case` field that holds a Boolean. Next, we need the `run`
function to check the `ignore_case` field’s value and use that to decide
whether to call the `search` function or the `search_case_insensitive`
function, as shown in Listing 12-22. This still won’t compile yet.

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore,does_not_compile
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
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}
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

<span class="caption">Listing 12-22: Calling either `search` or
`search_case_insensitive` based on the value in `config.ignore_case`</span>

Finally, we need to check for the environment variable. The functions for
working with environment variables are in the `env` module in the standard
library, so we bring that module into scope at the top of *src/lib.rs*. Then
we’ll use the `var` function from the `env` module to check to see if any value
has been set for an environment variable named `IGNORE_CASE`, as shown in
Listing 12-23.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
use std::env;
// --snip--

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

<span class="caption">Listing 12-23: Checking for any value in an environment
variable named `IGNORE_CASE`</span>

Here, we create a new variable `ignore_case`. To set its value, we call the
`env::var` function and pass it the name of the `IGNORE_CASE` environment
variable. The `env::var` function returns a `Result` that will be the
successful `Ok` variant that contains the value of the environment variable if
the environment variable is set to any value. It will return the `Err` variant
if the environment variable is not set.

We’re using the `is_ok` method on the `Result` to check whether the environment
variable is set, which means the program should do a case-insensitive search.
If the `IGNORE_CASE` environment variable isn’t set to anything, `is_ok` will
return false and the program will perform a case-sensitive search. We don’t
care about the *value* of the environment variable, just whether it’s set or
unset, so we’re checking `is_ok` rather than using `unwrap`, `expect`, or any
of the other methods we’ve seen on `Result`.

We pass the value in the `ignore_case` variable to the `Config` instance so the
`run` function can read that value and decide whether to call
`search_case_insensitive` or `search`, as we implemented in Listing 12-22.

Let’s give it a try! First, we’ll run our program without the environment
variable set and with the query `to`, which should match any line that contains
the word “to” in all lowercase:

```console
$ cargo run -- to poem.txt
warning: unused manifest key: workspace.workspace
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.0s
     Running `target/aarch64-apple-darwin/debug/minigrep to poem.txt`
Are you nobody, too?
How dreary to be somebody!
```

Looks like that still works! Now, let’s run the program with `IGNORE_CASE`
set to `1` but with the same query `to`.

```console
$ IGNORE_CASE=1 cargo run -- to poem.txt
```

If you’re using PowerShell, you will need to set the environment variable and
run the program as separate commands:

```console
PS> $Env:IGNORE_CASE=1; cargo run -- to poem.txt
```

This will make `IGNORE_CASE` persist for the remainder of your shell
session. It can be unset with the `Remove-Item` cmdlet:

```console
PS> Remove-Item Env:IGNORE_CASE
```

We should get lines that contain “to” that might have uppercase letters:

<!-- manual-regeneration
cd listings/ch12-an-io-project/listing-12-23
IGNORE_CASE=1 cargo run -- to poem.txt
can't extract because of the environment variable
-->

```console
Are you nobody, too?
How dreary to be somebody!
To tell your name the livelong day
To an admiring bog!
```

Excellent, we also got lines containing “To”! Our `minigrep` program can now do
case-insensitive searching controlled by an environment variable. Now you know
how to manage options set using either command line arguments or environment
variables.

Some programs allow arguments *and* environment variables for the same
configuration. In those cases, the programs decide that one or the other takes
precedence. For another exercise on your own, try controlling case sensitivity
through either a command line argument or an environment variable. Decide
whether the command line argument or the environment variable should take
precedence if the program is run with one set to case sensitive and one set to
ignore case.

The `std::env` module contains many more useful features for dealing with
environment variables: check out its documentation to see what is available.
