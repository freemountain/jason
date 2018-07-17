use std::fmt;
use std::error::Error;

use std::ffi::OsStr;
use std::path::Path;

use serde_json::Value;

pub trait Format {
    fn get_type(&self) -> Type;

    fn from_string(&self, input: &str) -> Result<Value, Box<Error>>;
    fn to_string(&self, input: &Value) -> Result<String, Box<Error>>;
}


#[derive(Copy, Clone, Enum)]
pub enum Type {
    YAML = 0,
    JSON = 1,
    CSV = 2,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            Type::YAML => "yaml",
            Type::JSON => "json",
            Type::CSV => "csv",
        };
        write!(f, "{}", printable)
    }
}

impl Type {
    pub fn from_extension(extension: &str) -> Option<Type> {
        match extension {
            "yaml" => Some(Type::YAML),
            "yml" => Some(Type::YAML),
            "json" => Some(Type::JSON),
            "csv" => Some(Type::CSV),
            _ => None,
        }
    }

    pub fn from_filename(filename: &str) -> Option<Type> {
        let extension = Path::new(filename).extension().and_then(OsStr::to_str);

        match extension {
            Some(a) => Type::from_extension(a),
            None => None,
        }
    }
}
