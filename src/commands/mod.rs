use crate::custom_error::CustomError;

use self::{manual_control::ManualControl, show_fans::ShowFans, unknown_command::UnknownCommand};

mod manual_control;
mod show_fans;
mod unknown_command;

pub trait Command {
    fn execute(&self, debug: bool) -> Result<String, CustomError>;
    fn from_args(args: &[String]) -> Result<Self, CustomError>
    where
        Self: Sized;
}

pub fn command_parser(args: &[String]) -> Result<Box<dyn Command>, CustomError> {
    if args.is_empty() {
        return Ok(Box::new(UnknownCommand::from_args(&[])?));
    }

    let command: Box<dyn Command> = match args[0].as_str() {
        "show" => Box::new(ShowFans::from_args(&args[1..])?),
        _ => Box::new(ManualControl::from_args(args)?),
    };
    Ok(command)
}
