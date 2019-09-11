use crate::*;
use clap::{App, Arg};

pub fn cli<'a, 'b>() -> App<'a, 'b> {
    App::new(NAME)
        .version(VERSION)
        .author(AUTHORS)
        .about(DESCRIPTION)
            .arg(Arg::with_name("selector")
                .required(true))
            .arg(Arg::with_name("file")
                .takes_value(true)
                .short("f")
                .long("file"))
}
