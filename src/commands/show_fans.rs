use crate::{custom_error::CustomError, fan_variants::FanVariants};

use super::Command;

pub struct ShowFans {
    pub available_fans: Vec<FanVariants>,
}

impl Command for ShowFans {
    fn execute(&self) -> Result<String, CustomError> {
        let mut output = String::new();

        output.push_str("Available fans:\n");

        for fan in &self.available_fans {
            output.push_str(&format!("  {}\n", fan));
        }

        return Ok(output);
    }

    fn from_args(_args: &[String]) -> Result<Self, CustomError>
    where
        Self: Sized,
    {
        let available_fans = FanVariants::get_available()?;

        return Ok(ShowFans { available_fans });
    }
}
