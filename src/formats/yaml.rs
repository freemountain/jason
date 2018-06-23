
use std::error::Error;

use serde_yaml;
use serde_json::Value;

use super::super::{Format, FormatType};


pub struct Yaml {}

impl Format for Yaml {
    fn get_type(&self) -> FormatType {
        FormatType::YAML
    }

    fn from_string(&self, input: &str) -> Result<Value, Box<Error>> {
        let result: Result<Value, serde_yaml::Error> = serde_yaml::from_str(input);

        match result {
            Ok(value) => Ok(value),
            Err(error) => Err(From::from(error)),
        }
    }
}
