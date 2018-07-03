use std::fmt;

#[derive(Debug, Clone)]
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
