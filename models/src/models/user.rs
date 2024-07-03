/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surreal_orm::{
    sql, Edge, LinkMany, LinkOne, Model, Node, Object, Relate, SurrealId, SurrealSimpleId,
};

#[derive(Node, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table = user)]
pub struct User<'a> {
    pub id: SurrealId<Self, String>,
    pub name: String,
    pub created: DateTime<Utc>,
    pub company: String,
    pub tags: Vec<&'a String>,
}




impl<'a> Default for User<'a> {
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

#[derive(Edge, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table = like)]
pub struct Like<In: Node + Default, Out: Node + Default> {
    pub id: SurrealSimpleId<Self>,
    // #[surreal_orm(ty = "option<array<float>>")]
    // pub score: Option<Vec<f64>>,
    #[serde(rename = "in", skip_serializing)]
    #[surreal_orm(link_many = "In")]
    pub in_: LinkMany<In>,
    #[serde(skip_serializing)]
    #[surreal_orm(link_many = "Out")]
    pub out: LinkMany<Out>,
    #[surreal_orm(nest_object = Time)]
    pub time: Time,
}
pub type CompanyLikeUser<'a> = Like<Company<'a>, User<'a>>;

type Mana<'a> = <User<'a> as surreal_orm::Model>::Id;
type Manaa<In: Node> = <In as surreal_orm::Model>::Id;


#[derive( Node, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table = company)]
pub struct Company<'b> {
    pub id: SurrealSimpleId<Self>,
    pub name: String,
    #[surreal_orm(link_many = "User<'b>")]
    pub users: LinkMany<User<'b>>,

    #[surreal_orm(relate(model = "CompanyLikeUser<'b>", connection = "->like->user"))]
    pub devs: Relate<User<'b>>,
}

#[derive(Object, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Time {
    // pub name: String,
    pub connected: DateTime<Utc>,
}

#[derive(Node, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table = organization)]
pub struct Organization<'a> {
    pub id: SurrealSimpleId<Self>,
    pub name: String,
    #[surreal_orm(link_many = "User<'a>")]
    pub users: LinkMany<User<'a>>,
    #[surreal_orm(nest_object = Time)]
    pub time: Time,
    pub age: u8,
}


