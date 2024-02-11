use crate::custom_error::CustomError;

use super::Command;

pub struct UnknownCommand();

impl Command for UnknownCommand {
    fn execute(&self, _debug: bool) -> Result<String, CustomError> {
        Ok("Unknown command".to_string())
    }

    fn from_args(_args: &[String]) -> Result<Self, CustomError>
    where
        Self: Sized,
    {
        Ok(UnknownCommand())
    }
}
