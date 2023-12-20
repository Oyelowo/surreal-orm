use crate::*;

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct MigrationFileBiPair {
    pub up: FileMetadata,
    pub down: FileMetadata,
}

impl TryFrom<MigrationFileBiPair> for Migration {
    type Error = MigrationError;

    fn try_from(migration: MigrationFileBiPair) -> Result<Self, Self::Error> {
        Ok(Self {
            id: Migration::create_id(&migration.up.name),
            name: migration.up.name.to_up().to_string(),
            timestamp: migration.up.name.timestamp(),
            checksum_up: migration.up.content.as_checksum()?,
            checksum_down: Some(migration.down.as_checksum()?),
        })
    }
}

#[derive(Clone, Debug)]
pub struct MigrationFileUni(FileMetadata);

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct FileMetadata {
    pub name: MigrationFilename,
    pub content: FileContent, // status: String,
}

impl TryFrom<MigrationFileUni> for Migration {
    type Error = MigrationError;

    fn try_from(migration: MigrationOneWay) -> Result<Self, Self::Error> {
        Ok(Self {
            id: Migration::create_id(&migration.name),
            name: migration.name.to_string(),
            timestamp: migration.name.timestamp(),
            checksum_up: Checksum::generate_from_content(&migration.content)?.into(),
            checksum_down: None,
        })
    }
}
