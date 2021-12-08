use common::{self, alt_good_morning, good_morning, maths, sum};


fn main() {
    example_shared_libaray();
}

fn example_shared_libaray() {
    let sum = sum!(3, 3, 5, 6);
    print!("Sum of some: {:?}", sum);

    let sum2 = common::sum!(4);
    print!("Sum of some: {:?}", sum2);

    good_morning();
    alt_good_morning();

    let added = maths::add_one(3);
    println!("Hello, world!, {}", added);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adder() {
        assert_eq!(super::maths::add_one(13), 14);
        assert_eq!(sum!(5, 5, 5), 15);
    }
}