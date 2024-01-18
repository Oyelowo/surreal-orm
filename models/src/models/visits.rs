/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::time::Duration;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surreal_orm::{Edge, LinkMany, LinkOne, Model, Node, Relate, SurrealSimpleId};

use super::planet::Planet;

// Visits

// Connects Alien to Planet via Visits
// pub type AlienVisitsPlanet = Visits<Alien, Planet<u64>>;

// // VisitsExplicit
// #[derive(Edge, Serialize, Deserialize, Debug, Clone, Default)]
// #[serde(rename_all = "camelCase")]
// #[surreal_orm(table_name = "visits_explicit")]
// pub struct VisitsExplicit<In: Node, Out: Node> {
//     #[surreal_orm(type_ = "record<visits_explicit>")]
//     pub id: SurrealSimpleId<Self>,
//     #[serde(rename = "in")]
//     #[surreal_orm(type_ = "record")]
//     pub in_: LinkOne<In>,
//     #[surreal_orm(type_ = "record")]
//     // #[surreal_orm(type_ = "record<planet>")]
//     pub out: LinkOne<Out>,
//     #[surreal_orm(type_ = "duration")]
//     pub time_visited: Duration,
// }
//
// // Connects Alien to Planet via Visits
// pub type AlienVisitsPlanetExplicit = VisitsExplicit<Alien, Planet<u64>>;
//
// #[derive(Serialize, Deserialize, Debug, Clone)]
// #[serde(rename_all = "camelCase")]
// // #[surreal_orm(table_name = "visits_with_explicit_attributes")]
// pub struct VisitsWithExplicitAttributes<In: Node, Out: Node> {
//     // #[surreal_orm(type_ = "record<visits_with_explicit_attributes>")]
//     pub id: SurrealSimpleId<Self>,
//
//     #[serde(rename = "in")]
//     // #[surreal_orm(type_ = "record")]
//     pub in_: LinkOne<In>,
//     // #[surreal_orm(type_ = "record")]
//     pub out: LinkOne<Out>,
//
//     // #[surreal_orm(type_ = "string")]
//     name: String,
//
//     // #[surreal_orm(type_ = "int")]
//     age: u8,
//
//     // #[surreal_orm(type_ = "datetime")]
//     created: DateTime<Utc>,
//
//     // #[surreal_orm(type_ = "duration")]
//     life_expectancy: Duration,
//
//     // #[surreal_orm(type_ = "geometry<polygon>")]
//     territory_area: geo::Polygon,
//
//     // #[surreal_orm(type_ = "geometry<point>")]
//     home: geo::Point,
//
//     // #[surreal_orm(type_ = "array<string>")]
//     tags: Vec<String>,
//
//     // #[surreal_orm(link_one = "Weapon", type_ = "record<weapon>")]
//     weapon: LinkOne<Weapon>,
//
//     // Again, we dont have to provide the type attribute, it can auto detect
//     // #[surreal_orm(link_many = "SpaceShip", type_ = "array<record<space_ship>>")]
//     space_ships: LinkMany<SpaceShip>,
// }
//
// pub type AlienVisitsPlanetWithExplicitAttributes = VisitsWithExplicitAttributes<Alien, Planet<u64>>;
//
//
//
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Nma<'a> {
    // pub id: SurrealSimpleId<Self>,
    pub time_visited: Duration,
    pub dfd: &'static str,
    pub age: &'a str,
}

// #[derive(Edge, Serialize, Deserialize, Debug, Clone)]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
// #[surreal_orm(table_name = "visits")]
pub struct Visits<'a, In: Node, Out>
where
    Out: Node,
{
    pub id: SurrealSimpleId<Self>,
    // pub id: SurrealSimpleId<Self>,
    // #[serde(rename = "in")]
    pub in_: LinkOne<In>,
    pub out: LinkOne<Out>,
    pub name: &'a str,
    pub hair_color: Option<&'a str>,
    // #[surreal_orm(type_ = "duration")]
    pub time_visited: Dura<'a>,
    // #[surreal_orm(link_one = "Planet")]
    pub mana: LinkOne<Planet<u64>>,
    // pub age: &'a u8,
    // pub zdf: &'static u8,
    // pub zdf: &'static str,
    // pub name: &'a String,
    // pub name2: &'a str,
    // pub created: DateTime<Utc>,
    // pub life_expectancy: Duration,
    // pub line_polygon: geo::LineString,
    // pub territory_area: geo::Polygon,
    // pub home: geo::Point,
    // pub tags: Vec<String>,
    // #[surreal_orm(link_one = "Weapon")]
    // pub weapon: LinkOne<Weapon>,
    // Again, we dont have to provide the type attribute, it can auto detect
    // #[surreal_orm(link_many = "SpaceShip")]
    // pub space_ships: LinkMany<SpaceShip>,
    // This is a read only field
    // #[surreal_orm(relate(model = "AlienVisitsPlanet", connection = "->visits->planet"))]
    // #[serde(skip_serializing, default)]
    // pub planets_to_visit: Relate<Planet<u64>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Dura<'a>(&'a str);
impl<'a> From<Dura<'a>> for surreal_orm::sql::Duration {
    fn from(d: Dura) -> Self {
        todo!()
        // *d.0.into()
    }
}

//
//
//
//
//
//
//
//
//
// =====================================
// Recursive expansion of the Edge macro
// =====================================

use surreal_orm::ToRaw as _;
impl<'a, In: Node, Out> Visits<'a, In, Out>
where
    Out: Node,
{
    pub const fn __get_serializable_field_names() -> [&'static str; 7usize] {
        [
            "surreal_orm :: Field :: new (\"id\")",
            "surreal_orm :: Field :: new (\"in\")",
            "surreal_orm :: Field :: new (\"out\")",
            "surreal_orm :: Field :: new (\"name\")",
            "surreal_orm :: Field :: new (\"hairColor\")",
            "surreal_orm :: Field :: new (\"timeVisited\")",
            "surreal_orm :: Field :: new (\"mana\")",
        ]
    }
}
impl<'a, In: Node, Out> surreal_orm::SchemaGetter for Visits<'a, In, Out>
where
    Out: Node,
{
    type Schema = ________internal_visits_schema::Visits;
    fn schema() -> visits::Schema {
        visits::Schema::new()
    }
    fn schema_prefixed(
        prefix: impl ::std::convert::Into<surreal_orm::ValueLike>,
    ) -> visits::Schema {
        visits::Schema::new_prefixed(prefix)
    }
}
#[allow(non_snake_case)]
impl<'a, In: Node, Out> surreal_orm::Edge for Visits<'a, In, Out>
where
    Out: Node,
{
    type In = In;
    type Out = Out;
    type TableNameChecker = ________internal_visits_schema::TableNameStaticChecker;
    #[allow(non_snake_case)]
    fn get_table_name() -> surreal_orm::Table {
        "visits".into()
    }
}
#[allow(non_snake_case)]
#[derive(surreal_orm::serde::Serialize, surreal_orm::serde::Deserialize, Debug, Clone, Default)]
pub struct VisitsNonNullUpdater<'a, In: Node, Out>
where
    Out: Node,
{
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: ::std::option::Option<SurrealSimpleId<Visits<'a, In, Out>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_: ::std::option::Option<LinkOne<In>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out: ::std::option::Option<LinkOne<Out>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: ::std::option::Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hairColor: ::std::option::Option<Option<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeVisited: ::std::option::Option<Dura<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mana: ::std::option::Option<LinkOne<Planet<u64>>>,
}
#[allow(non_snake_case)]
#[derive(surreal_orm::serde::Serialize, surreal_orm::serde::Deserialize, Debug, Clone)]
pub struct VisitsRenamedCreator {
    pub id: &'static str,
    pub in_: &'static str,
    pub out: &'static str,
    pub name: &'static str,
    pub hairColor: &'static str,
    pub timeVisited: &'static str,
    pub mana: &'static str,
}
#[allow(non_snake_case)]
impl<'a, In: Node, Out> surreal_orm::Model for Visits<'a, In, Out>
where
    Out: Node,
{
    type Id = SurrealSimpleId<Visits<'a, In, Out>>;
    type NonNullUpdater = VisitsNonNullUpdater<'a, In, Out>;
    type StructRenamedCreator = VisitsRenamedCreator;
    fn table_name() -> surreal_orm::Table {
        "visits".into()
    }
    fn get_id(self) -> Self::Id {
        self.id
    }
    fn get_id_as_thing(&self) -> surreal_orm::sql::Thing {
        surreal_orm::sql::thing(self.id.to_raw().as_str()).unwrap()
    }
    fn get_serializable_fields() -> ::std::vec::Vec<surreal_orm::Field> {
        return vec![
            surreal_orm::Field::new("id"),
            surreal_orm::Field::new("in"),
            surreal_orm::Field::new("out"),
            surreal_orm::Field::new("name"),
            surreal_orm::Field::new("hairColor"),
            surreal_orm::Field::new("timeVisited"),
            surreal_orm::Field::new("mana"),
        ];
    }
    fn get_linked_fields() -> ::std::vec::Vec<surreal_orm::Field> {
        return vec![surreal_orm::Field::new("mana")];
    }
    fn get_link_one_fields() -> ::std::vec::Vec<surreal_orm::Field> {
        return vec![surreal_orm::Field::new("mana")];
    }
    fn get_link_self_fields() -> ::std::vec::Vec<surreal_orm::Field> {
        return vec![];
    }
    fn get_link_one_and_self_fields() -> ::std::vec::Vec<surreal_orm::Field> {
        return vec![surreal_orm::Field::new("mana")];
    }
    fn get_link_many_fields() -> ::std::vec::Vec<surreal_orm::Field> {
        return vec![];
    }
    fn define_table() -> surreal_orm::Raw {
        surreal_orm::statements::define_table(Self::table_name()).to_raw()
    }
    fn define_fields() -> ::std::vec::Vec<surreal_orm::Raw> {
        vec![
            surreal_orm::statements::define_field(surreal_orm::Field::new("id"))
                .on_table(surreal_orm::Table::from(Self::table_name()))
                .type_(surreal_orm::FieldType::Record(::std::vec![
                    Self::table_name()
                ]))
                .to_raw(),
            surreal_orm::statements::define_field(surreal_orm::Field::new("in"))
                .on_table(surreal_orm::Table::from(Self::table_name()))
                .type_(surreal_orm::FieldType::Record(::std::vec![]))
                .to_raw(),
            surreal_orm::statements::define_field(surreal_orm::Field::new("out"))
                .on_table(surreal_orm::Table::from(Self::table_name()))
                .type_(surreal_orm::FieldType::Record(::std::vec![]))
                .to_raw(),
            surreal_orm::statements::define_field(surreal_orm::Field::new("name"))
                .on_table(surreal_orm::Table::from(Self::table_name()))
                .type_(surreal_orm::FieldType::String)
                .to_raw(),
            surreal_orm::statements::define_field(surreal_orm::Field::new("hairColor"))
                .on_table(surreal_orm::Table::from(Self::table_name()))
                .type_(surreal_orm::FieldType::Option(::std::boxed::Box::new(
                    surreal_orm::FieldType::String,
                )))
                .to_raw(),
            surreal_orm::statements::define_field(surreal_orm::Field::new("timeVisited"))
                .on_table(surreal_orm::Table::from(Self::table_name()))
                .type_("duration".parse::<surreal_orm::FieldType>().expect(
                    "Must have been checked at compile time. If not, this is a bug. Please report",
                ))
                .to_raw(),
            surreal_orm::statements::define_field(surreal_orm::Field::new("mana"))
                .on_table(surreal_orm::Table::from(Self::table_name()))
                .type_(surreal_orm::FieldType::Record(::std::vec![
                    Planet::table_name()
                ]))
                .to_raw(),
        ]
    }
    fn get_field_meta() -> ::std::vec::Vec<surreal_orm::FieldMetadata> {
        return vec![
            surreal_orm::FieldMetadata {
                name: "id".into(),
                old_name: ::std::option::Option::None,
                definition: ::std::vec![surreal_orm::statements::define_field(
                    surreal_orm::Field::new("id")
                )
                .on_table(surreal_orm::Table::from(Self::table_name()))
                .type_(surreal_orm::FieldType::Record(::std::vec![
                    Self::table_name()
                ]))
                .to_raw()],
            },
            surreal_orm::FieldMetadata {
                name: "in".into(),
                old_name: ::std::option::Option::None,
                definition: ::std::vec![surreal_orm::statements::define_field(
                    surreal_orm::Field::new("in")
                )
                .on_table(surreal_orm::Table::from(Self::table_name()))
                .type_(surreal_orm::FieldType::Record(::std::vec![]))
                .to_raw()],
            },
            surreal_orm::FieldMetadata {
                name: "out".into(),
                old_name: ::std::option::Option::None,
                definition: ::std::vec![surreal_orm::statements::define_field(
                    surreal_orm::Field::new("out")
                )
                .on_table(surreal_orm::Table::from(Self::table_name()))
                .type_(surreal_orm::FieldType::Record(::std::vec![]))
                .to_raw()],
            },
            surreal_orm::FieldMetadata {
                name: "name".into(),
                old_name: ::std::option::Option::None,
                definition: ::std::vec![surreal_orm::statements::define_field(
                    surreal_orm::Field::new("name")
                )
                .on_table(surreal_orm::Table::from(Self::table_name()))
                .type_(surreal_orm::FieldType::String)
                .to_raw()],
            },
            surreal_orm::FieldMetadata {
                name: "hairColor".into(),
                old_name: ::std::option::Option::None,
                definition: ::std::vec![surreal_orm::statements::define_field(
                    surreal_orm::Field::new("hairColor")
                )
                .on_table(surreal_orm::Table::from(Self::table_name()))
                .type_(surreal_orm::FieldType::Option(::std::boxed::Box::new(
                    surreal_orm::FieldType::String
                )))
                .to_raw()],
            },
            surreal_orm::FieldMetadata {
                name: "timeVisited".into(),
                old_name: ::std::option::Option::None,
                definition: ::std::vec![surreal_orm::statements::define_field(
                    surreal_orm::Field::new("timeVisited")
                )
                .on_table(surreal_orm::Table::from(Self::table_name()))
                .type_("duration".parse::<surreal_orm::FieldType>().expect(
                    "Must have been checked at compile time. If not, this is a bug. Please report"
                ))
                .to_raw()],
            },
            surreal_orm::FieldMetadata {
                name: "mana".into(),
                old_name: ::std::option::Option::None,
                definition: ::std::vec![surreal_orm::statements::define_field(
                    surreal_orm::Field::new("mana")
                )
                .on_table(surreal_orm::Table::from(Self::table_name()))
                .type_(surreal_orm::FieldType::Record(::std::vec![
                    Planet::table_name()
                ]))
                .to_raw()],
            },
        ];
    }
}
#[allow(non_snake_case)]
pub mod visits {
    pub use super::________internal_visits_schema::_____schema_def::Schema;
}
#[allow(non_snake_case)]
mod ________internal_visits_schema {
    use surreal_orm::Buildable as _;
    use surreal_orm::Erroneous as _;
    use surreal_orm::Node;
    use surreal_orm::Parametric as _;
    pub struct TableNameStaticChecker {
        pub visits: ::std::string::String,
    }
    type Planet<T> = <super::Planet<T> as surreal_orm::SchemaGetter>::Schema;
    pub(super) mod _____field_names {
        use super::super::*;
        use surreal_orm::Buildable as _;
        use surreal_orm::Parametric as _;
        #[derive(Debug, Clone)]
        pub struct Id_______________(pub surreal_orm::Field);
        impl ::std::convert::From<&str> for Id_______________ {
            fn from(field_name: &str) -> Self {
                Self(surreal_orm::Field::new(field_name))
            }
        }
        impl ::std::convert::From<surreal_orm::Field> for Id_______________ {
            fn from(field_name: surreal_orm::Field) -> Self {
                Self(field_name)
            }
        }
        impl ::std::convert::From<&Id_______________> for surreal_orm::ValueLike {
            fn from(value: &Id_______________) -> Self {
                let field: surreal_orm::Field = value.into();
                field.into()
            }
        }
        impl ::std::convert::From<Id_______________> for surreal_orm::ValueLike {
            fn from(value: Id_______________) -> Self {
                let field: surreal_orm::Field = value.into();
                field.into()
            }
        }
        impl ::std::convert::From<&Id_______________> for surreal_orm::Field {
            fn from(field_name: &Id_______________) -> Self {
                field_name.0.clone()
            }
        }
        impl ::std::convert::From<Id_______________> for surreal_orm::Field {
            fn from(field_name: Id_______________) -> Self {
                field_name.0
            }
        }
        impl ::std::ops::Deref for Id_______________ {
            type Target = surreal_orm::Field;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl ::std::ops::DerefMut for Id_______________ {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
        impl<T: surreal_orm::serde::Serialize> ::std::convert::From<self::Id_______________>
            for surreal_orm::SetterArg<T>
        {
            fn from(value: self::Id_______________) -> Self {
                Self::Field(value.into())
            }
        }
        impl<T: surreal_orm::serde::Serialize> ::std::convert::From<&self::Id_______________>
            for surreal_orm::SetterArg<T>
        {
            fn from(value: &self::Id_______________) -> Self {
                Self::Field(value.into())
            }
        }
        impl<'a, In: Node, Out> surreal_orm::SetterAssignable<SurrealSimpleId<Visits<'a, In, Out>>>
            for self::Id_______________
        where
            Out: Node,
        {
        }
        impl<'a, In: Node, Out> surreal_orm::Patchable<SurrealSimpleId<Visits<'a, In, Out>>>
            for self::Id_______________
        where
            Out: Node,
        {
        }
        #[derive(Debug, Clone)]
        pub struct In_______________(pub surreal_orm::Field);
        impl ::std::convert::From<&str> for In_______________ {
            fn from(field_name: &str) -> Self {
                Self(surreal_orm::Field::new(field_name))
            }
        }
        impl ::std::convert::From<surreal_orm::Field> for In_______________ {
            fn from(field_name: surreal_orm::Field) -> Self {
                Self(field_name)
            }
        }
        impl ::std::convert::From<&In_______________> for surreal_orm::ValueLike {
            fn from(value: &In_______________) -> Self {
                let field: surreal_orm::Field = value.into();
                field.into()
            }
        }
        impl ::std::convert::From<In_______________> for surreal_orm::ValueLike {
            fn from(value: In_______________) -> Self {
                let field: surreal_orm::Field = value.into();
                field.into()
            }
        }
        impl ::std::convert::From<&In_______________> for surreal_orm::Field {
            fn from(field_name: &In_______________) -> Self {
                field_name.0.clone()
            }
        }
        impl ::std::convert::From<In_______________> for surreal_orm::Field {
            fn from(field_name: In_______________) -> Self {
                field_name.0
            }
        }
        impl ::std::ops::Deref for In_______________ {
            type Target = surreal_orm::Field;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl ::std::ops::DerefMut for In_______________ {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
        impl<T: surreal_orm::serde::Serialize> ::std::convert::From<self::In_______________>
            for surreal_orm::SetterArg<T>
        {
            fn from(value: self::In_______________) -> Self {
                Self::Field(value.into())
            }
        }
        impl<T: surreal_orm::serde::Serialize> ::std::convert::From<&self::In_______________>
            for surreal_orm::SetterArg<T>
        {
            fn from(value: &self::In_______________) -> Self {
                Self::Field(value.into())
            }
        }
        impl<In: Node> surreal_orm::SetterAssignable<LinkOne<In>> for self::In_______________ {}
        impl<In: Node> surreal_orm::Patchable<LinkOne<In>> for self::In_______________ {}
        #[derive(Debug, Clone)]
        pub struct Out_______________(pub surreal_orm::Field);
        impl ::std::convert::From<&str> for Out_______________ {
            fn from(field_name: &str) -> Self {
                Self(surreal_orm::Field::new(field_name))
            }
        }
        impl ::std::convert::From<surreal_orm::Field> for Out_______________ {
            fn from(field_name: surreal_orm::Field) -> Self {
                Self(field_name)
            }
        }
        impl ::std::convert::From<&Out_______________> for surreal_orm::ValueLike {
            fn from(value: &Out_______________) -> Self {
                let field: surreal_orm::Field = value.into();
                field.into()
            }
        }
        impl ::std::convert::From<Out_______________> for surreal_orm::ValueLike {
            fn from(value: Out_______________) -> Self {
                let field: surreal_orm::Field = value.into();
                field.into()
            }
        }
        impl ::std::convert::From<&Out_______________> for surreal_orm::Field {
            fn from(field_name: &Out_______________) -> Self {
                field_name.0.clone()
            }
        }
        impl ::std::convert::From<Out_______________> for surreal_orm::Field {
            fn from(field_name: Out_______________) -> Self {
                field_name.0
            }
        }
        impl ::std::ops::Deref for Out_______________ {
            type Target = surreal_orm::Field;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl ::std::ops::DerefMut for Out_______________ {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
        impl<T: surreal_orm::serde::Serialize> ::std::convert::From<self::Out_______________>
            for surreal_orm::SetterArg<T>
        {
            fn from(value: self::Out_______________) -> Self {
                Self::Field(value.into())
            }
        }
        impl<T: surreal_orm::serde::Serialize> ::std::convert::From<&self::Out_______________>
            for surreal_orm::SetterArg<T>
        {
            fn from(value: &self::Out_______________) -> Self {
                Self::Field(value.into())
            }
        }
        impl<Out> surreal_orm::SetterAssignable<LinkOne<Out>> for self::Out_______________ where Out: Node {}
        impl<Out> surreal_orm::Patchable<LinkOne<Out>> for self::Out_______________ where Out: Node {}
        #[derive(Debug, Clone)]
        pub struct Name_______________(pub surreal_orm::Field);
        impl ::std::convert::From<&str> for Name_______________ {
            fn from(field_name: &str) -> Self {
                Self(surreal_orm::Field::new(field_name))
            }
        }
        impl ::std::convert::From<surreal_orm::Field> for Name_______________ {
            fn from(field_name: surreal_orm::Field) -> Self {
                Self(field_name)
            }
        }
        impl ::std::convert::From<&Name_______________> for surreal_orm::ValueLike {
            fn from(value: &Name_______________) -> Self {
                let field: surreal_orm::Field = value.into();
                field.into()
            }
        }
        impl ::std::convert::From<Name_______________> for surreal_orm::ValueLike {
            fn from(value: Name_______________) -> Self {
                let field: surreal_orm::Field = value.into();
                field.into()
            }
        }
        impl ::std::convert::From<&Name_______________> for surreal_orm::Field {
            fn from(field_name: &Name_______________) -> Self {
                field_name.0.clone()
            }
        }
        impl ::std::convert::From<Name_______________> for surreal_orm::Field {
            fn from(field_name: Name_______________) -> Self {
                field_name.0
            }
        }
        impl ::std::ops::Deref for Name_______________ {
            type Target = surreal_orm::Field;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl ::std::ops::DerefMut for Name_______________ {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
        impl<T: surreal_orm::serde::Serialize> ::std::convert::From<self::Name_______________>
            for surreal_orm::SetterArg<T>
        {
            fn from(value: self::Name_______________) -> Self {
                Self::Field(value.into())
            }
        }
        impl<T: surreal_orm::serde::Serialize> ::std::convert::From<&self::Name_______________>
            for surreal_orm::SetterArg<T>
        {
            fn from(value: &self::Name_______________) -> Self {
                Self::Field(value.into())
            }
        }
        impl<'a> surreal_orm::SetterAssignable<&'a str> for self::Name_______________ {}
        impl<'a> surreal_orm::Patchable<&'a str> for self::Name_______________ {}
        #[derive(Debug, Clone)]
        pub struct HairColor_______________(pub surreal_orm::Field);
        impl ::std::convert::From<&str> for HairColor_______________ {
            fn from(field_name: &str) -> Self {
                Self(surreal_orm::Field::new(field_name))
            }
        }
        impl ::std::convert::From<surreal_orm::Field> for HairColor_______________ {
            fn from(field_name: surreal_orm::Field) -> Self {
                Self(field_name)
            }
        }
        impl ::std::convert::From<&HairColor_______________> for surreal_orm::ValueLike {
            fn from(value: &HairColor_______________) -> Self {
                let field: surreal_orm::Field = value.into();
                field.into()
            }
        }
        impl ::std::convert::From<HairColor_______________> for surreal_orm::ValueLike {
            fn from(value: HairColor_______________) -> Self {
                let field: surreal_orm::Field = value.into();
                field.into()
            }
        }
        impl ::std::convert::From<&HairColor_______________> for surreal_orm::Field {
            fn from(field_name: &HairColor_______________) -> Self {
                field_name.0.clone()
            }
        }
        impl ::std::convert::From<HairColor_______________> for surreal_orm::Field {
            fn from(field_name: HairColor_______________) -> Self {
                field_name.0
            }
        }
        impl ::std::ops::Deref for HairColor_______________ {
            type Target = surreal_orm::Field;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl ::std::ops::DerefMut for HairColor_______________ {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
        impl<T: surreal_orm::serde::Serialize> ::std::convert::From<self::HairColor_______________>
            for surreal_orm::SetterArg<T>
        {
            fn from(value: self::HairColor_______________) -> Self {
                Self::Field(value.into())
            }
        }
        impl<T: surreal_orm::serde::Serialize> ::std::convert::From<&self::HairColor_______________>
            for surreal_orm::SetterArg<T>
        {
            fn from(value: &self::HairColor_______________) -> Self {
                Self::Field(value.into())
            }
        }
        impl<'a> surreal_orm::SetterAssignable<Option<&'a str>> for self::HairColor_______________ {}
        impl<'a> surreal_orm::Patchable<Option<&'a str>> for self::HairColor_______________ {}
        #[derive(Debug, Clone)]
        pub struct TimeVisited_______________(pub surreal_orm::Field);
        impl ::std::convert::From<&str> for TimeVisited_______________ {
            fn from(field_name: &str) -> Self {
                Self(surreal_orm::Field::new(field_name))
            }
        }
        impl ::std::convert::From<surreal_orm::Field> for TimeVisited_______________ {
            fn from(field_name: surreal_orm::Field) -> Self {
                Self(field_name)
            }
        }
        impl ::std::convert::From<&TimeVisited_______________> for surreal_orm::ValueLike {
            fn from(value: &TimeVisited_______________) -> Self {
                let field: surreal_orm::Field = value.into();
                field.into()
            }
        }
        impl ::std::convert::From<TimeVisited_______________> for surreal_orm::ValueLike {
            fn from(value: TimeVisited_______________) -> Self {
                let field: surreal_orm::Field = value.into();
                field.into()
            }
        }
        impl ::std::convert::From<&TimeVisited_______________> for surreal_orm::Field {
            fn from(field_name: &TimeVisited_______________) -> Self {
                field_name.0.clone()
            }
        }
        impl ::std::convert::From<TimeVisited_______________> for surreal_orm::Field {
            fn from(field_name: TimeVisited_______________) -> Self {
                field_name.0
            }
        }
        impl ::std::ops::Deref for TimeVisited_______________ {
            type Target = surreal_orm::Field;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl ::std::ops::DerefMut for TimeVisited_______________ {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
        impl<T: surreal_orm::serde::Serialize>
            ::std::convert::From<self::TimeVisited_______________> for surreal_orm::SetterArg<T>
        {
            fn from(value: self::TimeVisited_______________) -> Self {
                Self::Field(value.into())
            }
        }
        impl<T: surreal_orm::serde::Serialize>
            ::std::convert::From<&self::TimeVisited_______________> for surreal_orm::SetterArg<T>
        {
            fn from(value: &self::TimeVisited_______________) -> Self {
                Self::Field(value.into())
            }
        }
        impl<'a> surreal_orm::SetterAssignable<Dura<'a>> for self::TimeVisited_______________ {}
        impl<'a> surreal_orm::Patchable<Dura<'a>> for self::TimeVisited_______________ {}
        #[derive(Debug, Clone)]
        pub struct Mana_______________(pub surreal_orm::Field);
        impl ::std::convert::From<&str> for Mana_______________ {
            fn from(field_name: &str) -> Self {
                Self(surreal_orm::Field::new(field_name))
            }
        }
        impl ::std::convert::From<surreal_orm::Field> for Mana_______________ {
            fn from(field_name: surreal_orm::Field) -> Self {
                Self(field_name)
            }
        }
        impl ::std::convert::From<&Mana_______________> for surreal_orm::ValueLike {
            fn from(value: &Mana_______________) -> Self {
                let field: surreal_orm::Field = value.into();
                field.into()
            }
        }
        impl ::std::convert::From<Mana_______________> for surreal_orm::ValueLike {
            fn from(value: Mana_______________) -> Self {
                let field: surreal_orm::Field = value.into();
                field.into()
            }
        }
        impl ::std::convert::From<&Mana_______________> for surreal_orm::Field {
            fn from(field_name: &Mana_______________) -> Self {
                field_name.0.clone()
            }
        }
        impl ::std::convert::From<Mana_______________> for surreal_orm::Field {
            fn from(field_name: Mana_______________) -> Self {
                field_name.0
            }
        }
        impl ::std::ops::Deref for Mana_______________ {
            type Target = surreal_orm::Field;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl ::std::ops::DerefMut for Mana_______________ {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
        impl<T: surreal_orm::serde::Serialize> ::std::convert::From<self::Mana_______________>
            for surreal_orm::SetterArg<T>
        {
            fn from(value: self::Mana_______________) -> Self {
                Self::Field(value.into())
            }
        }
        impl<T: surreal_orm::serde::Serialize> ::std::convert::From<&self::Mana_______________>
            for surreal_orm::SetterArg<T>
        {
            fn from(value: &self::Mana_______________) -> Self {
                Self::Field(value.into())
            }
        }
        impl surreal_orm::SetterAssignable<LinkOne<Planet<u64>>> for self::Mana_______________ {}
        impl surreal_orm::Patchable<LinkOne<Planet<u64>>> for self::Mana_______________ {}
    }
    pub mod _____schema_def {
        use super::_____field_names;
        #[allow(non_snake_case)]
        #[derive(Debug, Clone)]
        pub struct Schema {
            pub id: _____field_names::Id_______________,
            pub in_: _____field_names::In_______________,
            pub out: _____field_names::Out_______________,
            pub name: _____field_names::Name_______________,
            pub hairColor: _____field_names::HairColor_______________,
            pub timeVisited: _____field_names::TimeVisited_______________,
            pub mana: _____field_names::Mana_______________,
            pub(super) ___________graph_traversal_string: ::std::string::String,
            pub(super) ___________bindings: surreal_orm::BindingsList,
            pub(super) ___________errors: ::std::vec::Vec<::std::string::String>,
        }
    }
    pub type Visits = _____schema_def::Schema;
    impl surreal_orm::Buildable for Visits {
        fn build(&self) -> ::std::string::String {
            self.___________graph_traversal_string.to_string()
        }
    }
    impl surreal_orm::Parametric for Visits {
        fn get_bindings(&self) -> surreal_orm::BindingsList {
            self.___________bindings.to_vec()
        }
    }
    impl surreal_orm::Erroneous for Visits {
        fn get_errors(&self) -> Vec<::std::string::String> {
            self.___________errors.to_vec()
        }
    }
    impl surreal_orm::Aliasable for &Visits {}
    impl surreal_orm::Parametric for &Visits {
        fn get_bindings(&self) -> surreal_orm::BindingsList {
            self.___________bindings.to_vec()
        }
    }
    impl surreal_orm::Buildable for &Visits {
        fn build(&self) -> ::std::string::String {
            self.___________graph_traversal_string.to_string()
        }
    }
    impl surreal_orm::Erroneous for &Visits {
        fn get_errors(&self) -> ::std::vec::Vec<::std::string::String> {
            self.___________errors.to_vec()
        }
    }
    impl Visits {
        pub fn new() -> Self {
            Self {
                id: "id".into(),
                in_: "in".into(),
                out: "out".into(),
                name: "name".into(),
                hairColor: "hairColor".into(),
                timeVisited: "timeVisited".into(),
                mana: "mana".into(),
                ___________graph_traversal_string: "".into(),
                ___________bindings: vec![],
                ___________errors: vec![],
            }
        }
        pub fn new_prefixed(prefix: impl ::std::convert::Into<surreal_orm::ValueLike>) -> Self {
            let prefix: surreal_orm::ValueLike = prefix.into();
            Self {
                id: surreal_orm::Field::new(format!("{}.{}", prefix.build(), "id"))
                    .with_bindings(prefix.get_bindings())
                    .into(),
                in_: surreal_orm::Field::new(format!("{}.{}", prefix.build(), "in"))
                    .with_bindings(prefix.get_bindings())
                    .into(),
                out: surreal_orm::Field::new(format!("{}.{}", prefix.build(), "out"))
                    .with_bindings(prefix.get_bindings())
                    .into(),
                name: surreal_orm::Field::new(format!("{}.{}", prefix.build(), "name"))
                    .with_bindings(prefix.get_bindings())
                    .into(),
                hairColor: surreal_orm::Field::new(format!("{}.{}", prefix.build(), "hairColor"))
                    .with_bindings(prefix.get_bindings())
                    .into(),
                timeVisited: surreal_orm::Field::new(format!(
                    "{}.{}",
                    prefix.build(),
                    "timeVisited"
                ))
                .with_bindings(prefix.get_bindings())
                .into(),
                mana: surreal_orm::Field::new(format!("{}.{}", prefix.build(), "mana"))
                    .with_bindings(prefix.get_bindings())
                    .into(),
                ___________graph_traversal_string: prefix.build(),
                ___________bindings: prefix.get_bindings(),
                ___________errors: vec![],
            }
        }
        pub fn empty() -> Self {
            Self {
                id: "".into(),
                in_: "".into(),
                out: "".into(),
                name: "".into(),
                hairColor: "".into(),
                timeVisited: "".into(),
                mana: "".into(),
                ___________graph_traversal_string: "".into(),
                ___________bindings: vec![],
                ___________errors: vec![],
            }
        }
        pub fn __________connect_edge_to_graph_traversal_string(
            connection: impl surreal_orm::Buildable + surreal_orm::Parametric + surreal_orm::Erroneous,
            clause: impl Into<surreal_orm::EdgeClause>,
        ) -> Self {
            let mut schema_instance = Self::empty();
            let clause: surreal_orm::EdgeClause = clause.into();
            let bindings = [
                connection.get_bindings().as_slice(),
                clause.get_bindings().as_slice(),
            ]
            .concat();
            let bindings = bindings.as_slice();
            schema_instance.___________bindings = bindings.into();
            let errors = [
                connection.get_errors().as_slice(),
                clause.get_errors().as_slice(),
            ]
            .concat();
            let errors = errors.as_slice();
            schema_instance.___________errors = errors.into();
            let schema_edge_str_with_arrow = format!("{}{}", connection.build(), clause.build(),);
            schema_instance
                .___________graph_traversal_string
                .push_str(schema_edge_str_with_arrow.as_str());
            let ___________graph_traversal_string =
                &schema_instance.___________graph_traversal_string;
            schema_instance.id = schema_instance
                .id
                .set_graph_string(format!("{}.{}", ___________graph_traversal_string, "id"))
                .____________update_many_bindings(bindings)
                .into();
            schema_instance.in_ = schema_instance
                .in_
                .set_graph_string(format!("{}.{}", ___________graph_traversal_string, "in"))
                .____________update_many_bindings(bindings)
                .into();
            schema_instance.out = schema_instance
                .out
                .set_graph_string(format!("{}.{}", ___________graph_traversal_string, "out"))
                .____________update_many_bindings(bindings)
                .into();
            schema_instance.name = schema_instance
                .name
                .set_graph_string(format!("{}.{}", ___________graph_traversal_string, "name"))
                .____________update_many_bindings(bindings)
                .into();
            schema_instance.hairColor = schema_instance
                .hairColor
                .set_graph_string(format!(
                    "{}.{}",
                    ___________graph_traversal_string, "hairColor"
                ))
                .____________update_many_bindings(bindings)
                .into();
            schema_instance.timeVisited = schema_instance
                .timeVisited
                .set_graph_string(format!(
                    "{}.{}",
                    ___________graph_traversal_string, "timeVisited"
                ))
                .____________update_many_bindings(bindings)
                .into();
            schema_instance.mana = schema_instance
                .mana
                .set_graph_string(format!("{}.{}", ___________graph_traversal_string, "mana"))
                .____________update_many_bindings(bindings)
                .into();
            schema_instance
        }
        pub fn mana<T: surreal_orm::validators::Int + serde::Serialize>(&self) -> Planet<T> {
            let clause = surreal_orm::Clause::from(surreal_orm::Empty);
            let normalized_field_name_str = if self.build().is_empty() {
                "mana".to_string()
            } else {
                format!(".{}", "mana")
            };
            Planet::<T>::__________connect_node_to_graph_traversal_string(
                self,
                clause.with_field(normalized_field_name_str),
            )
        }
    }
}
// #[macro_export]
// macro_rules! assert_impl {
//     ($type:ty, $trait:ident) => {
//         fn _assert_impl() {
//             // Function to assert trait implementation
//             // It's never called, but checked for correctness at compile time.
//             fn assert_impl<T: $trait>() {}
//             assert_impl::<$type>();
//         }
//     };
// }
// #[macro_export]
// macro_rules! assert_impl {
//     ($type:ty, $trait:path) => {
//         const _: fn() = || {
//             // Inner function to assert trait implementation
//             fn assert_impl<T: $trait>() {}
//             assert_impl::<$type>();
//         };
//     };
// }

// #[macro_export]
// macro_rules! assert_impl {
//     ($type:ty, $trait:path) => {
//         const _: fn() = || {
//             fn assert_impl<T: $trait>() {}
//             assert_impl::<$type>();
//         };
//     };
// }

// #[macro_export]
// macro_rules! assert_impl {
//     ($type:ty, $trait:path) => {
//         const _: fn() = || {
//             trait TraitAssert: $trait {}
//             impl TraitAssert for $type {}
//         };
//     };
// }

#[allow(non_snake_case)]
fn test_visits_edge_name<T: surreal_orm::validators::Int + serde::Serialize>() {
    surreal_orm::validators::assert_impl_one!(&'_ str: ::std::convert::Into<surreal_orm::sql::Strand>);
    surreal_orm::validators::assert_option::<Option<&'_ str>>();
    surreal_orm::validators::assert_impl_one!(&'_ str: ::std::convert::Into<surreal_orm::sql::Strand>);
    surreal_orm::validators::assert_impl_one!(Dura<'_> : ::std::convert::Into<surreal_orm::sql::Duration>);
    surreal_orm::validators::assert_type_eq_all!(
        LinkOne<Planet<u64>>,
        surreal_orm::LinkOne<Planet<_>>
    );
    type Mana = Planet<T>;
    surreal_orm::validators::assert_impl_one!(Mana:surreal_orm::Node);
    assert_impl!(Planet<T>, surreal_orm::Node);
}
