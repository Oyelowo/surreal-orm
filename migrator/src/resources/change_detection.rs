/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */
use std::{fmt::Display, ops::Deref};

use crate::*;
use surreal_query_builder::{Buildable, DbResources, Field, FieldChangeMeta, FieldMetadata, Table};

pub enum DeltaTypeField {
    NoChange,
    Create {
        right: DefineStatementRaw,
    },
    Remove {
        left: DefineStatementRaw,
    },
    Update {
        left: DefineStatementRaw,
        right: DefineStatementRaw,
    },
    Rename {
        right: DefineStatementRaw,
        new_name: Field,
        old_left: DefineStatementRaw,
        old_name: Field,
    },
}

// Change detection
pub(crate) struct FieldChangeDetectionMeta<'a, R: DbResources> {
    pub(crate) field_name: Field,
    pub(crate) left_defs: Fields,
    pub(crate) right_defs: Fields,
    pub(crate) table: Table,
    pub(crate) codebase_resources: &'a R,
}

impl<'a, R: DbResources> TryFrom<FieldChangeDetectionMeta<'a, R>> for DeltaTypeField {
    type Error = MigrationError;

    fn try_from(value: FieldChangeDetectionMeta<R>) -> MigrationResult<Self> {
        let FieldChangeDetectionMeta {
            field_name,
            left_defs,
            right_defs,
            table,
            codebase_resources,
        } = value;

        let left_def = left_defs.get_definition(&field_name.build());
        let right_def = right_defs.get_definition(&field_name.build());

        let res = match (left_def.cloned(), right_def.cloned()) {
            (None, None) => unreachable!(),
            (Some(l), Some(r)) => {
                if l.trim() != r.trim() {
                    DeltaTypeField::Update { left: l, right: r }
                } else {
                    DeltaTypeField::NoChange
                }
            }
            _ => {
                let foundfield_by_newname = RightDatabase::find_field_has_old_name(
                    codebase_resources,
                    &table,
                    By::NewName(field_name.clone()),
                )
                .map(FieldMetadataWrapper);

                let found_field_by_oldname = RightDatabase::find_field_has_old_name(
                    codebase_resources,
                    &table,
                    By::OldName(field_name.clone()),
                )
                .map(FieldMetadataWrapper);

                match (found_field_by_oldname, foundfield_by_newname) {
                    (Some(_l_meta), Some(r_meta)) => {
                        //
                        r_meta.handle_fieldname_change(&table, left_defs, right_defs)?
                    }
                    (None, Some(r_meta)) => {
                        //
                        r_meta.handle_fieldname_change(&table, left_defs, right_defs)?
                    }
                    (Some(_), None) => {
                        // Skip change since that has been handled up there on the right
                        DeltaTypeField::NoChange
                    }
                    // No explicit rename attribute used e.g old_name = "OldFieldName"
                    (None, None) => match (left_def, right_def) {
                        (None, Some(r)) => DeltaTypeField::Create { right: r.clone() },
                        (Some(l), None) => DeltaTypeField::Remove { left: l.clone() },
                        _ => unreachable!(),
                    },
                }
            }
        };
        Ok(res)
    }
}

#[derive(Debug, Clone)]
struct FieldMetadataWrapper(FieldMetadata);
impl Deref for FieldMetadataWrapper {
    type Target = FieldMetadata;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FieldMetadataWrapper {
    pub(crate) fn handle_fieldname_change(
        &self,
        table: &Table,
        left_defs: Fields,
        right_defs: Fields,
    ) -> MigrationResult<DeltaTypeField> {
        let r_meta = &self.0;

        let old_left = left_defs
            .get_definition(&r_meta.old_name.as_ref().unwrap().to_string().as_str())
            .ok_or_else(|| MigrationError::InvalidOldFieldName {
                new_name: r_meta.name.to_string(),
                table: table.to_string(),
                old_name: r_meta.old_name.as_ref().unwrap().to_string(),
                renamables: left_defs.get_names().join(", "),
            })?;

        let right_def = right_defs
            .get_definition(&r_meta.name.to_string().as_str())
            .ok_or_else(|| MigrationError::FieldNameDoesNotExist {
                field_expected: r_meta.name.to_string(),
                table: table.to_string(),
                valid_fields: right_defs.get_names().join(", "),
            })?;

        Ok(DeltaTypeField::Rename {
            right: right_def.clone(),
            new_name: r_meta.name.clone(),
            old_left: old_left.to_owned(),
            old_name: r_meta.old_name.clone().unwrap(),
        })
    }
}

#[derive(Debug, Clone)]
pub enum SingleFieldChangeType {
    Delete(FieldChangeMeta),
    Rename(FieldChangeMeta),
}

impl Display for SingleFieldChangeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SingleFieldChangeType::Delete(change) => write!(
                f,
                "Delete old field '{}' and create a new one '{}' on table '{}'",
                change.old_name, change.new_name, change.table
            ),
            SingleFieldChangeType::Rename(change) => write!(
                f,
                "Rename old field '{}' to new field '{}' on table '{}'",
                change.old_name, change.new_name, change.table
            ),
        }
    }
}

pub enum DeltaTypeResource {
    NoChange,
    Create {
        right: DefineStatementRaw,
    },
    Remove {
        left: DefineStatementRaw,
    },
    Update {
        left: DefineStatementRaw,
        right: DefineStatementRaw,
    },
}

impl From<(Option<DefineStatementRaw>, Option<DefineStatementRaw>)> for DeltaTypeResource {
    fn from(value: (Option<DefineStatementRaw>, Option<DefineStatementRaw>)) -> Self {
        match value {
            (None, Some(r)) => DeltaTypeResource::Create { right: r },
            (Some(l), None) => DeltaTypeResource::Remove { left: l },
            (Some(l), Some(r)) => {
                if l.trim() != r.trim() {
                    DeltaTypeResource::Update { left: l, right: r }
                } else {
                    DeltaTypeResource::NoChange
                }
            }
            (None, None) => unreachable!(),
        }
    }
}
