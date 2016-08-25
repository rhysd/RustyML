use parser::ParseError;
use std::io;
use std::result;

#[derive(Debug)]
pub enum Error {
    OnParse(ParseError),
    OnFileOpen(io::Error),
    OnFatal(String),
}

pub type Result<T> = result::Result<T, Error>;
