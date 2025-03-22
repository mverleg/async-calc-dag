use crate::exec::evaluate;
use crate::file::DiskFs;
use crate::file::Error;
use crate::file::Identifier;
use ::std::env::args;

mod ast;
mod parse;
mod exec;
mod file;
mod lazy_async;

#[tokio::main]
async fn main() {
    let mut argz = args();
    argz.next();
    let file = Identifier::of(&argz.next().expect("First arg should be file identifier"));
    let arg_vals = argz
        .map(|s| s.parse().expect("Second and subsequent args should be integers if provided"))
        .collect::<Vec<_>>();
    let mut fs = DiskFs::default();
    match evaluate(&mut fs, file, &arg_vals).await {
        Ok(val) => println!("{}", val),
        Err(Error::FileNotFound(f)) => println!("File not found: {}", f),
        Err(Error::CouldNotParse(f)) => println!("Could not parse: {}", f),
        Err(Error::DivideByZero(f, n)) => println!("Could not divide {} by 0 in {}", n, f),
        Err(Error::NoSuchArg(f, i)) => println!("No argument nr {} in {}", i, f),
    }
}

#[cfg(test)]
pub mod test {
    use crate::ast::Expr::Value;
    use crate::ast::Expr::{Arg, BinOp, Call};
    use crate::ast::Ast;
    use crate::ast::Op;
    use crate::exec::evaluate;
    use crate::file::{Error, MockFs};
    use crate::Identifier;

    #[tokio::test]
    async fn single_file() -> Result<(), Error> {
        let file = Ast {
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
        let file_iden = Identifier::of("test");
        let mut fs = MockFs(vec![(file_iden.clone(), file)].into_iter().collect());
        let res = evaluate(&mut fs, file_iden, &[]).await?;
        assert_eq!(res, 25);
        Ok(())
    }

    #[tokio::test]
    async fn multi_file() -> Result<(), Error> {
        let square_file = Ast {
            imports: vec![Identifier::of("square")],
            expression: BinOp(
                Op::Mul,
                Box::new(Arg(0)),
                Box::new(Arg(0)),
            ),
        };
        let main_file = Ast {
            imports: vec![],
            expression: BinOp(
                Op::Add,
                Box::new(Call(Identifier::of("square"), vec![Value(4)])),
                Box::new(Call(Identifier::of("square"), vec![Value(3)])),
            )
        };
        let main_iden = Identifier::of("main");
        let square_iden = Identifier::of("square");
        let mut fs = MockFs(vec![
            (square_iden, square_file),
            (main_iden.clone(), main_file),
        ].into_iter().collect());

        //write(Identifier::of("square"), file1).await;
        //write(Identifier::of("main"), file2).await;
        let res = evaluate(&mut fs, main_iden, &[]).await?;
        assert_eq!(res, 25);
        Ok(())
    }
}