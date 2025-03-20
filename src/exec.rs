use crate::parse::{Expr, Op};
use crate::parse::Identifier;
use crate::parse::File;

pub fn evaluate(iden: Identifier, file: &File) -> i64 {
    assert!(file.imports.is_empty());
    let context = Context { file_iden: iden, args: vec![] };
    eval(&context, &file.expression)
}

struct Context {
    file_iden: Identifier,
    args: Vec<i64>,
}

fn eval(context: &Context, expr: &Expr) -> i64 {
    match expr {
        Expr::Value(nr) => *nr,
        Expr::BinOp(op, left, right) => {
            let left = eval(context, left);
            let right = eval(context, right);
            match op {
                Op::Add => left.saturating_add(right),
                Op::Sub => left.saturating_sub(right),
                Op::Mul => left.saturating_mul(right),
                Op::Div => if right == 0 { 0 } else { left.saturating_div(right) },
                Op::Min => if left <= right { left } else { right },
                Op::Max => if left >= right { left } else { right },
                Op::Lt => if left < right { 1 } else { 0 },
                Op::Gt => if left > right { 1  } else { 0 },
                Op::Eq => if left == right { 1 } else { 0 },
                Op::And => if left != 0 && right != 0 { 1 } else { 0 },
                Op::Or => if left != 0 || right != 0 { 1 } else { 0 },
            }
        }
        Expr::Arg(ix) => context.args.get(*ix as usize).map(|nr| *nr).unwrap_or(0),
        Expr::Call(iden, args) => unimplemented!(),
        Expr::Delay(expr, wait_ms) => eval(context, expr),
    }
}