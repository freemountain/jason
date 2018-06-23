
use std::error::Error;

extern crate serde_json;
extern crate enum_map;

mod format_type;
mod format_dispatcher;

pub use self::format_type::FormatType;
pub use self::format_dispatcher::FormatDispatcher;



pub trait Format {
    fn get_type(&self) -> FormatType;

    fn from_string(&self, input: &str) -> Result<serde_json::Value, Box<Error>>;
}
