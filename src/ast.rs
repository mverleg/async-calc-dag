use crate::Identifier;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Min,
    Max,
    Lt,
    Gt,
    Eq,
    And,
    Or,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Expr {
    Value(i64),
    BinOp(Op, Box<Expr>, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    Arg(u32),
    Call(Identifier, Vec<Expr>),
    Delay(Box<Expr>, u32),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct File {
    pub imports: Vec<Identifier>,
    pub expression: Expr,
}
