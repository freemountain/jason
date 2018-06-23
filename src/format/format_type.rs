use std::fmt;

use std::ffi::OsStr;
use std::path::Path;

#[derive(Copy, Clone, Enum)]
pub enum FormatType {
    YAML = 0,
    JSON = 1,
    CSV = 2,
}

impl fmt::Display for FormatType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            FormatType::YAML => "yaml",
            FormatType::JSON => "json",
            FormatType::CSV => "csv",
        };
        write!(f, "{}", printable)
    }
}

impl FormatType {
    pub fn from_extension(extension: &str) -> Option<FormatType> {
        match extension {
            "yaml" => Some(FormatType::YAML),
            "yml" => Some(FormatType::YAML),
            "json" => Some(FormatType::JSON),
            "csv" => Some(FormatType::CSV),
            _ => None,
        }
    }

    pub fn from_filename(filename: &str) -> Option<FormatType> {
        let extension = Path::new(filename).extension().and_then(OsStr::to_str);

        match extension {
            Some(a) => FormatType::from_extension(a),
            None => None,
        }
    }
}
