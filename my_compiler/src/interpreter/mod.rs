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
    mem: HashMap<&'a str, Val>,
}
impl<'a> Env<'a> {
    pub fn new() -> Env<'a> {
        Env {
            mem: HashMap::new(),
        }
    }
    pub fn store(&mut self, ident: &'a str, val: Val) {
        self.mem.insert(ident, val);
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
    println!("{:?}", interp_expr(e, &env));
    env.store("test", Val::Empty);
    println!("{:?}", env);
}


/** 
 *  Interprets expresions in ast.
*/
fn interp_expr(e: Expr, env: &Env) -> Result<Val> {
    match e {
        Expr::Num(i) => Ok(Val::Num(i)),
        Expr::Bool(i) => Ok(Val::Bool(i)),
        Expr::UnOp(op, rv) => Ok(interp_unop(op, *rv, &env).unwrap()),
        Expr::BinOp(lv, op, rv) => Ok(interp_binop(*lv, op, *rv, &env).unwrap()),
        Expr::Assign(i, v) => interp_assign(*i, *v, &env),
        _ => Err(InterpError),
    }
}


/** 
 *  Interprets unary operations in ast.
*/
fn interp_unop(op: Op, e: Expr, env: &Env) -> Result<Val> {
    match op {
        Op::Sub => {
            let res = interp_expr(e, &env).unwrap();
            match res {
                Val::Num(i) => Ok(Val::Num(-i)),
                _ => Err(InterpError),
            }
        }
        Op::Not => {
            let res = interp_expr(e, &env).unwrap();
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
fn interp_binop(lv: Expr, op: Op, rv: Expr, env: &Env) -> Result<Val> {
    match op {
        Op::Add => Ok(Val::Num(
            get_int(interp_expr(lv, &env).unwrap()).unwrap()
            +
            get_int(interp_expr(rv, &env).unwrap()).unwrap()
        )),
        Op::Sub => Ok(Val::Num(
            get_int(interp_expr(lv, &env).unwrap()).unwrap()
            -
            get_int(interp_expr(rv, &env).unwrap()).unwrap()
        )),
        Op::Div => Ok(Val::Num(
            get_int(interp_expr(lv, &env).unwrap()).unwrap()
            /
            get_int(interp_expr(rv, &env).unwrap()).unwrap()
        )),
        Op::Multi => Ok(Val::Num(
            get_int(interp_expr(lv, &env).unwrap()).unwrap()
            *
            get_int(interp_expr(rv, &env).unwrap()).unwrap()
        )),
        Op::Mod => Ok(Val::Num(
            get_int(interp_expr(lv, &env).unwrap()).unwrap()
            %
            get_int(interp_expr(rv, &env).unwrap()).unwrap()
        )),
        Op::LessEqThen => Ok(Val::Bool(
            get_int(interp_expr(lv, &env).unwrap()).unwrap()
            <=
            get_int(interp_expr(rv, &env).unwrap()).unwrap()
        )),
        Op::LargEqThen => Ok(Val::Bool(
            get_int(interp_expr(lv, &env).unwrap()).unwrap()
            >=
            get_int(interp_expr(rv, &env).unwrap()).unwrap()
        )),
        Op::LessThen => Ok(Val::Bool(
            get_int(interp_expr(lv, &env).unwrap()).unwrap()
            <
            get_int(interp_expr(rv, &env).unwrap()).unwrap()
        )),
        Op::LargThen => Ok(Val::Bool(
            get_int(interp_expr(lv, &env).unwrap()).unwrap()
            >
            get_int(interp_expr(rv, &env).unwrap()).unwrap()
        )),
        Op::Equal => Ok(Val::Bool(
            get_int(interp_expr(lv, &env).unwrap()).unwrap()
            ==
            get_int(interp_expr(rv, &env).unwrap()).unwrap()
        )),
        Op::And => Ok(Val::Bool(
            get_bool(interp_expr(lv, &env).unwrap()).unwrap()
            &&
            get_bool(interp_expr(rv, &env).unwrap()).unwrap()
        )),
        Op::Or => Ok(Val::Bool(
            get_bool(interp_expr(lv, &env).unwrap()).unwrap()
            ||
            get_bool(interp_expr(rv, &env).unwrap()).unwrap()
        )),
        Op::NotEq => Ok(Val::Bool(
            get_bool(interp_expr(lv, &env).unwrap()).unwrap()
            !=
            get_bool(interp_expr(rv, &env).unwrap()).unwrap()
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



fn interp_assign(ident: Expr, value: Expr, env: &Env) -> Result<Val> {
    match ident {
        Expr::Assign(i, _t) =>{
            let val = interp_expr(value, &env).unwrap();
            // env.store(&i.to_string(), val.clone());
            return Ok(val);
        },
        _ => {
            let val = interp_expr(value, &env).unwrap();
            // env.store("test", val.clone());
            return Ok(val);
        },
    }
}
