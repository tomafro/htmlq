use std::fs;
use scraper::{Html, Selector};
use clap::{App, Arg};

const NAME: &'static str = env!("CARGO_PKG_NAME");
const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");
const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");

pub fn cli<'a, 'b>() -> App<'a, 'b> {
    App::new(NAME)
        .version(VERSION)
        .author(AUTHORS)
        .about(DESCRIPTION)
            .display_order(0)
            .arg(Arg::with_name("query")
                .required(true))
            .arg(Arg::with_name("file")
                .required(true))
}

fn main() {
    let matches = cli().get_matches();

    let query = matches.value_of("query").unwrap();
    let filename = matches.value_of("file").unwrap();

    let html = fs::read_to_string(filename).unwrap();

    let fragment = Html::parse_fragment(&html);
    let selector = Selector::parse(query).unwrap();

    let mut result = fragment.select(&selector).peekable();

    if result.peek().is_none() {
        std::process::exit(1);
    }

    for element in fragment.select(&selector) {
        println!("{}", element.html());
    }
}
