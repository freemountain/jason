use std::error::Error;
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

fn create_dispatcher(matches: &clap::ArgMatches) -> FormatDispatcher {
    let mut dispatcher = FormatDispatcher::new();

    dispatcher.add_format(Box::new(formats::Json {}));
    dispatcher.add_format(Box::new(formats::Yaml {}));
    dispatcher.add_format(create_csv(&matches));

    dispatcher
}

fn get_format_arg(matches: &clap::ArgMatches, name: &str) -> FormatType {
    matches
        .value_of(name)
        .and_then(FormatType::from_extension)
        .unwrap_or(FormatType::JSON)
}

fn parse_args<'a>() -> clap::ArgMatches<'a> {
    let format_args = vec![
        Arg::with_name("CSV:FORMAT")
            .takes_value(true)
            .long("csv:format"),
        Arg::with_name("CSV:SKIP")
            .takes_value(true)
            .long("csv:skip"),
        Arg::with_name("CSV:HEADERS")
            .takes_value(true)
            .long("csv:headers"),
    ];

    let args = vec![
        Arg::with_name("INPUT")
            .takes_value(true)
            .short("i")
            .long("input")
            .help("input format"),
        Arg::with_name("OUTPUT")
            .takes_value(true)
            .short("o")
            .long("output")
            .help("output format"),
    ];

    App::new("Jason")
        .version("0.0.1")
        .about("convert csv or yaml to json")
        .args(&args)
        .args(&format_args)
        .get_matches()
}

fn run() -> Option<(&'static str, Box<Error>)> {
    let args = parse_args();
    let input_format = get_format_arg(&args, "INPUT");
    let output_format = get_format_arg(&args, "OUTPUT");
    let dispatcher = create_dispatcher(&args);

    let may_input = read_stdin();
    if may_input.is_err() {
        return Some(("could not read stdin", Box::new(may_input.unwrap_err())));
    }
    let input = may_input.unwrap();

    let may_parsed = dispatcher.from_string(input_format, input.as_str());
    if may_parsed.is_err() {
        let error: Box<Error> = may_parsed.unwrap_err();
        return Some(("could parse input", error));
    }
    let parsed = may_parsed.unwrap();


    let may_stringified = dispatcher.to_string(output_format, &parsed);
    if may_stringified.is_err() {
        let error: Box<Error> = may_stringified.unwrap_err();
        return Some(("could stringify input", error));
    }
    let stringified = may_stringified.unwrap();
    println!("{}", stringified);

    None
}

fn main() {

    if let Some((reason, error)) = run() {
        println!("{} {}", reason, error);
        std::process::exit(1);
    }
}
