/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */

use crate::*;
use surreal_query_builder::*;

#[derive(Debug)]
pub(crate) struct ComparisonFields<'a, R: DbResources> {
    pub(crate) table: &'a Table,
    pub(crate) resources: &'a ComparisonsInit<'a>,
    pub(crate) codebase_resources: &'a R,
    pub(crate) prompter: &'a dyn Prompter,
}

impl<R: DbResources> TableResourcesMeta<Fields> for ComparisonFields<'_, R> {
    fn get_left(&self) -> Fields {
        self.resources
            .left_resources
            .get_table_fields(self.get_table())
            .unwrap_or_default()
    }

    fn get_right(&self) -> Fields {
        self.resources
            .right_resources
            .get_table_fields(self.get_table())
            .unwrap_or_default()
    }

    fn get_table(&self) -> &Table {
        self.table
    }

    // This does not use default implementation because it also has to handle
    // field name change/rename
    fn queries(&self) -> MigrationResult<Queries> {
        let table = self.get_table();
        let left_defs = self.get_left();
        let right_defs = self.get_right();

        let left_as_set = left_defs.get_names_as_set();
        let right_as_set = right_defs.get_names_as_set();

        let diff_left = left_as_set.difference(&right_as_set);
        let diff_right = right_as_set.difference(&left_as_set);
        let union = right_as_set.union(&left_as_set).collect::<Vec<_>>();

        let mut acc = Queries::default();

        let diff_left = diff_left.into_iter().collect::<Vec<_>>();
        let diff_right = diff_right.into_iter().collect::<Vec<_>>();
        let is_single_field_delete_or_renaming = diff_left.len() == 1 && diff_right.len() == 1;

        if is_single_field_delete_or_renaming {
            let old_field: Field = diff_left[0].to_string().into();
            let new_field: Field = diff_right[0].to_string().into();
            let explicit_old_name_used = RightDatabase::find_field_has_old_name(
                self.codebase_resources,
                table,
                By::NewName(new_field.clone()),
            );
            if explicit_old_name_used.is_none() {
                self.handle_prompt_field_renaming_or_deletion(&mut acc, old_field, new_field)?;
                return Ok(acc);
            }
        }

        for field_name in union {
            let change_meta = FieldChangeDetectionMeta {
                field_name: field_name.to_string().into(),
                table: table.to_owned(),
                left_defs: self.get_left(),
                right_defs: self.get_right(),
                codebase_resources: self.codebase_resources,
            };

            match DeltaTypeField::try_from(change_meta)? {
                DeltaTypeField::NoChange => {}
                DeltaTypeField::Create { right } => {
                    self.handle_create(&mut acc, &right)?;
                }
                DeltaTypeField::Update { left, right } => {
                    self.handle_update(&mut acc, &left, &right);
                }
                DeltaTypeField::Remove { left } => {
                    self.handle_remove(&mut acc, &left)?;
                }
                DeltaTypeField::Rename {
                    new_name, old_name, ..
                } => {
                    self.handle_rename(
                        &mut acc,
                        FieldChangeMeta {
                            table: table.to_owned(),
                            old_name,
                            new_name,
                        },
                    )?;
                }
            }
        }

        Ok(acc)
    }
}

impl<'a, R: DbResources> ComparisonFields<'a, R> {
    fn handle_rename(
        &'a self,
        acc: &'a mut Queries,
        field_change_meta: FieldChangeMeta,
    ) -> MigrationResult<&'a mut Queries> {
        let FieldChangeMeta {
            table,
            old_name,
            new_name,
        } = &field_change_meta;
        let left_defs = self.get_left();
        let right_defs = self.get_right();

        let old_field_def = left_defs.get_definition(old_name).ok_or_else(|| {
            MigrationError::InvalidOldFieldName {
                new_name: old_name.to_owned(),
                table: table.to_owned(),
                old_name: old_name.to_owned(),
                renamables: left_defs.get_names().join(", "),
            }
        })?;

        let new_field_def = right_defs.get_definition(new_name).ok_or_else(|| {
            MigrationError::FieldNameDoesNotExist {
                field_expected: new_name.to_owned(),
                table: table.to_owned(),
                valid_fields: right_defs.get_names().join(", "),
            }
        })?;

        // Field name change defs
        acc.add_comment_to_up(format!("Rename field {old_name} to {new_name}"));
        acc.add_up(QueryType::Define(new_field_def.clone()));
        let copy_old_to_new = UpdateStatementRaw::from(
            Raw::new(format!("UPDATE {table} SET {new_name} = {old_name}")).build(),
        );
        acc.add_up(QueryType::Update(copy_old_to_new));
        acc.add_up(QueryType::Remove(old_field_def.as_remove_statement()?));
        acc.add_comment_to_up("Rename field ending");

        // Field name change reversal defs
        acc.add_comment_to_down(format!(
            "Revert field name change. Change field {new_name} back to {old_name}"
        ));
        acc.add_down(QueryType::Define(old_field_def.clone()));
        let copy_new_to_old = UpdateStatementRaw::from(
            Raw::new(format!("UPDATE {table} SET {old_name} = {new_name}")).build(),
        );
        acc.add_down(QueryType::Update(copy_new_to_old));
        acc.add_down(QueryType::Remove(new_field_def.as_remove_statement()?));
        acc.add_comment_to_down("Revert field name change ending");
        Ok(acc)
    }

    fn handle_create(
        &'a self,
        acc: &'a mut Queries,
        right: &DefineStatementRaw,
    ) -> MigrationResult<&'a mut Queries> {
        acc.add_up(QueryType::Define(right.clone()));
        acc.add_down(QueryType::Remove(right.as_remove_statement()?));
        Ok(acc)
    }

    fn handle_remove(
        &'a self,
        acc: &'a mut Queries,
        left: &DefineStatementRaw,
    ) -> MigrationResult<&'a mut Queries> {
        acc.add_up(QueryType::Remove(left.as_remove_statement()?));
        acc.add_down(QueryType::Define(left.clone()));
        Ok(acc)
    }

    fn handle_update(
        &'a self,
        acc: &'a mut Queries,
        left: &DefineStatementRaw,
        right: &DefineStatementRaw,
    ) -> &'a mut Queries {
        acc.add_up(QueryType::Define(right.clone()));
        acc.add_down(QueryType::Define(left.clone()));
        acc
    }

    fn handle_prompt_field_renaming_or_deletion(
        &'a self,
        acc: &'a mut Queries,
        old_name: Field,
        new_name: Field,
    ) -> MigrationResult<&'a mut Queries> {
        let field_change_meta = FieldChangeMeta {
            table: self.table.to_owned(),
            old_name,
            new_name,
        };

        let prompt = self.prompter.prompt_single_field_rename_or_delete(
            SingleFieldChangeType::Delete(field_change_meta.clone()),
            SingleFieldChangeType::Rename(field_change_meta.clone()),
        )?;

        match prompt {
            SingleFieldChangeType::Rename(meta) => {
                self.handle_rename(acc, meta)?;
            }
            SingleFieldChangeType::Delete(meta) => {
                let new_name = meta.new_name.to_string();
                let old_name = meta.old_name.to_string();
                let left_defs = self.get_left();
                let right_defs = self.get_right();
                let left_def = left_defs.get_definition(old_name.as_str()).unwrap();
                let right_def = right_defs.get_definition(new_name.as_str()).unwrap();

                self.handle_create(acc, right_def)?;
                self.handle_remove(acc, left_def)?;
            }
        };
        Ok(acc)
    }
}
