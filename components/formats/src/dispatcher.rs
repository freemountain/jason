use std::error::Error as StdError;
use std::io::{Error, ErrorKind};

extern crate serde_json;

use super::Type;
use super::Format;
use enum_map::EnumMap;

pub struct Dispatcher {
    //formats: Has
    formats: EnumMap<Type, Option<Box<Format>>>,
}

impl Dispatcher {
    pub fn new() -> Dispatcher {
        Dispatcher {
            formats: Default::default(),
        }
    }

    pub fn add_format(&mut self, format: Box<Format>) {
        let format_type = (*format).get_type();
        self.formats[format_type] = Some(format);
    }

    pub fn has_format_type(&self, format_type: Type) -> bool {
        let may_format = (&self.formats[format_type]).as_ref();

        match may_format {
            Some(_) => true,
            None => false,
        }
    }

    pub fn from_string(
        &self,
        format_type: Type,
        input: &str,
    ) -> Result<serde_json::Value, Box<StdError>> {
        let may_format = (&self.formats[format_type]).as_ref();

        if may_format.is_none() {
            Err(Box::new(Error::new(
                ErrorKind::NotFound,
                format!("Format not found: {}", format_type),
            )))
        } else {
            let format = may_format.unwrap();

            format.from_string(input)
        }
    }

    pub fn to_string(
        &self,
        format_type: Type,
        input: &serde_json::Value,
    ) -> Result<String, Box<StdError>> {
        let may_format = (&self.formats[format_type]).as_ref();

        if may_format.is_none() {
            Err(Box::new(Error::new(
                ErrorKind::NotFound,
                format!("Format not found: {}", format_type),
            )))
        } else {
            let format = may_format.unwrap();

            format.to_string(input)
        }
    }
}
