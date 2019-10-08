use std::collections::HashMap;

use super::val::Val;

use super::enverror::{EnvError, Result};

use crate::parser::expr::Expr;
use crate::parser::varprefix::Prefix;


#[derive(Debug, PartialEq, Clone)]
struct Scope<'a> {
    mem_var: HashMap<String, (Prefix, Val)>,
    mem_func: HashMap<String, Expr<'a>>,
    prev: i32,
    return_scope: i32,
    
}
impl<'a> Scope<'a> {
    fn new(prev_pos: i32, return_pos: i32) -> Scope<'a> {
        Scope {
            mem_var: HashMap::new(),
            mem_func: HashMap::new(),
            prev: prev_pos,
            return_scope: return_pos,
        }
    }
    fn load_v(&mut self, key: &str) -> Result<(Prefix, Val)> {
        match self.mem_var.get(key) {
            Some(val) => Ok(val.clone()),
            _ => Err(EnvError),
        }
    }
    fn load_f(&mut self, key: &'a str) -> Result<Expr<'a>> {
        match self.mem_func.get(key) {
            Some(expr) => Ok(expr.clone()),
            _ => Err(EnvError),
        }
    }
    fn store_v(&mut self, key: &'a str, val: Val, prefix: Prefix) -> Option<(Prefix, Val)> {
        self.mem_var.insert(key.to_string(), (prefix, val.clone()))
    }
    fn store_f(&mut self, key: &'a str, func: Expr<'a>) -> Option<Expr<'a>> {
        self.mem_func.insert(key.to_string(), func.clone())
    }
    fn get_prev(&mut self) -> i32 {
        self.prev
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Env<'a> {
    scopes: Vec<Scope<'a>>,
    scope_pos: i32,
    return_pos: i32,
}
impl<'a> Env<'a> {
    pub fn new() -> Env<'a> {
        Env {
            scopes: Vec::new(),
            scope_pos: -1,
            return_pos: -1,
        }
    }
    pub fn crate_scope(&mut self) -> () {
        self.scopes.push(Scope::new(self.scope_pos, self.return_pos));
        self.scope_pos = (self.scopes.len() as i32) -1;
        self.return_pos = self.scope_pos;
    }
    pub fn pop_scope(&mut self) -> () {
        self.scope_pos = self.scopes[self.scope_pos as usize].return_scope;
        self.return_pos = self.scope_pos;
        self.scopes.pop();
    }
    pub fn store_var(&mut self, key: &'a str, val: Val, prefix: Prefix) -> Option<(Prefix, Val)> {
        let res = self.load_var(key);
        match res {
            Ok(_) => panic!("store_var {:?} {:?}", key, val),
            Err(_) =>  self.scopes[self.scope_pos as usize].store_v(key, val, prefix),
        }
    }
    pub fn store_func(&mut self, key: &'a str, func: Expr<'a>) -> Option<Expr<'a>> {
        let res = self.load_func(key);
        match res {
            Ok(_) => panic!("store_func"),
            Err(_) => self.scopes[self.scope_pos as usize].store_f(key, func),
        }
    }
    pub fn load_var(&mut self, key: &str) -> Result<Val> {
        let mut pos = self.scope_pos;
        while pos >= 0 {
            let res = self.scopes[pos as usize].load_v(key);
            match res {
                Ok(tup) => {
                        match tup.1 {
                            Val::Ident(s) => return self.load_var(&s),
                            _ => return Ok(tup.1),
                        };
                    },
                _ => {
                    pos = self.scopes[pos as usize].get_prev();
                },
            }
        }
        Err(EnvError)
    }
    pub fn load_func(&mut self, key: &'a str) -> Result<Expr<'a>> {
        let mut pos = self.scope_pos;
        while pos >= 0 {
            let res = self.scopes[pos as usize].load_f(key);
            match res {
                Ok(_) => {
                    self.return_pos = self.scope_pos;
                    self.scope_pos = pos; 
                    return res
                },
                _ => {
                    pos = self.scopes[pos as usize].get_prev();
                },
            }
        } 
        Err(EnvError)
    }
    pub fn assign_var(&mut self, key: &'a str, val: Val) -> Option<(Prefix, Val)> {
        let mut pos = self.scope_pos;
        while pos >= 0 {
            let res = self.scopes[pos as usize].load_v(key);
            match res {
                Ok(tup) => {
                    match tup.0 {
                        Prefix::BorrowMut => return self.scopes[pos as usize].store_v(key, val, tup.0),
                        Prefix::Mut => return self.scopes[pos as usize].store_v(key, val, tup.0),
                        Prefix::Borrow => panic!("Can't assign_var to none mut var value"),
                        Prefix::None => panic!("Can't assign_var to none mut value value"),
                    }
                },
                _ => {
                    pos = self.scopes[pos as usize].get_prev();
                },
            }
        } 
        panic!("assign_var");
    }
}
