use crate::custom_error::CustomError;

use super::Command;

pub struct UnknownCommand();

impl Command for UnknownCommand {
    fn execute(&self) -> Result<String, CustomError> {
        return Ok(format!("Unknown command"));
    }

    fn from_args(_args: &[String]) -> Result<Self, CustomError>
    where
        Self: Sized,
    {
        Ok(UnknownCommand())
    }
}
