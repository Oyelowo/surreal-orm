/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */
use std::{fmt::Display, ops::Deref};

use crate::*;
use surreal_query_builder::{
    Buildable, DbResources, Field, FieldChangeMeta, FieldMetadata, Raw, Table,
};

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
    pub(crate) diff_left: Vec<&'a String>,
    pub(crate) diff_right: Vec<&'a String>,
    pub(crate) prompter: &'a dyn Prompter,
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
            diff_left,
            diff_right,
            prompter,
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
                // bread -> cake
                // bread -> Migration/live Db state
                // cake -> codebase state

                // When we detect a signle field name change in a struct,
                // we want to give the user the option to choose to rename the field,
                // or delete even if the old_name attribute approach of renaming field
                // is not explicitly used.
                let is_single_field_delete_or_renaming =
                    diff_left.len() == 1 && diff_right.len() == 1;

                // This should only be done when the concerned field does not have
                // explicit old_name attribute set.
                // 1st time: bread
                // 2nd time: cake
                let autodetection_meta = || {
                    if is_single_field_delete_or_renaming {
                        let field_change_meta = FieldChangeMeta {
                            table: table.to_owned(),
                            // old should now be in migration dir / live db state
                            old_name: diff_left.first().unwrap().to_string().into(),
                            // new should be in code base state
                            new_name: diff_right.first().unwrap().to_string().into(),
                        };
                        let prompt = prompter
                            .prompt_single_field_rename_or_delete(
                                SingleFieldChangeType::Delete(field_change_meta.clone()),
                                SingleFieldChangeType::Rename(field_change_meta),
                            )
                            .unwrap();
                        Some(prompt)
                    } else {
                        None
                    }
                };

                let foundfield_by_newname_auto = || match autodetection_meta() {
                    Some(SingleFieldChangeType::Rename(meta)) => {
                        let new_field_def = right_defs
                            .get_definition(meta.new_name.to_string().as_str())
                            .unwrap();

                        Some(FieldMetadata {
                            name: meta.new_name,
                            old_name: Some(meta.old_name),
                            definition: vec![Raw::new(new_field_def.to_string())],
                        })
                    }
                    _ => None,
                };

                let foundfield_by_newname = RightDatabase::find_field_has_old_name(
                    codebase_resources,
                    &table,
                    By::NewName(field_name.clone()),
                )
                .or_else(foundfield_by_newname_auto)
                .map(FieldMetadataWrapper);

                // old name should be on the left i.e local migration directory state but also
                // used by user as codebase field attribute
                let found_field_by_oldname = RightDatabase::find_field_has_old_name(
                    codebase_resources,
                    &table,
                    By::OldName(field_name.clone()),
                )
                .map(FieldMetadataWrapper);
                log::error!("table: {:?}", table);
                log::error!("found_field_by_oldname: {:?}", found_field_by_oldname);
                log::error!("foundfield_by_newname: {:?}", foundfield_by_newname);

                match (found_field_by_oldname, foundfield_by_newname) {
                    // mutually exclusive since we are iterating over a union of
                    // left and right and will only encounter a field at a time
                    // and one field cannot be both new and old.
                    // . e.g rename lowo to dayo.
                    // lowo will now be new field in right with old name -dayo,
                    // dayo is expected to now be in left with old name - lowo.
                    // This is a rename.
                    (Some(_l_meta), Some(r_meta)) => {
                        r_meta.handle_fieldname_change(&table, field_name, left_defs, right_defs)?
                    }
                    (None, Some(r_meta)) => {
                        //
                        r_meta.handle_fieldname_change(&table, field_name, left_defs, right_defs)?
                    }
                    (Some(_), None) => {
                        // Dont make a change since that has been handled up there
                        DeltaTypeField::NoChange
                    }
                    // No explicit rename attribute used i.e old_name = "OldFieldName"
                    (None, None) => {
                        log::error!("left_def: {:?}", left_def);
                        log::error!("right_def: {:?}", right_def);
                        match (left_def, right_def) {
                            (None, Some(r)) => {
                                log::warn!("Create new field: {:?}", r);
                                DeltaTypeField::Create { right: r.clone() }
                            }
                            (Some(l), None) => {
                                log::warn!("Remove old field: {:?}", l);
                                DeltaTypeField::Remove { left: l.clone() }
                            }
                            _ => unreachable!(),
                        }
                    }
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
        current_field: Field,
        left_defs: Fields,
        right_defs: Fields,
    ) -> MigrationResult<DeltaTypeField> {
        let r_meta = &self;
        // skip left sine we are handling the renaming on the right i.e the
        // new field in the current codebase state.
        // let skip_left = self.old_name.as_ref().unwrap().to_string() == current_field.build();
        // if skip_left {
        //     return Ok(DeltaTypeField::NoChange);
        // }

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
