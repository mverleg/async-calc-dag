use crate::ast::Ast;
use crate::common::Error;
use crate::parse::unparse;
use crate::Identifier;
use futures::lock::Mutex;
use std::collections::HashMap;
use std::fmt;
use tokio::fs;

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
pub struct MockFs(pub HashMap<Identifier, Mutex<Option<File>>>);

#[cfg(test)]
impl MockFs {
    pub fn new(asts: Vec<(Identifier, Ast)>) -> MockFs {
        MockFs(asts.into_iter()
            .map(|(iden, json)| (iden, Mutex::new(Some(unparse(json)))))
            .collect())
    }
}

#[cfg(test)]
impl Fs for MockFs {
    async fn read(&self, iden: &Identifier) -> Result<File, Error> {
        let Some(file_guard) = self.0.get(iden) else {
            return Err(Error::FileNotFound(iden.clone()))
        };
        Ok(file_guard.lock().await.take()
            .unwrap_or_else(|| panic!("already read this verion of {iden}; it is a bug to read the same file twice")))
    }
}