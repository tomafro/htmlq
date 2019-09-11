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
    let fragment = Html::parse_fragment(&html?);

    if let Some(query) = &config.selector {
        let selector = Selector::parse(&query).unwrap();

        let mut result = fragment.select(&selector).peekable();

        if result.peek().is_none() {
            std::process::exit(1);
        }

        for element in fragment.select(&selector) {
            output(&config, &element);
        }
    }
    else {
        output(&config, &fragment.root_element());
    }

    Ok(())
}

fn output(config: &Config, element: &scraper::element_ref::ElementRef) {
    match &config.output {
        Some(output) => match output.as_ref() {
            "inner" => println!("{}", element.inner_html()),
            "text"  => println!("{}", element.text().collect::<Vec<_>>().join(" ")),
            _       => println!("{}", element.html())
        },
        _ => println!("{}", element.html())
    }
}

fn read_html(config: &Config) -> htmlq::Result<String> {
    let mut result = String::new();

    match &config.filename {
        None => std::io::BufReader::new(std::io::stdin()).read_to_string(&mut result)?,
        Some(filename) => std::io::BufReader::new(std::fs::File::open(filename)?).read_to_string(&mut result)?
    };

    Ok(result)
}
