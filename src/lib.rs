

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
            .display_order(0)
            .arg(Arg::with_name("query")
                .required(true))
            .arg(Arg::with_name("file"))
}
