use crate::*;
use clap::{App, Arg};

const SELECTOR_ARG: &'static str = "selector";
const FILENAME_ARG: &'static str = "file";
const OUTPUT_ARG: &'static str = "output";

pub fn cli<'a, 'b>() -> App<'a, 'b> {
    App::new(NAME)
        .version(VERSION)
        .author(AUTHORS)
        .about(DESCRIPTION)
            .arg(Arg::with_name(FILENAME_ARG)
                .takes_value(true)
                .short("f")
                .long("file"))
            .arg(Arg::with_name(OUTPUT_ARG)
                .takes_value(true)
                .short("o")
                .long("output")
                .possible_values(&["outer", "inner", "text"]))
            .arg(Arg::with_name(SELECTOR_ARG))
}

pub fn config() -> Config {
    let arguments = cli().get_matches();

    Config {
        selector: arguments.value_of(SELECTOR_ARG).map(str::to_string),
        filename: arguments.value_of(FILENAME_ARG).map(str::to_string),
        output: arguments.value_of(OUTPUT_ARG).map(str::to_string)
    }
}
