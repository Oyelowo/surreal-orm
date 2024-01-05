/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use crate::MigrationFilename;

pub fn mig_name_parser(s: &str) -> Result<MigrationFilename, String> {
    let filename = MigrationFilename::try_from(s.to_string()).map_err(|e| e.to_string())?;
    Ok(filename)
}
