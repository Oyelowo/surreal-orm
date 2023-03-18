use rand::rngs::{StdRng, ThreadRng};
use rand::{Rng, SeedableRng};
use std::cell::RefCell;

thread_local!(
    static THREAD_RNG: RefCell<StdRng> = RefCell::new(StdRng::seed_from_u64(123456789));
);

fn generate_param_name(prefix: &str, value: impl Into<String>) -> String {
    let value = value.into();

    let sanitized_uuid = THREAD_RNG.with(|rng| {
        let mut rng = rng.borrow_mut();
        uuid::Uuid::new_v4().simple().to_string()
    });

    let param = format!("_{}_{}", prefix, sanitized_uuid);
    param
}

#[test]
fn main() {
    let param1 = generate_param_name("prefix1", "value1");
    assert_eq!(param1, "rer".to_string());
    insta::assert_debug_snapshot!(param1, 3);
    // println!("Param 1: {}", param1);

    let param2 = generate_param_name("prefix2", "value2");
    println!("Param 2: {}", param2);
}
