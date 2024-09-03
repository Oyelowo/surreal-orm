/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */

#![allow(missing_docs)]
use std::fmt::{self, Display};

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_type_display() {
        assert_eq!(format!("{}", TokenType::EDDSA), "EDDSA");
        assert_eq!(format!("{}", TokenType::ES256), "ES256");
        assert_eq!(format!("{}", TokenType::ES384), "ES384");
        assert_eq!(format!("{}", TokenType::ES512), "ES512");
        assert_eq!(format!("{}", TokenType::HS256), "HS256");
        assert_eq!(format!("{}", TokenType::HS384), "HS384");
        assert_eq!(format!("{}", TokenType::HS512), "HS512");
        assert_eq!(format!("{}", TokenType::PS256), "PS256");
        assert_eq!(format!("{}", TokenType::PS384), "PS384");
        assert_eq!(format!("{}", TokenType::PS512), "PS512");
        assert_eq!(format!("{}", TokenType::RS256), "RS256");
        assert_eq!(format!("{}", TokenType::RS384), "RS384");
        assert_eq!(format!("{}", TokenType::RS512), "RS512");
    }

    #[test]
    fn test_token_target_display() {
        assert_eq!(format!("{}", TokenTarget::Namespace), "NAMESPACE");
        assert_eq!(format!("{}", TokenTarget::Database), "DATABASE");
        assert_eq!(
            format!("{}", TokenTarget::Scope("test".into())),
            "SCOPE test"
        );
    }
}
