use common;

fn main() {
    let added = common::add_one(3);
    println!("Hello, world!, {}", added);
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_adder() {
        // let result = common::maths::add_one(13);
        // let result = common::utils::maths::add_one(43);
        assert_eq!(common::add_one(13), 14);
    }
}

fn pp() {
    
}

// Reexporting
// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }

// pub use crate::front_of_house::hosting;

// pub fn eat_at_restaurant() {
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
// }