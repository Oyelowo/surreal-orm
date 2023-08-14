use rand::{rngs::StdRng, Rng, SeedableRng};
use std::cell::RefCell;

thread_local!(
    static THREAD_RNG: RefCell<StdRng> = RefCell::new(StdRng::seed_from_u64(123456789));
);

fn generate_param_name(prefix: &str, value: impl Into<String>) -> String {
    let _value = value.into();

    let sanitized_uuid = THREAD_RNG.with(|rng| {
        let mut rng = rng.borrow_mut();
        uuid::Uuid::from_u128(rng.gen()).simple().to_string()
    });

    let param = format!("_{}_{}", prefix, sanitized_uuid);
    param
}

fn main() {
    let param1 = generate_param_name("prefix1", "value1");
    assert_eq!(
        param1,
        "_prefix1_58f8532561f1f85bfb55b38845aaeaf1".to_string()
    );
    let param1 = generate_param_name("prefix1", "value1");
    assert_eq!(
        param1,
        "_prefix1_736a550a8b155d3b3d13e8c6cb4d4795".to_string()
    );
    println!("Param 1: {}", param1);

    let param2 = generate_param_name("prefix2", "value2");
    assert_eq!(
        param2,
        "_prefix2_7513ba823c5821eb2f0662031f944c83".to_string()
    );
    println!("Param 2: {}", param2);
}
