pub mod cli;

const NAME: &'static str = env!("CARGO_PKG_NAME");
const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");
const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
}

impl std::convert::From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IO(error)
    }
}

pub struct Config {
    pub selector: Option<String>,
    pub filename: Option<String>
}

pub type Result<T> = std::result::Result<T, Error>;
