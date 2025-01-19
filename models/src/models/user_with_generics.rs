/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surreal_orm::*;

#[derive(Node, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[orm(table = user)]
pub struct User<'a> {
    pub id: SurrealId<Self, String>,
    pub name: String,
    pub created: DateTime<Utc>,
    pub company: &'a str,
    pub tags: Vec<String>,
}

#[derive(Edge, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[orm(table = like)]
pub struct Like<In: Node, Out: Node> {
    pub id: SurrealSimpleId<Self>,

    #[orm(ty = "option<float>")]
    pub score: Option<f64>,

    #[orm(ty = "array<array<float>>")]
    pub scores_plural: Vec<Vec<f64>>,

    #[serde(rename = "in")]
    #[orm(link_many = In)]
    pub in_: LinkOne<In>,

    #[orm(link_one = Out)]
    pub out: LinkOne<Out>,

    #[orm(nest_object = Time)]
    pub time: Time,
}
pub type CompanyLikeUser<'a> = Like<Company<'a>, User<'a>>;

#[derive(Node, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[orm(table = company)]
pub struct Company<'b> {
    pub id: SurrealSimpleId<Self>,
    pub name: String,
    pub namex: &'b str,

    #[orm(link_many = "User<'b>")]
    #[serde(default)]
    pub users: LinkMany<User<'b>>,

    #[orm(relate(model = "CompanyLikeUser<'b>", connection = "->like->user"))]
    #[serde(skip_serializing, default)]
    pub devs: Relate<User<'b>>,
}

#[derive(Object, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Time {
    // pub name: String,
    pub connected: DateTime<Utc>,
}

#[derive(Node, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[orm(table = organization)]
pub struct Organization<'a> {
    pub id: SurrealSimpleId<Self>,
    pub name: String,

    #[orm(link_many = "User<'a>")]
    pub users: LinkMany<User<'a>>,

    #[orm(nest_object = Time)]
    pub time: Time,
    pub age: u8,
}
