/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surreal_orm::{
    LinkMany, LinkOne, Relate, SurrealEdge, SurrealId, SurrealModel, SurrealNode, SurrealObject,
    SurrealSimpleId,
};
use surrealdb::sql;

#[derive(SurrealNode, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "user")]
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

#[derive(SurrealEdge, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "like")]
pub struct Like<In: SurrealNode, Out: SurrealNode> {
    pub id: SurrealSimpleId<Self>,
    #[serde(rename = "in", skip_serializing)]
    pub in_: LinkOne<In>,
    #[serde(skip_serializing)]
    pub out: LinkOne<Out>,
    #[surreal_orm(nest_object = "Time")]
    pub time: Time,
}
pub type CompanyLikeUser = Like<Company, User>;

#[derive(SurrealNode, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "company")]
pub struct Company {
    pub id: SurrealSimpleId<Self>,
    pub name: String,
    #[surreal_orm(link_many = "User")]
    pub users: LinkMany<User>,

    #[surreal_orm(relate(model = "CompanyLikeUser", connection = "->like->user"))]
    pub devs: Relate<User>,
}

#[derive(SurrealObject, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Time {
    // pub name: String,
    pub connected: DateTime<Utc>,
}

#[derive(SurrealNode, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "organization")]
pub struct Organization {
    pub id: SurrealSimpleId<Self>,
    pub name: String,
    #[surreal_orm(link_many = "User")]
    pub users: LinkMany<User>,
    #[surreal_orm(nest_object = "Time")]
    pub time: Time,
    pub age: u8,
}
