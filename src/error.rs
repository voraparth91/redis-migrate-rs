use std;
use std::error;
use std::fmt;

#[derive(Debug)]
pub struct GenericError {
    message: String,
}

impl GenericError {
    pub fn new(message: &str) -> GenericError {
        GenericError {
            message: String::from(message),
        }
    }
}

impl<'a> fmt::Display for GenericError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Store error: {}", self.message)
    }
}

impl<'a> error::Error for GenericError {
    fn description(&self) -> &str {
        self.message.as_str()
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

impl From<redis::RedisError> for GenericError {
    fn from(err: redis::RedisError) -> GenericError {
        GenericError::new(&err.to_string())
    }
}

impl From<std::env::VarError> for GenericError {
    fn from(err: std::env::VarError) -> GenericError {
        GenericError::new(&err.to_string())
    }
}