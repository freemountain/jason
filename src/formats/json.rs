use std::error::Error as StdError;

use serde_json;
use serde_json::{Value, Error};

use super::super::{Format, FormatType};

pub struct Json {}

impl Format for Json {
    fn get_type(&self) -> FormatType {
        FormatType::JSON
    }

    fn from_string(&self, input: &str) -> Result<Value, Box<StdError>> {
        let result: Result<Value, Error> = serde_json::from_str(input);

        match result {
            Ok(value) => Ok(value),
            Err(error) => Err(From::from(error)),
        }
    }
}