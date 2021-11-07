use common::{self, packages};

fn main() {
    packages::local_function();
    common::get_shared_function();
    println!("Hello, world!");
}
