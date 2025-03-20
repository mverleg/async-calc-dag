use crate::parse::{File, Op};

mod parse;

fn main() {
    use crate::parse::Expr::*;
    let file1 = File {
        imports: vec![],
        expression: BinOp(
            Op::Add,
            Box::new(BinOp(
                Op::Mul,
                Box::new(Value(2)),
                Box::new(Value(2)))),
            Box::new(BinOp(
                Op::Mul,
                Box::new(Value(3)),
                Box::new(Value(3))))),
    };
    let file = std::fs::File::create("file1.acd.json").unwrap();
    serde_json::to_writer_pretty(file, &file1).unwrap();
}
