#![allow(incomplete_features)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![feature(generic_const_exprs)]

use async_graphql::ComplexObject;
use async_graphql::SimpleObject;
use serde::Deserialize;
use serde::Serialize;
use surrealdb_macros::links::LinkOne;
use surrealdb_macros::query_builder::query;
use surrealdb_macros::{
    links::{LinkMany, LinkSelf, Relate},
    Edge, SurrealdbModel,
};

#[derive(SurrealdbModel, SimpleObject, Debug, Serialize, Deserialize, Clone)]
pub struct Account {
    #[surrealdb(skip_serializing)]
    id: Option<String>,

    handle: String,
    password: String,
    email: String,

    #[surrealdb(link_self = "Account", skip_serializing)]
    #[graphql(skip)]
    friend: LinkSelf<Account>,

    #[graphql(skip)]
    #[surrealdb(link_self = "Account", skip_serializing)]
    teacher: LinkSelf<Account>,

    #[graphql(skip)]
    #[surrealdb(link_many = "Project", skip_serializing)]
    favourite_projects: LinkMany<Project>,

    #[surrealdb(
        relate(edge = "Account_Manage_Project", link = "->manage->Project"),
        skip_serializing
    )]
    #[graphql(skip)]
    managed_projects: Relate<Project>,
}

#[ComplexObject]
impl Account {
    async fn friend(&self) -> Option<&Account> {
        self.friend.value_ref()
    }

    async fn projects(&self) -> Vec<Option<&Project>> {
        self.favourite_projects
            .iter()
            .map(|x| x.value_ref())
            .collect()
    }

    async fn projects_ids(&self) -> Vec<Option<&String>> {
        self.favourite_projects.iter().map(|x| x.id()).collect()
    }
    async fn teacher(&self) -> Option<&Account> {
        self.teacher.value_ref()
    }
}

#[derive(SurrealdbModel, Debug, Serialize, Deserialize)]
#[surrealdb(relation_name = "manage")]
pub struct Account_Manage_Project {
    #[surrealdb(skip_serializing)]
    id: Option<String>,
    #[surrealdb(link_one = "Account", skip_serializing)]
    r#in: LinkOne<Account>,
    #[surrealdb(link_one = "Project", skip_serializing)]
    out: LinkOne<Project>,
    // when: Any,
    // destination: Any,
}

#[derive(SurrealdbModel, Debug, Serialize, Deserialize, SimpleObject, Clone)]
pub struct Project {
    #[surrealdb(skip_serializing)]
    id: Option<String>,
    name: String,

    // #[surrealdb(relate = "->has->Release")]
    #[graphql(skip)]
    #[surrealdb(relate(edge = "ProjectHasRelease", link = "->has->Release"))]
    releases: Relate<Release>,
    #[surrealdb(relate(edge = "Account_Manage_Project", link = "<-manage<-Account"))]
    #[graphql(skip)]
    authors: Relate<Account>,
}

#[derive(SurrealdbModel, Debug, Serialize, Deserialize)]
#[surrealdb(relation_name = "has")]
pub struct ProjectHasRelease {
    #[surrealdb(skip_serializing)]
    id: Option<String>,
    name: String,
    #[surrealdb(link_one = "Project", skip_serializing)]
    r#in: LinkOne<Project>,
    #[surrealdb(link_one = "Release", skip_serializing)]
    out: LinkOne<Release>,
}

#[derive(SurrealdbModel, Debug, Serialize, Deserialize, Default, SimpleObject, Clone)]
pub struct Release {
    #[surrealdb(skip_serializing)]
    id: Option<String>,
    name: String,
    // #[surrealdb(relate(edge = "ProjectHasRelease", link = "->has->Release"))]
}

use account::schema::model as account;
use project::schema::model as project;

#[derive(SurrealdbModel, Debug, Serialize, Deserialize, Clone)]
pub struct File {
    id: Option<String>,
    name: String,
    #[surrealdb(link_one = "Account", skip_serializing)]
    author: LinkOne<Account>,
}

#[test]
fn test_create_account_query() {
    let query = query()
        .create(
            Account::get_schema()
                .handle
                .as_named_label(&Account::get_schema().to_string()),
        )
        .set_model(&Account::get_schema())
        .unwrap()
        .build();

    assert_eq!(
        query,
        "CREATE Account:handle SET handle = $handle , password = $password , email = $email"
    );
}

#[test]
fn test_account_find_query() {
    let query = query()
        .select("*")
        .from(account)
        .filter(Account::get_schema().email.equals_parameterized())
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
    // let x = f.value().unwrap().friend.0.value();
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
    // let f: RefOne<Account> = RefOne::new_value(Account {
    //     id: Some("Account:John".to_owned()),
    //     ..Default::default()
    // });

    // once called, the Foreign should deserialize even the Values without calling
    // IntoKeys
    f.allow_value_serialize();

    assert_eq!(
        serde_json::to_string(&Account {
            id: Some("Account:John".to_owned()),
            ..Default::default()
        })
        .unwrap(),
        // serde_json::to_string(&f).unwrap()
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
    assert!(match &file.author.into_inner().value() {
        Some(acc) => acc.id == Some("Account:John".to_owned()),
        _ => false,
    });

    // build a json string where the author field is an ID string.
    let key_author_json = "{ \"name\": \"filename\", \"author\": \"Account:John\" }";
    let file: File = serde_json::from_str(&key_author_json).unwrap();

    // confirm the author field of the file is a Key with the account's ID
    assert!(match file.author.into_inner().key().as_deref() {
        Some(key) => key == &"Account:John".to_owned(),
        _ => false,
    });

    // build a json string where the author field is set to null.
    let unloaded_author_json = "{ \"name\": \"filename\", \"author\": null }";
    let file: File = serde_json::from_str(&unloaded_author_json).unwrap();

    // confirm the author field of the file is Unloaded
    assert!(file.author.into_inner().is_unloaded());
}

/// Test that a model can have fields that reference the `Self` type.
#[test]
fn test_model_self_reference() {
    assert_eq!("friend", Account::get_schema().friend.to_string());
    assert_eq!("Account", Account::get_schema().friend().to_string());
    assert_eq!(
        "friend.handle",
        Account::get_schema().friend().handle.to_string()
    );
}

#[test]
fn test_model_serializing_relations() {
    assert_eq!(
        "->manage->Project AS account_projects",
        Account::get_schema()
            .managed_projects
            .as_alias("account_projects")
    );
    assert_eq!(
        "Project",
        Account::get_schema().managed_projects().to_string()
    );
    assert_eq!(
        "->manage->Project.name AS project_names",
        Account::get_schema()
            .managed_projects()
            .name
            .as_alias("project_names")
    );

    assert_eq!(
        "->manage->Project->has->Release AS account_projects_releases",
        Account::get_schema()
            .managed_projects()
            .releases
            .as_alias("account_projects_releases")
    );

    assert_eq!(
        "->manage->Project->has->Release.name AS account_projects_release_names",
        Account::get_schema()
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
        .as_named_label(&Account::get_schema().to_string())
        .with(&Account::get_schema().managed_projects.with_id("other_id"));

    let query_two = Account::get_schema()
        .with_id("an_id")
        .with(&Account::get_schema().managed_projects.with_id("other_id"));

    assert_eq!("Account:an_id->manage->Project:other_id", query_two);

    assert_eq!(query_one, query_two);
}
