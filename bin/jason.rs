use std::fs::File;
use std::io::prelude::*;

extern crate jason;

use jason::{formats, FormatDispatcher, FormatType};

extern crate clap;
extern crate serde_yaml;

use clap::{App, Arg};

extern crate serde_json;

fn read_file(path: &str) -> Result<String, std::io::Error> {
    let r = File::open(path);

    if r.is_err() {
        Err(r.unwrap_err())
    } else {
        let mut f = r.unwrap();
        let mut buffer = String::new();
        f.read_to_string(&mut buffer).and(Ok(buffer))
    }
}

fn read_stdin() -> Result<String, std::io::Error> {
    let mut buffer = String::new();

    let result = std::io::stdin().read_to_string(&mut buffer);

    match result {
        Err(e) => Err(e),
        Ok(_) => Ok(buffer),
    }
}

fn split(input: &str) -> Vec<String> {
    let mut vec = Vec::new();

    for part in input.split(',') {
        vec.push(String::from(part));
    }

    vec
}

fn create_csv(agrs: &clap::ArgMatches) -> Box<formats::Csv> {
    let skip = agrs.value_of("CSV:SKIP")
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(0);

    let format = agrs.value_of("CSV:FORMAT")
        .and_then(formats::CsvOutputFormat::from_str)
        .unwrap_or(formats::CsvOutputFormat::Object);

    let headers = agrs.value_of("CSV:HEADERS").map(split).map(Box::new);

    Box::new(formats::Csv {
        format: format,
        headers: headers,
        skip: skip,
    })
}

fn main() {
    let matches = App::new("Jason")
        .version("0.0.1")
        .about("convert csv or yaml to json")
        .arg(
            Arg::with_name("INPUT")
                .takes_value(true)
                .short("i")
                .long("input")
                .help("input to parse"),
        )
        .arg(
            Arg::with_name("FORMAT")
                .takes_value(true)
                .short("f")
                .long("format"),
        )
        .arg(
            Arg::with_name("CSV:FORMAT")
                .takes_value(true)
                .long("csv:format"),
        )
        .arg(
            Arg::with_name("CSV:SKIP")
                .takes_value(true)
                .long("csv:skip"),
        )
        .arg(
            Arg::with_name("CSV:HEADERS")
                .takes_value(true)
                .long("csv:headers"),
        )
        .get_matches();

    let mut dispatcher = FormatDispatcher::new();
    dispatcher.add_format(Box::new(formats::Json {}));
    dispatcher.add_format(Box::new(formats::Yaml {}));
    dispatcher.add_format(create_csv(&matches));

    let input_arg = matches.value_of("INPUT");
    let format_arg = matches.value_of("FORMAT");

    let parsed_format_arg = format_arg.and_then(FormatType::from_extension);
    let parsed_input_format = input_arg.and_then(FormatType::from_filename);

    let format = parsed_format_arg
        .or(parsed_input_format)
        .expect("Please set the --format argument");

    let content = (match input_arg {
        Some(f) => read_file(f),
        None => read_stdin(),
    }).unwrap();

    let parsed = dispatcher.from_string(format, content.as_str()).unwrap();

    println!("{}", serde_json::to_string_pretty(&parsed).unwrap());
}
