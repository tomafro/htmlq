use htmlq::*;

use std::io::Read;
use scraper::{Html, Selector};
use clap::ArgMatches;

fn main() {
    run();
}

fn run() -> htmlq::Result<()> {
    let args = cli().get_matches();

    let html = read_html(args);
    let fragment = Html::parse_fragment(&html);

    let query = args.value_of("query").unwrap();
    let selector = Selector::parse(query).unwrap();

    let mut result = fragment.select(&selector).peekable();

    if result.peek().is_none() {
        std::process::exit(1);
    }

    for element in fragment.select(&selector) {
        println!("{}", element.html());
    }

    Ok(())
}

fn read_html(args: ArgMatches) -> String {
    let mut result = String::new();

    match args.value_of("file") {
        None => std::io::BufReader::new(std::io::stdin()).read_to_string(&mut html),
        Some(filename) => std::io::BufReader::new(std::fs::File::open(filename).unwrap()).read_to_string(&mut html)
    };

    result
}
