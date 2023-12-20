use crate::*;

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct MigrationTwoWay {
    pub name: MigrationFilename,
    pub up: FileContent,
    pub down: FileContent,
    // status: String,
}

impl TryFrom<MigrationTwoWay> for Migration {
    type Error = MigrationError;

    fn try_from(migration: MigrationTwoWay) -> Result<Self, Self::Error> {
        Ok(Self {
            id: Migration::create_id(&migration.name.to_up()),
            name: migration.name.to_up().to_string(),
            timestamp: migration.name.timestamp(),
            checksum_up: migration.up.as_checksum()?,
            checksum_down: Some(migration.down.as_checksum()?),
        })
    }
}

#[derive(Clone, Debug)]
pub struct MigrationOneWay {
    pub name: MigrationFilename,
    pub content: FileContent, // status: String,
}

impl MigrationOneWay {}

impl TryFrom<MigrationOneWay> for Migration {
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
