use std::collections::HashMap;

use super::val::Val;

use super::interperror::{InterpError, Result};

use crate::parser::expr::Expr;

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
    pub fn load_func(&mut self, key: &'a str, pv: Vec<Expr>) -> Result<(Expr, Env)>{
        match self.mem_func.get(key) {
            Some(tup) => Ok(tup.clone()),
            _ => Err(InterpError),
        }
    }
}