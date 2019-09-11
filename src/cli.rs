use crate::*;
use clap::{App, Arg};

const SELECTOR_ARG: &'static str = "selector";
const FILENAME_ARG: &'static str = "file";

pub fn cli<'a, 'b>() -> App<'a, 'b> {
    App::new(NAME)
        .version(VERSION)
        .author(AUTHORS)
        .about(DESCRIPTION)
            .arg(Arg::with_name(SELECTOR_ARG)
                .required(true))
            .arg(Arg::with_name(FILENAME_ARG)
                .takes_value(true)
                .short("f")
                .long("file"))
}

pub fn config() -> Config {
    let arguments = cli().get_matches();
    Config {
        selector: arguments.value_of(SELECTOR_ARG).map(str::to_string),
        filename: arguments.value_of(FILENAME_ARG).map(str::to_string)
    }
}
