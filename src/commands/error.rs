use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct CommandError<'a> {
    pub message: &'a str,
    pub cause: Option<Box<Error>>,
}

impl<'a> Error for CommandError<'a> {
    fn description(&self) -> &str {
        self.message
    }
}

impl<'a> fmt::Display for CommandError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl<'a> CommandError<'a> {
    pub fn from_message_with_error(message: &str, error: Box<Error>) -> CommandError {
        CommandError {
            message: message,
            cause: Some(error),
        }
    }

    pub fn from_message<'b>(message: &'b str) -> CommandError<'b> {
        CommandError {
            message: message,
            cause: None,
        }
    }
}

#[derive(Debug)]
pub struct CommandResult {
    pub code: i32,
    pub error: Option<Box<Error>>,
}

impl CommandResult {
    pub fn success() -> CommandResult {
        CommandResult {
            code: 0,
            error: None,
        }
    }

    pub fn from_error(error: Box<Error>) -> CommandResult {
        CommandResult {
            code: 1,
            error: Some(error),
        }
    }

    pub fn from_error_with_code(error: Box<Error>, code: i32) -> CommandResult {
        CommandResult {
            code: code,
            error: Some(error),
        }
    }

    pub fn from_error_message(message: &'static str) -> CommandResult {
        CommandResult {
            code: 1,
            error: Some(Box::new(CommandError::from_message(message))),
        }
    }

    pub fn from_error_message_code(message: &'static str, code: i32) -> CommandResult {
        CommandResult {
            code: code,
            error: Some(Box::new(CommandError::from_message(message))),
        }
    }

    pub fn from_message_with_error(message: &'static str, error: Box<Error>) -> CommandResult {
        CommandResult {
            code: 1,
            error: Some(Box::new(CommandError::from_message_with_error(
                message, error,
            ))),
        }
    }
}
