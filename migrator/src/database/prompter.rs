use inquire::error::InquireError;
use typed_builder::TypedBuilder;

use crate::table_fields::SingleFieldChangeType;

pub trait Prompter
where
    Self: std::fmt::Debug,
{
    // TODO: Rename to prompt_empty_migration_gen
    fn prompt(&self) -> Result<bool, InquireError> {
        let confirmation =
            inquire::Confirm::new("Are you sure you want to generate an empty migration? (y/n)")
                .with_default(false)
                .with_help_message("This is good if you want to write out some queries manually")
                .prompt();
        confirmation
    }

    fn prompt_field_rename(
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

#[derive(Debug, Default, Clone)]
pub struct RealPrompter;

impl Prompter for RealPrompter {}

#[derive(Debug, Default, Clone, TypedBuilder)]
pub struct MockPrompter {
    pub confirmation: bool,
}

impl Prompter for MockPrompter {
    fn prompt(&self) -> Result<bool, InquireError> {
        Ok(self.confirmation)
    }
}
