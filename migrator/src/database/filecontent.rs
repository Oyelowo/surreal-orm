use std::{fmt::Display, fs, path::PathBuf};

use crate::{Checksum, MigrationError, MigrationResult};

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct FileContent(String);

impl Display for FileContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let FileContent(content) = self;
        write!(f, "{}", content)
    }
}

impl FileContent {
    pub fn empty() -> Self {
        Self("".into())
    }

    pub fn as_checksum(&self) -> MigrationResult<Checksum> {
        Checksum::generate_from_content(self)
    }

    pub fn as_checksum_from_path(
        &self,
        file_path: impl Into<PathBuf>,
    ) -> MigrationResult<Checksum> {
        Checksum::generate_from_path(file_path)
    }

    pub fn from_file(file_path: impl Into<PathBuf>) -> MigrationResult<Self> {
        let file_path = file_path.into();
        let content = fs::read_to_string(&file_path)
            .map_err(|e| MigrationError::IoError(format!("Error: {}", e)))?;
        Ok(Self(content))
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl From<String> for FileContent {
    fn from(content: String) -> Self {
        Self(content)
    }
}
