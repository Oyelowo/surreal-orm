/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */
use std::fmt::Display;

use crate::*;
use surreal_query_builder::{DbResources, Field, FieldChangeMeta, FieldMetadata, Raw, Table};

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
    pub(crate) field_name: String,
    pub(crate) left_defs: Fields,
    pub(crate) right_defs: Fields,
    pub(crate) table: Table,
    pub(crate) codebase_resources: &'a R,
    pub(crate) diff_left: Vec<&'a String>,
    pub(crate) diff_right: Vec<&'a String>,
    pub(crate) prompter: &'a dyn Prompter,
}

impl<'a, R: DbResources> From<FieldChangeDetectionMeta<'a, R>> for DeltaTypeField {
    fn from(value: FieldChangeDetectionMeta<R>) -> Self {
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

        let left_def = left_defs.get_definition(&field_name);
        let right_def = right_defs.get_definition(&field_name);
        // When we detect a signle field name change in a struct,
        // we want to give the user the option to choose to rename the field,
        // or delete even if the old_name attribute approach of renaming field
        // is not explicitly used.
        let might_be_single_field_delete_or_renaming =
            diff_left.len() == 1 && diff_right.len() == 1;

        let autodetection_meta = if might_be_single_field_delete_or_renaming {
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
        };

        match (left_def.cloned(), right_def.cloned()) {
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
                let foundfield_by_newname_auto = match autodetection_meta {
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
                );
                // .or(foundfield_by_newname_auto);

                // old name should be on the left i.e local migration directory state but also
                // used by user as codebase field attribute
                let found_field_by_oldname = RightDatabase::find_field_has_old_name(
                    codebase_resources,
                    &table,
                    By::OldName(field_name),
                )
                .or(foundfield_by_newname_auto);
                // if let Some(meta) = &field_meta_with_old_name {
                //     let old_name = &meta.clone().old_name.expect("Should exist").to_string();
                //     if !left.contains(old_name) {
                //         return Err(MigrationError::InvalidOldFieldName {
                //             new_name: name.to_string(),
                //             table: table.to_string(),
                //             old_name: old_name.to_string(),
                //             renamables: left.clone().into_iter().collect::<Vec<_>>().join(", "),
                //         });
                //     }
                // }

                match (found_field_by_oldname, foundfield_by_newname) {
                    // mutually exclusive since we are iterating over a union of
                    // left and right and will only encounter a field at a time
                    // and one field cannot be both new and old.
                    // . e.g rename lowo to dayo.
                    // lowo will now be new field in right with old name -dayo,
                    // dayo is expected to now be in left with old name - lowo.
                    // This is a rename.
                    (Some(l_meta), Some(r_meta)) => {
                        panic!(
                            "You are using same old field name for new field name. This is likely
                        not intentional. Use a different name for the new field"
                        )
                    }
                    (None, Some(r_meta)) => {
                        let old_left = left_defs.get_definition(&r_meta.old_name.as_ref().unwrap().to_string().as_str()).unwrap_or_else(|| {
                            panic!(
                                "Could not find field with name {} in migration local directory state table definition. \
                                    Make sure you are using the correct case for the field name. \
                                    It should be one of these :{}",
                                r_meta.old_name.as_ref().unwrap(),
                                left_defs.get_names().join(",")
                            )
                        });
                        let right_def = right_defs.get_definition(&r_meta.name.to_string().as_str()).unwrap_or_else(|| {
                            panic!(
                                "Could not find field with name {} in migration local directory state table definition. \
                                    Make sure you are using the correct case for the field name. \
                                    It should be one of these :{}",
                                r_meta.name,
                                right_defs.get_names().join(",")
                            )
                        });

                        // up
                        // define new field
                        // Assign old to new
                        // delete old
                        //
                        // downdo
                        // define old field
                        // assign new to old
                        // delete new
                        DeltaTypeField::Rename {
                            right: right_def.clone(),
                            new_name: r_meta.name.clone(),
                            old_left: old_left.to_owned(),
                            old_name: r_meta.old_name.unwrap(),
                        }
                    }
                    (Some(_), None) => {
                        // Dont make a change since that has been handled up there
                        DeltaTypeField::NoChange
                    }
                    // No explicit rename attribute used i.e old_name = "OldFieldName"
                    (None, None) => match (left_def, right_def) {
                        (None, Some(r)) => DeltaTypeField::Create { right: r.clone() },
                        (Some(l), None) => DeltaTypeField::Remove { left: l.clone() },
                        _ => unreachable!(),
                    },
                }
            }
        }
    }
}

struct FieldChangeMetaWrapper(FieldChangeMeta);

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
