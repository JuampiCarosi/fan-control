use std::fmt::{Debug, Display};

#[derive()]
pub struct CustomError {
    pub display_message: String,
    pub internal_message: String,
    pub cause: Option<Box<dyn std::error::Error>>,
    pub is_fatal: bool,
}

impl Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_message)
    }
}

impl Debug for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut error_message = format!(
            "Display message: {}\nInternal message: {}\nIs fatal: {}",
            self.display_message, self.internal_message, self.is_fatal
        );

        if let Some(cause) = &self.cause {
            error_message.push_str(&format!("\nCause: {}", cause));
        };

        write!(f, "{}", error_message)
    }
}

impl CustomError {
    pub fn new_simple(display_message: &str) -> Self {
        CustomError {
            display_message: display_message.to_string(),
            internal_message: display_message.to_string(),
            cause: None,
            is_fatal: false,
        }
    }
}
