use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct DiscordIdConversionError {
    details: String
}

impl DiscordIdConversionError {
    pub fn new() -> DiscordIdConversionError {
        DiscordIdConversionError{
            details: "Could not convert discord id to numeric".to_string()
        }
    }
}

impl fmt::Display for DiscordIdConversionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for DiscordIdConversionError {
    fn description(&self) -> &str {
        &self.details
    }
}
