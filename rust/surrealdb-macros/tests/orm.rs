#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
use serde::Deserialize;
use serde::Serialize;

use surreal_simple_querybuilder::prelude::*;
use surrealdb_derive::SurrealdbModel;

#[derive(SurrealdbModel, Debug, Serialize, Deserialize, Default)]
struct Account {
    #[surrealdb(skip_serializing)]
    id: Option<String>,

    handle: String,
    password: String,
    email: String,

    #[surrealdb(reference_one = "Account", skip_serializing)]
    // friend: Foreign<std::sync::Arc<Account>>,
    friend: String,

    #[surrealdb(skip_serializing)]
    projects: ForeignVec<Project>,

    #[surrealdb(relate = "->manage->Project", skip_serializing)]
    managed_projects: ForeignVec<Project>,
}

#[derive(SurrealdbModel, Debug, Serialize, Deserialize, Default)]
struct Project {
    #[surrealdb(skip_serializing)]
    id: Option<String>,
    name: String,

    #[surrealdb(relate = "->has->Release")]
    releases: ForeignVec<Release>,
    #[surrealdb(relate = "<-manage<-Account")]
    authors: ForeignVec<Account>,
}

#[derive(SurrealdbModel, Debug, Serialize, Deserialize, Default)]
struct Release {
    #[surrealdb(skip_serializing)]
    id: Option<String>,
    name: String,
}

use account::schema::model as account;
use project::schema::model as project;

#[derive(Debug, Serialize, Deserialize)]
struct File {
    name: String,
    author: Foreign<Account>,
}

#[test]
fn test_create_account_query() {
    let query = QueryBuilder::new()
        .create(
            Account::schema
                .handle
                .as_named_label(&Account::schema.to_string()),
        )
        .set_model(&Account::schema)
        .unwrap()
        .build();

    assert_eq!(
        query,
        "CREATE Account:handle SET handle = $handle , password = $password , email = $email"
    );
}

#[test]
fn test_account_find_query() {
    let query = QueryBuilder::new()
        .select("*")
        .from(account)
        .filter(Account::schema.email.equals_parameterized())
        .build();

    assert_eq!(query, "SELECT * FROM Account WHERE email = $email");
}

#[test]
pub fn test_nodebuilder_relation() {
    let s = "Account".with("IS_FRIEND").with("Account:Mark").to_owned();

    assert_eq!("Account->IS_FRIEND->Account:Mark", s);
}

#[test]
pub fn test_nodebuilder_condition() {
    let should_be_friend_with_mark = true;
    let should_be_friend_with_john = false;

    let s = String::new()
        .with("IS_FRIEND")
        .if_then(should_be_friend_with_mark, |s| s.with("Account:Mark"))
        .if_then(should_be_friend_with_john, |s| s.with("Account:John"))
        .to_owned();

    assert_eq!("->IS_FRIEND->Account:Mark", s);
}

#[test]
pub fn test_as_named_label() {
    let user_handle = "John";
    let label = user_handle.as_named_label("Account");

    assert_eq!(label, "Account:John");
}

#[test]
pub fn test_foreign_serialize() {
    let f: Foreign<Account> = Foreign::new_key("Account:John".to_owned());

    // Confirm a foreign key is serialized into a simple string
    assert_eq!(
        serde_json::Value::String("Account:John".to_owned()),
        serde_json::to_value(f).unwrap()
    );

    let f: Foreign<Account> = Foreign::new_value(Account {
        id: Some("Account:John".to_owned()),
        ..Default::default()
    });

    // Confirm a loaded value uses the IntoKey trait during serialization
    assert_eq!(
        serde_json::Value::String("Account:John".to_owned()),
        serde_json::to_value(f).unwrap()
    );
}

#[test]
pub fn test_foreign_serialize_allowed() {
    let f: Foreign<Account> = Foreign::new_value(Account {
        id: Some("Account:John".to_owned()),
        ..Default::default()
    });

    // once called, the Foreign should deserialize even the Values without calling
    // IntoKeys
    f.allow_value_serialize();

    assert_eq!(
        serde_json::to_string(&Account {
            id: Some("Account:John".to_owned()),
            ..Default::default()
        })
        .unwrap(),
        serde_json::to_string(&f).unwrap()
    );
}

#[test]
pub fn test_foreign_serialize_allowed_vec() {
    let v = vec![
        Foreign::new_value(Account {
            id: Some("Account:John".to_owned()),
            ..Default::default()
        }),
        Foreign::new_key("Account:John".to_owned()),
    ];

    // once called, the Foreign should deserialize even the Values without calling
    // IntoKeys
    v.allow_value_serialize();

    assert_eq!(
        serde_json::to_value(&vec![
            serde_json::to_value(Account {
                id: Some("Account:John".to_owned()),
                ..Default::default()
            })
            .unwrap(),
            serde_json::to_value("Account:John").unwrap(),
        ])
        .unwrap(),
        serde_json::to_value(&v).unwrap()
    );

    let v = ForeignVec::<Account>::new_value(vec![
        Account {
            id: Some("Account:John".to_owned()),
            ..Default::default()
        },
        Account {
            id: Some("Account:Mark".to_owned()),
            ..Default::default()
        },
    ]);

    v.allow_value_serialize();

    assert_eq!(
        serde_json::to_value(vec![
            Account {
                id: Some("Account:John".to_owned()),
                ..Default::default()
            },
            Account {
                id: Some("Account:Mark".to_owned()),
                ..Default::default()
            },
        ])
        .unwrap(),
        serde_json::to_value(&v).unwrap()
    );
}

#[test]
fn test_foreign_deserialize() {
    let created_account = Account {
        id: Some("Account:John".to_owned()),
        handle: "JohnTheUser".to_owned(),
        password: "abc".to_owned(),
        email: "abc".to_owned(),
        ..Default::default()
    };

    // build a json string where the author field contains a fully built Account
    // object.
    let loaded_author_json = format!(
        "{{ \"name\": \"filename\", \"author\": {} }}",
        serde_json::to_string(&created_account).unwrap()
    );

    let file: File = serde_json::from_str(&loaded_author_json).unwrap();

    // confirm the `Foreign<Author>` contains a value
    assert!(match &file.author.value() {
        Some(acc) => acc.id == Some("Account:John".to_owned()),
        _ => false,
    });

    // build a json string where the author field is an ID string.
    let key_author_json = "{ \"name\": \"filename\", \"author\": \"Account:John\" }";
    let file: File = serde_json::from_str(&key_author_json).unwrap();

    // confirm the author field of the file is a Key with the account's ID
    assert!(match file.author.key().as_deref() {
        Some(key) => key == &"Account:John".to_owned(),
        _ => false,
    });

    // build a json string where the author field is set to null.
    let unloaded_author_json = "{ \"name\": \"filename\", \"author\": null }";
    let file: File = serde_json::from_str(&unloaded_author_json).unwrap();

    // confirm the author field of the file is Unloaded
    assert!(file.author.is_unloaded());
}

/// Test that a model can have fields that reference the `Self` type.
#[test]
fn test_model_self_reference() {
    assert_eq!("friend", Account::schema.friend.to_string());
    assert_eq!("Account", Account::schema.friend().to_string());
    assert_eq!("friend.handle", Account::schema.friend().handle.to_string());
}

#[test]
fn test_model_serializing_relations() {
    assert_eq!(
        "->manage->Project AS account_projects",
        Account::schema
            .managed_projects
            .as_alias("account_projects")
    );
    assert_eq!("Project", Account::schema.managed_projects().to_string());
    assert_eq!(
        "->manage->Project.name AS project_names",
        Account::schema
            .managed_projects()
            .name
            .as_alias("project_names")
    );

    assert_eq!(
        "->manage->Project->has->Release AS account_projects_releases",
        Account::schema
            .managed_projects()
            .releases
            .as_alias("account_projects_releases")
    );

    assert_eq!(
        "->manage->Project->has->Release.name AS account_projects_release_names",
        Account::schema
            .managed_projects()
            .releases()
            .name
            .as_alias("account_projects_release_names")
    );

    assert_eq!(
        "<-manage<-Account AS authors",
        project.authors.as_alias("authors")
    );
}

#[test]
fn test_with_id_edge() {
    let query_one = "an_id"
        .as_named_label(&Account::schema.to_string())
        .with(&Account::schema.managed_projects.with_id("other_id"));

    let query_two = account
        .with_id("an_id")
        .with(&Account::schema.managed_projects.with_id("other_id"));

    assert_eq!("Account:an_id->manage->Project:other_id", query_two);
    assert_eq!(query_one, query_two);
}
