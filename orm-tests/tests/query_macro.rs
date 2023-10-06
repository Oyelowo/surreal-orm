use surreal_orm::query;

#[test]
fn test_query_macro() {
    let query = query!("SELECT name, age, * FROM users");
    assert_eq!(query, "SELECT name, age, * FROM users");
}

#[test]
fn test_query_macro_with_params() {
    let query = query!("SELECT name, age, * FROM users WHERE name = $1 AND name = 'Oyelowo'");
    assert_eq!(
        query,
        "SELECT name, age, * FROM users WHERE name = $1 AND name = 'Oyelowo'"
    );
}

#[test]
fn test_query_macro_with_graph() {
    let query = query!("SELECT ->purchased->product<-purchased<-person->purchased->product FROM person:tobie PARALLEL");
    assert_eq!(
        query,
        "SELECT ->purchased->product<-purchased<-person->purchased->product FROM person:tobie PARALLEL"
    );
}
