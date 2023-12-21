use crate::*;

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
