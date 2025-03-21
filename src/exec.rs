use ::futures::future::try_join_all;
use ::std::thread::sleep;
use ::std::time::Duration;

use crate::ast::Expr;
use crate::ast::File;
use crate::ast::Op;
use crate::read::parse;
use crate::read::Identifier;
use crate::read::read;
use crate::read::Error;

pub async fn evaluate(iden: Identifier, args: &[i64]) -> Result<i64, Error> {
    let json = read(&iden).await?;
    let file = parse(&iden, json)?;
    evaluate_file(iden, file, args).await
}

pub async fn evaluate_file(iden: Identifier, file: File, args: &[i64]) -> Result<i64, Error> {
    assert!(file.imports.is_empty());
    let context = Context { file_iden: iden, args };
    eval(&context, &file.expression).await
}

struct Context<'a> {
    file_iden: Identifier,
    args: &'a [i64],
}

async fn eval(context: &Context<'_>, expr: &Expr) -> Result<i64, Error> {
    Ok(match expr {
        Expr::Value(nr) => *nr,
        Expr::BinOp(op, left, right) => {
            let left = Box::pin(eval(context, left)).await?;
            let right = Box::pin(eval(context, right)).await?;
            match op {
                Op::Add => left.saturating_add(right),
                Op::Sub => left.saturating_sub(right),
                Op::Mul => left.saturating_mul(right),
                Op::Div => if right == 0 { return Err(Error::DivideByZero(context.file_iden.clone(), left)) } else { left.saturating_div(right) },
                Op::Min => if left <= right { left } else { right },
                Op::Max => if left >= right { left } else { right },
                Op::Lt => if left < right { 1 } else { 0 },
                Op::Gt => if left > right { 1  } else { 0 },
                Op::Eq => if left == right { 1 } else { 0 },
                Op::And => if left != 0 && right != 0 { 1 } else { 0 },
                Op::Or => if left != 0 || right != 0 { 1 } else { 0 },
            }
        },
        Expr::If(conf, yes, no) => if Box::pin(eval(context, conf)).await? != 0
                { Box::pin(eval(context, yes)).await? } else { Box::pin(eval(context, no)).await? }
        Expr::Arg(ix) => match context.args.get(*ix as usize) {
            None => return Err(Error::NoSuchArg(context.file_iden.clone(), *ix)),
            Some(nr) => *nr,
        },
        Expr::Call(iden, args) => {
            let arg_vals = try_join_all(args.into_iter().map(|a| eval(context, a))).await?;
            Box::pin(evaluate(iden.clone(), &arg_vals)).await?
        },
        Expr::Delay(expr, wait_ms) => {
            // this intentionally uses blocking sleep, because it simulated heavy computation
            sleep(Duration::from_millis(*wait_ms as u64));
            Box::pin(eval(context, expr)).await?
        },
    })
}