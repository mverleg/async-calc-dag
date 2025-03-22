use crate::ast::Ast;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::fmt;
use tokio::fs;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
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

async fn read(iden: &Identifier) -> Result<File, Error> {
    fs::read_to_string(format!("{}.acd.json", iden.value)).await
        .map(|json| File { json })
        .map_err(|_| Error::FileNotFound(iden.clone()))
}

#[allow(unused)]
pub async fn write(iden: Identifier, file: Ast) {
    let json = serde_json::to_string_pretty(&file).unwrap();
    fs::write(format!("{}.acd.json", iden.value), json).await.unwrap();
}

#[derive(Debug)]
pub struct File {
    json: String
}

pub trait Fs: fmt::Debug {
    async fn read(&self, iden: &Identifier) -> Result<File, Error>;
}

#[derive(Debug, Default)]
pub struct DiskFs();

impl Fs for DiskFs {
    async fn read(&self, iden: &Identifier) -> Result<File, Error> {
        Ok(read(&iden).await?)
    }
}

#[derive(Debug)]
pub struct MockFs(pub HashMap<Identifier, File>);

impl Fs for MockFs {
    async fn read(&self, iden: &Identifier) -> Result<File, Error> {
        eprintln!("reading {iden}");
        match self.0.get(iden) {
            None => Err(Error::FileNotFound(iden.clone())),
            Some(file) => Ok(File { json: file.json.clone() })
        }
    }
}