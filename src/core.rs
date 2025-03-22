use crate::ast::Ast;
use crate::common::Error;
use crate::file::File;
use crate::lazy_async::ALazy;

pub struct Core {
    files: Vec<ALazy<File>>,
    asts: Vec<ALazy<Ast>>,
    output: Vec<ALazy<Result<i64, Error>>>,
}

impl Core {
    
}
