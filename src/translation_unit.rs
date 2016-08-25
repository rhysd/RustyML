use std::path::PathBuf;
use ast::Expr;

pub struct TranslationUnit<'a> {
    pub file: &'a PathBuf,
    pub ast: Expr,
}
