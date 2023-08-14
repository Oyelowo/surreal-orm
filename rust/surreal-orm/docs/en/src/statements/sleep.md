# Sleep Statement

The `SLEEP` statement in SurrealDB ORM is used to introduce a delay or pause in the execution
of a program or query. It allows you to control the timing of your operations by specifying a
duration to wait before proceeding further. This documentation covers the usage and examples of the `SLEEP` statement.

## Table of Contents

- [Sleep Statement Usage](#sleep-statement-usage)

## Sleep Statement Usage

The `SLEEP` statement is used to introduce a pause in the program or query execution. It takes a duration parameter to specify the length of the pause. Here's an example:

```rust
use std::time::Duration;

let statement = sleep(Duration::from_secs(43));
```

In the code snippet above, we create a `Duration` object with a duration of 43 seconds and pass it to the `sleep` function to create the `SLEEP` statement.

You can use the `SLEEP` statement to introduce delays or pauses in your program or query execution to control the timing of your operations effectively.

That concludes the documentation for the `SLEEP` statement in SurrealDB ORM.
Use the provided examples and explanations to introduce pauses in your program or query execution as needed.
