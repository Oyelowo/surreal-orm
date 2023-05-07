use surrealdb_models::Alien;
use surrealdb_orm::{Buildable, Last, SurrealdbNode};

#[test]
fn test_simple_array_element_access() {
    let last_tag = Alien::schema().tags(Last);
    assert_eq!(last_tag.fine_tune_params(), "tags[-1]");
}
