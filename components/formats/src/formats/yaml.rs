use std::error::Error;

use serde_json::Value;
use serde_yaml;

use super::super::{Format, Type};

pub struct Yaml {}

impl Format for Yaml {
    fn get_type(&self) -> Type {
        Type::YAML
    }

    fn to_string(&self, input: &Value) -> Result<String, Box<Error>> {
        serde_yaml::to_string(input).map_err(From::from)
    }

    fn from_string(&self, input: &str) -> Result<Value, Box<Error>> {
        let result: Result<Value, serde_yaml::Error> = serde_yaml::from_str(input);
        result.map_err(From::from)
    }
}
