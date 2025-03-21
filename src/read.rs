use std::fmt;
use crate::ast::File;
use serde::Deserialize;
use serde::Serialize;
use tokio::fs;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(transparent)]
pub struct Identifier {
    pub value: String,
}

impl Identifier {
    pub fn of(value: impl Into<String>) -> Self {
        // should validate input
        Self { value: value.into() }
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug, Clone)]
pub enum Error {
    FileNotFound(Identifier),
    CouldNotParse(Identifier),
    DivideByZero(Identifier, i64),
    NoSuchArg(Identifier, u32),
}

pub async fn read(iden: &Identifier) -> Result<String, Error> {
    fs::read_to_string(format!("{}.acd.json", iden.value)).await.map_err(|_| Error::FileNotFound(iden.clone()))
}

#[allow(unused)]
pub async fn write(iden: Identifier, file: File) {
    let json = serde_json::to_string_pretty(&file).unwrap();
    fs::write(format!("{}.acd.json", iden.value), json).await.unwrap();
}

pub async fn parse(iden: &Identifier, content: String) -> Result<File, Error> {
    serde_json::from_str(&content).map_err(|_| Error::CouldNotParse(iden.clone()))
}
