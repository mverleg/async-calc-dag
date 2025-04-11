use crate::ast::Ast;
use crate::common::Error;
use crate::common::Identifier;
use crate::file::File;
use crate::file::Fs;
use crate::lazy_async::ALazy;
use tokio::fs;

trait Mode {
    type FsType: Fs;
}

struct Prod {}
impl Mode for Prod {
    type FsType = ();
}

struct Test {}
impl Mode for Test {
    type FsType = ();
}

pub struct Core<T: Mode> {
    fs: Mode::FsType,
    files: Vec<ALazy<File>>,
    asts: Vec<ALazy<Ast>>,
    output: Vec<ALazy<Result<i64, Error>>>,
}

impl Core {
    pub async fn read(&mut self, iden: &Identifier) -> Result<File, Error> {
        self.fs.read(iden).await
    }

    pub async fn parse(&mut self, iden: &Identifier, content: File) -> Result<Ast, Error> {
        parse()
    }
}
