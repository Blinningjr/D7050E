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
    Span,
    SpanExpr,
    SpanOp,
    SpanMyType,
    expr::Expr,
    op::Op,
};


pub type SpanVal<'a> = (SpanExpr<'a>, Val);


/** 
 *  Interprets a ast.
*/
pub fn interp_ast<'a>(e: SpanExpr<'a>) -> () {
    let mut env = Env::new();
    // env.store_var("test".to_string(), Val::Num(5));
    println!("{:#?}", interp_expr(e, &mut env));
    println!("{:#?}", env);
}


/** 
 *  Interprets expresions in ast.
*/
fn interp_expr<'a>(e: SpanExpr<'a>, env: &mut Env<'a>) -> Result<SpanVal<'a>> {
    match (e.1).clone() {
        Expr::Num(i) => Ok((e, Val::Num(i))),
        Expr::Bool(i) => Ok((e, Val::Bool(i))),
        Expr::UnOp(op, rv) => interp_unop(e.clone(), op, *rv, env),
        Expr::BinOp(lv, op, rv) => interp_binop(e.clone(), *lv, op, *rv, env),
        Expr::Assign(i, v) => interp_assign(e.clone(), *i, *v, env),
        Expr::Ident(s) => Ok((e, env.load_var(s)?)),
        Expr::If(b, lb, rb) => interp_if(e.clone(), *b, *lb, *rb, env),
        Expr::While(expr, b) => interp_while(e.clone(), *expr, *b, env),
        Expr::FuncCall(i,p) => interp_func_call(e.clone(), *i, *p, env),
        Expr::Func(i, _, _, _) => store_func_in_env(e, *i, env),
        Expr::Funcs(v) => interp_funcs(e.clone(), v, env),
        _ => Err(InterpError),
    }
}


/** 
 *  Interprets unary operations in ast.
*/
fn interp_unop<'a>(expr: SpanExpr<'a>, op: SpanOp, e: SpanExpr<'a>, env: &mut Env<'a>) -> Result<SpanVal<'a>> {
    match op.1 {
        Op::Sub => {
            let res = interp_expr(e, env)?;
            match res.1 {
                Val::Num(i) => Ok((e, Val::Num(-i))),
                _ => Err(InterpError),
            }
        }
        Op::Not => {
            let res = interp_expr(e, env)?;
            match res.1 {
                Val::Bool(b) => Ok((e, Val::Bool(!b))),
                _ => Err(InterpError),
            }
        }
        _ => Err(InterpError),
    }
}


/** 
 *  Interprets binary operations in ast.
*/
fn interp_binop<'a>(expr: SpanExpr<'a>, lv: SpanExpr<'a>, op: SpanOp, rv: SpanExpr<'a>, env: &mut Env<'a>) -> Result<SpanVal<'a>> {
    let lr = interp_expr(lv, env)?.1;
    let rr = interp_expr(rv, env)?.1;
    match op.1 {
        Op::Add => Ok((expr, Val::Num(
            get_int(lr)?
            +
            get_int(rr)?
        ))),
        Op::Sub => Ok((expr, Val::Num(
            get_int(lr)?
            -
            get_int(rr)?
        ))),
        Op::Div => Ok((expr, Val::Num(
            get_int(lr)?
            /
            get_int(rr)?
        ))),
        Op::Multi => Ok((expr, Val::Num(
            get_int(lr)?
            *
            get_int(rr)?
        ))),
        Op::Mod => Ok((expr, Val::Num(
            get_int(lr)?
            %
            get_int(rr)?
        ))),
        Op::LessEqThen => Ok((expr, Val::Bool(
            get_int(lr)?
            <=
            get_int(rr)?
        ))),
        Op::LargEqThen => Ok((expr, Val::Bool(
            get_int(lr)?
            >=
            get_int(rr)?
        ))),
        Op::LessThen => Ok((expr, Val::Bool(
            get_int(lr)?
            <
            get_int(rr)?
        ))),
        Op::LargThen => Ok((expr, Val::Bool(
            get_int(lr)?
            >
            get_int(rr)?
        ))),
        Op::Equal => Ok((expr, Val::Bool(
            get_int(lr)?
            ==
            get_int(rr)?
        ))),
        Op::And => Ok((expr, Val::Bool(
            get_bool(lr)?
            &&
            get_bool(rr)?
        ))),
        Op::Or => Ok((expr, Val::Bool(
            get_bool(lr)?
            ||
            get_bool(rr)?
        ))),
        Op::NotEq => Ok((expr, Val::Bool(
            get_bool(lr)?
            !=
            get_bool(rr)?
        ))),
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
fn interp_assign<'a>(expr: SpanExpr<'a>, ident: SpanExpr<'a>, value: SpanExpr<'a>, env: &mut Env<'a>) -> Result<SpanVal<'a>> {
    match ident.1 {
        Expr::Assign(i, _t) =>{
            match i.1 {
                Expr::Ident(s) => {
                    let val = interp_expr(value, env)?;
                    env.store_var(s, (val.1).clone());
                    return Ok(val);
                },
                _ => Err(InterpError),
            }
        },
        Expr::Ident(s) => {
            let val = interp_expr(value, env)?;
            env.store_var(s, (val.1).clone());
            return Ok(val);
        },
        _ => Err(InterpError),
    }
}


/** 
 *  Interprets if statments in ast.
*/
fn interp_if<'a>(expr: SpanExpr<'a>, e: SpanExpr<'a>, lb: SpanExpr<'a>, rb: SpanExpr<'a>, env: &mut Env<'a>) -> Result<SpanVal<'a>> {
    let mut nenv = env.crate_next_env();
    if get_bool(interp_expr(e, env)?.1)? {
        match lb.1 {
            Expr::Body(es) => interp_body(lb, es, &mut nenv),
            _ => Err(InterpError),
        }
    } else {
        match rb.1 {
            Expr::Body(es) => interp_body(rb, es, &mut nenv),
            Expr::Empty => Ok((expr, Val::Empty)),
            _ => Err(InterpError),
        }
    }
}


/** 
 *  Interprets body in ast.
*/
fn interp_body<'a>(expr: SpanExpr<'a>, es: Vec<SpanExpr<'a>>, env: &mut Env<'a>) -> Result<SpanVal<'a>> {
    let mut res = Ok((expr, Val::Empty));
    for e in es {
        res = interp_expr(e, env);
    }
    return res;
}


/** 
 *  Interprets while in ast.
*/
fn interp_while<'a>(expr: SpanExpr<'a>, e: SpanExpr<'a>, b: SpanExpr<'a>, env: &mut Env<'a>) -> Result<SpanVal<'a>> {
    let mut nenv = env.crate_next_env();
    let mut res = Ok((expr, Val::Empty));
    let v = match b.1 {
        Expr::Body(v) => Ok(v),
        _ => Err(InterpError),
    };
    let mut w = get_bool(interp_expr(e, &mut nenv)?.1)?;
    while w {
        res = interp_body(expr, v.clone()?, &mut nenv);
        w = get_bool(interp_expr(e.clone(), &mut nenv)?.1)?;
    }
    return res;
}


/** 
 *  Interprets function calls in ast.
*/
fn interp_func_call<'a>(expr: SpanExpr<'a>, i: SpanExpr<'a>, p: SpanExpr<'a>, env: &mut Env<'a>) -> Result<SpanVal<'a>> {
    match i.1 {
        Expr::Ident(s) => {
            match p.1 {
                Expr::Param(v) => {
                    let (e, mut nenv) = env.load_func(s)?;
                    match e {
                        Expr::Func(i, p, t, b) => interp_func(*i.clone(), *p.clone(), v, t.clone(), *b.clone(), &mut nenv.clone()),
                        _ => Err(InterpError),
                    }
                },
                _ => Err(InterpError),
            }
        }
        _ => Err(InterpError),
    }
}


/** 
 *  Interprets function in ast.
*/
fn interp_func<'a>(_i: SpanExpr<'a>, p: SpanExpr<'a>, pv: Vec<SpanExpr<'a>>, _t: SpanMyType, b: SpanExpr<'a>, env: &mut Env<'a>) -> Result<SpanVal<'a>> {
    match p.1 {
        Expr::Param(param) => {
            let mut j = 0;
            for p_var in param { 
                match p_var.1 {
                    Expr::Ident(s) => {env.store_var(s, interp_expr(pv[j].clone(), &mut env.clone())?.1); ()},
                    Expr::Assign(ident, _t) => {interp_assign(p_var, *ident, pv[j].clone(), env); ()},
                    _ => (),
                }
                j += 1;
            }
        }
        _ => (),
    }
    match b.1 {
        Expr::Body(es) => interp_body(b, es, env),
        _ => Err(InterpError),
    }
}

/** 
 *  Store function in env.
*/
fn store_func_in_env<'a>(f: SpanExpr<'a>, i: SpanExpr<'a>, env: &mut Env<'a>) -> Result<SpanVal<'a>> {
    match i.1 {
        Expr::Ident(s) => Ok((f, env.store_func(s, f.1)?)),
        _ => Err(InterpError),
    }
}

/** 
 *  Interprets function in ast and store them in env.
*/
fn interp_funcs<'a>(expr: SpanExpr<'a>, funcs: Vec<SpanExpr<'a>>, env: &mut Env<'a>) -> Result<SpanVal<'a>> {
    for func in funcs {
        match (func.1).clone() {
            Expr::Func(i, _, _, _) => {store_func_in_env(func, *i, env); ()},
            _ => (),
        };
    }
    
    let (e, mut nenv) = env.load_func(&"main")?;
    match e {
        Expr::Func(i, p, t, b) => interp_func(*i.clone(), *p.clone(), Vec::new(), t.clone(), *b.clone(), &mut nenv),
        _ =>  Err(InterpError),
    }
}