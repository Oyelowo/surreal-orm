use std::{
    fmt::{self, Display},
    ops::Deref,
};

use surrealdb::sql;

use crate::{field::Binding, statements::SelectStatement, BindingsList, Field, Parametric};

pub struct Namespace(sql::Idiom);
pub struct Database(sql::Idiom);
pub struct Login(sql::Idiom);
pub struct Token(sql::Idiom);
pub struct Scope(sql::Idiom);
pub struct Table(sql::Table);
pub struct Event(sql::Idiom);
pub struct Index(sql::Idiom);

impl Table {
    pub fn new(name: impl Into<sql::Table>) -> Self {
        Self(name.into())
    }
}
macro_rules! impl_new_for_all {
    ($($types_:ty),*) => {
        $(
        impl $types_ {
            pub fn new(name: impl Into<String>) -> Self {
                Self(name.into().into())
            }
        }
    )*
    };
}

impl_new_for_all!(Namespace, Database, Login, Token, Scope, Event, Index);

macro_rules! impl_display_for_all {
    ($($types_:ty),*) => {
        $(
        impl Display for $types_ {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
        impl From<$types_> for String {
            fn from(value: $types_) -> Self {
                let value: String = value.0.to_string();
                value
            }
        }
        impl From<&str> for $types_ {
            fn from(value: &str) -> Self {
                Self(value.to_string().into())
            }
        }

        impl From<String> for $types_ {
            fn from(value: String) -> Self {
                Self(value.into())
            }
        }

        impl From<$types_> for sql::Value {
            fn from(value: $types_) -> Self {
                value.0.into()
            }
        }

    //     impl<T> From<T> for $types_
    //     where
    //         T: Into<String>,
    //     {
    //         fn from(value: T) -> Self {
    //             Self(value.into().into())
    //         }
    // }
    )*
    };
}
impl_display_for_all!(Namespace, Database, Login, Token, Scope, Table, Event, Index);

enum NamespaceOrDatabase {
    Namespace,
    Database,
}

impl Display for NamespaceOrDatabase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let stringified = match self {
            NamespaceOrDatabase::Namespace => "NAMESPACE",
            NamespaceOrDatabase::Database => "DATABASE",
        };
        write!(f, "{}", stringified)
    }
}

pub enum TokenType {
    EDDSA,
    ES256,
    ES384,
    ES512,
    HS256,
    HS384,
    HS512,
    PS256,
    PS384,
    PS512,
    RS256,
    RS384,
    RS512,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenType::EDDSA => write!(f, "EDDSA"),
            TokenType::ES256 => write!(f, "ES256"),
            TokenType::ES384 => write!(f, "ES384"),
            TokenType::ES512 => write!(f, "ES512"),
            TokenType::HS256 => write!(f, "HS256"),
            TokenType::HS384 => write!(f, "HS384"),
            TokenType::HS512 => write!(f, "HS512"),
            TokenType::PS256 => write!(f, "PS256"),
            TokenType::PS384 => write!(f, "PS384"),
            TokenType::PS512 => write!(f, "PS512"),
            TokenType::RS256 => write!(f, "RS256"),
            TokenType::RS384 => write!(f, "RS384"),
            TokenType::RS512 => write!(f, "RS512"),
        }
    }
}

pub enum TokenTarget {
    Namespace,
    Database,
    Scope(String),
}

impl Display for TokenTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let target_str = match self {
            TokenTarget::Namespace => "NAMESPACE".into(),
            TokenTarget::Database => "DATABASE".into(),
            TokenTarget::Scope(scope) => format!("SCOPE {}", scope),
        };
        write!(f, "{}", target_str)
    }
}

pub struct Name(sql::Idiom);

impl Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.to_string())
    }
}

impl Name {
    pub fn new(name: sql::Idiom) -> Self {
        Self(name)
    }
}

// impl From<sql::Idiom> for Name {
//     fn from(value: sql::Idiom) -> Self {
//         todo!()
//     }
// }

impl From<Name> for sql::Idiom {
    fn from(value: Name) -> Self {
        value.0
    }
}

impl From<Name> for sql::Value {
    fn from(value: Name) -> Self {
        value.0.into()
    }
}

impl<T> From<T> for Name
where
    T: Into<String>,
{
    fn from(value: T) -> Self {
        Self(value.into().into())
    }
}

pub struct Duration(sql::Duration);

impl From<self::Duration> for sql::Duration {
    fn from(value: self::Duration) -> Self {
        value.0
    }
}

impl From<Duration> for sql::Value {
    fn from(value: self::Duration) -> Self {
        value.0.into()
    }
}
impl From<sql::Duration> for self::Duration {
    fn from(value: sql::Duration) -> Self {
        Self(value)
    }
}

impl From<&std::time::Duration> for Duration {
    fn from(value: &std::time::Duration) -> Self {
        Self(value.to_owned().into())
    }
}

impl From<std::time::Duration> for Duration {
    fn from(value: std::time::Duration) -> Self {
        Self(value.into())
    }
}

impl Deref for Duration {
    type Target = sql::Duration;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone)]
pub enum Expression {
    SelectStatement(SelectStatement),
    Value(sql::Value),
}

impl Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let expression = match self {
            Expression::SelectStatement(s) => format!("({s})"),
            // Expression::SelectStatement(s) => s.get_bindings().first().unwrap().get_raw(),
            Expression::Value(v) => {
                let bindings = self.get_bindings();
                assert_eq!(bindings.len(), 1);
                format!("{}", self.get_bindings().first().expect("Param must have been generated for value. This is a bug. Please report here: ").get_param())
            }
        };
        write!(f, "{}", expression)
    }
}

impl Parametric for Expression {
    fn get_bindings(&self) -> BindingsList {
        match self {
            Expression::SelectStatement(s) => s
                .get_bindings()
                .into_iter()
                // query must have already been built and bound
                .map(|b| b.with_raw(format!("({s})")))
                .collect::<_>(),
            Expression::Value(sql_value) => {
                // let sql_value = sql::json(&serde_json::to_string(&v).unwrap()).unwrap();
                let sql_value: sql::Value = sql_value.to_owned();
                vec![Binding::new(sql_value.clone()).with_raw(sql_value.to_raw_string())]
            }
        }
    }
}

impl From<SelectStatement> for Expression {
    fn from(value: SelectStatement) -> Self {
        Self::SelectStatement(value)
    }
}

impl<T: Into<sql::Value>> From<T> for Expression {
    fn from(value: T) -> Self {
        Self::Value(value.into())
    }
}

#[derive(Debug)]
pub enum Return {
    None,
    Before,
    After,
    Diff,
    Projections(Vec<Field>),
}

impl From<Vec<&Field>> for Return {
    fn from(value: Vec<&Field>) -> Self {
        Self::Projections(value.into_iter().map(ToOwned::to_owned).collect::<Vec<_>>())
    }
}

impl From<Vec<Field>> for Return {
    fn from(value: Vec<Field>) -> Self {
        Self::Projections(value)
    }
}

impl<const N: usize> From<&[Field; N]> for Return {
    fn from(value: &[Field; N]) -> Self {
        Self::Projections(value.to_vec())
    }
}

impl<const N: usize> From<&[&Field; N]> for Return {
    fn from(value: &[&Field; N]) -> Self {
        Self::Projections(
            value
                .to_vec()
                .into_iter()
                .map(ToOwned::to_owned)
                .collect::<Vec<_>>(),
        )
    }
}
