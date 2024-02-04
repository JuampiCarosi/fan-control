use crate::{custom_error::CustomError, fan_variants::FanVariant};

use super::Command;

pub struct ManualControl {
    pub fan: FanVariant,
    pub speed: u16,
}

impl Command for ManualControl {
    fn execute(&self) -> Result<String, CustomError> {
        self.fan.set_manual_mode()?;
        let max_speed = self.fan.get_max_speed()?;
        let min_speed = self.fan.get_min_speed()?;
        let fan_rpm_range = max_speed - min_speed;
        let rpm_to_add = self.speed * fan_rpm_range / 100;
        let new_speed = min_speed + rpm_to_add;
        self.fan.set_speed(new_speed)?;

        Ok(format!("{} set to {}%", self.fan, self.speed))
    }

    fn from_args(args: &[String]) -> Result<Self, CustomError>
    where
        Self: Sized,
    {
        if args.len() != 2 {
            return Err(CustomError {
                display_message: "Error: manual control command requires 2 arguments".to_string(),
                internal_message: format!(
                    "Error: manual control command requires 2 arguments, got {:?}",
                    args
                ),
                cause: None,
                is_fatal: false,
            });
        };

        let fan = FanVariant::from_str(&args[0])?;
        let speed = args[1].parse::<u16>().map_err(|e| CustomError {
            display_message: "Error: speed must be a number".to_string(),
            internal_message: format!("Error: speed must be a number, got {:?}", e),
            cause: None,
            is_fatal: false,
        })?;

        Ok(ManualControl { fan, speed })
    }
}

// fn parse_interror(e: ParseIntError) -> CustomError {
//     let parsed_error: error::Error = e.into();
//     CustomError {
//         display_message: "Error: speed must be a number".to_string(),
//         internal_message: format!("Error: speed must be a number, got {:?}", e),
//         cause: Some(Rc::new(e.into())),
//         is_fatal: false,
//     }
// }
