use surrealdb_models::{organization_schema, Organization};
use surrealdb_orm::{index, Buildable, SetterAssignable, SurrealdbNode, ToRaw, E};

#[test]
fn test_setter() {
    let organization_schema::Organization { name, time, .. } = Organization::schema();
    let org = Organization::schema();

    org.age.equal(34);
    let org = org.time(index(0)).connected.equal(chrono::Utc::now());

    assert_eq!(
        org.fine_tune_params(),
        "time[$_param_00000001].connected = $_param_00000002"
    );
    assert!(org.to_raw().build().len() > 60,);
}
