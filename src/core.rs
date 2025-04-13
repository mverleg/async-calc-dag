use crate::ast::Ast;
use crate::cache::Cache;
use crate::common::Error;
use crate::common::Identifier;
use crate::file::DiskFs;
use crate::file::File;
use crate::file::Fs;
use crate::file::MockFs;
use crate::lazy_async::ALazy;
use crate::parse::parse;

pub trait Mode {
    type FsType: Fs;
}

pub struct Prod {}
impl Mode for Prod {
    type FsType = DiskFs;
}

pub struct Test {}
impl Mode for Test {
    type FsType = MockFs;
}

pub struct Core<T: Mode> {
    fs: <T as Mode>::FsType,
    files: Cache<Identifier, File>,
    asts: Cache<Identifier, ALazy<Ast>>,
    output: Cache<Identifier, ALazy<Result<i64, Error>>>,
    // TODO: can data structure be optimized? or jut make sure to not borrow map entries long?
}

impl <T: Mode> Core<T> {
    // TODO @mverleg: for each:
    // use everywhere, don't use direct way
    // track which dependencies used by whom
    // set flag to detect cycles?
    // cache the result
    pub async fn read(&self, iden: &Identifier) -> &Result<File, Error> {
        self.files.get(iden, || self.fs.read(iden)).await
    }

    pub async fn parse(&mut self, iden: &Identifier, content: File) -> Result<Ast, Error> {
        parse(iden, content)
    }
}
