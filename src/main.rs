mod parse;
mod exec;

fn main() {

}

#[cfg(test)]
mod test {
    use crate::exec::evaluate;
    use crate::parse::Expr::BinOp;
    use crate::parse::Expr::Value;
    use crate::parse::File;
    use crate::parse::Identifier;
    use crate::parse::Op;

    #[test]
    fn single_file() {
        let file = File {
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
        let res = evaluate(Identifier::of("test"), &file);
        assert_eq!(res, 25);
    }
}