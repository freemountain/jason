extern crate clap;
extern crate jason;
extern crate serde_json;

use jason::commands::{convert, jsonnet, CommandResult};
use clap::{App, SubCommand};

fn run() -> CommandResult {
    let app = App::new("jason").version("0.0.1");

    let convert_name = "convert";
    let convert_cmd = convert::add_args(SubCommand::with_name(convert_name));

    let jsonnet_name = "jsonnet";
    let jsonnet_cmd = jsonnet::add_args(SubCommand::with_name(jsonnet_name));

    let matches: clap::ArgMatches<'static> = app.subcommand(convert_cmd).subcommand(jsonnet_cmd).get_matches();
    let (cmd_name, may_cmd_matches) = matches.subcommand();

    if cmd_name == "" {
        return CommandResult::from_error_message("No subcommand was used");
    }

    if may_cmd_matches.is_none() {
        return CommandResult::from_error_message("Could not parse arguments");
    }

    let cmd_matches = may_cmd_matches.unwrap();

    if cmd_name == convert_name {
        convert::run(cmd_matches)
    } else if cmd_name == jsonnet_name {
        jsonnet::run(cmd_matches)
    } else {
        CommandResult::from_error_message("format!(could not find command {}, cmd_name).as_str()")
    }
}

fn main() {
    let result = run();
    let msg = result.error.map(|cause| format!("{}", cause));
    if msg.is_some() {
        eprintln!("{}", msg.unwrap());
    }
    std::process::exit(result.code);
}
