use clap::{App, Arg};

const NAME: &'static str = env!("CARGO_PKG_NAME");
const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");
const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");

#[derive(Debug)]
pub enum Error {
    Problem,
}

pub type Result<T> = std::result::Result<T, Error>;

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
