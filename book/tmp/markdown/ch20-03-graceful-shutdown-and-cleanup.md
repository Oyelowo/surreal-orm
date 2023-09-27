## Graceful Shutdown and Cleanup

The code in Listing 20-20 is responding to requests asynchronously through the
use of a thread pool, as we intended. We get some warnings about the `workers`,
`id`, and `thread` fields that we’re not using in a direct way that reminds us
we’re not cleaning up anything. When we use the less elegant <span
class="keystroke">ctrl-c</span> method to halt the main thread, all other
threads are stopped immediately as well, even if they’re in the middle of
serving a request.

Next, then, we’ll implement the `Drop` trait to call `join` on each of the
threads in the pool so they can finish the requests they’re working on before
closing. Then we’ll implement a way to tell the threads they should stop
accepting new requests and shut down. To see this code in action, we’ll modify
our server to accept only two requests before gracefully shutting down its
thread pool.

### Implementing the `Drop` Trait on `ThreadPool`

Let’s start with implementing `Drop` on our thread pool. When the pool is
dropped, our threads should all join to make sure they finish their work.
Listing 20-22 shows a first attempt at a `Drop` implementation; this code won’t
quite work yet.

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore,does_not_compile
# use std::{
#     sync::{mpsc, Arc, Mutex},
#     thread,
# };
# 
# pub struct ThreadPool {
#     workers: Vec<Worker>,
#     sender: mpsc::Sender<Job>,
# }
# 
# type Job = Box<dyn FnOnce() + Send + 'static>;
# 
# impl ThreadPool {
#     /// Create a new ThreadPool.
#     ///
#     /// The size is the number of threads in the pool.
#     ///
#     /// # Panics
#     ///
#     /// The `new` function will panic if the size is zero.
#     pub fn new(size: usize) -> ThreadPool {
#         assert!(size > 0);
# 
#         let (sender, receiver) = mpsc::channel();
# 
#         let receiver = Arc::new(Mutex::new(receiver));
# 
#         let mut workers = Vec::with_capacity(size);
# 
#         for id in 0..size {
#             workers.push(Worker::new(id, Arc::clone(&receiver)));
#         }
# 
#         ThreadPool { workers, sender }
#     }
# 
#     pub fn execute<F>(&self, f: F)
#     where
#         F: FnOnce() + Send + 'static,
#     {
#         let job = Box::new(f);
# 
#         self.sender.send(job).unwrap();
#     }
# }
# 
impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            worker.thread.join().unwrap();
        }
    }
}
# 
# struct Worker {
#     id: usize,
#     thread: thread::JoinHandle<()>,
# }
# 
# impl Worker {
#     fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
#         let thread = thread::spawn(move || loop {
#             let job = receiver.lock().unwrap().recv().unwrap();
# 
#             println!("Worker {id} got a job; executing.");
# 
#             job();
#         });
# 
#         Worker { id, thread }
#     }
# }
```

<span class="caption">Listing 20-22: Joining each thread when the thread pool
goes out of scope</span>

First, we loop through each of the thread pool `workers`. We use `&mut` for
this because `self` is a mutable reference, and we also need to be able to
mutate `worker`. For each worker, we print a message saying that this
particular worker is shutting down, and then we call `join` on that worker’s
thread. If the call to `join` fails, we use `unwrap` to make Rust panic and go
into an ungraceful shutdown.

Here is the error we get when we compile this code:

```console
$ cargo check
warning: unused manifest key: workspace.workspace
    Checking hello v0.1.0 (file:///projects/hello)
error[E0507]: cannot move out of `worker.thread` which is behind a mutable reference
  --> src/lib.rs:52:13
   |
52 |             worker.thread.join().unwrap();
   |             ^^^^^^^^^^^^^ ------ `worker.thread` moved due to this method call
   |             |
   |             move occurs because `worker.thread` has type `JoinHandle<()>`, which does not implement the `Copy` trait
   |
note: this function takes ownership of the receiver `self`, which moves `worker.thread`
  --> /rustc/d5a82bbd26e1ad8b7401f6a718a9c57c96905483/library/std/src/thread/mod.rs:1581:17

For more information about this error, try `rustc --explain E0507`.
error: could not compile `hello` due to previous error
```

The error tells us we can’t call `join` because we only have a mutable borrow
of each `worker` and `join` takes ownership of its argument. To solve this
issue, we need to move the thread out of the `Worker` instance that owns
`thread` so `join` can consume the thread. We did this in Listing 17-15: if
`Worker` holds an `Option<thread::JoinHandle<()>>` instead, we can call the
`take` method on the `Option` to move the value out of the `Some` variant and
leave a `None` variant in its place. In other words, a `Worker` that is running
will have a `Some` variant in `thread`, and when we want to clean up a
`Worker`, we’ll replace `Some` with `None` so the `Worker` doesn’t have a
thread to run.

So we know we want to update the definition of `Worker` like this:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore,does_not_compile
# use std::{
#     sync::{mpsc, Arc, Mutex},
#     thread,
# };
# 
# pub struct ThreadPool {
#     workers: Vec<Worker>,
#     sender: mpsc::Sender<Job>,
# }
# 
# type Job = Box<dyn FnOnce() + Send + 'static>;
# 
# impl ThreadPool {
#     /// Create a new ThreadPool.
#     ///
#     /// The size is the number of threads in the pool.
#     ///
#     /// # Panics
#     ///
#     /// The `new` function will panic if the size is zero.
#     pub fn new(size: usize) -> ThreadPool {
#         assert!(size > 0);
# 
#         let (sender, receiver) = mpsc::channel();
# 
#         let receiver = Arc::new(Mutex::new(receiver));
# 
#         let mut workers = Vec::with_capacity(size);
# 
#         for id in 0..size {
#             workers.push(Worker::new(id, Arc::clone(&receiver)));
#         }
# 
#         ThreadPool { workers, sender }
#     }
# 
#     pub fn execute<F>(&self, f: F)
#     where
#         F: FnOnce() + Send + 'static,
#     {
#         let job = Box::new(f);
# 
#         self.sender.send(job).unwrap();
#     }
# }
# 
# impl Drop for ThreadPool {
#     fn drop(&mut self) {
#         for worker in &mut self.workers {
#             println!("Shutting down worker {}", worker.id);
# 
#             worker.thread.join().unwrap();
#         }
#     }
# }
# 
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}
# 
# impl Worker {
#     fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
#         let thread = thread::spawn(move || loop {
#             let job = receiver.lock().unwrap().recv().unwrap();
# 
#             println!("Worker {id} got a job; executing.");
# 
#             job();
#         });
# 
#         Worker { id, thread }
#     }
# }
```

Now let’s lean on the compiler to find the other places that need to change.
Checking this code, we get two errors:

```console
$ cargo check
warning: unused manifest key: workspace.workspace
    Checking hello v0.1.0 (file:///projects/hello)
error[E0599]: no method named `join` found for enum `Option` in the current scope
  --> src/lib.rs:52:27
   |
52 |             worker.thread.join().unwrap();
   |                           ^^^^ method not found in `Option<JoinHandle<()>>`
   |
note: the method `join` exists on the type `JoinHandle<()>`
  --> /rustc/d5a82bbd26e1ad8b7401f6a718a9c57c96905483/library/std/src/thread/mod.rs:1581:5
help: consider using `Option::expect` to unwrap the `JoinHandle<()>` value, panicking if the value is an `Option::None`
   |
52 |             worker.thread.expect("REASON").join().unwrap();
   |                          +++++++++++++++++

error[E0308]: mismatched types
  --> src/lib.rs:72:22
   |
72 |         Worker { id, thread }
   |                      ^^^^^^ expected enum `Option`, found struct `JoinHandle`
   |
   = note: expected enum `Option<JoinHandle<()>>`
            found struct `JoinHandle<_>`
help: try wrapping the expression in `Some`
   |
72 |         Worker { id, thread: Some(thread) }
   |                      +++++++++++++      +

Some errors have detailed explanations: E0308, E0599.
For more information about an error, try `rustc --explain E0308`.
error: could not compile `hello` due to 2 previous errors
```

Let’s address the second error, which points to the code at the end of
`Worker::new`; we need to wrap the `thread` value in `Some` when we create a
new `Worker`. Make the following changes to fix this error:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore,does_not_compile
# use std::{
#     sync::{mpsc, Arc, Mutex},
#     thread,
# };
# 
# pub struct ThreadPool {
#     workers: Vec<Worker>,
#     sender: mpsc::Sender<Job>,
# }
# 
# type Job = Box<dyn FnOnce() + Send + 'static>;
# 
# impl ThreadPool {
#     /// Create a new ThreadPool.
#     ///
#     /// The size is the number of threads in the pool.
#     ///
#     /// # Panics
#     ///
#     /// The `new` function will panic if the size is zero.
#     pub fn new(size: usize) -> ThreadPool {
#         assert!(size > 0);
# 
#         let (sender, receiver) = mpsc::channel();
# 
#         let receiver = Arc::new(Mutex::new(receiver));
# 
#         let mut workers = Vec::with_capacity(size);
# 
#         for id in 0..size {
#             workers.push(Worker::new(id, Arc::clone(&receiver)));
#         }
# 
#         ThreadPool { workers, sender }
#     }
# 
#     pub fn execute<F>(&self, f: F)
#     where
#         F: FnOnce() + Send + 'static,
#     {
#         let job = Box::new(f);
# 
#         self.sender.send(job).unwrap();
#     }
# }
# 
# impl Drop for ThreadPool {
#     fn drop(&mut self) {
#         for worker in &mut self.workers {
#             println!("Shutting down worker {}", worker.id);
# 
#             worker.thread.join().unwrap();
#         }
#     }
# }
# 
# struct Worker {
#     id: usize,
#     thread: Option<thread::JoinHandle<()>>,
# }
# 
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // --snip--

#         let thread = thread::spawn(move || loop {
#             let job = receiver.lock().unwrap().recv().unwrap();
# 
#             println!("Worker {id} got a job; executing.");
# 
#             job();
#         });
# 
        Worker {
            id,
            thread: Some(thread),
        }
    }
}
```

The first error is in our `Drop` implementation. We mentioned earlier that we
intended to call `take` on the `Option` value to move `thread` out of `worker`.
The following changes will do so:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore,not_desired_behavior
# use std::{
#     sync::{mpsc, Arc, Mutex},
#     thread,
# };
# 
# pub struct ThreadPool {
#     workers: Vec<Worker>,
#     sender: mpsc::Sender<Job>,
# }
# 
# type Job = Box<dyn FnOnce() + Send + 'static>;
# 
# impl ThreadPool {
#     /// Create a new ThreadPool.
#     ///
#     /// The size is the number of threads in the pool.
#     ///
#     /// # Panics
#     ///
#     /// The `new` function will panic if the size is zero.
#     pub fn new(size: usize) -> ThreadPool {
#         assert!(size > 0);
# 
#         let (sender, receiver) = mpsc::channel();
# 
#         let receiver = Arc::new(Mutex::new(receiver));
# 
#         let mut workers = Vec::with_capacity(size);
# 
#         for id in 0..size {
#             workers.push(Worker::new(id, Arc::clone(&receiver)));
#         }
# 
#         ThreadPool { workers, sender }
#     }
# 
#     pub fn execute<F>(&self, f: F)
#     where
#         F: FnOnce() + Send + 'static,
#     {
#         let job = Box::new(f);
# 
#         self.sender.send(job).unwrap();
#     }
# }
# 
impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
# 
# struct Worker {
#     id: usize,
#     thread: Option<thread::JoinHandle<()>>,
# }
# 
# impl Worker {
#     fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
#         let thread = thread::spawn(move || loop {
#             let job = receiver.lock().unwrap().recv().unwrap();
# 
#             println!("Worker {id} got a job; executing.");
# 
#             job();
#         });
# 
#         Worker {
#             id,
#             thread: Some(thread),
#         }
#     }
# }
```

As discussed in Chapter 17, the `take` method on `Option` takes the `Some`
variant out and leaves `None` in its place. We’re using `if let` to destructure
the `Some` and get the thread; then we call `join` on the thread. If a worker’s
thread is already `None`, we know that worker has already had its thread
cleaned up, so nothing happens in that case.

### Signaling to the Threads to Stop Listening for Jobs

With all the changes we’ve made, our code compiles without any warnings.
However, the bad news is this code doesn’t function the way we want it to yet.
The key is the logic in the closures run by the threads of the `Worker`
instances: at the moment, we call `join`, but that won’t shut down the threads
because they `loop` forever looking for jobs. If we try to drop our
`ThreadPool` with our current implementation of `drop`, the main thread will
block forever waiting for the first thread to finish.

To fix this problem, we’ll need a change in the `ThreadPool` `drop`
implementation and then a change in the `Worker` loop.

First, we’ll change the `ThreadPool` `drop` implementation to explicitly drop
the `sender` before waiting for the threads to finish. Listing 20-23 shows the
changes to `ThreadPool` to explicitly drop `sender`. We use the same `Option`
and `take` technique as we did with the thread to be able to move `sender` out
of `ThreadPool`:

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground,not_desired_behavior
# use std::{
#     sync::{mpsc, Arc, Mutex},
#     thread,
# };
# 
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}
// --snip--
# 
# type Job = Box<dyn FnOnce() + Send + 'static>;
# 
impl ThreadPool {
#     /// Create a new ThreadPool.
#     ///
#     /// The size is the number of threads in the pool.
#     ///
#     /// # Panics
#     ///
#     /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        // --snip--

#         assert!(size > 0);
# 
#         let (sender, receiver) = mpsc::channel();
# 
#         let receiver = Arc::new(Mutex::new(receiver));
# 
#         let mut workers = Vec::with_capacity(size);
# 
#         for id in 0..size {
#             workers.push(Worker::new(id, Arc::clone(&receiver)));
#         }
# 
        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
# 
# struct Worker {
#     id: usize,
#     thread: Option<thread::JoinHandle<()>>,
# }
# 
# impl Worker {
#     fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
#         let thread = thread::spawn(move || loop {
#             let job = receiver.lock().unwrap().recv().unwrap();
# 
#             println!("Worker {id} got a job; executing.");
# 
#             job();
#         });
# 
#         Worker {
#             id,
#             thread: Some(thread),
#         }
#     }
# }
```

<span class="caption">Listing 20-23: Explicitly drop `sender` before joining
the worker threads</span>

Dropping `sender` closes the channel, which indicates no more messages will be
sent. When that happens, all the calls to `recv` that the workers do in the
infinite loop will return an error. In Listing 20-24, we change the `Worker`
loop to gracefully exit the loop in that case, which means the threads will
finish when the `ThreadPool` `drop` implementation calls `join` on them.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
# use std::{
#     sync::{mpsc, Arc, Mutex},
#     thread,
# };
# 
# pub struct ThreadPool {
#     workers: Vec<Worker>,
#     sender: Option<mpsc::Sender<Job>>,
# }
# 
# type Job = Box<dyn FnOnce() + Send + 'static>;
# 
# impl ThreadPool {
#     /// Create a new ThreadPool.
#     ///
#     /// The size is the number of threads in the pool.
#     ///
#     /// # Panics
#     ///
#     /// The `new` function will panic if the size is zero.
#     pub fn new(size: usize) -> ThreadPool {
#         assert!(size > 0);
# 
#         let (sender, receiver) = mpsc::channel();
# 
#         let receiver = Arc::new(Mutex::new(receiver));
# 
#         let mut workers = Vec::with_capacity(size);
# 
#         for id in 0..size {
#             workers.push(Worker::new(id, Arc::clone(&receiver)));
#         }
# 
#         ThreadPool {
#             workers,
#             sender: Some(sender),
#         }
#     }
# 
#     pub fn execute<F>(&self, f: F)
#     where
#         F: FnOnce() + Send + 'static,
#     {
#         let job = Box::new(f);
# 
#         self.sender.as_ref().unwrap().send(job).unwrap();
#     }
# }
# 
# impl Drop for ThreadPool {
#     fn drop(&mut self) {
#         drop(self.sender.take());
# 
#         for worker in &mut self.workers {
#             println!("Shutting down worker {}", worker.id);
# 
#             if let Some(thread) = worker.thread.take() {
#                 thread.join().unwrap();
#             }
#         }
#     }
# }
# 
# struct Worker {
#     id: usize,
#     thread: Option<thread::JoinHandle<()>>,
# }
# 
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");

                    job();
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
```

<span class="caption">Listing 20-24: Explicitly break out of the loop when
`recv` returns an error</span>

To see this code in action, let’s modify `main` to accept only two requests
before gracefully shutting down the server, as shown in Listing 20-25.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
# use hello::ThreadPool;
# use std::fs;
# use std::io::prelude::*;
# use std::net::TcpListener;
# use std::net::TcpStream;
# use std::thread;
# use std::time::Duration;
# 
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}
# 
# fn handle_connection(mut stream: TcpStream) {
#     let mut buffer = [0; 1024];
#     stream.read(&mut buffer).unwrap();
# 
#     let get = b"GET / HTTP/1.1\r\n";
#     let sleep = b"GET /sleep HTTP/1.1\r\n";
# 
#     let (status_line, filename) = if buffer.starts_with(get) {
#         ("HTTP/1.1 200 OK", "hello.html")
#     } else if buffer.starts_with(sleep) {
#         thread::sleep(Duration::from_secs(5));
#         ("HTTP/1.1 200 OK", "hello.html")
#     } else {
#         ("HTTP/1.1 404 NOT FOUND", "404.html")
#     };
# 
#     let contents = fs::read_to_string(filename).unwrap();
# 
#     let response = format!(
#         "{}\r\nContent-Length: {}\r\n\r\n{}",
#         status_line,
#         contents.len(),
#         contents
#     );
# 
#     stream.write_all(response.as_bytes()).unwrap();
#     stream.flush().unwrap();
# }
```

<span class="caption">Listing 20-25: Shut down the server after serving two
requests by exiting the loop</span>

You wouldn’t want a real-world web server to shut down after serving only two
requests. This code just demonstrates that the graceful shutdown and cleanup is
in working order.

The `take` method is defined in the `Iterator` trait and limits the iteration
to the first two items at most. The `ThreadPool` will go out of scope at the
end of `main`, and the `drop` implementation will run.

Start the server with `cargo run`, and make three requests. The third request
should error, and in your terminal you should see output similar to this:

<!-- manual-regeneration
cd listings/ch20-web-server/listing-20-25
cargo run
curl http://127.0.0.1:7878
curl http://127.0.0.1:7878
curl http://127.0.0.1:7878
third request will error because server will have shut down
copy output below
Can't automate because the output depends on making requests
-->

```console
$ cargo run
   Compiling hello v0.1.0 (file:///projects/hello)
    Finished dev [unoptimized + debuginfo] target(s) in 1.0s
     Running `target/debug/hello`
Worker 0 got a job; executing.
Shutting down.
Shutting down worker 0
Worker 3 got a job; executing.
Worker 1 disconnected; shutting down.
Worker 2 disconnected; shutting down.
Worker 3 disconnected; shutting down.
Worker 0 disconnected; shutting down.
Shutting down worker 1
Shutting down worker 2
Shutting down worker 3
```

You might see a different ordering of workers and messages printed. We can see
how this code works from the messages: workers 0 and 3 got the first two
requests. The server stopped accepting connections after the second connection,
and the `Drop` implementation on `ThreadPool` starts executing before worker 3
even starts its job. Dropping the `sender` disconnects all the workers and
tells them to shut down. The workers each print a message when they disconnect,
and then the thread pool calls `join` to wait for each worker thread to finish.

Notice one interesting aspect of this particular execution: the `ThreadPool`
dropped the `sender`, and before any worker received an error, we tried to join
worker 0. Worker 0 had not yet gotten an error from `recv`, so the main thread
blocked waiting for worker 0 to finish. In the meantime, worker 3 received a
job and then all threads received an error. When worker 0 finished, the main
thread waited for the rest of the workers to finish. At that point, they had
all exited their loops and stopped.

Congrats! We’ve now completed our project; we have a basic web server that uses
a thread pool to respond asynchronously. We’re able to perform a graceful
shutdown of the server, which cleans up all the threads in the pool.

Here’s the full code for reference:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
use hello::ThreadPool;
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
```

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");

                    job();
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
```

We could do more here! If you want to continue enhancing this project, here are
some ideas:

* Add more documentation to `ThreadPool` and its public methods.
* Add tests of the library’s functionality.
* Change calls to `unwrap` to more robust error handling.
* Use `ThreadPool` to perform some task other than serving web requests.
* Find a thread pool crate on [crates.io](https://crates.io/) and implement a
  similar web server using the crate instead. Then compare its API and
  robustness to the thread pool we implemented.

## Summary

Well done! You’ve made it to the end of the book! We want to thank you for
joining us on this tour of Rust. You’re now ready to implement your own Rust
projects and help with other peoples’ projects. Keep in mind that there is a
welcoming community of other Rustaceans who would love to help you with any
challenges you encounter on your Rust journey.
