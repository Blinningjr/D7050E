pub mod interperror;
use interperror::{Result, InterpError};

pub mod val;
use val::Val;

pub mod env;
use env::Env;
use env::FormerEnv;


/**
 *  Imports from parser.
 */
#[path = "../parser/mod.rs"]
mod parser;
use crate::parser::{
    // Span,
    SpanExpr,
    expr::Expr,
    op::Op,
    // IResult,
};


pub type SpanVal<'a> = (SpanExpr<'a>, Val);


/** 
 *  Interprets a ast.
*/
pub fn interp_ast<'a>(e: SpanExpr<'a>) -> () {
    let mut env = Env::new();
    // env.store_var("test".to_string(), Val::Num(5));
    println!("{:#?}", interp_expr(e, &mut env));
    // println!("{:#?}", env);
}


/** 
 *  Interprets expresions in ast.
*/
fn interp_expr<'a>(e: SpanExpr<'a>, env: &mut Env<'a>) -> Result<SpanVal<'a>> {
    match (e.1).clone() {
        Expr::Num(i) => Ok((e, Val::Num(i))),
        Expr::Bool(i) => Ok((e, Val::Bool(i))),
        Expr::UnOp(_, _) => interp_unop(e, env),
        Expr::BinOp(_, _, _) => interp_binop(e, env),
        Expr::Assign(_, _) => interp_assign(e, env),
        Expr::Ident(s) => Ok((e, env.load_var(s)?)),
        Expr::If(_, _, _) => interp_if(e, env),
        Expr::While(_, _) => interp_while(e, env),
        Expr::FuncCall(_, _) => interp_func_call(e, env),
        Expr::Func(i, _, _, _) => store_func_in_env(e, *i, env),
        Expr::Funcs(v) => interp_funcs(v, env),
        _ => Err(InterpError),
    }
}


/** 
 *  Interprets unary operations in ast.
*/
fn interp_unop<'a>(e: SpanExpr<'a>, env: &mut Env<'a>) -> Result<SpanVal<'a>> {
    match (e.1).clone() {
        Expr::UnOp(op, rv) => {
            match op.1 {
                Op::Sub => {
                    let res = interp_expr(*rv.clone(), env)?;
                    match res.1 {
                        Val::Num(i) => Ok((e, Val::Num(-i))),
                        _ => Err(InterpError),
                    }
                }
                Op::Not => {
                    let res = interp_expr(*rv.clone(), env)?;
                    match res.1 {
                        Val::Bool(b) => Ok((e, Val::Bool(!b))),
                        _ => Err(InterpError),
                    }
                }
                _ => Err(InterpError),
            }
        },
        _ => Err(InterpError),
    }
}


/** 
 *  Interprets binary operations in ast.
*/
fn interp_binop<'a>(e: SpanExpr<'a>, env: &mut Env<'a>) -> Result<SpanVal<'a>> {
    match (e.1).clone() {
        Expr::BinOp(lv, op, rv) => {
            let lr = interp_expr(*lv, env)?.1;
            let rr = interp_expr(*rv, env)?.1;
            match op.1 {
                Op::Add => Ok((e, Val::Num(
                    get_int(lr)?
                    +
                    get_int(rr)?
                ))),
                Op::Sub => Ok((e, Val::Num(
                    get_int(lr)?
                    -
                    get_int(rr)?
                ))),
                Op::Div => Ok((e, Val::Num(
                    get_int(lr)?
                    /
                    get_int(rr)?
                ))),
                Op::Multi => Ok((e, Val::Num(
                    get_int(lr)?
                    *
                    get_int(rr)?
                ))),
                Op::Mod => Ok((e, Val::Num(
                    get_int(lr)?
                    %
                    get_int(rr)?
                ))),
                Op::LessEqThen => Ok((e, Val::Bool(
                    get_int(lr)?
                    <=
                    get_int(rr)?
                ))),
                Op::LargEqThen => Ok((e, Val::Bool(
                    get_int(lr)?
                    >=
                    get_int(rr)?
                ))),
                Op::LessThen => Ok((e, Val::Bool(
                    get_int(lr)?
                    <
                    get_int(rr)?
                ))),
                Op::LargThen => Ok((e, Val::Bool(
                    get_int(lr)?
                    >
                    get_int(rr)?
                ))),
                Op::Equal => Ok((e, Val::Bool(
                    get_int(lr)?
                    ==
                    get_int(rr)?
                ))),
                Op::And => Ok((e, Val::Bool(
                    get_bool(lr)?
                    &&
                    get_bool(rr)?
                ))),
                Op::Or => Ok((e, Val::Bool(
                    get_bool(lr)?
                    ||
                    get_bool(rr)?
                ))),
                Op::NotEq => Ok((e, Val::Bool(
                    get_bool(lr)?
                    !=
                    get_bool(rr)?
                ))),
                _ => Err(InterpError),
            }
        },
        _ => Err(InterpError),
    }
}


/** 
 *  Get i32 value from Val.
*/
fn get_int(v: Val) -> Result<i32> {
    match v {
        Val::Num(i) => Ok(i),
        _ => Err(InterpError),
    }
}


/** 
 *  Get bool value from Val.
*/
fn get_bool(v: Val) -> Result<bool> {
    match v {
        Val::Bool(b) => Ok(b),
        _ => Err(InterpError),
    }
}


/** 
 *  Interprets assignments in ast.
*/
fn interp_assign<'a>(e: SpanExpr<'a>, env: &mut Env<'a>) -> Result<SpanVal<'a>> {
    match (e.1).clone() {
        Expr::Assign(ident, value) => assign_help(*ident, *value, env),
        _ => Err(InterpError),
    }
}


/** 
 *  assing help func.
*/
fn assign_help<'a>(ident: SpanExpr<'a>, value: SpanExpr<'a>, env: &mut Env<'a>) -> Result<SpanVal<'a>> {
    match ident.1 {
        Expr::Assign(i, _t) =>{
            match i.1 {
                Expr::Ident(s) => {
                    let val = interp_expr(value, env)?;
                    env.store_var(s, (val.1).clone())?;
                    return Ok(val);
                },
                _ => Err(InterpError),
            }
        },
        Expr::Ident(s) => {
            let val = interp_expr(value, env)?;
            env.store_var(s, (val.1).clone())?;
            return Ok(val);
        },
        _ => Err(InterpError),
    }
}


/** 
 *  Interprets if statments in ast.
*/
fn interp_if<'a>(e: SpanExpr<'a>, env: &mut Env<'a>) -> Result<SpanVal<'a>> {
    match (e.1).clone() {
        Expr::If(b, lb, rb) => {
            let mut nenv = env.crate_next_env();
            if get_bool(interp_expr(*b, env)?.1)? {
                match lb.1.clone() {
                    Expr::Body(_) => interp_body(*lb, &mut nenv),
                    _ => Err(InterpError),
                }
            } else {
                match rb.1.clone() {
                    Expr::Body(_) => interp_body(*rb, &mut nenv),
                    Expr::Empty => Ok((e, Val::Empty)),
                    _ => Err(InterpError),
                }
            }
        },
        _ => Err(InterpError),
    }
}


/** 
 *  Interprets body in ast.
*/
fn interp_body<'a>(e: SpanExpr<'a>, env: &mut Env<'a>) -> Result<SpanVal<'a>> {
    match (e.1).clone() {
        Expr::Body(es) => {
            let mut res = Ok((e, Val::Empty));
            for e in es {
                match e.1 {
                    Expr::Return(v) => {
                        let val = interp_expr(*v, env)?;
                        return match val.1 {
                            Val::Num(v) => Ok((val.0, Val::ReturnNum(v))),
                            Val::Bool(b) => Ok((val.0, Val::ReturnBool(b))),
                            Val::Empty => Ok((val.0, Val::ReturnEmpty)),
                            _ => Ok(val),
                        };
                    },
                    _ => {
                        res = interp_expr(e, env);
                        match res.clone()?.1 {
                            Val::ReturnBool(_) => return res,
                            Val::ReturnNum(_) => return res,
                            Val::ReturnEmpty => return res,
                            _ => (),
                        };
                    },
                }
            }
            return res;
        },
        _ => Err(InterpError),
    }
}


/** 
 *  Interprets while in ast.
*/
fn interp_while<'a>(e: SpanExpr<'a>, env: &mut Env<'a>) -> Result<SpanVal<'a>> {
    match (e.1).clone() {
        Expr::While(expr, b) => {
            let mut nenv = env.crate_next_env();
            let mut res = Ok((e.clone(), Val::Empty));
            let mut w = get_bool(interp_expr(*expr.clone(), &mut nenv)?.1)?;
            while w {
                res = interp_body( *b.clone(), &mut nenv);
                w = get_bool(interp_expr(*expr.clone(), &mut nenv)?.1)?;
            }
            return res;
        },
        _ => Err(InterpError),
    }
}


/** 
 *  Interprets function calls in ast.
*/
fn interp_func_call<'a>(e: SpanExpr<'a>, env: &mut Env<'a>) -> Result<SpanVal<'a>> {
    match (e.1).clone() {
        Expr::FuncCall(i,p) => {
            match i.1 {
                Expr::Ident(s) => {
                    match p.1 {
                        Expr::Param(v) => {
                            let (e, nenv) = env.load_func(s)?;
                            match e {
                                Expr::Func(_, _, _, _) => interp_func(e, v, &mut nenv.clone()),
                                _ => Err(InterpError),
                            }
                        },
                        _ => Err(InterpError),
                    }
                }
                _ => Err(InterpError),
            }
        },
        _ => Err(InterpError),
    }
}


/** 
 *  Interprets function in ast.
*/
fn interp_func<'a>(e: Expr<'a>, pv: Vec<SpanExpr<'a>>, env: &mut Env<'a>) -> Result<SpanVal<'a>> {
    match (e).clone() {
        Expr::Func(_i, p, _t, b) => {
            match p.1 {
                Expr::Param(param) => {
                    let mut j = 0;
                    for p_var in param { 
                        match p_var.1 {
                            Expr::Ident(s) => {env.store_var(s, interp_expr(pv[j].clone(), &mut env.clone())?.1)?; ()},
                            Expr::Assign(ident, _t) => {assign_help(*ident, pv[j].clone(), env)?; ()},
                            _ => (),
                        }
                        j += 1;
                    }
                }
                _ => (),
            }
            match b.1.clone() {
                Expr::Body(_) => interp_body(*b, env),
                _ => Err(InterpError),
            }
        },
        _ => Err(InterpError),
    }
}

/** 
 *  Store function in env.
*/
fn store_func_in_env<'a>(f: SpanExpr<'a>, i: SpanExpr<'a>, env: &mut Env<'a>) -> Result<SpanVal<'a>> {
    match i.1 {
        Expr::Ident(s) => Ok((f.clone(), env.store_func(s, f.1)?)),
        _ => Err(InterpError),
    }
}

/** 
 *  Interprets function in ast and store them in env.
*/
fn interp_funcs<'a>(funcs: Vec<SpanExpr<'a>>, env: &mut Env<'a>) -> Result<SpanVal<'a>> {
    for func in funcs {
        match (func.1).clone() {
            Expr::Func(i, _, _, _) => {store_func_in_env(func, *i, env)?; ()},
            _ => (),
        };
    }
    
    let (e, mut nenv) = env.load_func(&"main")?;
    match e {
        Expr::Func(_, _, _, _) => interp_func(e, Vec::new(), &mut nenv),
        _ =>  Err(InterpError),
    }
}
