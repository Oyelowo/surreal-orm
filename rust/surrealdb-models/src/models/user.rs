use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql;
use surrealdb_orm::{
    LinkMany, LinkOne, Relate, SurrealId, SurrealSimpleId, SurrealdbEdge, SurrealdbModel,
    SurrealdbNode, SurrealdbObject,
};

#[derive(SurrealdbNode, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[surrealdb(table_name = "user")]
pub struct User {
    pub id: SurrealId<Self, String>,
    pub name: String,
    pub created: DateTime<Utc>,
    pub company: String,
    pub tags: Vec<String>,
}

impl Default for User {
    fn default() -> Self {
        Self {
            id: User::create_id(sql::Id::rand().to_string()),
            name: Default::default(),
            created: Default::default(),
            company: Default::default(),
            tags: Default::default(),
        }
    }
}

#[derive(SurrealdbEdge, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surrealdb(table_name = "like")]
pub struct Like<In: SurrealdbNode, Out: SurrealdbNode> {
    // pub id: SurrealId<Like<In, Out>>,
    pub id: SurrealSimpleId<Self>,
    #[serde(rename = "in", skip_serializing)]
    pub in_: LinkOne<In>,
    #[serde(skip_serializing)]
    pub out: LinkOne<Out>,
    #[surrealdb(nest_object = "Time")]
    pub time: Time,
}
pub type CompanyLikeUser = Like<Company, User>;

#[derive(SurrealdbNode, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surrealdb(table_name = "company")]
pub struct Company {
    pub id: SurrealSimpleId<Self>,
    pub name: String,
    #[surrealdb(link_many = "User")]
    pub users: LinkMany<User>,

    #[surrealdb(relate(model = "CompanyLikeUser", connection = "->like->user"))]
    pub devs: Relate<User>,
}

#[derive(SurrealdbObject, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Time {
    // pub name: String,
    pub connected: DateTime<Utc>,
}

#[derive(SurrealdbNode, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surrealdb(table_name = "organization")]
pub struct Organization {
    pub id: SurrealSimpleId<Self>,
    pub name: String,
    #[surrealdb(link_many = "User")]
    pub users: LinkMany<User>,
    #[surrealdb(nest_object = "Time")]
    pub time: Time,
    pub age: u8,
}
