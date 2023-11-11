/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::{
    fmt::Display,
    fs::{self, File},
    io::Write,
};

use chrono::{DateTime, Utc};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1, take_while_m_n},
    combinator::{all_consuming, cut, map_res},
    error::context,
    sequence::tuple,
    IResult,
};

use crate::*;

#[derive(Debug, Clone)]
pub struct MigrationNameBasicInfo {
    timestamp: u64,
    name: String,
}

#[derive(Debug, Clone)]
pub enum MigrationFileName {
    Up(MigrationNameBasicInfo),
    Down(MigrationNameBasicInfo),
    Unidirectional(MigrationNameBasicInfo),
}

impl MigrationFileName {
    pub fn filename(&self) -> String {
        match self {
            MigrationFileName::Up(MigrationNameBasicInfo { timestamp, name }) => {
                format!("{timestamp}_{name}.up.surql")
            }
            MigrationFileName::Down(MigrationNameBasicInfo { timestamp, name }) => {
                format!("{timestamp}_{name}.down.surql")
            }
            MigrationFileName::Unidirectional(MigrationNameBasicInfo { timestamp, name }) => {
                format!("{timestamp}_{name}.surql")
            }
        }
    }

    pub fn timestamp(&self) -> u64 {
        match self {
            MigrationFileName::Up(MigrationNameBasicInfo { timestamp, .. }) => *timestamp,
            MigrationFileName::Down(MigrationNameBasicInfo { timestamp, .. }) => *timestamp,
            MigrationFileName::Unidirectional(MigrationNameBasicInfo { timestamp, .. }) => {
                *timestamp
            }
        }
    }

    /// just the file name without extension nor timestamp
    pub fn simple_name(&self) -> String {
        match self {
            MigrationFileName::Up(MigrationNameBasicInfo { name, .. }) => name.clone(),
            MigrationFileName::Down(MigrationNameBasicInfo { name, .. }) => name.clone(),
            MigrationFileName::Unidirectional(MigrationNameBasicInfo { name, .. }) => name.clone(),
        }
    }

    pub fn extension(&self) -> String {
        match self {
            MigrationFileName::Up(_) => "up.surql".to_string(),
            MigrationFileName::Down(_) => "down.surql".to_string(),
            MigrationFileName::Unidirectional(_) => "surql".to_string(),
        }
    }

    pub fn basename(&self) -> String {
        match self {
            MigrationFileName::Up(MigrationNameBasicInfo { timestamp, name }) => {
                format!("{timestamp}_{name}")
            }
            MigrationFileName::Down(MigrationNameBasicInfo { timestamp, name }) => {
                format!("{timestamp}_{name}")
            }
            MigrationFileName::Unidirectional(MigrationNameBasicInfo { timestamp, name }) => {
                format!("{timestamp}_{name}")
            }
        }
    }

    pub fn to_up(&self) -> MigrationFileName {
        match self {
            MigrationFileName::Up(_) => self.clone(),
            MigrationFileName::Down(MigrationNameBasicInfo { timestamp, name }) => {
                MigrationFileName::Up(MigrationNameBasicInfo {
                    timestamp: *timestamp,
                    name: name.clone(),
                })
            }
            MigrationFileName::Unidirectional(MigrationNameBasicInfo { timestamp, name }) => {
                MigrationFileName::Up(MigrationNameBasicInfo {
                    timestamp: *timestamp,
                    name: name.clone(),
                })
            }
        }
    }

    pub fn to_down(&self) -> MigrationFileName {
        match self {
            MigrationFileName::Up(MigrationNameBasicInfo { timestamp, name }) => {
                MigrationFileName::Down(MigrationNameBasicInfo {
                    timestamp: *timestamp,
                    name: name.clone(),
                })
            }
            MigrationFileName::Down(_) => self.clone(),
            MigrationFileName::Unidirectional(MigrationNameBasicInfo { timestamp, name }) => {
                MigrationFileName::Down(MigrationNameBasicInfo {
                    timestamp: *timestamp,
                    name: name.clone(),
                })
            }
        }
    }

    pub fn to_unidirectional(&self) -> MigrationFileName {
        match self {
            MigrationFileName::Up(MigrationNameBasicInfo { timestamp, name }) => {
                MigrationFileName::Unidirectional(MigrationNameBasicInfo {
                    timestamp: *timestamp,
                    name: name.clone(),
                })
            }
            MigrationFileName::Down(MigrationNameBasicInfo { timestamp, name }) => {
                MigrationFileName::Unidirectional(MigrationNameBasicInfo {
                    timestamp: *timestamp,
                    name: name.clone(),
                })
            }
            MigrationFileName::Unidirectional(_) => self.clone(),
        }
    }

    pub fn create_file(&self, query: String, file_namager: &FileManager) -> MigrationResult<()> {
        let file_name = self.to_string();
        let migration_dir = file_namager.resolve_migration_directory(true)?;
        let file_path = migration_dir.join(file_name);

        // Ensure the migrations directory exists
        if let Err(err) = fs::create_dir_all(migration_dir) {
            return Err(MigrationError::IoError(format!(
                "Failed to create migrations directory: {}",
                err
            )));
        }

        let mut file = File::create(&file_path).map_err(|e| {
            MigrationError::IoError(format!(
                "Failed to create file path: {}. Error: {}",
                file_path.to_string_lossy(),
                e
            ))
        })?;

        file.write_all(query.as_bytes()).map_err(|e| {
            MigrationError::IoError(format!(
                "Failed to create file. Filename - {}: {}",
                file_path.to_string_lossy(),
                e
            ))
        })?;

        Ok(())
    }

    pub fn create_up(timestamp: DateTime<Utc>, name: &String) -> MigrationResult<Self> {
        let timestamp = Self::format_timestamp(timestamp)?;

        let name = name.to_string();
        // let timestamp = Utc::now().timestamp_millis();
        Ok(Self::Up(MigrationNameBasicInfo { timestamp, name }))
    }

    pub fn create_down(timestamp: DateTime<Utc>, name: impl Into<String>) -> MigrationResult<Self> {
        let timestamp = Self::format_timestamp(timestamp)?;

        let name = name.into();
        Ok(Self::Down(MigrationNameBasicInfo { timestamp, name }))
    }

    pub fn create_oneway(
        timestamp: DateTime<Utc>,
        name: impl Into<String>,
    ) -> MigrationResult<Self> {
        let timestamp = Self::format_timestamp(timestamp)?;

        let name = name.into();
        Ok(Self::Unidirectional(MigrationNameBasicInfo {
            timestamp,
            name,
        }))
    }

    fn format_timestamp(timestamp: DateTime<Utc>) -> Result<u64, MigrationError> {
        let timestamp = timestamp
            .format("%Y%m%d%H%M%S")
            .to_string()
            .parse::<u64>()
            .map_err(|e| MigrationError::InvalidTimestamp(e.to_string()))?;
        Ok(timestamp)
    }
}

// parse_migration_name
impl TryFrom<String> for MigrationFileName {
    type Error = MigrationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let (_, migration_name) = parse_migration_name(value.clone().as_str())
            .map_err(|_e| MigrationError::InvalidMigrationName(value))?;
        Ok(migration_name)
    }
}

#[derive(Debug, Clone)]
enum Direction {
    Up,
    Down,
    OneWay,
}
// .up.surql or .down.surql or .surql
fn parse_direction(input: &str) -> IResult<&str, Direction> {
    use nom::combinator::value;

    let (input, direction) = alt((
        value(Direction::Up, tag(".up.surql")),
        value(Direction::Down, tag(".down.surql")),
        value(Direction::OneWay, tag(".surql")),
    ))(input)?;
    Ok((input, direction))
}

fn is_valid_migration_identifier(c: char) -> bool {
    c.is_alphanumeric() || c == '_' || c == '-'
}

fn parse_u64(input: &str) -> Result<u64, std::num::ParseIntError> {
    input.parse()
}

// format: <timestamp>_<name>.<direction>.surql
// 14 numbers followed by _ and then name of migration
fn parse_migration_name_unconsumed(input: &str) -> IResult<&str, MigrationFileName> {
    let (input, timestamp) = map_res(
        take_while_m_n(14, 14, |c: char| c.is_ascii_digit()),
        parse_u64,
    )(input)?;
    let (input, _) = tag("_")(input)?;
    let (input, (name, direction)) =
        tuple((take_while1(is_valid_migration_identifier), parse_direction))(input)?;
    let basic_info = MigrationNameBasicInfo {
        timestamp,
        name: name.to_string(),
    };

    let m2 = match direction {
        Direction::Up => MigrationFileName::Up(basic_info),
        Direction::Down => MigrationFileName::Down(basic_info),
        Direction::OneWay => MigrationFileName::Unidirectional(basic_info),
    };

    Ok((input, m2))
}

fn parse_migration_name(input: &str) -> IResult<&str, MigrationFileName> {
    all_consuming(cut(context(
        "Unexpected characters found after parsing",
        parse_migration_name_unconsumed,
    )))(input)
}

impl Display for MigrationFileName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let file_name_str = match self {
            MigrationFileName::Up(MigrationNameBasicInfo { timestamp, name }) => {
                format!("{timestamp}_{name}.up.surql")
            }
            MigrationFileName::Down(MigrationNameBasicInfo { timestamp, name }) => {
                format!("{timestamp}_{name}.down.surql")
            }
            MigrationFileName::Unidirectional(MigrationNameBasicInfo { timestamp, name }) => {
                format!("{timestamp}_{name}.surql")
            }
        };
        write!(f, "{file_name_str}")
    }
}
