use crate::ast::Ast;
use crate::file::File;
use crate::file::Error;
use crate::file::Identifier;

pub fn parse(iden: &Identifier, content: File) -> Result<Ast, Error> {
    serde_json::from_str(&content.json).map_err(|_| Error::CouldNotParse(iden.clone()))
}

