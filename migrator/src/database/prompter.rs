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
pub struct RealPrompter;

impl Prompter for RealPrompter {}

pub struct TrueMockPrompter;

impl Prompter for TrueMockPrompter {
    fn prompt(&self) -> Result<bool, InquireError> {
        Ok(true)
    }
}

pub struct FalseMockPrompter;

impl Prompter for FalseMockPrompter {
    fn prompt(&self) -> Result<bool, InquireError> {
        Ok(false)
    }
}
