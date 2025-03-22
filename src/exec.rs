use crate::ast::Expr;
use crate::ast::Op;
use crate::file::Error;
use crate::file::Fs;
use crate::file::Identifier;
use ::futures::future::try_join_all;
use ::std::thread::sleep;
use ::std::time::Duration;

pub async fn evaluate(fs: &impl Fs, iden: Identifier, args: &[i64]) -> Result<i64, Error> {
    let file = fs.read(&iden).await?;
    assert!(file.imports.is_empty());
    let context = Context { file_iden: iden, args };
    eval(fs, &context, &file.expression).await
}

struct Context<'a> {
    file_iden: Identifier,
    args: &'a [i64],
}

async fn eval(fs: &impl Fs, context: &Context<'_>, expr: &Expr) -> Result<i64, Error> {
    Ok(match expr {
        Expr::Value(nr) => *nr,
        Expr::BinOp(op, left, right) => {
            let left = Box::pin(eval(fs, context, left)).await?;
            let right = Box::pin(eval(fs, context, right)).await?;
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
        Expr::If(conf, yes, no) => if Box::pin(eval(fs, context, conf)).await? != 0
                { Box::pin(eval(fs, context, yes)).await? } else { Box::pin(eval(fs, context, no)).await? }
        Expr::Arg(ix) => match context.args.get(*ix as usize) {
            None => return Err(Error::NoSuchArg(context.file_iden.clone(), *ix)),
            Some(nr) => *nr,
        },
        Expr::Call(iden, args) => {
            let arg_vals = try_join_all(args.into_iter().map(|a| eval(fs, context, a))).await?;
            Box::pin(evaluate(fs, iden.clone(), &arg_vals)).await?
        },
        Expr::Delay(expr, wait_ms) => {
            // this intentionally uses blocking sleep, because it simulated heavy computation
            sleep(Duration::from_millis(*wait_ms as u64));
            Box::pin(eval(fs, context, expr)).await?
        },
    })
}
