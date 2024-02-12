use crate::custom_error::CustomError;

use self::{
    auto_control::AutoControl, manual_control::ManualControl, show_fans::ShowFans,
    unknown_command::UnknownCommand,
};

mod auto_control;
mod manual_control;
mod show_fans;
mod unknown_command;

pub trait Command {
    fn execute(&self, debug: bool) -> Result<String, CustomError>;
    fn args_match(args: &[String]) -> Result<bool, CustomError>
    where
        Self: Sized;
    fn from_args(args: &[String]) -> Result<Self, CustomError>
    where
        Self: Sized;
}

pub fn command_parser(args: &[String]) -> Result<Box<dyn Command>, CustomError> {
    if args.is_empty() {
        return Ok(Box::new(UnknownCommand::from_args(&[])?));
    }

    let command: Box<dyn Command> = match args {
        args if ShowFans::args_match(args)? => Box::new(ShowFans::from_args(args)?),
        args if ManualControl::args_match(args)? => Box::new(ManualControl::from_args(args)?),
        args if AutoControl::args_match(args)? => Box::new(AutoControl::from_args(args)?),
        _ => Box::new(UnknownCommand::from_args(args)?),
    };
    Ok(command)
}
