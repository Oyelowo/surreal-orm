use surrealdb_models::{organization_schema, Organization};
use surrealdb_orm::{
    index, Buildable, Field, SchemaGetter, SetterAssignable, SurrealdbNode, ToRaw, E,
};

#[test]
fn test_setter() {
    let organization_schema::Organization { name, time, .. } = Organization::schema();
    let org = Organization::schema();

    org.age.equal_to(34);
    org.age.equal_to(Field::new("age"));
    let org = org.time().connected.equal_to(chrono::Utc::now());

    assert_eq!(org.fine_tune_params(), "time.connected = $_param_00000001");
    assert!(org.to_raw().build().len() > 40,);
}
