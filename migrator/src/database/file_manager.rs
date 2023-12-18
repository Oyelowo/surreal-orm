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
    ops::Deref,
    path::Path,
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

#[derive(Debug, Clone, Hash)]
pub struct MigrationNameBasicInfo {
    timestamp: u64,
    name: String,
}

#[derive(Debug, Clone, Hash)]
pub enum MigrationFilename {
    Up(MigrationNameBasicInfo),
    Down(MigrationNameBasicInfo),
    Unidirectional(MigrationNameBasicInfo),
}

pub struct MigrationFilenames(Vec<MigrationFilename>);

impl Deref for MigrationFilenames {
    type Target = Vec<MigrationFilename>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Vec<MigrationFilename>> for MigrationFilenames {
    fn from(value: Vec<MigrationFilename>) -> Self {
        Self(value)
    }
}

impl MigrationFilenames {
    pub fn all(&self) -> Vec<MigrationFilename> {
        self.0.clone()
    }

    pub fn up(&self) -> Vec<MigrationFilename> {
        self.0
            .iter()
            .filter(|m| matches!(m, MigrationFilename::Up(_)))
            .cloned()
            .collect()
    }

    pub fn down(&self) -> Vec<MigrationFilename> {
        self.0
            .iter()
            .filter(|m| matches!(m, MigrationFilename::Down(_)))
            .cloned()
            .collect()
    }

    pub fn bidirectional(&self) -> Vec<MigrationFilename> {
        self.0
            .iter()
            .filter(|m| {
                matches!(m, MigrationFilename::Up(_)) || matches!(m, MigrationFilename::Down(_))
            })
            .cloned()
            .collect()
    }

    pub fn bidirectional_pair_meta_checked(
        &self,
        migration_dir: &Path,
    ) -> MigrationResult<Vec<MigrationTwoWay>> {
        let bidirectional = self.bidirectional();

        let mut bidirectional_pair = Vec::new();
        for migration in bidirectional {
            let up = migration_dir.join(migration.to_up().to_string());
            let down = migration_dir.join(migration.to_down().to_string());

            let up = FileContent::from_file(&up).map_err(|e| {
                MigrationError::MigrationFilePathDoesNotExist {
                    path: up.to_string_lossy().to_string(),
                    error: e.to_string(),
                }
            })?;
            let down = FileContent::from_file(&down).map_err(|e| {
                MigrationError::MigrationFilePathDoesNotExist {
                    path: down.to_string_lossy().to_string(),
                    error: e.to_string(),
                }
            })?;

            bidirectional_pair.push(MigrationTwoWay {
                name: migration,
                up,
                down,
            });
        }

        bidirectional_pair.sort();
        bidirectional_pair.dedup();

        Ok(bidirectional_pair)
    }

    pub fn bidirectional_pair_meta_down_unchecked(
        &self,
        migration_dir: &Path,
    ) -> MigrationResult<Vec<MigrationTwoWay>> {
        let bidirectional = self.bidirectional();

        let mut bidirectional_pair = Vec::new();
        for migration in bidirectional {
            let up = migration_dir.join(migration.to_up().to_string());
            let down = migration_dir.join(migration.to_down().to_string());

            let up = FileContent::from_file(&up).map_err(|e| {
                MigrationError::MigrationFilePathDoesNotExist {
                    path: up.to_string_lossy().to_string(),
                    error: e.to_string(),
                }
            })?;

            let down = FileContent::from_file(&down).unwrap_or(FileContent::empty());

            bidirectional_pair.push(MigrationTwoWay {
                name: migration,
                up,
                down,
            });
        }

        bidirectional_pair.sort();
        bidirectional_pair.dedup();

        Ok(bidirectional_pair)
    }

    pub fn unidirectional(&self) -> Vec<MigrationFilename> {
        self.0
            .iter()
            .filter(|m| matches!(m, MigrationFilename::Unidirectional(_)))
            .cloned()
            .collect()
    }

    pub fn unidirectional_pair_meta(
        &self,
        migration_dir: &Path,
    ) -> MigrationResult<Vec<MigrationOneWay>> {
        let unidirectional = self.unidirectional();

        let mut unidirectional_pair: Vec<MigrationOneWay> = Vec::new();
        for migration in unidirectional {
            let up = migration_dir.join(migration.to_unidirectional().to_string());

            let content = FileContent::from_file(&up).map_err(|e| {
                MigrationError::MigrationFilePathDoesNotExist {
                    path: up.to_string_lossy().to_string(),
                    error: e.to_string(),
                }
            })?;

            unidirectional_pair.push(MigrationOneWay {
                name: migration,
                content,
            });
        }

        unidirectional_pair.sort_by(|a, b| a.name.cmp(&b.name));
        unidirectional_pair.dedup_by(|a, b| a.name.eq(&b.name));

        Ok(unidirectional_pair)
    }
}

impl PartialEq for MigrationFilename {
    fn eq(&self, other: &Self) -> bool {
        self.timestamp() == other.timestamp()
            && self.simple_name() == other.simple_name()
            && self.extension() == other.extension()
    }
}

impl Ord for MigrationFilename {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.timestamp().cmp(&other.timestamp())
    }
}

impl PartialOrd for MigrationFilename {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for MigrationFilename {}

pub struct Filename(String);

impl From<String> for Filename {
    fn from(value: String) -> Self {
        Self(value)
    }
}

#[derive(Debug, PartialEq)]
pub struct SimpleName(String);

impl Display for SimpleName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl SimpleName {
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl From<String> for SimpleName {
    fn from(value: String) -> Self {
        Self(value)
    }
}

#[derive(Debug, PartialEq)]
pub struct Extension(String);

impl From<String> for Extension {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl Display for Extension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, ".{}", self.0)
    }
}

#[derive(Debug, PartialEq, Ord, PartialOrd, Eq)]
pub struct Basename(String);

impl From<String> for Basename {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl Display for Basename {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl MigrationFilename {
    pub fn filename(&self) -> Filename {
        match self {
            MigrationFilename::Up(MigrationNameBasicInfo { timestamp, name }) => {
                format!("{timestamp}_{name}.up.surql")
            }
            MigrationFilename::Down(MigrationNameBasicInfo { timestamp, name }) => {
                format!("{timestamp}_{name}.down.surql")
            }
            MigrationFilename::Unidirectional(MigrationNameBasicInfo { timestamp, name }) => {
                format!("{timestamp}_{name}.surql")
            }
        }
        .into()
    }

    pub fn timestamp(&self) -> Timestamp {
        match self {
            MigrationFilename::Up(MigrationNameBasicInfo { timestamp, .. }) => *timestamp,
            MigrationFilename::Down(MigrationNameBasicInfo { timestamp, .. }) => *timestamp,
            MigrationFilename::Unidirectional(MigrationNameBasicInfo { timestamp, .. }) => {
                *timestamp
            }
        }
        .into()
    }

    /// just the file name without extension nor timestamp
    pub fn simple_name(&self) -> SimpleName {
        match self {
            MigrationFilename::Up(MigrationNameBasicInfo { name, .. }) => name.clone(),
            MigrationFilename::Down(MigrationNameBasicInfo { name, .. }) => name.clone(),
            MigrationFilename::Unidirectional(MigrationNameBasicInfo { name, .. }) => name.clone(),
        }
        .into()
    }

    pub fn extension(&self) -> Extension {
        match self {
            MigrationFilename::Up(_) => "up.surql".to_string(),
            MigrationFilename::Down(_) => "down.surql".to_string(),
            MigrationFilename::Unidirectional(_) => "surql".to_string(),
        }
        .into()
    }

    pub fn basename(&self) -> Basename {
        match self {
            MigrationFilename::Up(MigrationNameBasicInfo { timestamp, name }) => {
                format!("{timestamp}_{name}")
            }
            MigrationFilename::Down(MigrationNameBasicInfo { timestamp, name }) => {
                format!("{timestamp}_{name}")
            }
            MigrationFilename::Unidirectional(MigrationNameBasicInfo { timestamp, name }) => {
                format!("{timestamp}_{name}")
            }
        }
        .into()
    }

    pub fn to_up(&self) -> MigrationFilename {
        match self {
            MigrationFilename::Up(_) => self.clone(),
            MigrationFilename::Down(MigrationNameBasicInfo { timestamp, name }) => {
                MigrationFilename::Up(MigrationNameBasicInfo {
                    timestamp: *timestamp,
                    name: name.clone(),
                })
            }
            MigrationFilename::Unidirectional(MigrationNameBasicInfo { timestamp, name }) => {
                MigrationFilename::Up(MigrationNameBasicInfo {
                    timestamp: *timestamp,
                    name: name.clone(),
                })
            }
        }
    }

    pub fn to_down(&self) -> MigrationFilename {
        match self {
            MigrationFilename::Up(MigrationNameBasicInfo { timestamp, name }) => {
                MigrationFilename::Down(MigrationNameBasicInfo {
                    timestamp: *timestamp,
                    name: name.clone(),
                })
            }
            MigrationFilename::Down(_) => self.clone(),
            MigrationFilename::Unidirectional(MigrationNameBasicInfo { timestamp, name }) => {
                MigrationFilename::Down(MigrationNameBasicInfo {
                    timestamp: *timestamp,
                    name: name.clone(),
                })
            }
        }
    }

    pub fn to_unidirectional(&self) -> MigrationFilename {
        match self {
            MigrationFilename::Up(MigrationNameBasicInfo { timestamp, name }) => {
                MigrationFilename::Unidirectional(MigrationNameBasicInfo {
                    timestamp: *timestamp,
                    name: name.clone(),
                })
            }
            MigrationFilename::Down(MigrationNameBasicInfo { timestamp, name }) => {
                MigrationFilename::Unidirectional(MigrationNameBasicInfo {
                    timestamp: *timestamp,
                    name: name.clone(),
                })
            }
            MigrationFilename::Unidirectional(_) => self.clone(),
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
        let timestamp = Self::format_timestamp(timestamp.into())?;

        let name = name.to_string();
        Ok(Self::Up(MigrationNameBasicInfo { timestamp, name }))
    }

    pub fn create_down(timestamp: DateTime<Utc>, name: impl Into<String>) -> MigrationResult<Self> {
        let timestamp = Self::format_timestamp(timestamp.into())?;

        let name = name.into();
        Ok(Self::Down(MigrationNameBasicInfo { timestamp, name }))
    }

    pub fn create_oneway(
        timestamp: DateTime<Utc>,
        name: impl Into<String>,
    ) -> MigrationResult<Self> {
        let timestamp = Self::format_timestamp(timestamp.into())?;

        let name = name.into();
        Ok(Self::Unidirectional(MigrationNameBasicInfo {
            timestamp,
            name,
        }))
    }

    fn format_timestamp(timestamp: DateTime<Utc>) -> MigrationResult<u64> {
        let timestamp = timestamp
            .format("%Y%m%d%H%M%S")
            .to_string()
            .parse::<u64>()
            .map_err(|e| MigrationError::InvalidTimestamp(e.to_string()))?;
        Ok(timestamp)
    }
}

// parse_migration_name
impl TryFrom<String> for MigrationFilename {
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
fn parse_migration_name_unconsumed(input: &str) -> IResult<&str, MigrationFilename> {
    let (input, timestamp) = map_res(
        take_while_m_n(14, 20, |c: char| c.is_ascii_digit()),
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
        Direction::Up => MigrationFilename::Up(basic_info),
        Direction::Down => MigrationFilename::Down(basic_info),
        Direction::OneWay => MigrationFilename::Unidirectional(basic_info),
    };

    Ok((input, m2))
}

fn parse_migration_name(input: &str) -> IResult<&str, MigrationFilename> {
    all_consuming(cut(context(
        "Unexpected characters found after parsing",
        parse_migration_name_unconsumed,
    )))(input)
}

impl Display for MigrationFilename {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let file_name_str = match self {
            MigrationFilename::Up(MigrationNameBasicInfo { timestamp, name }) => {
                format!("{timestamp}_{name}.up.surql")
            }
            MigrationFilename::Down(MigrationNameBasicInfo { timestamp, name }) => {
                format!("{timestamp}_{name}.down.surql")
            }
            MigrationFilename::Unidirectional(MigrationNameBasicInfo { timestamp, name }) => {
                format!("{timestamp}_{name}.surql")
            }
        };
        write!(f, "{file_name_str}")
    }
}
