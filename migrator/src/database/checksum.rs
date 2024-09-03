/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */
use std::{fmt::Display, fs::File, io::BufReader, path::PathBuf};

use serde::{Deserialize, Serialize};
use sha2::{self, Digest, Sha256};

use crate::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Checksum(String);

impl From<String> for Checksum {
    fn from(checksum: String) -> Self {
        Self(checksum)
    }
}

impl Display for Checksum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Checksum(checksum) = self;
        write!(f, "{checksum}")
    }
}

impl Checksum {
    pub fn generate_from_content(content: &FileContent) -> MigrationResult<Self> {
        let mut hasher = Sha256::new();
        hasher.update(content.to_string().as_bytes());

        let hash = hasher.finalize();
        Ok(format!("{hash:x}").into())
    }

    pub fn generate_from_path(file_path: impl Into<PathBuf>) -> MigrationResult<Self> {
        let file_path: PathBuf = file_path.into();
        let file = File::open(&file_path).map_err(|e| {
            MigrationError::IoError(format!(
                "Failed to open migration file: {}. Error: {}",
                file_path.to_string_lossy(),
                e
            ))
        })?;

        let mut reader = BufReader::new(file);
        let mut hasher = Sha256::new();

        std::io::copy(&mut reader, &mut hasher).map_err(|e| {
            MigrationError::IoError(format!(
                "Failed to read migration file: {}. Error: {}",
                file_path.to_string_lossy(),
                e
            ))
        })?;

        let hash = hasher.finalize();
        Ok(format!("{hash:x}").into())
    }

    pub fn verify(
        &self,
        migration_filename: &MigrationFilename,
        content: &FileContent,
    ) -> MigrationResult<()> {
        let checksum = Checksum::generate_from_content(content)?;
        if checksum != *self {
            return Err(MigrationError::ChecksumMismatch {
                migration_name: migration_filename.to_string(),
                expected_checksum: self.to_string(),
                actual_checksum: checksum.to_string(),
            });
        }
        Ok(())
    }
}
