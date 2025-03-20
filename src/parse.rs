
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Identifier {
    pub value: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Min,
    Max,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Expr {
    Value(i64),
    BinOp(Op, Box<Expr>, Box<Expr>),
    Arg(u32),
    Call(Identifier, Vec<Expr>),
    Delay(Box<Expr>, u32),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct File {
    pub imports: Vec<Identifier>,
    pub expression: Expr,
}
