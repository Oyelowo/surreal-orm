use crate::*;
use std::ops::Deref;

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct MigrationFileTwoWayPair {
    pub up: FileMetadata,
    pub down: FileMetadata,
}

impl MigrationFileTwoWayPair {
    pub fn new(up: FileMetadata, down: FileMetadata) -> Self {
        Self { up, down }
    }
}

impl TryFrom<MigrationFileTwoWayPair> for Migration {
    type Error = MigrationError;

    fn try_from(migration: MigrationFileTwoWayPair) -> Result<Self, Self::Error> {
        Ok(Self {
            id: Migration::create_id(&migration.up.name),
            name: migration.up.name.to_up().to_string(),
            timestamp: migration.up.name.timestamp(),
            checksum_up: migration.up.content.as_checksum()?,
            checksum_down: Some(migration.down.content.as_checksum()?),
        })
    }
}

#[derive(Clone, Debug)]
pub struct MigrationFileOneWay(FileMetadata);

impl MigrationFileOneWay {
    pub fn new(file: FileMetadata) -> Self {
        Self(file)
    }

    pub fn file_meta(&self) -> &FileMetadata {
        &self.0
    }

    pub fn name(&self) -> &MigrationFilename {
        &self.0.name
    }

    pub fn content(&self) -> &FileContent {
        &self.0.content
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct FileMetadata {
    pub name: MigrationFilename,
    pub content: FileContent, // status: String,
}

impl FileMetadata {
    pub fn new(name: MigrationFilename, content: FileContent) -> Self {
        Self { name, content }
    }
}

impl TryFrom<MigrationFileOneWay> for Migration {
    type Error = MigrationError;

    fn try_from(migration: MigrationFileOneWay) -> Result<Self, Self::Error> {
        let migration = migration.0;
        Ok(Self {
            id: Migration::create_id(&migration.name),
            name: migration.name.to_string(),
            timestamp: migration.name.timestamp(),
            checksum_up: Checksum::generate_from_content(&migration.content)?.into(),
            checksum_down: None,
        })
    }
}

impl From<MigrationFileTwoWayPair> for MigrationFileOneWay {
    fn from(m: MigrationFileTwoWayPair) -> Self {
        Self::new(FileMetadata::new(m.up.name, m.up.content))
    }
}

#[derive(Debug, Clone)]
pub enum MigrationFile {
    OneWay(MigrationFileOneWay),
    TwoWay(MigrationFileTwoWayPair),
}

impl MigrationFile {
    pub fn create_file(&self, file_manager: &MigrationConfig) -> MigrationResult<()> {
        match self {
            Self::OneWay(m) => m.name().create_file(m.content(), file_manager)?,
            Self::TwoWay(m) => {
                m.up.name.create_file(&m.up.content, file_manager)?;
                m.down.name.create_file(&m.down.content, file_manager)?;
            }
        };

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct PendingMigrationFile(MigrationFile);

impl Deref for PendingMigrationFile {
    type Target = MigrationFile;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<PendingMigrationFile> for MigrationFile {
    fn from(m: PendingMigrationFile) -> Self {
        m.0
    }
}

impl From<MigrationFile> for PendingMigrationFile {
    fn from(m: MigrationFile) -> Self {
        Self(m)
    }
}

impl MigrationFile {
    pub fn name_forward(&self) -> &MigrationFilename {
        match self {
            Self::OneWay(m) => &m.name(),
            Self::TwoWay(m) => &m.up.name,
        }
    }

    pub fn up_content(&self) -> &FileContent {
        match self {
            Self::OneWay(m) => &m.content(),
            Self::TwoWay(m) => &m.up.content,
        }
    }

    pub fn down_content(&self) -> Option<&FileContent> {
        match self {
            Self::OneWay(_) => None,
            Self::TwoWay(m) => Some(&m.down.content),
        }
    }
}

impl From<MigrationFileOneWay> for MigrationFile {
    fn from(m: MigrationFileOneWay) -> Self {
        Self::OneWay(m)
    }
}

impl From<MigrationFileTwoWayPair> for MigrationFile {
    fn from(m: MigrationFileTwoWayPair) -> Self {
        Self::TwoWay(m)
    }
}
