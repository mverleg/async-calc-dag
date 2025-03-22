use crate::common::Error;
use crate::Identifier;
use std::collections::HashMap;
use std::fmt;
use tokio::fs;
use crate::ast::Ast;
use crate::parse::unparse;

async fn read(iden: &Identifier) -> Result<File, Error> {
    fs::read_to_string(format!("{}.acd.json", iden.value)).await
        .map(|json| File { json })
        .map_err(|_| Error::FileNotFound(iden.clone()))
}

#[derive(Debug)]
pub struct File {
    json: String
}

impl File {
    pub fn new(json: impl Into<String>) -> File {
        File { json: json.into() }
    }
}

impl File {
    pub fn json(&self) -> &str {
        &self.json
    }
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

#[cfg(test)]
#[derive(Debug)]
pub struct MockFs(pub HashMap<Identifier, File>);

#[cfg(test)]
impl MockFs {
    pub fn new(asts: Vec<(Identifier, Ast)>) -> MockFs {
        MockFs(asts.into_iter()
            .map(|(iden, json)| (iden, unparse(json)))
            .collect())
    }
}

#[cfg(test)]
impl Fs for MockFs {
    async fn read(&self, iden: &Identifier) -> Result<File, Error> {
        eprintln!("reading {iden}");
        match self.0.get(iden) {
            None => Err(Error::FileNotFound(iden.clone())),
            Some(file) => Ok(File { json: file.json.clone() })
        }
    }
}