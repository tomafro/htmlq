pub mod cli;

const NAME: &'static str = env!("CARGO_PKG_NAME");
const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");
const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");

#[derive(Debug)]
pub enum Error {
    Problem,
}

pub struct Config {
    pub selector: Option<String>,
    pub filename: Option<String>
}

pub type Result<T> = std::result::Result<T, Error>;
