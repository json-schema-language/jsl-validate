use clap::{App, AppSettings, Arg};
use failure::Error;
use jsl::{Config, Schema, Validator};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{stdin, BufRead, BufReader, Read};
use std::process::exit;

fn main() -> Result<(), Error> {
    let matches = App::new("jsl-validate")
        .version("0.1")
        .about("Validates newline-delimited JSON against a JSL schema")
        .setting(AppSettings::ColoredHelp)
        .arg(
            Arg::with_name("schema")
                .help("Where to read the schema from. Dash (hypen) indicates stdin")
                .required(true),
        )
        .arg(
            Arg::with_name("INPUT")
                .help("Where to read instances from. Dash (hypen) indicates stdin")
                .default_value("-"),
        )
        .arg(
            Arg::with_name("max-errors")
                .help("Maximum number of errors to produce per instance. Zero indicates returning all errors")
                .short("e")
                .long("max-errors")
                .default_value("0"),
        )
        .arg(
            Arg::with_name("max-depth")
                .help("Maximum recursion depth for evaluation before erroring")
                .short("d")
                .long("max-depth")
                .default_value("32"),
        )
        .arg(
            Arg::with_name("strict")
                .help("Whether to use strict instance semantics. Default is to not use strict instance semantics")
                .short("s")
                .long("strict"),
        )
        .get_matches();

    let schema = BufReader::new(match matches.value_of("schema").unwrap() {
        "-" => Box::new(stdin()) as Box<Read>,
        file @ _ => Box::new(File::open(file)?) as Box<Read>,
    });

    let input = BufReader::new(match matches.value_of("INPUT").unwrap() {
        "-" => Box::new(stdin()) as Box<Read>,
        file @ _ => Box::new(File::open(file)?) as Box<Read>,
    });

    let max_errors = matches.value_of("max-errors").unwrap().parse()?;
    let max_depth = matches.value_of("max-depth").unwrap().parse()?;
    let strict_instance_semantics = matches.is_present("strict");

    let mut config = Config::new();
    config
        .max_errors(max_errors)
        .max_depth(max_depth)
        .strict_instance_semantics(strict_instance_semantics);

    let schema = Schema::from_serde(serde_json::from_reader(schema)?)?;
    let validator = Validator::new_with_config(config);

    let mut has_errors = false;
    for line in input.lines() {
        #[derive(Serialize, Deserialize)]
        struct OutputError {
            #[serde(rename = "instancePath")]
            instance_path: String,

            #[serde(rename = "schemaPath")]
            schema_path: String,
        }

        let errors: Vec<_> = validator
            .validate(&schema, &serde_json::from_str(&line?)?)?
            .into_iter()
            .map(|err| OutputError {
                instance_path: err.instance_path().to_string(),
                schema_path: err.schema_path().to_string(),
            })
            .collect();

        has_errors = has_errors || !errors.is_empty();
        println!("{}", serde_json::to_string(&errors)?);
    }

    exit(if has_errors { 1 } else { 0 });
}
