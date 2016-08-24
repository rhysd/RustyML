peg_file! grammar("grammar.rustpeg");

use ast::Expr;

pub type ParseError = grammar::ParseError;
pub type ParseResult = Result<Expr, ParseError>;

pub fn parse(code: &str) -> ParseResult {
    grammar::expr(code)
}
