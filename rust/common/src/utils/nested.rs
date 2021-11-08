// use crate::maths;
use crate::utils::maths;
// use super::utils::maths;

pub fn get_test_function() {
    println!("called `my::nested::function()`");
}

#[allow(dead_code)]
fn private_function() {
    let kk = maths::add_one(4);
    println!("called `my::nested::private_function()`");
}
