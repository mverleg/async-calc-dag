use crate::ast::Ast;
use crate::file::File;
use crate::Error;
use crate::Identifier;

pub fn parse(iden: &Identifier, content: File) -> Result<Ast, Error> {
    serde_json::from_str(content.json()).map_err(|_| Error::CouldNotParse(iden.clone()))
}

#[allow(unused)]
pub fn unparse(ast: Ast) -> File {
    File::new(serde_json::to_string_pretty(&ast).unwrap())
}


