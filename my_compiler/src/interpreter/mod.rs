pub mod interperror;
use interperror::{Result, InterpError};

pub mod val;
use val::Val;

pub mod env;
use env::Env;
// use env::NextEnv;


/**
 *  Imports from parser.
 */
#[path = "../parser/mod.rs"]
mod parser;
use crate::parser::{
    SpanExpr,
    SpanOp,
    SpanMyType,
    expr::Expr,
    op::Op,
};


/** 
 *  Interprets a ast.
*/
pub fn interp_ast<'a>(e: SpanExpr<'a>) -> () {
    let mut env = Env::new();
    // env.store_var("test".to_string(), Val::Num(5));
    println!("{:?}", interp_expr(e, &mut env));
    println!("{:?}", env);
}


/** 
 *  Interprets expresions in ast.
*/
fn interp_expr<'a>(e: SpanExpr<'a>, env: &mut Env<'a>) -> Result<Val> {
    match (e.1).clone() {
        Expr::Num(i) => Ok(Val::Num(i)),
        Expr::Bool(i) => Ok(Val::Bool(i)),
        Expr::UnOp(op, rv) => Ok(interp_unop(op, *rv, env).unwrap()),
        Expr::BinOp(lv, op, rv) => Ok(interp_binop(*lv, op, *rv, env).unwrap()),
        Expr::Assign(i, v) => interp_assign(*i, *v, env),
        Expr::Ident(s) => env.load_var(s),
        Expr::If(b, lb, rb) => interp_if(*b, *lb, *rb, env),
        Expr::While(expr, b) => interp_while(*expr, *b, env),
        Expr::FuncCall(i,p) => interp_func_call(*i, *p, env),
        Expr::Func(i, _, _, _) => store_func_in_env(e, *i, env),
        Expr::Funcs(v) => interp_funcs(v, env),
        _ => Err(InterpError),
    }
}


/** 
 *  Interprets unary operations in ast.
*/
fn interp_unop<'a>(op: SpanOp, e: SpanExpr<'a>, env: &mut Env<'a>) -> Result<Val> {
    match op.1 {
        Op::Sub => {
            let res = interp_expr(e, env).unwrap();
            match res {
                Val::Num(i) => Ok(Val::Num(-i)),
                _ => Err(InterpError),
            }
        }
        Op::Not => {
            let res = interp_expr(e, env).unwrap();
            match res {
                Val::Bool(b) => Ok(Val::Bool(!b)),
                _ => Err(InterpError),
            }
        }
        _ => Err(InterpError),
    }
}


/** 
 *  Interprets binary operations in ast.
*/
fn interp_binop<'a>(lv: SpanExpr<'a>, op: SpanOp, rv: SpanExpr<'a>, env: &mut Env<'a>) -> Result<Val> {
    match op.1 {
        Op::Add => Ok(Val::Num(
            get_int(interp_expr(lv, env).unwrap()).unwrap()
            +
            get_int(interp_expr(rv, env).unwrap()).unwrap()
        )),
        Op::Sub => Ok(Val::Num(
            get_int(interp_expr(lv, env).unwrap()).unwrap()
            -
            get_int(interp_expr(rv, env).unwrap()).unwrap()
        )),
        Op::Div => Ok(Val::Num(
            get_int(interp_expr(lv, env).unwrap()).unwrap()
            /
            get_int(interp_expr(rv, env).unwrap()).unwrap()
        )),
        Op::Multi => Ok(Val::Num(
            get_int(interp_expr(lv, env).unwrap()).unwrap()
            *
            get_int(interp_expr(rv, env).unwrap()).unwrap()
        )),
        Op::Mod => Ok(Val::Num(
            get_int(interp_expr(lv, env).unwrap()).unwrap()
            %
            get_int(interp_expr(rv, env).unwrap()).unwrap()
        )),
        Op::LessEqThen => Ok(Val::Bool(
            get_int(interp_expr(lv, env).unwrap()).unwrap()
            <=
            get_int(interp_expr(rv, env).unwrap()).unwrap()
        )),
        Op::LargEqThen => Ok(Val::Bool(
            get_int(interp_expr(lv, env).unwrap()).unwrap()
            >=
            get_int(interp_expr(rv, env).unwrap()).unwrap()
        )),
        Op::LessThen => Ok(Val::Bool(
            get_int(interp_expr(lv, env).unwrap()).unwrap()
            <
            get_int(interp_expr(rv, env).unwrap()).unwrap()
        )),
        Op::LargThen => Ok(Val::Bool(
            get_int(interp_expr(lv, env).unwrap()).unwrap()
            >
            get_int(interp_expr(rv, env).unwrap()).unwrap()
        )),
        Op::Equal => Ok(Val::Bool(
            get_int(interp_expr(lv, env).unwrap()).unwrap()
            ==
            get_int(interp_expr(rv, env).unwrap()).unwrap()
        )),
        Op::And => Ok(Val::Bool(
            get_bool(interp_expr(lv, env).unwrap()).unwrap()
            &&
            get_bool(interp_expr(rv, env).unwrap()).unwrap()
        )),
        Op::Or => Ok(Val::Bool(
            get_bool(interp_expr(lv, env).unwrap()).unwrap()
            ||
            get_bool(interp_expr(rv, env).unwrap()).unwrap()
        )),
        Op::NotEq => Ok(Val::Bool(
            get_bool(interp_expr(lv, env).unwrap()).unwrap()
            !=
            get_bool(interp_expr(rv, env).unwrap()).unwrap()
        )),
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
fn interp_assign<'a>(ident: SpanExpr<'a>, value: SpanExpr<'a>, env: &mut Env<'a>) -> Result<Val> {
    match ident.1 {
        Expr::Assign(i, _t) =>{
            match i.1 {
                Expr::Ident(s) => {
                    let val = interp_expr(value, env).unwrap();
                    env.store_var(s.to_string(), val.clone());
                    return Ok(val);
                },
                _ => Ok(Val::Empty),
            }
        },
        Expr::Ident(s) => {
            let val = interp_expr(value, env).unwrap();
            env.store_var(s.to_string(), val.clone());
            return Ok(val);
        },
        _ => Ok(Val::Empty),
    }
}


/** 
 *  Interprets if statments in ast.
*/
fn interp_if<'a>(e: SpanExpr<'a>, lb: SpanExpr<'a>, rb: SpanExpr<'a>, env: &mut Env<'a>) -> Result<Val> {
    if get_bool(interp_expr(e, env).unwrap()).unwrap() {
        match lb.1 {
            Expr::Body(es) => interp_body(es, env),
            _ => Err(InterpError),
        }
    } else {
        match rb.1 {
            Expr::Body(es) => interp_body(es, env),
            Expr::Empty => Ok(Val::Empty),
            _ => Err(InterpError),
        }
    }
}


/** 
 *  Interprets body in ast.
*/
fn interp_body<'a>(es: Vec<SpanExpr<'a>>, env: &mut Env<'a>) -> Result<Val> {
    let mut res = Ok(Val::Empty);
    for e in es {
        res = interp_expr(e, env);
    }
    return res;
}


/** 
 *  Interprets while in ast.
*/
fn interp_while<'a>(e: SpanExpr<'a>, b: SpanExpr<'a>, env: &mut Env<'a>) -> Result<Val> {
    let mut res = Ok(Val::Empty);
    let v = match b.1 {
        Expr::Body(v) => Ok(v),
        _ => Err(InterpError),
    };
    let mut w = get_bool(interp_expr(e.clone(), env).unwrap()).unwrap();
    while w {
        res = interp_body(v.clone().unwrap(), env);
        w = get_bool(interp_expr(e.clone(), env).unwrap()).unwrap();
    }
    return res;
}


/** 
 *  Interprets function calls in ast.
*/
fn interp_func_call<'a>(i: SpanExpr<'a>, p: SpanExpr<'a>, env: &mut Env<'a>) -> Result<Val> {
    match i.1 {
        Expr::Ident(s) => {
            match p.1 {
                Expr::Param(v) => {
                    let tup = env.load_func(s).unwrap();
                    match &tup.0 {
                        Expr::Func(i, p, t, b) => interp_func(*i.clone(), *p.clone(), v, t.clone(), *b.clone(), &mut tup.1.clone()),
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
fn interp_func<'a>(_i: SpanExpr<'a>, p: SpanExpr<'a>, pv: Vec<SpanExpr<'a>>, _t: SpanMyType, b: SpanExpr<'a>, env: &mut Env<'a>) -> Result<Val> {
    match p.1 {
        Expr::Param(param) => {
            let mut j = 0;
            for p_var in param { 
                match p_var.1 {
                    Expr::Ident(s) => {env.store_var(s.to_string(), interp_expr(pv[j].clone(), &mut env.clone()).unwrap()); ()},
                    Expr::Assign(ident, _t) => {interp_assign(*ident, pv[j].clone(), env); ()},
                    _ => (),
                }
                j += 1;
            }
        }
        _ => (),
    }
    match b.1 {
        Expr::Body(es) => interp_body(es, env),
        _ => Err(InterpError),
    }
}

/** 
 *  Store function in env.
*/
fn store_func_in_env<'a>(f: SpanExpr<'a>, i: SpanExpr<'a>, env: &mut Env<'a>) -> Result<Val> {
    match i.1 {
        Expr::Ident(s) => {env.store_func( s.to_string(), f.1); ()},
        _ => (),
    };
    return Ok(Val::Empty);
}

/** 
 *  Interprets function in ast and store them in env.
*/
fn interp_funcs<'a>(funcs: Vec<SpanExpr<'a>>, env: &mut Env<'a>) -> Result<Val> {
    for func in funcs {
        match (func.1).clone() {
            Expr::Func(i, _, _, _) => {store_func_in_env(func, *i, env); ()},
            _ => (),
        };
    }
    let mut tup = env.load_func(&"main").unwrap();
    match &tup.0 {
        Expr::Func(i, p, t, b) => interp_func(*i.clone(), *p.clone(), Vec::new(), t.clone(), *b.clone(), &mut tup.1),
        _ =>  Err(InterpError),
    }
}