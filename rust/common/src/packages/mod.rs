// Similarly `mod inaccessible` and `mod nested` will locate the `nested.rs`
// and `inaccessible.rs` files and insert them here under their respective
// modules
mod inaccessible;
pub mod nested;

pub fn local_function() {
    println!("called `my::function()`");
}

fn local_private_function() {
    println!("called `my::private_function()`");
}

pub fn local_indirect_access() {

    print!("called `my::indirect_access()`, that\n> ");

    local_private_function();
}