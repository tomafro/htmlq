use htmlq::*;
use htmlq::cli::config;

use std::io::Read;
use scraper::{Html, Selector};

fn main() {
    run().unwrap();
}

fn run() -> htmlq::Result<()> {
    let config = config();

    let html = read_html(&config);
    let fragment = Html::parse_fragment(&html);

    let query = config.selector.unwrap();
    let selector = Selector::parse(&query).unwrap();

    let mut result = fragment.select(&selector).peekable();

    if result.peek().is_none() {
        std::process::exit(1);
    }

    for element in fragment.select(&selector) {
        println!("{}", element.html());
    }

    Ok(())
}

fn read_html(config: &Config) -> String {
    let mut result = String::new();

    match &config.filename {
        None => std::io::BufReader::new(std::io::stdin()).read_to_string(&mut result),
        Some(filename) => std::io::BufReader::new(std::fs::File::open(filename).unwrap()).read_to_string(&mut result)
    };

    result
}
