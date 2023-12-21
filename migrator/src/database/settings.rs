use clap::Parser;
use std::{fmt::Display, str::FromStr};

use crate::MigrationError;

#[derive(Debug, Clone)]
pub enum Direction {
    Up,
    Down,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let direction = match self {
            Self::Up => "up",
            Self::Down => "down",
        };
        write!(f, "{direction}")
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub enum MigrationFlag {
    #[default]
    TwoWay,
    OneWay,
}

impl Display for MigrationFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let flag = match self {
            Self::TwoWay => "two_way",
            Self::OneWay => "one_way",
        };
        write!(f, "{flag}")
    }
}

impl MigrationFlag {
    pub fn is_twoway(&self) -> bool {
        matches!(self, Self::TwoWay)
    }

    pub fn options() -> Vec<String> {
        vec![Self::TwoWay.to_string(), Self::OneWay.to_string()]
    }
}

impl TryFrom<String> for MigrationFlag {
    type Error = MigrationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "two_way" => Ok(Self::TwoWay),
            "one_way" => Ok(Self::OneWay),
            _ => Err(MigrationError::InvalidMigrationFlag(
                value,
                Self::options().join(", "),
            )),
        }
    }
}

#[derive(Parser, Default, Debug, Clone, Copy, PartialEq)]
pub enum Mode {
    #[default]
    Strict,
    Lax,
}

impl FromStr for Mode {
    type Err = MigrationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "strict" => Ok(Self::Strict),
            "lax" => Ok(Self::Lax),
            _ => Err(MigrationError::InvalidMigrationMode(
                s.to_string(),
                Self::options().join(", "),
            )),
        }
    }
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mode = match self {
            Self::Strict => "strict",
            Self::Lax => "lax",
        };
        write!(f, "{mode}")
    }
}

impl Mode {
    pub fn options() -> Vec<String> {
        vec![Self::Strict.to_string(), Self::Lax.to_string()]
    }

    pub fn is_strict(&self) -> bool {
        matches!(self, Self::Strict)
    }

    pub fn is_relaxed(&self) -> bool {
        matches!(self, Self::Lax)
    }
}
