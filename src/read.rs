use tokio::fs;
use crate::ast::File;
use crate::ast::Identifier;

#[derive(Debug, Clone)]
pub enum Error {
    FileNotFound(Identifier),
    CouldNotParse(Identifier),
}

pub async fn read(iden: Identifier) -> Result<String, Error> {
    fs::read_to_string(&iden.value).await.map_err(|_| Error::FileNotFound(iden))
}

pub async fn parse(iden: Identifier, content: String) -> Result<File, Error> {
    serde_json::from_str(&content).map_err(|_| Error::CouldNotParse(iden))
}
