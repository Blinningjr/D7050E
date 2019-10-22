// #![allow(dead_code)]

/**
 *  Imports from parser.
 */
#[path = "../parser/mod.rs"]
mod parser;
use crate::parser::{
    SpanExpr,
    expr::Expr,
    op::Op,
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
        Expr::Let(_, _, _, _, _) => borrowcheck_let(e, env),
        Expr::Assign(_, _, _) => borrowcheck_assign(e, env),
        Expr::Var(_, _) => borrowcheck_var(e, env),
        Expr::Body(_) => borrowcheck_body(e, env),
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


/** 
 *  Borrowcheck let in ast.
*/
fn borrowcheck_let<'a>(e: SpanExpr<'a>, env: &mut Env<'a>) -> IResult<'a, SpanExpr<'a>, Prefix> {
    match (e.1).clone() {
        Expr::Let(p, ident, prefix, _t, value) => {
            let val = match p.1 {
                Prefix::Borrow => panic!("borrowcheck_let Prefix::Borrow"),
                Prefix::BorrowMut => panic!("borrowcheck_let Prefix::BorrowMut"),
                Prefix::DeRef(_) => panic!("borrowcheck_let Prefix::DeRef"),
                _ => borrowcheck_expr(*value, env)?,
            };
            if p.1 != val.1 {
                panic!("borrowcheck_let");
            }
            env.store_var(ident, prefix.1);
            return Ok(val);
        },
        _ => panic!("borrowcheck_let"),
    }
}


/** 
 *  Borrowcheck assign in ast.
*/
fn borrowcheck_assign<'a>(e: SpanExpr<'a>, env: &mut Env<'a>) -> IResult<'a, SpanExpr<'a>, Prefix> {
    match (e.1).clone() {
        Expr::Assign(prefix, ident, value) => {
            let val = borrowcheck_expr(*value, env)?;
            let stored_prefix = env.load_var(ident).unwrap();
            if stored_prefix != val.1 {
                panic!("borrowcheck_assign");
            }
            match prefix.1 {
                Prefix::Borrow => panic!("borrowcheck_assign Prefix::Borrow"),
                Prefix::BorrowMut => panic!("borrowcheck_assign Prefix::BorrowMut"),
                Prefix::Mut => panic!("borrowcheck_assign Prefix::Mut"),
                Prefix::DeRef(n) => {
                    if stored_prefix != Prefix::BorrowMut {
                        panic!("borrowcheck_assign Prefix::Mut");
                    }
                },
                Prefix::None => (),
                Prefix::ReturnPrefix(_) => panic!("borrowcheck_assign"),
            };
            return Ok(val);
        },
        _ => panic!("borrowcheck_assign"),
    }
}

/** 
 *  Borrowcheck var in ast.
*/
fn borrowcheck_var<'a>(e: SpanExpr<'a>, env: &mut Env<'a>) -> IResult<'a, SpanExpr<'a>, Prefix> {
    match (e.1).clone() {
        Expr::Var(prefix, ident) => {
            let stored_prefix = env.load_var(ident).unwrap();
            match prefix.1 {
                Prefix::Mut => panic!("borrowcheck_var"),
                Prefix::DeRef(n) => {
                    match stored_prefix {
                        Prefix::Borrow => return Ok((e, prefix.1)),
                        Prefix::BorrowMut => return Ok((e, prefix.1)),
                        _ => return Ok((e, prefix.1)),
                    }
                },
                _ => return Ok((e, prefix.1)),
            }
        },
        _ => panic!("borrowcheck_var"),
    }
}


/** 
 *   Borrowcheck body in ast.
*/
fn borrowcheck_body<'a>(e: SpanExpr<'a>, env: &mut Env<'a>) -> IResult<'a, SpanExpr<'a>, Prefix> {
    match (e.1).clone() {
        Expr::Body(es) => {
            for expr in es {
                match expr.1 {
                    Expr::Return(v) => {
                        let val = borrowcheck_expr(*v, env)?;
                        // typecheck_funcs_in_list(e.clone(), env);
                        // env.pop_scope();
                        return match val.1 {
                            Prefix::Borrow => Ok((e, Prefix::ReturnPrefix(Box::new(Prefix::Borrow)))),
                            Prefix::BorrowMut => Ok((e, Prefix::ReturnPrefix(Box::new(Prefix::BorrowMut)))),
                            Prefix::DeRef(_) => panic!("borrowcheck_body"),
                            Prefix::Mut => panic!("borrowcheck_body"),
                            Prefix::None =>  Ok((e, Prefix::ReturnPrefix(Box::new(Prefix::None)))),
                            Prefix::ReturnPrefix(_) =>  Ok(val),
                        };
                    },
                    _ => {
                        let res = borrowcheck_expr(expr, env);
                        match res.clone()?.1 {
                            Prefix::ReturnPrefix(_) => {
                                // typecheck_funcs_in_list(e.clone(), env);
                                // env.pop_scope();
                                return res;
                            },
                            _ => (),
                        };
                    },
                }
            }
            // typecheck_funcs_in_list(e.clone(), env);
            // env.pop_scope();
            return Ok((e, Prefix::None));
        },
        _ => panic!("borrowcheck_body"),
    }
}
