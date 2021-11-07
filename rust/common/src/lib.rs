#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub mod packages;

pub fn get_shared_function() ->  &'static str {
    packages::nested::function();
    packages::local_indirect_access();
    packages::local_function();
    return "Oyelowo"
}