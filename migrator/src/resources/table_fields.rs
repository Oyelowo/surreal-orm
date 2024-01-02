/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::fmt::Display;

use crate::*;
use surreal_query_builder::*;

use super::{ChangeMeta, FieldChangeDetectionMeta};

#[derive(Debug)]
pub(crate) struct ComparisonFields<'a, R: DbResources> {
    pub(crate) table: &'a Table,
    pub(crate) resources: &'a ComparisonsInit<'a>,
    pub(crate) codebase_resources: &'a R,
    // pub(crate) rename_or_delete_single_field_change_prompt: bool,
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
        let right = self.get_right().get_names_as_set();

        let left = self.get_left().get_names_as_set();

        let diff_left = left.difference(&right);
        let diff_right = right.difference(&left);
        let union = right.union(&left).collect::<Vec<_>>();

        let diff_left = diff_left.into_iter().collect::<Vec<_>>();
        let diff_right = diff_right.into_iter().collect::<Vec<_>>();
        let is_potentially_renaming = diff_left.len() == 1 && diff_right.len() == 1;

        let potentially_old_name = if is_potentially_renaming {
            diff_left.first()
        } else {
            None
        };

        let mut acc = Queries::default();
        for name in union {
            let def_right = self.get_right().get_definition(name).cloned();
            let def_left = self.get_left().get_definition(name).cloned();
            let table = self.get_table();

            let change_meta = FieldChangeDetectionMeta {
                field_name,
                table: table.to_owned(),
                left_defs: self.get_left(),
                right_defs: self.get_right(),
                codebase_resources: self.codebase_resources,
            };

            match DeltaType::from(change_meta) {
                DeltaType::NoChange => {}
                DeltaType::Create { right } => {
                    acc.add_up(QueryType::Define(right));
                    acc.add_down(QueryType::Remove(right.as_remove_statement()?));
                }
                DeltaType::Update { left, right } => {
                    acc.add_up(QueryType::Define(right));
                    acc._add_down(QueryType::Define(left));
                }
                DeltaType::Remove { left } => {
                    acc.add_down(left.as_remove_statement());
                    acc.add_down(QueryType::Define(left));
                }
                DeltaType::Rename {
                    right,
                    new_name,
                    old_left,
                    old_name,
                } => {
                    acc.add_up(QueryType::Define(right));
                    let copy_old_to_new = UpdateStatementRaw::from(
                        Raw::new(format!("UPDATE {table} SET {new_name} = {old_name}")).build(),
                    );
                    acc.add_up(QueryType::Update(copy_old_to_new));
                    acc.add_up(QueryType::Remove(old_left.as_remove_statement()?));

                    acc.add_down(QueryType::Define(old_left));
                    let copy_new_to_old = UpdateStatementRaw::from(
                        Raw::new(format!("UPDATE {table} SET {old_name} = {new_name}",)).build(),
                    );
                    acc.add_down(QueryType::Update(copy_new_to_old));
                    acc.add_down(QueryType::Remove(right.as_remove_statement()?));
                }
            }
            //                 self.get_field_meta_from_prompt(new_name, potentially_old_name)
        }

        Ok(acc)
    }
}

impl<'a, R: DbResources> ComparisonFields<'a, R> {
    fn get_field_meta_from_prompt(
        &self,
        new_name: &String,
        potentially_old_name: Option<&&String>,
    ) -> Option<SingleFieldChangeType> {
        let _field_meta_implicit = FieldMetadata {
            name: new_name.to_string().into(),
            old_name: potentially_old_name.map(|on| on.to_string().into()),
            definition: vec![self
                .get_right()
                .get_definition(new_name)
                .expect("should exist. bug!")
                .clone()
                .into()],
        };

        let field_change_meta = FieldChangeMeta {
            table: self.get_table().to_owned(),
            old_name: potentially_old_name
                .cloned()
                .map(|n| n.to_string().into())
                .expect("should exist"),
            new_name: new_name.to_string().into(),
        };

        let delete_option = SingleFieldChangeType::Delete(field_change_meta.clone());
        let rename_option = SingleFieldChangeType::Rename(field_change_meta);

        let confirmation = self
            .prompter
            .prompt_single_field_rename_or_delete(delete_option, rename_option);

        Some(confirmation.expect("Invalid confirmation"))
    }
}

#[derive(Debug, Clone)]
pub struct FieldChangeMeta {
    table: Table,
    old_name: Field,
    new_name: Field,
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
