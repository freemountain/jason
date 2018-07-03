extern crate enum_map;
#[macro_use]
extern crate enum_map_derive;

#[macro_use]
extern crate serde_json;
extern crate csv;
extern crate serde_yaml;
extern crate serde;

#[macro_use]
extern crate quick_error;

mod format;

pub mod formats;
pub use format::{Format, FormatDispatcher, FormatType};
