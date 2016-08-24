use std::path::PathBuf;
use std::io;
use std::io::Read;
use std::fs::File;
use parser::{parse, ParseError};
use ast::Expr;

#[derive(Debug)]
pub enum CompileError {
    ParseError(ParseError),
    FileOpenError(io::Error),
    InternalError(String),
}

pub struct Compiler {
    pub files: Vec<PathBuf>,
    // Note: Add more flags here (e.g. optimization level)
}

impl Compiler {
    pub fn parse_file(&self, file: &PathBuf) -> Result<Expr, CompileError> {
        let mut f = match File::open(file.to_str().unwrap()) {
            Ok(opened) => opened,
            Err(err) => return Err(CompileError::FileOpenError(err)),
        };

        let mut buf = String::new();
        match f.read_to_string(&mut buf) {
            Ok(_) => {},
            Err(err) => return Err(CompileError::FileOpenError(err)),
        };

        let result = parse(buf.as_str());
        match result {
            Ok(parsed) => Ok(parsed),
            Err(err) => Err(CompileError::ParseError(err)),
        }
    }

    pub fn compile(&self) -> Result<Expr /*Temporary*/, CompileError> {
        if self.files.len() > 1 {
            return Err(CompileError::InternalError("Currently only single file can be compiled".to_string()));
        }

        let parsed = try!(self.parse_file(self.files.first().unwrap()));

        // TODO: Temporary
        Ok(parsed)
    }
}
