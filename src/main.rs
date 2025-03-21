use std::env::args;
use crate::exec::evaluate;
use crate::read::Error;
use crate::read::Identifier;

mod ast;
mod exec;
mod read;

#[tokio::main]
async fn main() {
    let mut argz = args();
    argz.next();
    let file = Identifier::of(&argz.next().expect("First arg should be file identifier"));
    let arg_vals = argz
        .map(|s| s.parse().expect("Second and subsequent args should be integers if provided"))
        .collect::<Vec<_>>();
    match evaluate(file, &arg_vals).await {
        Ok(val) => println!("{}", val),
        Err(Error::FileNotFound(f)) => println!("File not found: {}", f),
        Err(Error::CouldNotParse(f)) => println!("Could not parse: {}", f),
        Err(Error::DivideByZero(f, n)) => println!("Could not divide {} by 0 in {}", n, f),
        Err(Error::NoSuchArg(f, i)) => println!("No argument nr {} in {}", i, f),
    }
}

#[cfg(test)]
pub mod test {
    use crate::Identifier;
    use crate::read::Error;
    use crate::ast::Expr::{Arg, BinOp, Call};
    use crate::ast::Expr::Value;
    use crate::ast::File;
    use crate::ast::Op;
    use crate::exec::evaluate_file;

    #[tokio::test]
    async fn single_file() -> Result<(), Error> {
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
        let res = evaluate_file(Identifier::of("test"), file, &[]).await?;
        assert_eq!(res, 25);
        Ok(())
    }

    #[tokio::test]
    async fn multi_file() -> Result<(), Error> {
        let file1 = File {
            imports: vec![Identifier::of("square")],
            expression: BinOp(
                Op::Mul,
                Box::new(Arg(0)),
                Box::new(Arg(0)),
            ),
        };
        let file2 = File {
            imports: vec![Identifier::of("square")],
            expression: BinOp(
                Op::Add,
                Box::new(Call(Identifier::of("square"), vec![Value(4)])),
                Box::new(Call(Identifier::of("square"), vec![Value(3)])),
                )
        };
        //write(Identifier::of("square"), file1).await;
        //write(Identifier::of("main"), file2).await;
        //let res = evaluate_file(Identifier::of("test"), file, &[]).await?;
        //assert_eq!(res, 25);
        Ok(())
    }
}