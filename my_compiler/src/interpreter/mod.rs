/**
 *  std imports.
 */
use std::{
    error,
    fmt,
};

/** 
 *  Needed for creating InterpError. 
 *  src: https://doc.rust-lang.org/std/str/trait.FromStr.html
 */
pub type Result<T> = std::result::Result<T, InterpError>;
#[derive(Debug, Clone)]
pub struct InterpError;


/** 
 * 
 */
impl fmt::Display for InterpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error something is wrong")
    }
}


/** 
 *  This is important for other errors to wrap this one.
 */ 
impl error::Error for InterpError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}


/**
 *  Imports from parser.
 */
#[path = "../parser/mod.rs"]
mod parser;
use crate::parser::Expr;
use crate::parser::Op;
use crate::parser::MyType;

use std::collections::HashMap;


/** 
 *  Defins Env that stores variables and functions.
*/
#[derive(Debug, PartialEq, Clone)]
pub struct Env<'a> {
    mem_var: HashMap<String, Val>,
    mem_func: HashMap<String, (Expr<'a>, Env<'a>)>,
}
impl<'a> Env<'a> {
    pub fn new() -> Env<'a> {
        Env {
            mem_var: HashMap::new(),
            mem_func: HashMap::new(),
        }
    }
    pub fn store_var(&mut self, ident: String, val: Val) {
        self.mem_var.insert(ident, val);
    }
    pub fn store_func(&mut self, ident: String, func: Expr<'a> , env: Env<'a>) {
        self.mem_func.insert(ident, (func, env));
    }
    pub fn load_var(&mut self, key: &'a str) -> Result<Val>{
        match self.mem_var.get(key) {
            Some(val) => Ok(val.clone()),
            None => Err(InterpError),
        }
    }
    pub fn load_func(&mut self, key: &'a str, pv: Vec<Expr>) -> Result<Val>{
        match self.mem_func.get(key) {
            Some(tup) => {
                match &tup.0 {
                    Expr::Func(i, p, t, b) => interp_func(*i.clone(), *p.clone(), pv, t.clone(), *b.clone(), &mut tup.1.clone()),
                    _ => Err(InterpError),
                }
            },
            None => Err(InterpError),
        }
    }
}


/** 
 *  Defins Val so bool and i32 can be returnd.
*/
#[derive(Debug, PartialEq, Clone)]
enum Val {
    Num(i32),
    Bool(bool),
    Empty,
}


/** 
 *  Interprets a ast.
*/
pub fn interp_ast(e: Expr) -> () {
    let mut env = Env::new();
    // env.store_var("test".to_string(), Val::Num(5));
    println!("{:?}", interp_expr(e, &mut env));
    println!("{:?}", env);
}


/** 
 *  Interprets expresions in ast.
*/
fn interp_expr<'a>(e: Expr<'a>, env: &mut Env<'a>) -> Result<Val> {
    match e.clone() {
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
fn interp_unop<'a>(op: Op, e: Expr<'a>, env: &mut Env<'a>) -> Result<Val> {
    match op {
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
fn interp_binop<'a>(lv: Expr<'a>, op: Op, rv: Expr<'a>, env: &mut Env<'a>) -> Result<Val> {
    match op {
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
fn interp_assign<'a>(ident: Expr<'a>, value: Expr<'a>, env: &mut Env<'a>) -> Result<Val> {
    match ident {
        Expr::Assign(i, _t) =>{
            let val = interp_expr(value, env).unwrap();
            env.store_var(i.to_string(), val.clone());
            return Ok(val);
        },
        _ => {
            let val = interp_expr(value, env).unwrap();
            env.store_var(ident.to_string(), val.clone());
            return Ok(val);
        },
    }
}


/** 
 *  Interprets if statments in ast.
*/
fn interp_if<'a>(e: Expr<'a>, lb: Expr<'a>, rb: Expr<'a>, env: &mut Env<'a>) -> Result<Val> {
    let mut res = Ok(Val::Empty);
    if get_bool(interp_expr(e, env).unwrap()).unwrap() {
        match lb {
            Expr::Body(es) => res = interp_body(es, env),
            _ => res = Err(InterpError),
        };
    } else {
        match rb {
            Expr::Body(es) => res = interp_body(es, env),
            Expr::Empty => res = Ok(Val::Empty),
            _ => res = Err(InterpError),
        };
    }
    return res;
}


/** 
 *  Interprets body in ast.
*/
fn interp_body<'a>(es: Vec<Expr<'a>>, env: &mut Env<'a>) -> Result<Val> {
    let mut res = Ok(Val::Empty);
    for e in es {
        res = interp_expr(e, env);
    }
    return res;
}


/** 
 *  Interprets while in ast.
*/
fn interp_while<'a>(e: Expr<'a>, b: Expr<'a>, env: &mut Env<'a>) -> Result<Val> {
    let mut res = Ok(Val::Empty);
    let v = match b {
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
fn interp_func_call<'a>(i: Expr<'a>, p: Expr<'a>, env: &mut Env<'a>) -> Result<Val> {
    match i {
        Expr::Ident(s) => {
            match p {
                Expr::Param(v) => env.load_func(s, v),
                _ => Err(InterpError),
            }
        }
        _ => Err(InterpError),
    }
}


/** 
 *  Interprets function in ast.
*/
fn interp_func<'a>(i: Expr<'a>, p: Expr<'a>, pv: Vec<Expr<'a>>, t: MyType, b: Expr<'a>, env: &mut Env<'a>) -> Result<Val> {
    let mut res = Ok(Val::Empty);
    match p {
        Expr::Param(param) => {
            let mut j = 0;
            for p_var in param { 
                match p_var {
                    Expr::Ident(s) => env.store_var(s.to_string(), interp_expr(pv[j].clone(), &mut env.clone()).unwrap()),
                    Expr::Assign(ident, _t) => res = interp_assign(*ident, pv[j].clone(), env),
                    _ => res = Err(InterpError),
                }
                j += 1;
            }
        }
        _ => res = Err(InterpError),
    }
    match b {
        Expr::Body(es) => res = interp_body(es, env),
        _ => res = Err(InterpError),
    }
    return res;
}

/** 
 *  Store function in env.
*/
fn store_func_in_env<'a>(f: Expr<'a>, i: Expr<'a>, env: &mut Env<'a>) -> Result<Val> {
    env.store_func(i.to_string(), f, env.clone());
    return Ok(Val::Empty);
}

/** 
 *  Interprets function in ast and store them in env.
*/
fn interp_funcs<'a>(funcs: Vec<Expr<'a>>, env: &mut Env<'a>) -> Result<Val> {
    let mut res = Ok(Val::Empty);
    for func in funcs {
        match func.clone() {
            Expr::Func(i, _, _, _) => res = store_func_in_env(func, *i, env),
            _ => res = Err(InterpError),
        }
    }
    res = env.load_func(&"main", Vec::new());
    return res;
}