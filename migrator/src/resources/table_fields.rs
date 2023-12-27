/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use std::fmt::Display;

use crate::*;
use surreal_query_builder::*;

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

            let field_meta_with_old_name = RightDatabase::find_field_has_old_name(
                self.codebase_resources,
                table,
                By::NewName(name.to_string()),
            );

            if let Some(meta) = &field_meta_with_old_name {
                let old_name = &meta.clone().old_name.expect("Should exist").to_string();
                if !left.contains(old_name) {
                    return Err(MigrationError::InvalidOldFieldName {
                        new_name: name.to_string(),
                        table: table.to_string(),
                        old_name: old_name.to_string(),
                        renamables: left.clone().into_iter().collect::<Vec<_>>().join(", "),
                    });
                }
            }

            match DeltaType::from((def_left, def_right)) {
                DeltaType::NoChange => {}
                DeltaType::Create { right } => {
                    acc.add_up(QueryType::Define(right.clone()));

                    let new_name = name;

                    let field_meta_with_old_name =
                        if is_potentially_renaming && field_meta_with_old_name.is_none() {
                            self.get_field_meta_from_prompt(new_name, potentially_old_name)
                        } else {
                            field_meta_with_old_name.map(|f| {
                                SingleFieldChangeType::Rename(FieldChangeMeta {
                                    table: self.get_table().to_owned(),
                                    old_name: f.old_name.clone().unwrap(),
                                    new_name: new_name.to_string().into(),
                                })
                            })
                        };

                    if let Some(SingleFieldChangeType::Rename(meta)) = &field_meta_with_old_name {
                        let old_name = meta.old_name.clone();
                        let copy_old_to_new = UpdateStatementRaw::from(
                            Raw::new(format!("UPDATE {table} SET {new_name} = {old_name}",))
                                .build(),
                        );

                        acc.add_up(QueryType::Update(copy_old_to_new));
                    }

                    if let Some(
                        SingleFieldChangeType::Rename(meta) | SingleFieldChangeType::Delete(meta),
                    ) = &field_meta_with_old_name
                    {
                        acc.add_up(QueryType::Remove(
                            // We are using the define statement for new name but to generate
                            // remove statement for old name, that's why we're passing the old
                            // field name here to override the new field name when generating
                            // the remove statement
                            right.as_remove_statement_with_name_override(Some(
                                meta.old_name.clone().into(),
                            ))?,
                        ));

                        let old_name = meta.old_name.clone();
                        let left = self.get_left();
                        let error = format!("The field - {new_name} - on table - {table} - already renamed or never existed before. \
                            Make sure you are using the correct case for the field name. \
                            It should be one of these :{}", left.get_names().join(","));

                        let old_field_def_from_left = left
                            .get_definition(&old_name.to_string())
                            .unwrap_or_else(|| panic!("{}", error));

                        // Do some validations here:
                        acc.add_down(QueryType::Define(old_field_def_from_left.to_owned()));
                    }

                    if let Some(SingleFieldChangeType::Rename(meta)) = &field_meta_with_old_name {
                        let old_name = meta.old_name.clone();

                        let copy_new_to_old = UpdateStatementRaw::from(
                            Raw::new(format!("UPDATE {table} SET {old_name} = {new_name}",))
                                .build(),
                        );
                        acc.add_down(QueryType::Update(copy_new_to_old));
                    }

                    acc.add_down(QueryType::Remove(right.as_remove_statement()?));
                    acc.add_new_line_to_down();
                }
                DeltaType::Remove { left } => {
                    if !is_potentially_renaming && field_meta_with_old_name.is_none() {
                        acc.add_up(QueryType::Remove(left.as_remove_statement()?));

                        acc.add_new_line_to_up();

                        acc.add_down(QueryType::Define(left));

                        acc.add_new_line_to_down();
                    }
                }
                DeltaType::Update { left, right } => {
                    acc.add_up(QueryType::Define(right));
                    acc.add_new_line_to_up();

                    acc.add_down(QueryType::Define(left));
                    acc.add_new_line_to_down();
                }
            }
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
            .prompt_field_rename(delete_option, rename_option);

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
