use std::path::PathBuf;

use error::Error;
use translation_unit::TranslationUnit;
use parser::parse;

pub fn compile<'a>(files: &'a Vec<PathBuf>) -> Result<Vec<TranslationUnit<'a>>, Error> {
    if files.len() > 1 {
        return Err(Error::OnFatal("Currently only single file can be compiled".to_string()));
    }

    let units = try!(
        files.iter().map(|f| parse(f)).collect::<Result<Vec<_>, _>>()
    );

    Ok(units) // TODO: Temporary
}

