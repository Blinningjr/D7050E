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
    mem: HashMap<&'a str, Expr<'a>>,
}
impl<'a> Env<'a> {
    pub fn new() -> Env<'a> {
        Env {
            mem: HashMap::new(),
        }
    }
}


/** 
 *  Defins Val so bool and i32 can be returnd.
*/
#[derive(Debug, PartialEq)]
enum Val {
    Num(i32),
    Bool(bool),
}


/** 
 *  Interprets a ast.
*/
pub fn interp_ast(e: Expr) -> () {
    println!("{:?}", interp_expr(e, Env::new()));
}


/** 
 *  Interprets expresions in ast.
*/
fn interp_expr(e: Expr, env: Env) -> Result<Val> {
    match e {
        Expr::Num(i) => Ok(Val::Num(i)),
        Expr::Bool(i) => Ok(Val::Bool(i)),
        Expr::UnOp(op, rv) => Ok(interp_unop(op, *rv, env.clone()).unwrap()),
        Expr::BinOp(lv, op, rv) => Ok(interp_binop(*lv, op, *rv, env.clone()).unwrap()),
        _ => Err(InterpError),
    }
}


/** 
 *  Interprets unary operations in ast.
*/
fn interp_unop(op: Op, e: Expr, env: Env) -> Result<Val> {
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
fn interp_binop(lv: Expr, op: Op, rv: Expr, env: Env) -> Result<Val> {
    match op {
        Op::Add => Ok(Val::Num(
            get_int(interp_expr(lv, env.clone()).unwrap()).unwrap()
            +
            get_int(interp_expr(rv, env.clone()).unwrap()).unwrap()
        )),
        Op::Sub => Ok(Val::Num(
            get_int(interp_expr(lv, env.clone()).unwrap()).unwrap()
            -
            get_int(interp_expr(rv, env.clone()).unwrap()).unwrap()
        )),
        Op::Div => Ok(Val::Num(
            get_int(interp_expr(lv, env.clone()).unwrap()).unwrap()
            /
            get_int(interp_expr(rv, env.clone()).unwrap()).unwrap()
        )),
        Op::Multi => Ok(Val::Num(
            get_int(interp_expr(lv, env.clone()).unwrap()).unwrap()
            *
            get_int(interp_expr(rv, env.clone()).unwrap()).unwrap()
        )),
        Op::Mod => Ok(Val::Num(
            get_int(interp_expr(lv, env.clone()).unwrap()).unwrap()
            %
            get_int(interp_expr(rv, env.clone()).unwrap()).unwrap()
        )),
        Op::LessEqThen => Ok(Val::Bool(
            get_int(interp_expr(lv, env.clone()).unwrap()).unwrap()
            <=
            get_int(interp_expr(rv, env.clone()).unwrap()).unwrap()
        )),
        Op::LargEqThen => Ok(Val::Bool(
            get_int(interp_expr(lv, env.clone()).unwrap()).unwrap()
            >=
            get_int(interp_expr(rv, env.clone()).unwrap()).unwrap()
        )),
        Op::LessThen => Ok(Val::Bool(
            get_int(interp_expr(lv, env.clone()).unwrap()).unwrap()
            <
            get_int(interp_expr(rv, env.clone()).unwrap()).unwrap()
        )),
        Op::LargThen => Ok(Val::Bool(
            get_int(interp_expr(lv, env.clone()).unwrap()).unwrap()
            >
            get_int(interp_expr(rv, env.clone()).unwrap()).unwrap()
        )),
        Op::Equal => Ok(Val::Bool(
            get_int(interp_expr(lv, env.clone()).unwrap()).unwrap()
            ==
            get_int(interp_expr(rv, env.clone()).unwrap()).unwrap()
        )),
        Op::And => Ok(Val::Bool(
            get_bool(interp_expr(lv, env.clone()).unwrap()).unwrap()
            &&
            get_bool(interp_expr(rv, env.clone()).unwrap()).unwrap()
        )),
        Op::Or => Ok(Val::Bool(
            get_bool(interp_expr(lv, env.clone()).unwrap()).unwrap()
            ||
            get_bool(interp_expr(rv, env.clone()).unwrap()).unwrap()
        )),
        Op::NotEq => Ok(Val::Bool(
            get_bool(interp_expr(lv, env.clone()).unwrap()).unwrap()
            !=
            get_bool(interp_expr(rv, env.clone()).unwrap()).unwrap()
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
