peg_file! grammar("grammar.rustpeg");

use std::io;
use std::io::Read;
use std::fs::File;
use std::path::PathBuf;

use error::{Result, Error};
use translation_unit::TranslationUnit;
use ast::Expr;

pub type ParseError = grammar::ParseError;

pub fn set_line_col(ast: Expr) -> Expr {
    ast
}

pub fn parse_raw(code: &str) -> grammar::ParseResult<Expr> {
    grammar::expr(code).map(|ast| set_line_col(ast))
}

fn read_from(file: &PathBuf) -> io::Result<String> {
    let mut f = try!(File::open(file.to_str().unwrap()));
    let mut buf = String::new();
    try!(f.read_to_string(&mut buf));
    Ok(buf)
}

pub fn parse(file: &PathBuf) -> Result<TranslationUnit> {
    let result = match read_from(file) {
        Ok(code) => parse_raw(code.as_str()),
        Err(e) => return Err(Error::OnFileOpen(e)),
    };
    match result {
        Ok(parsed) => Ok(TranslationUnit { file: file, ast: parsed }),
        Err(err) => Err(Error::OnParse(err)),
    }
}
