mod export_data;
mod greet;
pub mod maths;

pub use export_data::*;
pub use greet::*;

pub fn local_function() {
    greet::good_morning();
    local_indirect_access();
    println!("called `my::function()`");
}

fn local_private_function() {
    println!("called `my::private_function()`");
}

pub fn local_indirect_access() {
    print!("called `my::indirect_access()`, that\n> ");
    local_private_function();
}
