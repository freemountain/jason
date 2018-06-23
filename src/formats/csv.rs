use std::error::Error;
use std::fmt;

use csv;
use serde_json;
use serde_json::Value;

use super::super::{Format, FormatType};

pub enum CsvOutputFormat {
    Array,
    Object,
}

impl fmt::Display for CsvOutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            CsvOutputFormat::Array => "array",
            CsvOutputFormat::Object => "object",
        };
        write!(f, "{}", printable)
    }
}

impl CsvOutputFormat {
    pub fn from_str(input: &str) -> Option<CsvOutputFormat> {
        match input {
            "array" => Some(CsvOutputFormat::Array),
            "object" => Some(CsvOutputFormat::Object),
            _ => None,
        }
    }
}

pub struct Csv {
    pub format: CsvOutputFormat,
    pub headers: Option<Box<Vec<String>>>,
    pub skip: usize,
}

fn get_map_values(record: serde_json::Map<String, Value>) -> Vec<Value> {
    let mut vec = Vec::new();

    for (_, val) in record.clone().into_iter() {
        vec.push(val);
    }

    vec
}

impl Format for Csv {
    fn get_type(&self) -> FormatType {
        FormatType::CSV
    }

    fn from_string(&self, input: &str) -> Result<Value, Box<Error>> {
        let mut rdr = csv::Reader::from_reader(input.as_bytes());
        let mut rows: Vec<Value> = Vec::new();

        if self.headers.is_some() {
            let headers = self.headers.as_ref().unwrap().as_ref();
            let mut h = csv::StringRecord::new();

            for header in headers.into_iter() {
                h.push_field(header);
            }

            rdr.set_headers(h);
        }
        for result in rdr.deserialize().skip(self.skip) {
            if result.is_err() {
                return Err(From::from(result.unwrap_err()));
            }

            let record: serde_json::Map<String, Value> = result.unwrap();
            let json_row = match self.format {
                CsvOutputFormat::Array => Value::Array(get_map_values(record)),
                CsvOutputFormat::Object => Value::Object(record),
            };

            rows.push(json_row);
        }

        Ok(Value::Array(rows))
    }
}
