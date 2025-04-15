use crate::ast::Ast;
use crate::common::{HasId, Error};
use crate::common::Share;
use crate::parse::unparse;
use crate::Identifier;
use ::std::collections::HashMap;
use ::std::fmt;
use ::tokio::fs;
use ::tokio::sync::Mutex;
use arcstr::ArcStr;
use hipstr::LocalHipStr;

async fn read(iden: &Identifier) -> Result<File, Error> {
    fs::read_to_string(format!("{}.acd.json", iden.value)).await
        .map(|json| File { json: ArcStr::from(json) })
        .map_err(|_| Error::FileNotFound(iden.clone()))
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct File {
    json: ArcStr,
}

impl File {
    pub fn new(json: impl Into<ArcStr>) -> File {
        File { json: json.into() }
    }
}

impl Share for File {
    fn share(&self) -> File {
        File { json: self.json.clone() }
    }
}

impl File {
    pub fn json(&self) -> &str {
        &self.json
    }
}

impl HasId for File {
    type Uid = ArcStr;
    // TODO @mverleg: have some kind of fast identity, like name+timestamp? or just pre-compute string hash

    fn id(&self) -> Self::Uid {
        self.json.clone()
    }
}

pub trait Fs: fmt::Debug {
    async fn read(&self, iden: &Identifier) -> Result<File, Error>;
}

#[derive(Debug, Default)]
pub struct DiskFs();

#[derive(Debug)]
pub struct MockFs(pub HashMap<Identifier, Mutex<Option<File>>>);

impl HasId for DiskFs {
    type Uid = LocalHipStr<'static>;

    fn id(&self) -> Self::Uid {
        // TODO: replace by real path
        LocalHipStr::from("/path/to/fs")
    }
}

impl Fs for DiskFs {
    async fn read(&self, iden: &Identifier) -> Result<File, Error> {
        Ok(read(&iden).await?)
    }
}

impl MockFs {
    #[allow(unused)]
    pub fn new(asts: Vec<(Identifier, Ast)>) -> MockFs {
        MockFs(asts.into_iter()
            .map(|(iden, json)| (iden, Mutex::new(Some(unparse(json)))))
            .collect())
    }
}

impl HasId for MockFs {
    type Uid = ();

    fn id(&self) -> Self::Uid {
        // TODO: probably each mock should be different, but it hasn't come up
        ()
    }
}

impl Fs for MockFs {
    async fn read(&self, iden: &Identifier) -> Result<File, Error> {
        let Some(file_guard) = self.0.get(iden) else {
            return Err(Error::FileNotFound(iden.clone()))
        };
        Ok(file_guard.lock().await.take()
            .unwrap_or_else(|| panic!("already read this verion of {iden}; it is a bug to read the same file twice")))
    }
}
