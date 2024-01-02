/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
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

impl<'a, R: DbResources> TableResourcesMeta<Fields> for ComparisonFields<'a, R> {
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

        let right_as_set = left_defs.get_names_as_set();
        let left_as_set = right_defs.get_names_as_set();

        let diff_left = left_as_set.difference(&right_as_set);
        let diff_right = right_as_set.difference(&left_as_set);
        let union = right_as_set.union(&left_as_set).collect::<Vec<_>>();

        let diff_left = diff_left.into_iter().collect::<Vec<_>>();
        let diff_right = diff_right.into_iter().collect::<Vec<_>>();

        let mut acc = Queries::default();

        let might_be_single_field_delete_or_renaming =
            diff_left.len() == 1 && diff_right.len() == 1;

        let old_name = diff_left.first().map(|n| Field::new(n.to_string()));
        let new_name = diff_right.first().map(|n| Field::new(n.to_string()));

        if might_be_single_field_delete_or_renaming {
            let field_change_meta = FieldChangeMeta {
                table: table.to_owned(),
                old_name: old_name.clone().unwrap().to_owned(),
                new_name: new_name.clone().unwrap().to_owned(),
            };

            let prompt = self.prompter.prompt_single_field_rename_or_delete(
                SingleFieldChangeType::Delete(field_change_meta.clone()),
                SingleFieldChangeType::Rename(field_change_meta.clone()),
            )?;

            match prompt {
                SingleFieldChangeType::Rename(meta) => {
                    self.handle_rename(&mut acc, meta)?;
                }
                SingleFieldChangeType::Delete(meta) => {
                    let new_name = meta.new_name.to_string();
                    let old_name = meta.old_name.to_string();
                    let right_def = left_defs.get_definition(new_name.as_str()).unwrap();
                    let left_def = right_defs.get_definition(old_name.as_str()).unwrap();

                    self.handle_create(&mut acc, right_def)?;
                    self.handle_remove(&mut acc, left_def)?;
                }
            };

            return Ok(acc);
        }

        for field_name in union {
            let change_meta = FieldChangeDetectionMeta {
                field_name: field_name.to_string().into(),
                table: table.to_owned(),
                left_defs: self.get_left(),
                right_defs: self.get_right(),
                codebase_resources: self.codebase_resources,
            };

            //
            match DeltaTypeField::try_from(change_meta)? {
                DeltaTypeField::NoChange => {}
                DeltaTypeField::Create { right } => {
                    self.handle_create(&mut acc, &right)?;
                }
                DeltaTypeField::Update { left, right } => {
                    acc.add_up(QueryType::Define(right));
                    acc.add_down(QueryType::Define(left));
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
        } = field_change_meta;
        let left_defs = self.get_left();
        let right_defs = self.get_right();

        let right_def = left_defs
            .get_definition(new_name.to_string().as_str())
            .unwrap();
        let left_def = right_defs
            .get_definition(old_name.to_string().as_str())
            .unwrap();

        acc.add_up(QueryType::Define(right_def.clone()));
        let copy_old_to_new = UpdateStatementRaw::from(
            Raw::new(format!("UPDATE {table} SET {new_name} = {old_name}")).build(),
        );
        acc.add_up(QueryType::Update(copy_old_to_new));
        acc.add_up(QueryType::Remove(left_def.as_remove_statement()?));

        acc.add_down(QueryType::Define(left_def.clone()));
        let copy_new_to_old = UpdateStatementRaw::from(
            Raw::new(format!("UPDATE {table} SET {old_name} = {new_name}",)).build(),
        );
        acc.add_down(QueryType::Update(copy_new_to_old));
        acc.add_down(QueryType::Remove(right_def.as_remove_statement()?));
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
}
