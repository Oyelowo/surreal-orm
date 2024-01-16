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

use crate::{Alien, Planet, SpaceShip, Weapon};

// Visits
#[derive(Edge, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "visits")]
pub struct Visits<'a, In: Node, Out: Node> {
    // pub id: SurrealSimpleId<Self>,
    pub id: SurrealSimpleId<Visits<In, Out>>,
    // pub id: SurrealSimpleId<Self>,
    #[serde(rename = "in")]
    pub in_: LinkOne<In>,
    pub out: LinkOne<Out>,
    pub time_visited: Duration,
    pub age: &'a u8,
    pub created: DateTime<Utc>,
    pub life_expectancy: Duration,
    pub line_polygon: geo::LineString,
    pub territory_area: geo::Polygon,
    pub home: geo::Point,
    pub tags: Vec<String>,
    #[surreal_orm(link_one = "Weapon")]
    pub weapon: LinkOne<Weapon>,

    // Again, we dont have to provide the type attribute, it can auto detect
    #[surreal_orm(link_many = "SpaceShip")]
    pub space_ships: LinkMany<SpaceShip>,

    // This is a read only field
    #[surreal_orm(relate(model = "AlienVisitsPlanet", connection = "->visits->planet"))]
    #[serde(skip_serializing, default)]
    pub planets_to_visit: Relate<Planet<u64>>,
}

// Connects Alien to Planet via Visits
pub type AlienVisitsPlanet = Visits<Alien, Planet<u64>>;

// VisitsExplicit
#[derive(Edge, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[surreal_orm(table_name = "visits_explicit")]
pub struct VisitsExplicit<In: Node, Out: Node> {
    #[surreal_orm(type_ = "record<visits_explicit>")]
    pub id: SurrealSimpleId<Self>,
    #[serde(rename = "in")]
    #[surreal_orm(type_ = "record")]
    pub in_: LinkOne<In>,
    #[surreal_orm(type_ = "record")]
    // #[surreal_orm(type_ = "record<planet>")]
    pub out: LinkOne<Out>,
    #[surreal_orm(type_ = "duration")]
    pub time_visited: Duration,
}

// Connects Alien to Planet via Visits
pub type AlienVisitsPlanetExplicit = VisitsExplicit<Alien, Planet<u64>>;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
// #[surreal_orm(table_name = "visits_with_explicit_attributes")]
pub struct VisitsWithExplicitAttributes<In: Node, Out: Node> {
    // #[surreal_orm(type_ = "record<visits_with_explicit_attributes>")]
    pub id: SurrealSimpleId<Self>,

    #[serde(rename = "in")]
    // #[surreal_orm(type_ = "record")]
    pub in_: LinkOne<In>,
    // #[surreal_orm(type_ = "record")]
    pub out: LinkOne<Out>,

    // #[surreal_orm(type_ = "string")]
    name: String,

    // #[surreal_orm(type_ = "int")]
    age: u8,

    // #[surreal_orm(type_ = "datetime")]
    created: DateTime<Utc>,

    // #[surreal_orm(type_ = "duration")]
    life_expectancy: Duration,

    // #[surreal_orm(type_ = "geometry<polygon>")]
    territory_area: geo::Polygon,

    // #[surreal_orm(type_ = "geometry<point>")]
    home: geo::Point,

    // #[surreal_orm(type_ = "array<string>")]
    tags: Vec<String>,

    // #[surreal_orm(link_one = "Weapon", type_ = "record<weapon>")]
    weapon: LinkOne<Weapon>,

    // Again, we dont have to provide the type attribute, it can auto detect
    // #[surreal_orm(link_many = "SpaceShip", type_ = "array<record<space_ship>>")]
    space_ships: LinkMany<SpaceShip>,
}

pub type AlienVisitsPlanetWithExplicitAttributes = VisitsWithExplicitAttributes<Alien, Planet<u64>>;

//
//
//
//
//
//

use surreal_orm::ToRaw as _;
impl<In: Node, Out: Node> VisitsWithExplicitAttributes<In, Out> {
    pub const fn __get_serializable_field_names() -> [&'static str; 12usize] {
        [
            "surreal_orm :: Field :: new (\"id\")",
            "surreal_orm :: Field :: new (\"in\")",
            "surreal_orm :: Field :: new (\"out\")",
            "surreal_orm :: Field :: new (\"name\")",
            "surreal_orm :: Field :: new (\"age\")",
            "surreal_orm :: Field :: new (\"created\")",
            "surreal_orm :: Field :: new (\"lifeExpectancy\")",
            "surreal_orm :: Field :: new (\"territoryArea\")",
            "surreal_orm :: Field :: new (\"home\")",
            "surreal_orm :: Field :: new (\"tags\")",
            "surreal_orm :: Field :: new (\"weapon\")",
            "surreal_orm :: Field :: new (\"spaceShips\")",
        ]
    }
}
impl<In: Node, Out: Node> surreal_orm::SchemaGetter for VisitsWithExplicitAttributes<In, Out> {
    type Schema =
        ________internal_visits_with_explicit_attributes_schema::VisitsWithExplicitAttributes;
    fn schema() -> visits_with_explicit_attributes::Schema {
        visits_with_explicit_attributes::Schema::new()
    }
    fn schema_prefixed(
        prefix: impl ::std::convert::Into<surreal_orm::ValueLike>,
    ) -> visits_with_explicit_attributes::Schema {
        visits_with_explicit_attributes::Schema::new_prefixed(prefix)
    }
}
#[allow(non_snake_case)]
impl<In: Node, Out: Node> surreal_orm::Edge for VisitsWithExplicitAttributes<In, Out> {
    type In = In;
    type Out = Out;
    type TableNameChecker =
        ________internal_visits_with_explicit_attributes_schema::TableNameStaticChecker;
    #[allow(non_snake_case)]
    fn get_table_name() -> surreal_orm::Table {
        "visits_with_explicit_attributes".into()
    }
}
#[allow(non_snake_case)]
#[derive(surreal_orm::serde::Serialize, surreal_orm::serde::Deserialize, Debug, Clone, Default)]
pub struct VisitsWithExplicitAttributesNonNullUpdater {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: ::std::option::Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age: ::std::option::Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: ::std::option::Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifeExpectancy: ::std::option::Option<Duration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub territoryArea: ::std::option::Option<geo::Polygon>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home: ::std::option::Option<geo::Point>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: ::std::option::Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weapon: ::std::option::Option<LinkOne<Weapon>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spaceShips: ::std::option::Option<LinkMany<SpaceShip>>,
}
#[allow(non_snake_case)]
#[derive(surreal_orm::serde::Serialize, surreal_orm::serde::Deserialize, Debug, Clone)]
pub struct VisitsWithExplicitAttributesRenamedCreator {
    pub id: &'static str,
    pub in_: &'static str,
    pub out: &'static str,
    pub name: &'static str,
    pub age: &'static str,
    pub created: &'static str,
    pub lifeExpectancy: &'static str,
    pub territoryArea: &'static str,
    pub home: &'static str,
    pub tags: &'static str,
    pub weapon: &'static str,
    pub spaceShips: &'static str,
}
#[allow(non_snake_case)]
impl<In: Node, Out: Node> surreal_orm::Model for VisitsWithExplicitAttributes<In, Out> {
    type Id = SurrealSimpleId<Self>;
    type NonNullUpdater = VisitsWithExplicitAttributesNonNullUpdater;
    type StructRenamedCreator = VisitsWithExplicitAttributesRenamedCreator;
    fn table_name() -> surreal_orm::Table {
        "visits_with_explicit_attributes".into()
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
            surreal_orm::Field::new("age"),
            surreal_orm::Field::new("created"),
            surreal_orm::Field::new("lifeExpectancy"),
            surreal_orm::Field::new("territoryArea"),
            surreal_orm::Field::new("home"),
            surreal_orm::Field::new("tags"),
            surreal_orm::Field::new("weapon"),
            surreal_orm::Field::new("spaceShips"),
        ];
    }
    fn get_linked_fields() -> ::std::vec::Vec<surreal_orm::Field> {
        return vec![
            surreal_orm::Field::new("weapon"),
            surreal_orm::Field::new("spaceShips"),
        ];
    }
    fn get_link_one_fields() -> ::std::vec::Vec<surreal_orm::Field> {
        return vec![surreal_orm::Field::new("weapon")];
    }
    fn get_link_self_fields() -> ::std::vec::Vec<surreal_orm::Field> {
        return vec![];
    }
    fn get_link_one_and_self_fields() -> ::std::vec::Vec<surreal_orm::Field> {
        return vec![surreal_orm::Field::new("weapon")];
    }
    fn get_link_many_fields() -> ::std::vec::Vec<surreal_orm::Field> {
        return vec![surreal_orm::Field::new("spaceShips")];
    }
    fn define_table() -> surreal_orm::Raw {
        surreal_orm::statements::define_table(Self::table_name()).to_raw()
    }
    fn define_fields() -> ::std::vec::Vec<surreal_orm::Raw> {
        vec![surreal_orm::statements::define_field(surreal_orm::Field::new("id")).on_table(surreal_orm::Table::from(Self::table_name())).type_("record<visits_with_explicit_attributes>".parse:: <surreal_orm::FieldType>().expect("Must have been checked at compile time. If not, this is a bug. Please report")).to_raw(),surreal_orm::statements::define_field(surreal_orm::Field::new("in")).on_table(surreal_orm::Table::from(Self::table_name())).type_("record<any>".parse:: <surreal_orm::FieldType>().expect("Must have been checked at compile time. If not, this is a bug. Please report")).to_raw(),surreal_orm::statements::define_field(surreal_orm::Field::new("out")).on_table(surreal_orm::Table::from(Self::table_name())).type_("record<any>".parse:: <surreal_orm::FieldType>().expect("Must have been checked at compile time. If not, this is a bug. Please report")).to_raw(),surreal_orm::statements::define_field(surreal_orm::Field::new("name")).on_table(surreal_orm::Table::from(Self::table_name())).type_("string".parse:: <surreal_orm::FieldType>().expect("Must have been checked at compile time. If not, this is a bug. Please report")).to_raw(),surreal_orm::statements::define_field(surreal_orm::Field::new("age")).on_table(surreal_orm::Table::from(Self::table_name())).type_("int".parse:: <surreal_orm::FieldType>().expect("Must have been checked at compile time. If not, this is a bug. Please report")).to_raw(),surreal_orm::statements::define_field(surreal_orm::Field::new("created")).on_table(surreal_orm::Table::from(Self::table_name())).type_("datetime".parse:: <surreal_orm::FieldType>().expect("Must have been checked at compile time. If not, this is a bug. Please report")).to_raw(),surreal_orm::statements::define_field(surreal_orm::Field::new("lifeExpectancy")).on_table(surreal_orm::Table::from(Self::table_name())).type_("duration".parse:: <surreal_orm::FieldType>().expect("Must have been checked at compile time. If not, this is a bug. Please report")).to_raw(),surreal_orm::statements::define_field(surreal_orm::Field::new("territoryArea")).on_table(surreal_orm::Table::from(Self::table_name())).type_("geometry<polygon>".parse:: <surreal_orm::FieldType>().expect("Must have been checked at compile time. If not, this is a bug. Please report")).to_raw(),surreal_orm::statements::define_field(surreal_orm::Field::new("home")).on_table(surreal_orm::Table::from(Self::table_name())).type_("geometry<point>".parse:: <surreal_orm::FieldType>().expect("Must have been checked at compile time. If not, this is a bug. Please report")).to_raw(),surreal_orm::statements::define_field(surreal_orm::Field::new("tags")).on_table(surreal_orm::Table::from(Self::table_name())).type_("array<string>".parse:: <surreal_orm::FieldType>().expect("Must have been checked at compile time. If not, this is a bug. Please report")).to_raw(),surreal_orm::statements::define_field(surreal_orm::Field::new("weapon")).on_table(surreal_orm::Table::from(Self::table_name())).type_("record<weapon>".parse:: <surreal_orm::FieldType>().expect("Must have been checked at compile time. If not, this is a bug. Please report")).to_raw(),surreal_orm::statements::define_field(surreal_orm::Field::new("spaceShips")).on_table(surreal_orm::Table::from(Self::table_name())).type_("array<record<space_ship>>".parse:: <surreal_orm::FieldType>().expect("Must have been checked at compile time. If not, this is a bug. Please report")).to_raw()]
    }
    fn get_field_meta() -> ::std::vec::Vec<surreal_orm::FieldMetadata> {
        return vec![surreal_orm::FieldMetadata {
      name:"id".into(),old_name: ::std::option::Option::None,definition: ::std::vec![surreal_orm::statements::define_field(surreal_orm::Field::new("id")).on_table(surreal_orm::Table::from(Self::table_name())).type_("record<visits_with_explicit_attributes>".parse:: <surreal_orm::FieldType>().expect("Must have been checked at compile time. If not, this is a bug. Please report")).to_raw()]
    },surreal_orm::FieldMetadata {
      name:"in".into(),old_name: ::std::option::Option::None,definition: ::std::vec![surreal_orm::statements::define_field(surreal_orm::Field::new("in")).on_table(surreal_orm::Table::from(Self::table_name())).type_("record<any>".parse:: <surreal_orm::FieldType>().expect("Must have been checked at compile time. If not, this is a bug. Please report")).to_raw()]
    },surreal_orm::FieldMetadata {
      name:"out".into(),old_name: ::std::option::Option::None,definition: ::std::vec![surreal_orm::statements::define_field(surreal_orm::Field::new("out")).on_table(surreal_orm::Table::from(Self::table_name())).type_("record<any>".parse:: <surreal_orm::FieldType>().expect("Must have been checked at compile time. If not, this is a bug. Please report")).to_raw()]
    },surreal_orm::FieldMetadata {
      name:"name".into(),old_name: ::std::option::Option::None,definition: ::std::vec![surreal_orm::statements::define_field(surreal_orm::Field::new("name")).on_table(surreal_orm::Table::from(Self::table_name())).type_("string".parse:: <surreal_orm::FieldType>().expect("Must have been checked at compile time. If not, this is a bug. Please report")).to_raw()]
    },surreal_orm::FieldMetadata {
      name:"age".into(),old_name: ::std::option::Option::None,definition: ::std::vec![surreal_orm::statements::define_field(surreal_orm::Field::new("age")).on_table(surreal_orm::Table::from(Self::table_name())).type_("int".parse:: <surreal_orm::FieldType>().expect("Must have been checked at compile time. If not, this is a bug. Please report")).to_raw()]
    },surreal_orm::FieldMetadata {
      name:"created".into(),old_name: ::std::option::Option::None,definition: ::std::vec![surreal_orm::statements::define_field(surreal_orm::Field::new("created")).on_table(surreal_orm::Table::from(Self::table_name())).type_("datetime".parse:: <surreal_orm::FieldType>().expect("Must have been checked at compile time. If not, this is a bug. Please report")).to_raw()]
    },surreal_orm::FieldMetadata {
      name:"lifeExpectancy".into(),old_name: ::std::option::Option::None,definition: ::std::vec![surreal_orm::statements::define_field(surreal_orm::Field::new("lifeExpectancy")).on_table(surreal_orm::Table::from(Self::table_name())).type_("duration".parse:: <surreal_orm::FieldType>().expect("Must have been checked at compile time. If not, this is a bug. Please report")).to_raw()]
    },surreal_orm::FieldMetadata {
      name:"territoryArea".into(),old_name: ::std::option::Option::None,definition: ::std::vec![surreal_orm::statements::define_field(surreal_orm::Field::new("territoryArea")).on_table(surreal_orm::Table::from(Self::table_name())).type_("geometry<polygon>".parse:: <surreal_orm::FieldType>().expect("Must have been checked at compile time. If not, this is a bug. Please report")).to_raw()]
    },surreal_orm::FieldMetadata {
      name:"home".into(),old_name: ::std::option::Option::None,definition: ::std::vec![surreal_orm::statements::define_field(surreal_orm::Field::new("home")).on_table(surreal_orm::Table::from(Self::table_name())).type_("geometry<point>".parse:: <surreal_orm::FieldType>().expect("Must have been checked at compile time. If not, this is a bug. Please report")).to_raw()]
    },surreal_orm::FieldMetadata {
      name:"tags".into(),old_name: ::std::option::Option::None,definition: ::std::vec![surreal_orm::statements::define_field(surreal_orm::Field::new("tags")).on_table(surreal_orm::Table::from(Self::table_name())).type_("array<string>".parse:: <surreal_orm::FieldType>().expect("Must have been checked at compile time. If not, this is a bug. Please report")).to_raw()]
    },surreal_orm::FieldMetadata {
      name:"weapon".into(),old_name: ::std::option::Option::None,definition: ::std::vec![surreal_orm::statements::define_field(surreal_orm::Field::new("weapon")).on_table(surreal_orm::Table::from(Self::table_name())).type_("record<weapon>".parse:: <surreal_orm::FieldType>().expect("Must have been checked at compile time. If not, this is a bug. Please report")).to_raw()]
    },surreal_orm::FieldMetadata {
      name:"spaceShips".into(),old_name: ::std::option::Option::None,definition: ::std::vec![surreal_orm::statements::define_field(surreal_orm::Field::new("spaceShips")).on_table(surreal_orm::Table::from(Self::table_name())).type_("array<record<space_ship>>".parse:: <surreal_orm::FieldType>().expect("Must have been checked at compile time. If not, this is a bug. Please report")).to_raw()]
    }];
    }
}
#[allow(non_snake_case)]
pub mod visits_with_explicit_attributes {
    pub use super::________internal_visits_with_explicit_attributes_schema::_____schema_def::Schema;
}
#[allow(non_snake_case)]
mod ________internal_visits_with_explicit_attributes_schema {
    use surreal_orm::Buildable as _;
    use surreal_orm::Erroneous as _;
    use surreal_orm::Node;
    use surreal_orm::Parametric as _;
    pub struct TableNameStaticChecker {
        pub visits_with_explicit_attributes: ::std::string::String,
    }
    type Weapon = <super::Weapon as surreal_orm::SchemaGetter>::Schema;
    type SpaceShip = <super::SpaceShip as surreal_orm::SchemaGetter>::Schema;
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
        impl<T> surreal_orm::SetterAssignable<SurrealSimpleId<T>> for self::Id_______________ where
            T: Model + Serialize
        {
        }
        impl surreal_orm::Patchable<SurrealSimpleId<Self>> for self::Id_______________ {}
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
        impl<Out: Node> surreal_orm::SetterAssignable<LinkOne<Out>> for self::Out_______________ {}
        impl<Out: Node> surreal_orm::Patchable<LinkOne<Out>> for self::Out_______________ {}
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
        impl surreal_orm::SetterAssignable<String> for self::Name_______________ {}
        impl surreal_orm::Patchable<String> for self::Name_______________ {}
        #[derive(Debug, Clone)]
        pub struct Age_______________(pub surreal_orm::Field);
        impl ::std::convert::From<&str> for Age_______________ {
            fn from(field_name: &str) -> Self {
                Self(surreal_orm::Field::new(field_name))
            }
        }
        impl ::std::convert::From<surreal_orm::Field> for Age_______________ {
            fn from(field_name: surreal_orm::Field) -> Self {
                Self(field_name)
            }
        }
        impl ::std::convert::From<&Age_______________> for surreal_orm::ValueLike {
            fn from(value: &Age_______________) -> Self {
                let field: surreal_orm::Field = value.into();
                field.into()
            }
        }
        impl ::std::convert::From<Age_______________> for surreal_orm::ValueLike {
            fn from(value: Age_______________) -> Self {
                let field: surreal_orm::Field = value.into();
                field.into()
            }
        }
        impl ::std::convert::From<&Age_______________> for surreal_orm::Field {
            fn from(field_name: &Age_______________) -> Self {
                field_name.0.clone()
            }
        }
        impl ::std::convert::From<Age_______________> for surreal_orm::Field {
            fn from(field_name: Age_______________) -> Self {
                field_name.0
            }
        }
        impl ::std::ops::Deref for Age_______________ {
            type Target = surreal_orm::Field;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl ::std::ops::DerefMut for Age_______________ {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
        impl<T: surreal_orm::serde::Serialize> ::std::convert::From<self::Age_______________>
            for surreal_orm::SetterArg<T>
        {
            fn from(value: self::Age_______________) -> Self {
                Self::Field(value.into())
            }
        }
        impl<T: surreal_orm::serde::Serialize> ::std::convert::From<&self::Age_______________>
            for surreal_orm::SetterArg<T>
        {
            fn from(value: &self::Age_______________) -> Self {
                Self::Field(value.into())
            }
        }
        impl surreal_orm::SetterAssignable<u8> for self::Age_______________ {}
        impl surreal_orm::Patchable<u8> for self::Age_______________ {}
        impl surreal_orm::SetterNumeric<u8> for self::Age_______________ {}
        impl ::std::convert::From<self::Age_______________> for surreal_orm::NumberLike {
            fn from(val: self::Age_______________) -> Self {
                val.0.into()
            }
        }
        impl ::std::convert::From<&self::Age_______________> for surreal_orm::NumberLike {
            fn from(val: &self::Age_______________) -> Self {
                val.clone().0.into()
            }
        }
        impl<T: ::std::convert::Into<surreal_orm::NumberLike>> ::std::ops::Add<T> for Age_______________ {
            type Output = surreal_orm::Operation;
            fn add(self, rhs: T) -> Self::Output {
                let rhs: surreal_orm::NumberLike = rhs.into();
                surreal_orm::Operation {
                    query_string: format!("{} + {}", self.build(), rhs.build()),
                    bindings: [self.get_bindings(), rhs.get_bindings()].concat(),
                    errors: vec![],
                }
            }
        }
        impl<T: ::std::convert::Into<surreal_orm::NumberLike>> ::std::ops::Sub<T> for Age_______________ {
            type Output = surreal_orm::Operation;
            fn sub(self, rhs: T) -> Self::Output {
                let rhs: surreal_orm::NumberLike = rhs.into();
                surreal_orm::Operation {
                    query_string: format!("{} - {}", self.build(), rhs.build()),
                    bindings: [self.get_bindings(), rhs.get_bindings()].concat(),
                    errors: vec![],
                }
            }
        }
        impl<T: ::std::convert::Into<surreal_orm::NumberLike>> ::std::ops::Mul<T> for Age_______________ {
            type Output = surreal_orm::Operation;
            fn mul(self, rhs: T) -> Self::Output {
                let rhs: surreal_orm::NumberLike = rhs.into();
                surreal_orm::Operation {
                    query_string: format!("{} * {}", self.build(), rhs.build()),
                    bindings: [self.get_bindings(), rhs.get_bindings()].concat(),
                    errors: vec![],
                }
            }
        }
        impl<T: ::std::convert::Into<surreal_orm::NumberLike>> ::std::ops::Div<T> for Age_______________ {
            type Output = surreal_orm::Operation;
            fn div(self, rhs: T) -> Self::Output {
                let rhs: surreal_orm::NumberLike = rhs.into();
                surreal_orm::Operation {
                    query_string: format!("{} / {}", self.build(), rhs.build()),
                    bindings: [self.get_bindings(), rhs.get_bindings()].concat(),
                    errors: vec![],
                }
            }
        }
        impl<T: ::std::convert::Into<surreal_orm::NumberLike>> ::std::ops::Add<T> for &Age_______________ {
            type Output = surreal_orm::Operation;
            fn add(self, rhs: T) -> Self::Output {
                let rhs: surreal_orm::NumberLike = rhs.into();
                surreal_orm::Operation {
                    query_string: format!("{} + {}", self.build(), rhs.build()),
                    bindings: [self.get_bindings(), rhs.get_bindings()].concat(),
                    errors: vec![],
                }
            }
        }
        impl<T: ::std::convert::Into<surreal_orm::NumberLike>> ::std::ops::Sub<T> for &Age_______________ {
            type Output = surreal_orm::Operation;
            fn sub(self, rhs: T) -> Self::Output {
                let rhs: surreal_orm::NumberLike = rhs.into();
                surreal_orm::Operation {
                    query_string: format!("{} - {}", self.build(), rhs.build()),
                    bindings: [self.get_bindings(), rhs.get_bindings()].concat(),
                    errors: vec![],
                }
            }
        }
        impl<T: ::std::convert::Into<surreal_orm::NumberLike>> ::std::ops::Mul<T> for &Age_______________ {
            type Output = surreal_orm::Operation;
            fn mul(self, rhs: T) -> Self::Output {
                let rhs: surreal_orm::NumberLike = rhs.into();
                surreal_orm::Operation {
                    query_string: format!("{} * {}", self.build(), rhs.build()),
                    bindings: [self.get_bindings(), rhs.get_bindings()].concat(),
                    errors: vec![],
                }
            }
        }
        impl<T: ::std::convert::Into<surreal_orm::NumberLike>> ::std::ops::Div<T> for &Age_______________ {
            type Output = surreal_orm::Operation;
            fn div(self, rhs: T) -> Self::Output {
                let rhs: surreal_orm::NumberLike = rhs.into();
                surreal_orm::Operation {
                    query_string: format!("{} / {}", self.build(), rhs.build()),
                    bindings: [self.get_bindings(), rhs.get_bindings()].concat(),
                    errors: vec![],
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct Created_______________(pub surreal_orm::Field);
        impl ::std::convert::From<&str> for Created_______________ {
            fn from(field_name: &str) -> Self {
                Self(surreal_orm::Field::new(field_name))
            }
        }
        impl ::std::convert::From<surreal_orm::Field> for Created_______________ {
            fn from(field_name: surreal_orm::Field) -> Self {
                Self(field_name)
            }
        }
        impl ::std::convert::From<&Created_______________> for surreal_orm::ValueLike {
            fn from(value: &Created_______________) -> Self {
                let field: surreal_orm::Field = value.into();
                field.into()
            }
        }
        impl ::std::convert::From<Created_______________> for surreal_orm::ValueLike {
            fn from(value: Created_______________) -> Self {
                let field: surreal_orm::Field = value.into();
                field.into()
            }
        }
        impl ::std::convert::From<&Created_______________> for surreal_orm::Field {
            fn from(field_name: &Created_______________) -> Self {
                field_name.0.clone()
            }
        }
        impl ::std::convert::From<Created_______________> for surreal_orm::Field {
            fn from(field_name: Created_______________) -> Self {
                field_name.0
            }
        }
        impl ::std::ops::Deref for Created_______________ {
            type Target = surreal_orm::Field;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl ::std::ops::DerefMut for Created_______________ {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
        impl<T: surreal_orm::serde::Serialize> ::std::convert::From<self::Created_______________>
            for surreal_orm::SetterArg<T>
        {
            fn from(value: self::Created_______________) -> Self {
                Self::Field(value.into())
            }
        }
        impl<T: surreal_orm::serde::Serialize> ::std::convert::From<&self::Created_______________>
            for surreal_orm::SetterArg<T>
        {
            fn from(value: &self::Created_______________) -> Self {
                Self::Field(value.into())
            }
        }
        impl surreal_orm::SetterAssignable<DateTime<Utc>> for self::Created_______________ {}
        impl surreal_orm::Patchable<DateTime<Utc>> for self::Created_______________ {}
        #[derive(Debug, Clone)]
        pub struct LifeExpectancy_______________(pub surreal_orm::Field);
        impl ::std::convert::From<&str> for LifeExpectancy_______________ {
            fn from(field_name: &str) -> Self {
                Self(surreal_orm::Field::new(field_name))
            }
        }
        impl ::std::convert::From<surreal_orm::Field> for LifeExpectancy_______________ {
            fn from(field_name: surreal_orm::Field) -> Self {
                Self(field_name)
            }
        }
        impl ::std::convert::From<&LifeExpectancy_______________> for surreal_orm::ValueLike {
            fn from(value: &LifeExpectancy_______________) -> Self {
                let field: surreal_orm::Field = value.into();
                field.into()
            }
        }
        impl ::std::convert::From<LifeExpectancy_______________> for surreal_orm::ValueLike {
            fn from(value: LifeExpectancy_______________) -> Self {
                let field: surreal_orm::Field = value.into();
                field.into()
            }
        }
        impl ::std::convert::From<&LifeExpectancy_______________> for surreal_orm::Field {
            fn from(field_name: &LifeExpectancy_______________) -> Self {
                field_name.0.clone()
            }
        }
        impl ::std::convert::From<LifeExpectancy_______________> for surreal_orm::Field {
            fn from(field_name: LifeExpectancy_______________) -> Self {
                field_name.0
            }
        }
        impl ::std::ops::Deref for LifeExpectancy_______________ {
            type Target = surreal_orm::Field;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl ::std::ops::DerefMut for LifeExpectancy_______________ {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
        impl<T: surreal_orm::serde::Serialize>
            ::std::convert::From<self::LifeExpectancy_______________>
            for surreal_orm::SetterArg<T>
        {
            fn from(value: self::LifeExpectancy_______________) -> Self {
                Self::Field(value.into())
            }
        }
        impl<T: surreal_orm::serde::Serialize>
            ::std::convert::From<&self::LifeExpectancy_______________>
            for surreal_orm::SetterArg<T>
        {
            fn from(value: &self::LifeExpectancy_______________) -> Self {
                Self::Field(value.into())
            }
        }
        impl surreal_orm::SetterAssignable<Duration> for self::LifeExpectancy_______________ {}
        impl surreal_orm::Patchable<Duration> for self::LifeExpectancy_______________ {}
        #[derive(Debug, Clone)]
        pub struct TerritoryArea_______________(pub surreal_orm::Field);
        impl ::std::convert::From<&str> for TerritoryArea_______________ {
            fn from(field_name: &str) -> Self {
                Self(surreal_orm::Field::new(field_name))
            }
        }
        impl ::std::convert::From<surreal_orm::Field> for TerritoryArea_______________ {
            fn from(field_name: surreal_orm::Field) -> Self {
                Self(field_name)
            }
        }
        impl ::std::convert::From<&TerritoryArea_______________> for surreal_orm::ValueLike {
            fn from(value: &TerritoryArea_______________) -> Self {
                let field: surreal_orm::Field = value.into();
                field.into()
            }
        }
        impl ::std::convert::From<TerritoryArea_______________> for surreal_orm::ValueLike {
            fn from(value: TerritoryArea_______________) -> Self {
                let field: surreal_orm::Field = value.into();
                field.into()
            }
        }
        impl ::std::convert::From<&TerritoryArea_______________> for surreal_orm::Field {
            fn from(field_name: &TerritoryArea_______________) -> Self {
                field_name.0.clone()
            }
        }
        impl ::std::convert::From<TerritoryArea_______________> for surreal_orm::Field {
            fn from(field_name: TerritoryArea_______________) -> Self {
                field_name.0
            }
        }
        impl ::std::ops::Deref for TerritoryArea_______________ {
            type Target = surreal_orm::Field;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl ::std::ops::DerefMut for TerritoryArea_______________ {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
        impl<T: surreal_orm::serde::Serialize>
            ::std::convert::From<self::TerritoryArea_______________> for surreal_orm::SetterArg<T>
        {
            fn from(value: self::TerritoryArea_______________) -> Self {
                Self::Field(value.into())
            }
        }
        impl<T: surreal_orm::serde::Serialize>
            ::std::convert::From<&self::TerritoryArea_______________>
            for surreal_orm::SetterArg<T>
        {
            fn from(value: &self::TerritoryArea_______________) -> Self {
                Self::Field(value.into())
            }
        }
        impl surreal_orm::SetterAssignable<geo::Polygon> for self::TerritoryArea_______________ {}
        impl surreal_orm::Patchable<geo::Polygon> for self::TerritoryArea_______________ {}
        #[derive(Debug, Clone)]
        pub struct Home_______________(pub surreal_orm::Field);
        impl ::std::convert::From<&str> for Home_______________ {
            fn from(field_name: &str) -> Self {
                Self(surreal_orm::Field::new(field_name))
            }
        }
        impl ::std::convert::From<surreal_orm::Field> for Home_______________ {
            fn from(field_name: surreal_orm::Field) -> Self {
                Self(field_name)
            }
        }
        impl ::std::convert::From<&Home_______________> for surreal_orm::ValueLike {
            fn from(value: &Home_______________) -> Self {
                let field: surreal_orm::Field = value.into();
                field.into()
            }
        }
        impl ::std::convert::From<Home_______________> for surreal_orm::ValueLike {
            fn from(value: Home_______________) -> Self {
                let field: surreal_orm::Field = value.into();
                field.into()
            }
        }
        impl ::std::convert::From<&Home_______________> for surreal_orm::Field {
            fn from(field_name: &Home_______________) -> Self {
                field_name.0.clone()
            }
        }
        impl ::std::convert::From<Home_______________> for surreal_orm::Field {
            fn from(field_name: Home_______________) -> Self {
                field_name.0
            }
        }
        impl ::std::ops::Deref for Home_______________ {
            type Target = surreal_orm::Field;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl ::std::ops::DerefMut for Home_______________ {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
        impl<T: surreal_orm::serde::Serialize> ::std::convert::From<self::Home_______________>
            for surreal_orm::SetterArg<T>
        {
            fn from(value: self::Home_______________) -> Self {
                Self::Field(value.into())
            }
        }
        impl<T: surreal_orm::serde::Serialize> ::std::convert::From<&self::Home_______________>
            for surreal_orm::SetterArg<T>
        {
            fn from(value: &self::Home_______________) -> Self {
                Self::Field(value.into())
            }
        }
        impl surreal_orm::SetterAssignable<geo::Point> for self::Home_______________ {}
        impl surreal_orm::Patchable<geo::Point> for self::Home_______________ {}
        #[derive(Debug, Clone)]
        pub struct Tags_______________(pub surreal_orm::Field);
        impl ::std::convert::From<&str> for Tags_______________ {
            fn from(field_name: &str) -> Self {
                Self(surreal_orm::Field::new(field_name))
            }
        }
        impl ::std::convert::From<surreal_orm::Field> for Tags_______________ {
            fn from(field_name: surreal_orm::Field) -> Self {
                Self(field_name)
            }
        }
        impl ::std::convert::From<&Tags_______________> for surreal_orm::ValueLike {
            fn from(value: &Tags_______________) -> Self {
                let field: surreal_orm::Field = value.into();
                field.into()
            }
        }
        impl ::std::convert::From<Tags_______________> for surreal_orm::ValueLike {
            fn from(value: Tags_______________) -> Self {
                let field: surreal_orm::Field = value.into();
                field.into()
            }
        }
        impl ::std::convert::From<&Tags_______________> for surreal_orm::Field {
            fn from(field_name: &Tags_______________) -> Self {
                field_name.0.clone()
            }
        }
        impl ::std::convert::From<Tags_______________> for surreal_orm::Field {
            fn from(field_name: Tags_______________) -> Self {
                field_name.0
            }
        }
        impl ::std::ops::Deref for Tags_______________ {
            type Target = surreal_orm::Field;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl ::std::ops::DerefMut for Tags_______________ {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
        impl<T: surreal_orm::serde::Serialize> ::std::convert::From<self::Tags_______________>
            for surreal_orm::SetterArg<T>
        {
            fn from(value: self::Tags_______________) -> Self {
                Self::Field(value.into())
            }
        }
        impl<T: surreal_orm::serde::Serialize> ::std::convert::From<&self::Tags_______________>
            for surreal_orm::SetterArg<T>
        {
            fn from(value: &self::Tags_______________) -> Self {
                Self::Field(value.into())
            }
        }
        impl surreal_orm::SetterAssignable<Vec<String>> for self::Tags_______________ {}
        impl surreal_orm::Patchable<Vec<String>> for self::Tags_______________ {}
        impl surreal_orm::SetterArray<String> for self::Tags_______________ {}
        #[derive(Debug, Clone)]
        pub struct Weapon_______________(pub surreal_orm::Field);
        impl ::std::convert::From<&str> for Weapon_______________ {
            fn from(field_name: &str) -> Self {
                Self(surreal_orm::Field::new(field_name))
            }
        }
        impl ::std::convert::From<surreal_orm::Field> for Weapon_______________ {
            fn from(field_name: surreal_orm::Field) -> Self {
                Self(field_name)
            }
        }
        impl ::std::convert::From<&Weapon_______________> for surreal_orm::ValueLike {
            fn from(value: &Weapon_______________) -> Self {
                let field: surreal_orm::Field = value.into();
                field.into()
            }
        }
        impl ::std::convert::From<Weapon_______________> for surreal_orm::ValueLike {
            fn from(value: Weapon_______________) -> Self {
                let field: surreal_orm::Field = value.into();
                field.into()
            }
        }
        impl ::std::convert::From<&Weapon_______________> for surreal_orm::Field {
            fn from(field_name: &Weapon_______________) -> Self {
                field_name.0.clone()
            }
        }
        impl ::std::convert::From<Weapon_______________> for surreal_orm::Field {
            fn from(field_name: Weapon_______________) -> Self {
                field_name.0
            }
        }
        impl ::std::ops::Deref for Weapon_______________ {
            type Target = surreal_orm::Field;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl ::std::ops::DerefMut for Weapon_______________ {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
        impl<T: surreal_orm::serde::Serialize> ::std::convert::From<self::Weapon_______________>
            for surreal_orm::SetterArg<T>
        {
            fn from(value: self::Weapon_______________) -> Self {
                Self::Field(value.into())
            }
        }
        impl<T: surreal_orm::serde::Serialize> ::std::convert::From<&self::Weapon_______________>
            for surreal_orm::SetterArg<T>
        {
            fn from(value: &self::Weapon_______________) -> Self {
                Self::Field(value.into())
            }
        }
        impl surreal_orm::SetterAssignable<LinkOne<Weapon>> for self::Weapon_______________ {}
        impl surreal_orm::Patchable<LinkOne<Weapon>> for self::Weapon_______________ {}
        #[derive(Debug, Clone)]
        pub struct SpaceShips_______________(pub surreal_orm::Field);
        impl ::std::convert::From<&str> for SpaceShips_______________ {
            fn from(field_name: &str) -> Self {
                Self(surreal_orm::Field::new(field_name))
            }
        }
        impl ::std::convert::From<surreal_orm::Field> for SpaceShips_______________ {
            fn from(field_name: surreal_orm::Field) -> Self {
                Self(field_name)
            }
        }
        impl ::std::convert::From<&SpaceShips_______________> for surreal_orm::ValueLike {
            fn from(value: &SpaceShips_______________) -> Self {
                let field: surreal_orm::Field = value.into();
                field.into()
            }
        }
        impl ::std::convert::From<SpaceShips_______________> for surreal_orm::ValueLike {
            fn from(value: SpaceShips_______________) -> Self {
                let field: surreal_orm::Field = value.into();
                field.into()
            }
        }
        impl ::std::convert::From<&SpaceShips_______________> for surreal_orm::Field {
            fn from(field_name: &SpaceShips_______________) -> Self {
                field_name.0.clone()
            }
        }
        impl ::std::convert::From<SpaceShips_______________> for surreal_orm::Field {
            fn from(field_name: SpaceShips_______________) -> Self {
                field_name.0
            }
        }
        impl ::std::ops::Deref for SpaceShips_______________ {
            type Target = surreal_orm::Field;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl ::std::ops::DerefMut for SpaceShips_______________ {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
        impl<T: surreal_orm::serde::Serialize> ::std::convert::From<self::SpaceShips_______________>
            for surreal_orm::SetterArg<T>
        {
            fn from(value: self::SpaceShips_______________) -> Self {
                Self::Field(value.into())
            }
        }
        impl<T: surreal_orm::serde::Serialize>
            ::std::convert::From<&self::SpaceShips_______________> for surreal_orm::SetterArg<T>
        {
            fn from(value: &self::SpaceShips_______________) -> Self {
                Self::Field(value.into())
            }
        }
        impl surreal_orm::SetterAssignable<LinkMany<SpaceShip>> for self::SpaceShips_______________ {}
        impl surreal_orm::Patchable<LinkMany<SpaceShip>> for self::SpaceShips_______________ {}
        impl surreal_orm::SetterArray<<SpaceShip as surreal_orm::Model>::Id>
            for self::SpaceShips_______________
        {
        }
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
            pub age: _____field_names::Age_______________,
            pub created: _____field_names::Created_______________,
            pub lifeExpectancy: _____field_names::LifeExpectancy_______________,
            pub territoryArea: _____field_names::TerritoryArea_______________,
            pub home: _____field_names::Home_______________,
            pub tags: _____field_names::Tags_______________,
            pub weapon: _____field_names::Weapon_______________,
            pub spaceShips: _____field_names::SpaceShips_______________,
            pub(super) ___________graph_traversal_string: ::std::string::String,
            pub(super) ___________bindings: surreal_orm::BindingsList,
            pub(super) ___________errors: ::std::vec::Vec<::std::string::String>,
        }
    }
    pub type VisitsWithExplicitAttributes = _____schema_def::Schema;
    impl surreal_orm::Buildable for VisitsWithExplicitAttributes {
        fn build(&self) -> ::std::string::String {
            self.___________graph_traversal_string.to_string()
        }
    }
    impl surreal_orm::Parametric for VisitsWithExplicitAttributes {
        fn get_bindings(&self) -> surreal_orm::BindingsList {
            self.___________bindings.to_vec()
        }
    }
    impl surreal_orm::Erroneous for VisitsWithExplicitAttributes {
        fn get_errors(&self) -> Vec<::std::string::String> {
            self.___________errors.to_vec()
        }
    }
    impl surreal_orm::Aliasable for &VisitsWithExplicitAttributes {}
    impl surreal_orm::Parametric for &VisitsWithExplicitAttributes {
        fn get_bindings(&self) -> surreal_orm::BindingsList {
            self.___________bindings.to_vec()
        }
    }
    impl surreal_orm::Buildable for &VisitsWithExplicitAttributes {
        fn build(&self) -> ::std::string::String {
            self.___________graph_traversal_string.to_string()
        }
    }
    impl surreal_orm::Erroneous for &VisitsWithExplicitAttributes {
        fn get_errors(&self) -> ::std::vec::Vec<::std::string::String> {
            self.___________errors.to_vec()
        }
    }
    impl VisitsWithExplicitAttributes {
        pub fn new() -> Self {
            Self {
                id: "id".into(),
                in_: "in".into(),
                out: "out".into(),
                name: "name".into(),
                age: "age".into(),
                created: "created".into(),
                lifeExpectancy: "lifeExpectancy".into(),
                territoryArea: "territoryArea".into(),
                home: "home".into(),
                tags: "tags".into(),
                weapon: "weapon".into(),
                spaceShips: "spaceShips".into(),
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
                age: surreal_orm::Field::new(format!("{}.{}", prefix.build(), "age"))
                    .with_bindings(prefix.get_bindings())
                    .into(),
                created: surreal_orm::Field::new(format!("{}.{}", prefix.build(), "created"))
                    .with_bindings(prefix.get_bindings())
                    .into(),
                lifeExpectancy: surreal_orm::Field::new(format!(
                    "{}.{}",
                    prefix.build(),
                    "lifeExpectancy"
                ))
                .with_bindings(prefix.get_bindings())
                .into(),
                territoryArea: surreal_orm::Field::new(format!(
                    "{}.{}",
                    prefix.build(),
                    "territoryArea"
                ))
                .with_bindings(prefix.get_bindings())
                .into(),
                home: surreal_orm::Field::new(format!("{}.{}", prefix.build(), "home"))
                    .with_bindings(prefix.get_bindings())
                    .into(),
                tags: surreal_orm::Field::new(format!("{}.{}", prefix.build(), "tags"))
                    .with_bindings(prefix.get_bindings())
                    .into(),
                weapon: surreal_orm::Field::new(format!("{}.{}", prefix.build(), "weapon"))
                    .with_bindings(prefix.get_bindings())
                    .into(),
                spaceShips: surreal_orm::Field::new(format!("{}.{}", prefix.build(), "spaceShips"))
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
                age: "".into(),
                created: "".into(),
                lifeExpectancy: "".into(),
                territoryArea: "".into(),
                home: "".into(),
                tags: "".into(),
                weapon: "".into(),
                spaceShips: "".into(),
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
            schema_instance.age = schema_instance
                .age
                .set_graph_string(format!("{}.{}", ___________graph_traversal_string, "age"))
                .____________update_many_bindings(bindings)
                .into();
            schema_instance.created = schema_instance
                .created
                .set_graph_string(format!(
                    "{}.{}",
                    ___________graph_traversal_string, "created"
                ))
                .____________update_many_bindings(bindings)
                .into();
            schema_instance.lifeExpectancy = schema_instance
                .lifeExpectancy
                .set_graph_string(format!(
                    "{}.{}",
                    ___________graph_traversal_string, "lifeExpectancy"
                ))
                .____________update_many_bindings(bindings)
                .into();
            schema_instance.territoryArea = schema_instance
                .territoryArea
                .set_graph_string(format!(
                    "{}.{}",
                    ___________graph_traversal_string, "territoryArea"
                ))
                .____________update_many_bindings(bindings)
                .into();
            schema_instance.home = schema_instance
                .home
                .set_graph_string(format!("{}.{}", ___________graph_traversal_string, "home"))
                .____________update_many_bindings(bindings)
                .into();
            schema_instance.tags = schema_instance
                .tags
                .set_graph_string(format!("{}.{}", ___________graph_traversal_string, "tags"))
                .____________update_many_bindings(bindings)
                .into();
            schema_instance.weapon = schema_instance
                .weapon
                .set_graph_string(format!(
                    "{}.{}",
                    ___________graph_traversal_string, "weapon"
                ))
                .____________update_many_bindings(bindings)
                .into();
            schema_instance.spaceShips = schema_instance
                .spaceShips
                .set_graph_string(format!(
                    "{}.{}",
                    ___________graph_traversal_string, "spaceShips"
                ))
                .____________update_many_bindings(bindings)
                .into();
            schema_instance
        }
        pub fn tags(&self, clause: impl Into<surreal_orm::NodeAliasClause>) -> surreal_orm::Field {
            let clause: surreal_orm::NodeAliasClause = clause.into();
            let clause: surreal_orm::NodeClause = clause.into_inner();
            let normalized_field_name_str = if self.build().is_empty() {
                "tags".to_string()
            } else {
                format!(".{}", "tags")
            };
            let clause: surreal_orm::NodeClause = clause.into();
            let bindings = self
                .get_bindings()
                .into_iter()
                .chain(clause.get_bindings().into_iter())
                .collect::<Vec<_>>();
            let errors = self
                .get_errors()
                .into_iter()
                .chain(clause.get_errors().into_iter())
                .collect::<Vec<_>>();
            let field =
                surreal_orm::Field::new(format!("{normalized_field_name_str}{}", clause.build()))
                    .with_bindings(bindings)
                    .with_errors(errors);
            field
        }
        pub fn weapon(&self) -> Weapon {
            let clause = surreal_orm::Clause::from(surreal_orm::Empty);
            let normalized_field_name_str = if self.build().is_empty() {
                "weapon".to_string()
            } else {
                format!(".{}", "weapon")
            };
            Weapon::__________connect_node_to_graph_traversal_string(
                self,
                clause.with_field(normalized_field_name_str),
            )
        }
        pub fn spaceShips(&self, clause: impl Into<surreal_orm::NodeAliasClause>) -> SpaceShip {
            let clause: surreal_orm::NodeAliasClause = clause.into();
            let clause: surreal_orm::NodeClause = clause.into_inner();
            let normalized_field_name_str = if self.build().is_empty() {
                "spaceShips".to_string()
            } else {
                format!(".{}", "spaceShips")
            };
            SpaceShip::__________connect_node_to_graph_traversal_string(
                self,
                clause.with_field(normalized_field_name_str),
            )
        }
    }
}
#[allow(non_snake_case)]
fn test_visitswithexplicitattributes_edge_name() {
    surreal_orm::validators::assert_impl_one!(String: ::std::convert::Into< ::std::string::String>);
    surreal_orm::validators::is_int::<u8>();
    surreal_orm::validators::assert_impl_one!(DateTime<Utc> : ::std::convert::Into<surreal_orm::sql::Datetime>);
    surreal_orm::validators::assert_impl_one!(Duration: ::std::convert::Into<surreal_orm::sql::Duration>);
    surreal_orm::validators::assert_impl_one!(geo::Polygon: ::std::convert::Into<surreal_orm::sql::Geometry>);
    surreal_orm::validators::assert_impl_one!(geo::Point: ::std::convert::Into<surreal_orm::sql::Geometry>);
    surreal_orm::validators::assert_is_vec::<Vec<String>>();
    surreal_orm::validators::assert_type_eq_all!(LinkOne<Weapon>, surreal_orm::LinkOne<Weapon>);
    surreal_orm::validators::assert_impl_one!(Weapon:surreal_orm::Node);
    type IweaponRefChecker = <Weapon as surreal_orm::Node>::TableNameChecker;
    surreal_orm::validators::assert_fields!(IweaponRefChecker:weapon);
    surreal_orm::validators::assert_type_eq_all!(
        LinkMany<SpaceShip>,
        surreal_orm::LinkMany<SpaceShip>
    );
    surreal_orm::validators::assert_impl_one!(SpaceShip:surreal_orm::Node);
    type IspaceShipsRefChecker = <SpaceShip as surreal_orm::Node>::TableNameChecker;
    surreal_orm::validators::assert_fields!(IspaceShipsRefChecker:space_ship);
    surreal_orm::validators::assert_is_vec::<LinkMany<SpaceShip>>();
}
