use surrealdb_models::{organization_schema, Organization};
use surrealdb_orm::{index, Buildable, SetterAssignable, SurrealdbNode, E};

#[test]
fn test_setter() {
    fn erer() {
        let organization_schema::Organization { name, time, .. } = Organization::schema();
        // let xx = time.equal(Time::default());
        let org = Organization::schema();

        org.age.equal(34);
        let org = org.time(index(0)).connected.equal(chrono::Utc::now());
        assert_eq!(org.build(), "time[0].connected = '2021-08-22T15:00:00Z'");
        // time.connected = "255"
        // let xx = org.time(E).connected.equal(chrono::Utc::now());
        // let xx = org.name.equal("".to_string());
        // let xx = org.age.equal(3999943);
        // let xx = org.age.equal(3999943.34);
    }
}
