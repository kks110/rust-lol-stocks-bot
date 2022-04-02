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

impl Default for DiscordIdConversionError {
    fn default() -> Self {
        DiscordIdConversionError::new()
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


#[derive(Debug)]
pub struct NoAffordableStock {
    details: String
}

impl NoAffordableStock {
    pub fn new() -> NoAffordableStock {
        NoAffordableStock{
            details: "No teams that you can afford".to_string()
        }
    }
}

impl Default for NoAffordableStock {
    fn default() -> Self {
        NoAffordableStock::new()
    }
}

impl fmt::Display for NoAffordableStock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for NoAffordableStock {
    fn description(&self) -> &str {
        &self.details
    }
}


