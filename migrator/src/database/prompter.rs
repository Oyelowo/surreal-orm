/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */
use inquire::error::InquireError;
use typed_builder::TypedBuilder;

use crate::*;

pub trait Prompter
where
    Self: std::fmt::Debug,
{
    // TODO: Rename to prompt_empty_migration_gen
    fn prompt_empty_migrations_trigger(&self) -> Result<bool, InquireError> {
        let confirmation =
            inquire::Confirm::new("Are you sure you want to generate an empty migration? (y/n)")
                .with_default(false)
                .with_help_message("This is good if you want to write out some queries manually")
                .prompt();
        confirmation
    }

    fn prompt_single_field_rename_or_delete(
        &self,
        delete_option: SingleFieldChangeType,
        rename_option: SingleFieldChangeType,
    ) -> Result<SingleFieldChangeType, InquireError> {
        let confirmation = inquire::Select::new("Do you want to rename \
                                this field or delete the old one completely without transferring the data",
            vec![delete_option, rename_option]
        )
        .with_help_message("Use the arrow keys to navigate. Press enter to select.")
        .prompt();
        confirmation
    }
}

// What is use in the actual codebase and typically just uses the default implementation
#[derive(Debug, Default, Clone)]
pub struct RealPrompter;

impl Prompter for RealPrompter {}

#[derive(Debug, Clone, Copy)]
pub enum RenameOrDelete {
    Rename,
    Delete,
}

#[derive(Debug, Clone, Copy, TypedBuilder)]
pub struct MockPrompter {
    // triggered when empty migration(s) are about to be generated.
    // This is good if you want to write out some queries manually.
    // Prompts the user to confirm the generation of empty migrations.
    // If true, empty migrations will be generated.
    // If false, the program will exit/abort.
    pub allow_empty_migrations_gen: bool,

    // triggered when a single field is changed/renamed without using
    // the explicit `old_name` attribute to indicate that youre
    // are performing a renaming operation. In that case, we
    // anticipate that the user might actually want to perform
    // a rename operation rather than deleting the old field
    // competely without transferring the data to the new field.
    // So, we prompt the user to confirm the operation if they want
    // to rename - which would transfer the data to the new field,
    // or delete the old field completely without transferring the data.
    pub rename_or_delete_single_field_change: RenameOrDelete,
}

impl Default for MockPrompter {
    fn default() -> Self {
        Self {
            allow_empty_migrations_gen: true,
            rename_or_delete_single_field_change: RenameOrDelete::Rename,
        }
    }
}

impl Prompter for MockPrompter {
    fn prompt_empty_migrations_trigger(&self) -> Result<bool, InquireError> {
        Ok(self.allow_empty_migrations_gen)
    }

    fn prompt_single_field_rename_or_delete(
        &self,
        delete_option: SingleFieldChangeType,
        rename_option: SingleFieldChangeType,
    ) -> Result<SingleFieldChangeType, InquireError> {
        match self.rename_or_delete_single_field_change {
            RenameOrDelete::Rename => Ok(rename_option),
            RenameOrDelete::Delete => Ok(delete_option),
        }
    }
}
