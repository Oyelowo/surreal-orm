use common::{greet, maths, add};

fn main() {
    let added = maths::add_one(3);
    greet::good_morning();
    println!("Hello, world!, {}", added);

    // let p = add!(4, 5);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_adder() {
        assert_eq!(super::maths::add_one(13), 14);
        assert_eq!(add!(5,5,5), 15);
    }
}
