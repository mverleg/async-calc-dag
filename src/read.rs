use std::fmt;
use crate::ast::File;
use serde::Deserialize;
use serde::Serialize;
use tokio::fs;

#[derive(Debug, Deserialize, Serialize, Clone)]
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
}

pub async fn read(iden: &Identifier) -> Result<String, Error> {
    fs::read_to_string(format!("{}.acd.json", iden.value)).await.map_err(|_| Error::FileNotFound(iden.clone()))
}

pub async fn parse(iden: &Identifier, content: String) -> Result<File, Error> {
    serde_json::from_str(&content).map_err(|_| Error::CouldNotParse(iden.clone()))
}
