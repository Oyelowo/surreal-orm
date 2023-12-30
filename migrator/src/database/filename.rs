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
    path::{Path, PathBuf},
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
    basename: Basename,
}

#[derive(Debug, Clone, Hash)]
pub enum MigrationFilename {
    Up(MigrationNameBasicInfo),
    Down(MigrationNameBasicInfo),
    Unidirectional(MigrationNameBasicInfo),
}

#[derive(Debug, Clone)]
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

    pub fn bidirectional_pair_meta_sorted_desc_checked(
        &self,
        migration_dir: &Path,
    ) -> MigrationResult<Vec<MigrationFileTwoWayPair>> {
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

            bidirectional_pair.push(MigrationFileTwoWayPair {
                up: FileMetadata {
                    name: migration.to_up(),
                    content: up,
                },
                down: FileMetadata {
                    name: migration.to_down(),
                    content: down,
                },
            });
        }

        bidirectional_pair.sort_by(|a, b| b.up.name.cmp(&a.up.name));
        bidirectional_pair.dedup_by(|a, b| a.up.name == b.up.name);

        Ok(bidirectional_pair)
    }

    pub fn bidirectional_pair_meta_down_sorted_desc_unchecked(
        &self,
        migration_dir: &Path,
    ) -> MigrationResult<Vec<MigrationFileTwoWayPair>> {
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

            bidirectional_pair.push(MigrationFileTwoWayPair {
                up: FileMetadata {
                    name: migration.to_up(),
                    content: up,
                },
                down: FileMetadata {
                    name: migration.to_down(),
                    content: down,
                },
            });
        }

        bidirectional_pair.sort_by(|a, b| b.up.name.cmp(&a.up.name));
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
    ) -> MigrationResult<Vec<MigrationFileOneWay>> {
        let unidirectional = self.unidirectional();

        let mut unidirectional_pair: Vec<MigrationFileOneWay> = Vec::new();
        for migration in unidirectional {
            let up = migration_dir.join(migration.to_unidirectional().to_string());

            let content = FileContent::from_file(&up).map_err(|e| {
                MigrationError::MigrationFilePathDoesNotExist {
                    path: up.to_string_lossy().to_string(),
                    error: e.to_string(),
                }
            })?;

            unidirectional_pair.push(MigrationFileOneWay::new(FileMetadata {
                name: migration,
                content,
            }));
        }

        unidirectional_pair.sort_by(|a, b| a.file_meta().name.cmp(&b.file_meta().name));
        unidirectional_pair.dedup_by(|a, b| a.file_meta().name.eq(&b.file_meta().name));

        Ok(unidirectional_pair)
    }
}

impl PartialEq for MigrationFilename {
    fn eq(&self, other: &Self) -> bool {
        self.timestamp() == other.timestamp()
            && self.basename() == other.basename()
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
pub struct Extension(String);

impl From<String> for Extension {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl Display for Extension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Hash, Debug, PartialEq, Ord, PartialOrd, Eq)]
pub struct Basename(String);

impl Basename {
    pub fn normalize_ensure(&self) -> Basename {
        let name = self
            .to_string()
            .split(|c: char| c != '_' && !c.is_alphanumeric())
            .collect::<Vec<_>>()
            .join("_");

        name.into()
    }
}

impl From<&'static str> for Basename {
    fn from(value: &'static str) -> Self {
        Self(value.to_string())
    }
}
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
            MigrationFilename::Up(MigrationNameBasicInfo {
                timestamp,
                basename: name,
            }) => {
                format!("{timestamp}_{name}.up.surql")
            }
            MigrationFilename::Down(MigrationNameBasicInfo {
                timestamp,
                basename: name,
            }) => {
                format!("{timestamp}_{name}.down.surql")
            }
            MigrationFilename::Unidirectional(MigrationNameBasicInfo {
                timestamp,
                basename: name,
            }) => {
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
            MigrationFilename::Up(MigrationNameBasicInfo { basename: name, .. }) => {
                format!("{name}")
            }
            MigrationFilename::Down(MigrationNameBasicInfo { basename: name, .. }) => {
                format!("{name}")
            }
            MigrationFilename::Unidirectional(MigrationNameBasicInfo {
                basename: name, ..
            }) => {
                format!("{name}")
            }
        }
        .into()
    }

    pub fn to_up(&self) -> MigrationFilename {
        match self {
            MigrationFilename::Up(_) => self.clone(),
            MigrationFilename::Down(MigrationNameBasicInfo {
                timestamp,
                basename: name,
            }) => MigrationFilename::Up(MigrationNameBasicInfo {
                timestamp: *timestamp,
                basename: name.clone(),
            }),
            MigrationFilename::Unidirectional(MigrationNameBasicInfo {
                timestamp,
                basename: name,
            }) => MigrationFilename::Up(MigrationNameBasicInfo {
                timestamp: *timestamp,
                basename: name.clone(),
            }),
        }
    }

    pub fn to_down(&self) -> MigrationFilename {
        match self {
            MigrationFilename::Up(MigrationNameBasicInfo {
                timestamp,
                basename: name,
            }) => MigrationFilename::Down(MigrationNameBasicInfo {
                timestamp: *timestamp,
                basename: name.clone(),
            }),
            MigrationFilename::Down(_) => self.clone(),
            MigrationFilename::Unidirectional(MigrationNameBasicInfo {
                timestamp,
                basename: name,
            }) => MigrationFilename::Down(MigrationNameBasicInfo {
                timestamp: *timestamp,
                basename: name.clone(),
            }),
        }
    }

    pub fn to_unidirectional(&self) -> MigrationFilename {
        match self {
            MigrationFilename::Up(MigrationNameBasicInfo {
                timestamp,
                basename: name,
            }) => MigrationFilename::Unidirectional(MigrationNameBasicInfo {
                timestamp: *timestamp,
                basename: name.clone(),
            }),
            MigrationFilename::Down(MigrationNameBasicInfo {
                timestamp,
                basename: name,
            }) => MigrationFilename::Unidirectional(MigrationNameBasicInfo {
                timestamp: *timestamp,
                basename: name.clone(),
            }),
            MigrationFilename::Unidirectional(_) => self.clone(),
        }
    }

    pub fn create_file(
        &self,
        query: &FileContent,
        file_namager: &MigrationConfig,
    ) -> MigrationResult<()> {
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

    pub fn create_up(timestamp: DateTime<Utc>, basename: &Basename) -> MigrationResult<Self> {
        let timestamp = Self::format_timestamp(timestamp.into())?;
        let basename = basename.clone();

        Ok(Self::Up(MigrationNameBasicInfo {
            timestamp,
            basename,
        }))
    }

    pub fn create_down(timestamp: DateTime<Utc>, basename: &Basename) -> MigrationResult<Self> {
        let timestamp = Self::format_timestamp(timestamp.into())?;

        let basename = basename.clone();
        Ok(Self::Down(MigrationNameBasicInfo {
            timestamp,
            basename,
        }))
    }

    pub fn create_oneway(timestamp: DateTime<Utc>, basename: &Basename) -> MigrationResult<Self> {
        let timestamp = Self::format_timestamp(timestamp.into())?;
        let basename = basename.clone();

        Ok(Self::Unidirectional(MigrationNameBasicInfo {
            timestamp,
            basename,
        }))
    }

    fn format_timestamp(timestamp: DateTime<Utc>) -> MigrationResult<u64> {
        let timestamp = timestamp
            .format("%Y%m%d%H%M%S%3f")
            .to_string()
            .parse::<u64>()
            .map_err(|e| MigrationError::InvalidTimestamp(e.to_string()))?;
        Ok(timestamp)
    }

    pub fn fullpath(&self, migration_dir: &PathBuf) -> PathBuf {
        let file_name = self.to_string();
        migration_dir.join(file_name)
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
    let (input, (basename, direction)) =
        tuple((take_while1(is_valid_migration_identifier), parse_direction))(input)?;
    let basic_info = MigrationNameBasicInfo {
        timestamp,
        basename: basename.to_string().into(),
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
            MigrationFilename::Up(MigrationNameBasicInfo {
                timestamp,
                basename: name,
            }) => {
                format!("{timestamp}_{name}.up.surql")
            }
            MigrationFilename::Down(MigrationNameBasicInfo {
                timestamp,
                basename: name,
            }) => {
                format!("{timestamp}_{name}.down.surql")
            }
            MigrationFilename::Unidirectional(MigrationNameBasicInfo {
                timestamp,
                basename: name,
            }) => {
                format!("{timestamp}_{name}.surql")
            }
        };
        write!(f, "{file_name_str}")
    }
}
