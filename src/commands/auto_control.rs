use super::Command;
use crate::{custom_error::CustomError, fan_variants::FanVariant};

pub struct AutoControl {
    pub fan: Option<FanVariant>,
}

impl Command for AutoControl {
    fn execute(&self, debug: bool) -> Result<String, CustomError> {
        if let Some(fan) = &self.fan {
            fan.set_auto_mode()?;
            Ok(format!("{} set to auto mode", fan))
        } else {
            let available_fans = FanVariant::get_available_fans()?;
            for fan in available_fans {
                fan.set_auto_mode()?;
                if debug {
                    println!("{} set to auto mode", fan);
                }
            }
            Ok("All fans set to auto mode".to_string())
        }
    }

    fn from_args(args: &[String]) -> Result<Self, CustomError>
    where
        Self: Sized,
    {
        if args.len() == 0 {
            return Ok(AutoControl { fan: None });
        };

        let fan = FanVariant::from_str(&args[0])?;

        Ok(AutoControl { fan: Some(fan) })
    }

    fn args_match(args: &[String]) -> Result<bool, CustomError> {
        println!("{:?}", args);
        if args.len() == 1 {
            return Ok(args[0] == "auto");
        }

        Ok(FanVariant::is_valid_label(&args[0])? && args[1] == "auto")
    }
}
