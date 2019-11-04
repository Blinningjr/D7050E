// use std::collections::HashMap;

// use super::enverror::{EnvError, Result};
// use crate::parser::expr::Expr;
// use crate::parser::mytype::MyType;


// /** 
//  *  Defines Scope. 
//  *  Scope stores the variables and functions declarde in scope. 
//  *  It allso stores the previous scope and the scope to return to.  
//  */
// #[derive(Debug, PartialEq, Clone)]
// struct Scope<'a> {
//     mem_var: HashMap<String, MyType>,
//     mem_func: HashMap<String, (Vec<MyType>, MyType)>,
//     mem_func_to_check: Vec<Expr<'a>>,
//     prev: i32,
    
// }

// impl<'a> Scope<'a> {
        
//     /**
//      *  Creates a new scope.
//      */
//     fn new(prev_pos: i32) -> Scope<'a> {
//         Scope {
//             mem_var: HashMap::new(),
//             mem_func: HashMap::new(),
//             mem_func_to_check: Vec::new(),
//             prev: prev_pos,
//         }
//     }

//     /**
//      *  Loads variable with name "key" form scope.
//      */
//     fn load_v(&mut self, key: &str) -> Result<MyType> {
//         match self.mem_var.get(key) {
//             Some(t) => Ok(t.clone()),
//             _ => Err(EnvError),
//         }
//     }

//     /**
//      *  Loads function with name "key" form scope.
//      */
//     fn load_f(&mut self, key: &'a str) -> Result<(Vec<MyType>, MyType)> {
//         match self.mem_func.get(key) {
//             Some(ts) => Ok(ts.clone()),
//             _ => Err(EnvError),
//         }
//     }

//     /**
//      *  Stores variable to scope.
//      */
//     fn store_v(&mut self, key: &str, t: MyType) -> Option<MyType> {
//         self.mem_var.insert(key.to_string(), t)
//     }

//     /**
//      *  Stores function to scope.
//      */
//     fn store_f(&mut self, key: &'a str, ts: Vec<MyType>, t: MyType, expr: Expr<'a>) -> Option<(Vec<MyType>, MyType)> {
//         self.mem_func_to_check.push(expr);
//         self.mem_func.insert(key.to_string(), (ts, t))
//     }

//     /**
//      *  Gets previous scope.
//      */
//     fn get_prev(&mut self) -> i32 {
//         self.prev
//     }

//     fn get_f(&mut self) -> Option<Expr<'a>> {
//         self.mem_func_to_check.pop()
//     }

//     fn get_fs_len(&mut self) -> i32 {
//         self.mem_func_to_check.len() as i32
//     }
// }


// /**
//  *  Defines Env.
//  *  Env stores all the scopes in to a vector.
//  *  Env also stores the current scope in use. 
//  *  And the scope that will be saved as return scope for the next created scope.
//  */
// #[derive(Debug, PartialEq, Clone)]
// pub struct Env<'a> {
//     scopes: Vec<Scope<'a>>,
//     scope_pos: i32,
// }

// impl<'a> Env<'a> {
    
//     /**
//      *  Creates a new Env.
//      *  Ops! It dose not create a initial scope.
//      */
//     pub fn new() -> Env<'a> {
//         Env {
//             scopes: Vec::new(),
//             scope_pos: -1,
//         }
//     }

//     /** 
//      *  Creates a new Scope and stores it in the vec scopes.
//      *  Sets the new scope position and return scope position.
//      */
//     pub fn crate_scope(&mut self) -> () {
//         self.scopes.push(Scope::new(self.scope_pos));
//         self.scope_pos = (self.scopes.len() as i32) -1;
//     }

//     /**
//      *  Removes top scope.
//      *  Also sets the correct scope position and return scope position.
//      */
//     pub fn pop_scope(&mut self) -> () {
//         self.scope_pos = self.scopes[self.scope_pos as usize].prev;
//         self.scopes.pop();
//     }

//     /**
//      *  Stores a variable in the current scope. 
//      *  Panic!: If there already exists a variable with the same name in the current scope 
//      *  or one of it's previouse scopes.
//      */
//     pub fn store_var(&mut self, key: &'a str, t: MyType) -> Option<MyType> {
//         let res = self.load_var(key);
//         match res {
//             Ok(_) => panic!("store_var: {:?} {:?}", key, t),
//             Err(_) => return self.scopes[self.scope_pos as usize].store_v(key, t),
//         }
//     }

//     /**
//      *  Stores a function in the current scope. 
//      *  Panic!: If there already exists a function with the same name in the current scope 
//      *  or one of it's previouse scopes.
//      */
//     pub fn store_func(&mut self, key: &'a str, ts: Vec<MyType>, t: MyType, expr: Expr<'a>) -> Option<(Vec<MyType>, MyType)> {
//         let res = self.load_func(key);
//         match res {
//             Ok(_) => panic!("store_func"),
//             Err(_) => self.scopes[self.scope_pos as usize].store_f(key, ts, t, expr),
//         }
//     }

//     /**
//      *  Loads the variable.
//      */
//     pub fn load_var(&mut self, key: &str) -> Result<MyType> {
//         self.help_load_var(key, self.scope_pos)
//     }

//     /**
//      *  Helper function to load_var.
//      */
//     fn help_load_var(&mut self, key: &str, pos: i32 ) -> Result<MyType> {
//         if pos >= 0 {
//             let res = self.scopes[pos as usize].load_v(key);
//             match res {
//                 Ok(tup) => return Ok(tup),
//                 _ => {
//                     let p = self.scopes[pos as usize].get_prev();
//                     return self.help_load_var(key, p);
//                 },
//             }
//         }
//         Err(EnvError)
//     }

//     /**
//      *  Loads function from current scope or one of it's previouse scopes.
//      */
//     pub fn load_func(&mut self, key: &'a str) -> Result<(Vec<MyType>, MyType)> {
//         let mut pos = self.scope_pos;
//         while pos >= 0 {
//             let res = self.scopes[pos as usize].load_f(key);
//             match res {
//                 Ok(_) => {
//                     // self.scope_pos = pos; 
//                     return res
//                 },
//                 _ => {
//                     pos = self.scopes[pos as usize].get_prev();
//                 },
//             }
//         } 
//         Err(EnvError)
//     }

//     pub fn get_funcs_len(&mut self) -> i32 {
//         self.scopes[self.scope_pos as usize].get_fs_len()
//     }

//     pub fn get_func(&mut self) -> Option<Expr<'a>> {
//         self.scopes[self.scope_pos as usize].get_f()
//     }
// }
