/**
 *  Imports from parser.
 */
#[path = "../parser/mod.rs"]
mod parser;
use crate::parser::{
    SpanExpr,
    SpanOp,
    expr::Expr,
    op::Op,
    mytype::MyType,
    IResult,
    varprefix::Prefix,
};

#[path = "../interpreter/enverror.rs"]
pub mod enverror;
use enverror::{Result, EnvError};

pub mod env;
pub use env::Env;

/** 
 *  Borrowcheckast.
*/
pub fn borrowcheck_ast<'a>(e: SpanExpr<'a>) -> IResult<'a, SpanExpr<'a>, Prefix> {
    let mut env = Env::new();
    env.crate_scope();
    borrowcheck_expr(e, &mut env)
}


/** 
 *  Borrowcheck expresions in ast.
*/
fn borrowcheck_expr<'a>(e: SpanExpr<'a>, env: &mut Env<'a>) -> IResult<'a, SpanExpr<'a>, Prefix> {
    match (e.1).clone() {
        Expr::Num(_) => Ok((e, Prefix::None)),
        Expr::Bool(_) => Ok((e, Prefix::None)),
        Expr::UnOp(_, _) => borrowcheck_unop(e, env),
        Expr::BinOp(_, _, _) => borrowcheck_binop(e, env),
        // Expr::let(_, _, _, _, _) => borrowcheck_let(e, env),
        _ => panic!("borrowcheck_expr {:#?}", e),
    }
}

/** 
 *  Borrowcheck unop in ast.
*/
fn borrowcheck_unop<'a>(e: SpanExpr<'a>, env: &mut Env<'a>) -> IResult<'a, SpanExpr<'a>, Prefix> {
    match (e.1).clone() {
        Expr::UnOp(_, expr) => {
            let v = borrowcheck_expr(*expr, env)?.1;
            match v {
                Prefix::BorrowMut => panic!("borrowcheck_unop"),
                _ => Ok((e, Prefix::None)),
            }
        },
        _ => panic!("borrowcheck_unop"),
    }
}


/** 
 *  Borrowcheck binop in ast.
*/
fn borrowcheck_binop<'a>(e: SpanExpr<'a>, env: &mut Env<'a>) -> IResult<'a, SpanExpr<'a>, Prefix> {
    match (e.1).clone() {
        Expr::BinOp(le, op, re) => {
            let lp = borrowcheck_expr(*le, env)?.1;
            let rp = borrowcheck_expr(*re, env)?.1;
            match op.1 {
                Op::Equal => return Ok((e, Prefix::None)),
                Op::NotEq => return Ok((e, Prefix::None)),
                _ => {
                    match lp {
                        Prefix::BorrowMut => panic!("borrowcheck_binop"),
                        _ => (),
                    };
                    match rp {
                        Prefix::BorrowMut => panic!("borrowcheck_binop"),
                        _ => (),
                    };
                    return Ok((e, Prefix::None));
                },
            }
        },
        _ => panic!("borrowcheck_binop"),
    }
}


// /** 
//  *  Borrowcheck let in ast.
// */
// fn borrowcheck_let<'a>(e: SpanExpr<'a>, env: &mut Env<'a>) -> IResult<'a, SpanExpr<'a>, Prefix> {
//     match (e.1).clone() {
//         Expr::Let(_, _, _, _, _) => {
//             
//         },
//         _ => panic!("borrowcheck_let"),
//     }
// }
