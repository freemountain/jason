use std::error::Error as StdError;

use serde_json;
use serde_json::{Error, Value};

use super::super::{Format, FormatType};

pub struct Json {}

impl Format for Json {
    fn get_type(&self) -> FormatType {
        FormatType::JSON
    }

    fn to_string(&self, input: &Value) -> Result<String, Box<StdError>> {
        serde_json::to_string_pretty(input).map_err(From::from)
    }

    fn from_string(&self, input: &str) -> Result<Value, Box<StdError>> {
        let result: Result<Value, Error> = serde_json::from_str(input);
        result.map_err(From::from)
    }
}
