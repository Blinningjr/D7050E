use std::collections::HashMap;

use super::val::Val;

use super::interperror::{InterpError, Result};

use crate::parser::expr::Expr;

#[derive(Debug, PartialEq, Clone)]
pub enum FormerEnv<'a> {
    Empty,
    Next(Box<Env<'a>>),
}

/** 
 *  Defins Env that stores variables and functions.
*/
#[derive(Debug, PartialEq, Clone)]
pub struct Env<'a> {
    mem_var: HashMap<String, Val>,
    mem_func: HashMap<String, Expr<'a>>,
    mem_former_env: FormerEnv<'a>,
}
impl<'a> Env<'a> {
    pub fn new() -> Env<'a> {
        Env {
            mem_var: HashMap::new(),
            mem_func: HashMap::new(),
            mem_former_env: FormerEnv::Empty,
        }
    }
    pub fn store_var(&mut self, ident: &'a str, val: Val) -> Result<Val> {
        let res = self.load_var(ident);
        match res {
            Ok(_) => Err(InterpError),
            Err(_) =>  {self.mem_var.insert(ident.to_string(), val.clone()); Ok(val.clone())},
        }
    }
    pub fn store_func(&mut self, ident:  &'a str, func: Expr<'a>) -> Result<Val> {
        let res = self.load_func(ident);
        match res {
            Ok(_) => Err(InterpError),
            Err(_) =>  {self.mem_func.insert(ident.to_string(), func); Ok(Val::Empty)},
        }
    }
    pub fn load_var(&mut self, key: &'a str) -> Result<Val>{
        match self.mem_var.get(key) {
            Some(val) => Ok(val.clone()),
            _ => {
                match &mut self.mem_former_env {
                    FormerEnv::Empty => Err(InterpError),
                    FormerEnv::Next(e) => e.load_var(key),
                }
            },
        }
    }
    pub fn load_func(&mut self, key: &'a str) -> Result<(Expr<'a>, Env<'a>)>{
        match self.mem_func.get(key) {
            Some(e) => {
                Ok((e.clone(), self.crate_next_env()))
            },
            _ => {
                match &mut self.mem_former_env {
                    FormerEnv::Empty => Err(InterpError),
                    FormerEnv::Next(e) => e.load_func(key),
                }
            },
        }
    }
    pub fn crate_next_env(&mut self) ->  Env<'a> {
        let mut env = Env::new();
        env.mem_former_env = FormerEnv::Next(Box::new(self.clone()));
        env
    }
    pub fn update_var(&mut self, ident: String, val: Val) -> Result<Val> {
        match self.mem_var.get(&ident) {
            Some(_) => {self.mem_var.insert(ident, val.clone()); Ok(val.clone())},
            _ => {
                match &mut self.mem_former_env {
                    FormerEnv::Empty => Err(InterpError),
                    FormerEnv::Next(e) => e.update_var(ident, val),
                }
            },
        }
    }
}