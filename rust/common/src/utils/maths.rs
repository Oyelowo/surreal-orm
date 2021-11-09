pub fn add_one(x: i32) -> i32 {
    x + 1
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
        assert_eq!(super::add_one(3), 4);
    }
}
