use std::error::Error;

use super::super::formats::formats;
use super::super::formats::{Dispatcher, Type};
use super::super::utils::{read_stdin, split};

use super::clap::{App, Arg, ArgMatches};
use super::CommandResult;

fn create_csv(agrs: &ArgMatches) -> Box<formats::Csv> {
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

fn create_dispatcher(matches: &ArgMatches) -> Dispatcher {
    let mut dispatcher = Dispatcher::new();

    dispatcher.add_format(Box::new(formats::Json {}));
    dispatcher.add_format(Box::new(formats::Yaml {}));
    dispatcher.add_format(create_csv(&matches));

    dispatcher
}

fn get_format_arg(matches: &ArgMatches, name: &str) -> Type {
    matches
        .value_of(name)
        .and_then(Type::from_extension)
        .unwrap_or(Type::JSON)
}

pub fn add_args<'a, 'b>(cmd: App<'a, 'b>) -> App<'a, 'b> {
    cmd.version("0.0.1")
        .about("convert csv or yaml to json")
        .args(&vec![
            Arg::with_name("CSV:FORMAT")
                .takes_value(true)
                .long("csv:format"),
            Arg::with_name("CSV:SKIP")
                .takes_value(true)
                .long("csv:skip"),
            Arg::with_name("CSV:HEADERS")
                .takes_value(true)
                .long("csv:headers"),
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
        ])
}

pub fn run<'a>(args: &ArgMatches) -> CommandResult {
    let input_format = get_format_arg(&args, "INPUT");
    let output_format = get_format_arg(&args, "OUTPUT");
    let dispatcher = create_dispatcher(&args);

    let may_input = read_stdin();
    if may_input.is_err() {
        return CommandResult::from_message_with_error(
            "could not read stdin",
            Box::new(may_input.unwrap_err()),
        );
    }
    let input = may_input.unwrap();

    let may_parsed = dispatcher.from_string(input_format, input.as_str());
    if may_parsed.is_err() {
        let error: Box<Error> = may_parsed.unwrap_err();
        return CommandResult::from_message_with_error("could parse input", error);
    }
    let parsed = may_parsed.unwrap();

    let may_stringified = dispatcher.to_string(output_format, &parsed);
    if may_stringified.is_err() {
        let error: Box<Error> = may_stringified.unwrap_err();
        return CommandResult::from_message_with_error("could stringify input", error);
    }
    let stringified = may_stringified.unwrap();
    println!("{}", stringified);
    CommandResult::success()
}
