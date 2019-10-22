#![allow(dead_code)]

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
        Expr::If(_, _, _) => borrowcheck_if(e, env),
        Expr::While(_, _) => borrowcheck_while(e, env),
        Expr::Func(_, _, _, _) => add_func_to_borrowchecking_list(e, env),
        Expr::FuncCall(_, _) => borrowcheck_func_call(e, env),
        Expr::Funcs(_) => borrowcheck_funcs(e, env),
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
                        borrowcheck_funcs_in_list(e.clone(), env);
                        env.pop_scope();
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
                                borrowcheck_funcs_in_list(e.clone(), env);
                                env.pop_scope();
                                return res;
                            },
                            _ => (),
                        };
                    },
                }
            }
            borrowcheck_funcs_in_list(e.clone(), env);
            env.pop_scope();
            return Ok((e, Prefix::None));
        },
        _ => panic!("borrowcheck_body"),
    }
}


/** 
 *  Borrowcheck if in ast.
*/
fn borrowcheck_if<'a>(e: SpanExpr<'a>, env: &mut Env<'a>) -> IResult<'a, SpanExpr<'a>, Prefix> {
    match (e.1).clone() {
        Expr::If(b, ib, eb) => {
            let val = borrowcheck_expr(*b, env)?;
            match val.1 {
                Prefix::DeRef(_) => panic!("borrowcheck_if"),
                Prefix::Mut => panic!("borrowcheck_if"),
                Prefix::ReturnPrefix(_) => panic!("borrowcheck_if"),
                _ => (),
            }
            let ib_r = borrowcheck_body(*ib, env)?;
            let eb_r = borrowcheck_body(*eb, env)?;
            match ib_r.1 {
                Prefix::ReturnPrefix(ip) => {
                    match eb_r.1 {
                        Prefix::ReturnPrefix(ep) => {
                                if ip != ep {
                                    panic!("borrowcheck_if");
                                }
                                return Ok((e, *ip));
                            },
                        _ => return Ok((e, *ip)),
                    }
                },
                _ => (),
            };
            match eb_r.1 {
                Prefix::ReturnPrefix(_) => return Ok(eb_r),
                _ => return Ok((e, Prefix::None)),
            }
        },
        _ => panic!("borrowcheck_if"),
    }
}


/** 
 *  Borrowcheck while in ast.
*/
fn borrowcheck_while<'a>(e: SpanExpr<'a>, env: &mut Env<'a>) -> IResult<'a, SpanExpr<'a>, Prefix> {
    match (e.1).clone() {
        Expr::While(b, body)=> {
            let val = borrowcheck_expr(*b, env)?;
            match val.1 {
                Prefix::DeRef(_) => panic!("borrowcheck_while"),
                Prefix::Mut => panic!("borrowcheck_while"),
                Prefix::ReturnPrefix(_) => panic!("borrowcheck_while"),
                _ => (),
            }
            let body_r = borrowcheck_body(*body, env)?;
            match body_r.1 {
                Prefix::ReturnPrefix(_) => return Ok(body_r),
                _ => return Ok((e, Prefix::None)),
            }
        },
        _ => panic!("borrowcheck_while"),
    }
}


/** 
 *  Borrowcheck func in ast.
*/
fn add_func_to_borrowchecking_list<'a>(e: SpanExpr<'a>, env: &mut Env<'a>) -> IResult<'a, SpanExpr<'a>, Prefix> {
    match (e.1).clone() {
        Expr::Func(ident, param, _, _) => {
            let mut t_param = Vec::new();
            let mut t_var = Vec::new();
            for v in param {
                match v.1 {
                    Expr::VarWithType(p, i, _) => {
                        t_param.push(p.clone().1);
                        t_var.push((i, p.clone().1));
                    }
                    _ => panic!("add_func_to_borrowchecking_list"),
                }
            }
            env.store_func(ident, t_param, Prefix::None, e.clone().1);
            return Ok((e, Prefix::None));
        },
        _ => panic!("add_func_to_borrowchecking_list"),
    }
}


/** 
 *  Borrowcheck func call in ast.
*/
fn borrowcheck_func_call<'a>(e: SpanExpr<'a>, env: &mut Env<'a>) -> IResult<'a, SpanExpr<'a>, Prefix> {
    match (e.1).clone() {
        Expr::FuncCall(i, param) => {
            let temp = env.load_func(i);
            let param_p;
            let return_p;
            match temp {
                Ok(tup) => {param_p = tup.0; return_p = tup.1;},
                _ => panic!("borrowcheck_func_call"),
            };
            if param_p.len() != param.len() {
                panic!("borrowcheck_func_call");
            }
            let mut i = 0;
            for p in param_p {
                if p != borrowcheck_expr(param[i].clone(), env)?.1 {
                    panic!("borrowcheck_func_call");
                }
                i = i + 1;
            }
            return Ok((e, return_p));
        },
        _ => panic!("borrowcheck_func_call"),
    }
}


/** 
 *  Borrowcheck funcs in ast.
*/
fn borrowcheck_funcs<'a>(e: SpanExpr<'a>, env: &mut Env<'a>) -> IResult<'a, SpanExpr<'a>, Prefix> {
    match (e.1).clone() {
        Expr::Funcs(es) => {
            for expr in es {
                add_func_to_borrowchecking_list(expr, env);
            }

            borrowcheck_funcs_in_list(e.clone(), env);
            env.pop_scope();
            return Ok((e, Prefix::None));
        },
        _ => panic!("borrowcheck_funcs"),
    }
}


/** 
 *  Adds to list of func that need borrowchecking in ast.
*/
fn borrowcheck_funcs_in_list<'a>(expr: SpanExpr<'a>, env: &mut Env<'a>) -> IResult<'a, SpanExpr<'a>, Prefix> {
    let mut res = Prefix::None;
    while env.get_funcs_len() > 0 {
        let e;
        match env.get_func() {
            Some(expr) => e = expr,
            _ => panic!("borrowcheck_funcs_in_list"),
        }
        match e.clone() {
            Expr::Func(ident, param, _, body) => {
                env.crate_scope();
                for v in param {
                    match v.1 {
                        Expr::VarWithType(p, i, _) => {
                            env.store_var(i, p.clone().1);
                        }
                        _ => panic!("borrowcheck_funcs_in_list"),
                    }
                }
                let mut body_p = borrowcheck_body(*body, env)?.1;
                match body_p {
                    Prefix::ReturnPrefix(p) => body_p = *p,
                    _ => body_p = Prefix::None,
                }
                res = body_p;
            },
            _ => panic!("borrowcheck_funcs_in_list"),
        }
    }
    return Ok((expr, res));
}
