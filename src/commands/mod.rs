use crate::custom_error::CustomError;

use self::{show_fans::ShowFans, unknown_command::UnknownCommand};

mod show_fans;
mod unknown_command;

pub trait Command {
    fn execute(&self) -> Result<String, CustomError>;
    fn from_args(args: &[String]) -> Result<Self, CustomError>
    where
        Self: Sized;
}

pub fn command_parser(args: &[String]) -> Result<Box<dyn Command>, CustomError> {
    if args.len() == 0 {
        return Ok(Box::new(ShowFans::from_args(args)?));
    }

    let command: Box<dyn Command> = match args[0].as_str() {
        "" => Box::new(ShowFans::from_args(&args[1..])?),
        _ => Box::new(UnknownCommand::from_args(&args[1..])?),
    };
    Ok(command)
}
