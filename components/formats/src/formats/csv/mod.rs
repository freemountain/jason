use std::error::Error;

use csv;
use serde::de::Error as SerdeError;
use serde::de::Unexpected;
use serde_json;
use serde_json::Value;

use super::super::{Format, Type};

mod csv_error;
mod csv_output_format;
pub use self::csv_error::CsvError;
pub use self::csv_output_format::CsvOutputFormat;

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
/*
fn to_simple<'a>(buffer: &'a mut String, value: &'a Value) -> Option<&'a str> {
    match value {
        &Value::String(ref s) => Some(&s[..]),
        Value::Number(ref n) => {
            let n_f64 = n.as_f64().map(|v| v.to_string());
            let n_i64 = n.as_i64().map(|v| v.to_string());
            let n_u64 = n.as_u64().map(|v| v.to_string());

            n_f64.or(n_i64).or(n_u64).and_then(move |v| {
                let offset = buffer.len();
                buffer.push_str(&v);

                buffer.get(offset..v.len())
            })
        }
        Value::Bool(b) => match b {
            true => Some("true"),
            false => Some("false"),
        },
        _ => None,
    }
}
*/

fn to_simple(value: &Value) -> Result<String, serde_json::Error> {
    match value {
        &Value::String(ref s) => Ok(s.to_string()),
        Value::Number(ref n) => {
            let n_f64 = n.as_f64().map(|v| v.to_string());
            let n_i64 = n.as_i64().map(|v| v.to_string());
            let n_u64 = n.as_u64().map(|v| v.to_string());

            Ok(n_f64.or(n_i64).or(n_u64).unwrap())
        }
        Value::Bool(b) => match b {
            true => Ok("true".to_string()),
            false => Ok("false".to_string()),
        },
        Value::Object(_) => Err(serde_json::Error::invalid_type(
            Unexpected::Other("json object"),
            &"a string,number or bool",
        )),
        Value::Array(_) => Err(serde_json::Error::invalid_type(
            Unexpected::Other("json array"),
            &"a string,number or bool",
        )),
        Value::Null => Err(serde_json::Error::invalid_type(
            Unexpected::Other("json null"),
            &"a string,number or bool",
        )),
    }
}

fn to_simple_csv(value: Value, row: usize, field: String) -> Result<String, CsvError> {
    to_simple(&value).map_err(|err| CsvError::JsonAt(err, row, field))
}

fn get_fields_from_array(array: Vec<Value>, row: usize) -> Vec<Result<String, CsvError>> {
    array
        .into_iter()
        .enumerate()
        .map(|(index, v)| to_simple_csv(v, row, index.to_string()))
        .collect()
}

fn get_fields_from_object(
    headers: &Vec<String>,
    object: serde_json::Map<String, Value>,
    row: usize,
) -> Vec<Result<String, CsvError>> {
    headers
        .iter()
        .map(|key| match object.get(key) {
            Some(v) => to_simple_csv(v.clone(), row, key.to_string()),
            None => Err(CsvError::UnkownFieldAt(row, key.to_string())),
        })
        .collect()
}

fn _get_fields(
    headers: Option<&Vec<String>>,
    row_number: usize,
    row: Value,
) -> Result<Vec<String>, CsvError> {
    let may_mapped_row = match headers {
        None => serde_json::from_value::<Vec<Value>>(row)
            .map_err(|e| CsvError::JsonAt(e, row_number, "".to_string()))
            .map(|v| get_fields_from_array(v, row_number)),
        Some(h) => serde_json::from_value::<serde_json::Map<String, Value>>(row)
            .map_err(|e| CsvError::JsonAt(e, row_number, "".to_string()))
            .map(|r| get_fields_from_object(h, r, row_number)),
    };

    match may_mapped_row {
        Ok(mapped_row) => {
            let all_fields_valid = mapped_row.iter().all(|field| field.is_ok());

            match all_fields_valid {
                true => Ok(mapped_row
                    .into_iter()
                    .map(|field| field.unwrap())
                    .collect::<Vec<String>>()),
                false => {
                    let errors: Vec<CsvError> = mapped_row
                        .into_iter()
                        .filter(|field| field.is_err())
                        .map(|field| field.unwrap_err())
                        .collect();
                    Err(CsvError::Multiple(errors))
                }
            }
        }
        Err(err) => Err(err),
    }
}

impl Csv {
    fn get_fields_from_object(
        &self,
        object: serde_json::Map<String, Value>,
        row: usize,
    ) -> Option<Vec<Result<String, CsvError>>> {
        self.headers.clone().map(|headers| {
            headers
                .iter()
                .map(|key| match object.get(key) {
                    Some(v) => to_simple_csv(v.clone(), row, key.to_string()),
                    None => Err(CsvError::UnkownFieldAt(row, key.to_string())),
                })
                .collect()
        })
    }

    fn get_row_record(&self, row_number: usize, row: Value) -> Result<csv::StringRecord, CsvError> {
        let as_array = |row| {
            serde_json::from_value::<Vec<Value>>(row)
                .map_err(|e| CsvError::JsonAt(e, row_number, "".to_string()))
                .map(|v| get_fields_from_array(v, row_number))
        };

        let as_object = |row| {
            serde_json::from_value::<serde_json::Map<String, Value>>(row)
                .map_err(|e| CsvError::JsonAt(e, row_number, "".to_string()))
                .map(|r| self.get_fields_from_object(r, row_number).unwrap())
        };

        let may_mapped_row = match self.headers {
            None => as_array(row),
            Some(_) => as_object(row),
        };

        let as_record = |mapped_row: &Vec<Result<String, CsvError>>| {
            let mut record = csv::StringRecord::new();

            for field_option in mapped_row.iter() {
                let field = field_option.as_ref().unwrap();
                record.push_field(&field);
            }

            record
        };

        match may_mapped_row {
            Ok(mapped_row) => {
                let all_fields_valid = mapped_row.iter().all(|field| field.is_ok());

                match all_fields_valid {
                    true => Ok(as_record(&mapped_row)),
                    false => {
                        let errors: Vec<CsvError> = mapped_row
                            .into_iter()
                            .filter(|field| field.is_err())
                            .map(|field| field.unwrap_err())
                            .collect();
                        Err(CsvError::Multiple(errors))
                    }
                }
            }
            Err(err) => Err(err),
        }
    }

    fn get_header_record(&self) -> Option<csv::StringRecord> {
        if !self.headers.is_some() {
            None
        } else {
            let headers = self.headers.as_ref().unwrap().as_ref();
            let mut h = csv::StringRecord::new();

            for header in headers.into_iter() {
                h.push_field(&header);
            }

            Some(h)
        }
    }

    fn write_rows(&self, rows: Vec<Value>) -> Result<String, CsvError> {
        let mut writer = csv::WriterBuilder::new().from_writer(vec![]);
        let header_record = self.get_header_record();
        if header_record.is_some() {
            writer
                .write_byte_record(&header_record.unwrap().into_byte_record())
                .unwrap();
        }

        let written_rows: Vec<Result<(), CsvError>> = rows.into_iter()
            .skip(self.skip)
            .enumerate()
            .map(|(i, row)| {
                self.get_row_record(i, row).and_then(|record| {
                    writer
                        .write_byte_record(&record.into_byte_record())
                        .map_err(|err| CsvError::Csv(err))
                })
            })
            .collect();

        let collect_errors = |results: Vec<Result<(), CsvError>>| {
            let errors = results
                .into_iter()
                .filter(|result| result.is_err())
                .map(|result| result.unwrap_err())
                .collect::<Vec<CsvError>>();
            CsvError::Multiple(errors)
        };

        match written_rows.iter().all(|r| r.is_ok()) {
            true => {
                writer.flush().unwrap();
                let u8_buffer = writer.into_inner().unwrap();
                let string_buffer = String::from_utf8(u8_buffer).unwrap();

                Ok(string_buffer)
            }
            false => Err(collect_errors(written_rows)),
        }
    }
}

impl Format for Csv {
    fn get_type(&self) -> Type {
        Type::CSV
    }

    fn to_string(&self, input: &Value) -> Result<String, Box<Error>> {
        serde_json::from_value::<Vec<Value>>(input.clone())
            .map_err(|err| CsvError::Json(err))
            .and_then(|r| self.write_rows(r))
            .map_err(|err| Box::new(err) as Box<Error>)
    }

    fn from_string(&self, input: &str) -> Result<Value, Box<Error>> {
        let mut rdr = csv::Reader::from_reader(input.as_bytes());
        let mut rows: Vec<Value> = Vec::new();

        let headers = self.get_header_record();
        if headers.is_some() {
            rdr.set_headers(headers.unwrap());
        }

        for result in rdr.deserialize().skip(self.skip) {
            if result.is_err() {
                return Err(Box::new(CsvError::Csv(result.unwrap_err()))); //From::from(result.unwrap_err()));
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
