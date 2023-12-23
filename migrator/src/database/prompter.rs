use inquire::error::InquireError;

pub trait Prompter {
    fn prompt(&self) -> Result<bool, InquireError> {
        let confirmation =
            inquire::Confirm::new("Are you sure you want to generate an empty migration? (y/n)")
                .with_default(false)
                .with_help_message("This is good if you want to write out some queries manually")
                .prompt();
        confirmation
    }
}

#[derive(Debug, Default, Clone)]
pub struct RealPrompter;

impl Prompter for RealPrompter {}

#[derive(Debug, Default, Clone)]
pub struct MockPrompter {
    pub confirmation: bool,
}

impl Prompter for MockPrompter {
    fn prompt(&self) -> Result<bool, InquireError> {
        Ok(self.confirmation)
    }
}
