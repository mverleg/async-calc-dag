mod ast;
mod exec;
mod read;

fn main() {

}

#[cfg(test)]
mod test {
    use crate::exec::evaluate;
    use crate::ast::Expr::BinOp;
    use crate::ast::Expr::Value;
    use crate::ast::File;
    use crate::ast::Identifier;
    use crate::ast::Op;

    #[test]
    fn single_file() {
        let file = File {
            imports: vec![],
            expression: BinOp(
                Op::Add,
                Box::new(BinOp(
                    Op::Mul,
                    Box::new(Value(4)),
                    Box::new(Value(4)))),
                Box::new(BinOp(
                    Op::Mul,
                    Box::new(Value(3)),
                    Box::new(Value(3))))),
        };
        let res = evaluate(Identifier::of("test"), &file);
        assert_eq!(res, 25);
    }
}