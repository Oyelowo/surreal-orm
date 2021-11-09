// use crate::utils::maths; //absolute from the root
use super::maths; // relative

pub fn good_morning() {
    private_function();
    println!("called `my::nested::function()`");
}


fn private_function() {
    let sum = maths::add_one(4);
    println!("called `my::nested::private_function(), {}", sum);
}
