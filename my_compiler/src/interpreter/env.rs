// use std::collections::HashMap;

// use super::val::Val;

// use super::interperror::{InterpError, Result};

// use crate::parser::expr::Expr;

// #[derive(Debug, PartialEq, Clone)]
// pub enum NextEnv<'a> {
//     Empty,
//     Next(Box<Env<'a>>)
// }

// /** 
//  *  Defins Env that stores variables and functions.
// */
// #[derive(Debug, PartialEq, Clone)]
// pub struct Env<'a> {
//     mem_var: HashMap<String, Val>,
//     mem_func: HashMap<String, Expr<'a>>,
//     mem_next: NextEnv<'a>,
// }
// impl<'a> Env<'a> {
//     pub fn new() -> Env<'a> {
//         Env {
//             mem_var: HashMap::new(),
//             mem_func: HashMap::new(),
//             mem_next: NextEnv::Empty,
//         }
//     }
//     pub fn store_var(&mut self, ident: String, val: Val) -> std::option::Option<Val> {
//         match self.mem_var.get(&ident) {
//             Some(_) => self.mem_var.insert(ident, val),
//             _ => {
//                 match &mut self.mem_next {
//                     NextEnv::Empty => self.mem_var.insert(ident, val),
//                     NextEnv::Next(e) => e.store_var(ident, val),
//                 }
//             },
//         }
//     }
//     pub fn store_func(&mut self, ident: String, func: Expr<'a>) -> std::option::Option<Expr<'a>> {
//         match self.mem_func.get(&ident) {
//             Some(_) => self.mem_func.insert(ident, func),
//             _ => {
//                 match &mut self.mem_next {
//                     NextEnv::Empty => self.mem_func.insert(ident, func),
//                     NextEnv::Next(e) => e.store_func(ident, func),
//                 }
//             },
//         }
//     }
//     pub fn load_var(&mut self, key: &'a str) -> Result<Val>{
//         match self.mem_var.get(key) {
//             Some(val) => Ok(val.clone()),
//             _ => {
//                 match &mut self.mem_next {
//                     NextEnv::Empty => Err(InterpError),
//                     NextEnv::Next(e) => e.load_var(key),
//                 }
//             },
//         }
//     }
//     pub fn load_func(&mut self, key: &'a str) -> Result<(Expr, Env<'a>)>{
//         match self.mem_func.get(key) {
//             Some(e) => {
//                 let mut env = self.clone();
//                 self.mem_next = NextEnv::Empty;
//                 Ok((e.clone(), env))
//             },
//             _ => {
//                 match &mut self.mem_next {
//                     NextEnv::Empty => Err(InterpError),
//                     NextEnv::Next(e) => e.load_func(key),
//                 }
//             },
//         }
//     }
// }