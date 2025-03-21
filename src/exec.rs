use ::std::thread::sleep;
use ::std::time::Duration;
use ::futures::future::try_join_all;

use crate::ast::Expr;
use crate::ast::Op;
use crate::ast::Identifier;
use crate::ast::File;
use crate::read::read;
use crate::read::parse;
use crate::read::Error;

pub async fn evaluate(iden: Identifier, file: &File, args: &[i64]) -> Result<i64, Error> {
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
            let left = eval(context, left).await?;
            let right = eval(context, right).await?;
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
        Expr::If(conf, yes, no) => if eval(context, conf).await? != 0
                { eval(context, yes).await? } else { eval(context, no).await? }
        Expr::Arg(ix) => context.args.get(*ix as usize).map(|nr| *nr).unwrap_or(0),
        Expr::Call(iden, args) => {
            let json = read(iden).await?;
            let file = parse(iden, json).await?;
            let arg_vals = try_join_all(args.into_iter().map(|a| eval(context, a))).await?;
            evaluate(iden.clone(), &file, &arg_vals).await?
        },
        Expr::Delay(expr, wait_ms) => {
            // this intentionally uses blocking sleep, because it simulated heavy computation
            sleep(Duration::from_millis(*wait_ms as u64));
            eval(context, expr).await?
        },
    })
}