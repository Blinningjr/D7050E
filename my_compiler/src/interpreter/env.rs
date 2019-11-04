// use std::collections::HashMap;

// use super::val::Val;

// use super::enverror::{EnvError, Result};

// use crate::parser::expr::Expr;
// use crate::parser::varprefix::Prefix;


// /** 
//  *  Defines Scope. 
//  *  Scope stores the variables and functions declarde in scope. 
//  *  It allso stores the previous scope and the scope to return to.  
//  */
// #[derive(Debug, PartialEq, Clone)]
// struct Scope<'a> {
//     mem_var: HashMap<String, (Prefix, Val)>,
//     mem_func: HashMap<String, Expr<'a>>,
//     prev: i32,
//     return_scope: i32,
    
// }

// impl<'a> Scope<'a> {
        
//     /**
//      *  Creates a new scope.
//      */
//     fn new(prev_pos: i32, return_pos: i32) -> Scope<'a> {
//         Scope {
//             mem_var: HashMap::new(),
//             mem_func: HashMap::new(),
//             prev: prev_pos,
//             return_scope: return_pos,
//         }
//     }

//     /**
//      *  Loads variable with name "key" form scope.
//      */
//     fn load_v(&mut self, key: &str) -> Result<(Prefix, Val)> {
//         match self.mem_var.get(key) {
//             Some(val) => Ok(val.clone()),
//             _ => Err(EnvError),
//         }
//     }

//     /**
//      *  Loads function with name "key" form scope.
//      */
//     fn load_f(&mut self, key: &'a str) -> Result<Expr<'a>> {
//         match self.mem_func.get(key) {
//             Some(expr) => Ok(expr.clone()),
//             _ => Err(EnvError),
//         }
//     }

//     /**
//      *  Stores variable to scope.
//      */
//     fn store_v(&mut self, key: &str, val: Val, prefix: Prefix) -> Option<(Prefix, Val)> {
//         self.mem_var.insert(key.to_string(), (prefix, val.clone()))
//     }

//     /**
//      *  Stores function to scope.
//      */
//     fn store_f(&mut self, key: &'a str, func: Expr<'a>) -> Option<Expr<'a>> {
//         self.mem_func.insert(key.to_string(), func.clone())
//     }

//     /**
//      *  Gets previous scope.
//      */
//     fn get_prev(&mut self) -> i32 {
//         self.prev
//     }

//     /**
//      *  Gets return scope.
//      */
//     fn get_return(&mut self) -> i32 {
//         self.return_scope
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
//     return_pos: i32,
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
//             return_pos: -1,
//         }
//     }

//     /** 
//      *  Creates a new Scope and stores it in the vec scopes.
//      *  Sets the new scope position and return scope position.
//      */
//     pub fn crate_scope(&mut self) -> () {
//         self.scopes.push(Scope::new(self.scope_pos, self.return_pos));
//         self.scope_pos = (self.scopes.len() as i32) -1;
//         self.return_pos = self.scope_pos;
//     }

//     /**
//      *  Removes top scope.
//      *  Also sets the correct scope position and return scope position.
//      */
//     pub fn pop_scope(&mut self) -> () {
//         self.scope_pos = self.scopes[self.scope_pos as usize].return_scope;
//         self.return_pos = self.scope_pos;
//         self.scopes.pop();
//     }

//     /**
//      *  Stores a variable in the current scope. 
//      *  Panic!: If there already exists a variable with the same name in the current scope 
//      *  or one of it's previouse scopes.
//      */
//     pub fn store_var(&mut self, key: &'a str, val: Val, prefix: Prefix) -> Option<(Prefix, Val)> {
//         let res = self.load_var(key, 0);
//         match res {
//             Ok(_) => panic!("store_var {:?} {:?}", key, val),
//             Err(_) =>  {
//                 let mut value = val.clone();
//                 match val {
//                     Val::Ident(i, _) => value = Val::Ident(i.clone(), self.get_var_scope(&i)),
//                     _ => (),
//                 }
//                 return self.scopes[self.scope_pos as usize].store_v(key, value, prefix)
//             },
//         }
//     }

//     /**
//      *  Stores a function in the current scope. 
//      *  Panic!: If there already exists a function with the same name in the current scope 
//      *  or one of it's previouse scopes.
//      */
//     pub fn store_func(&mut self, key: &'a str, func: Expr<'a>) -> Option<Expr<'a>> {
//         let res = self.load_func(key);
//         match res {
//             Ok(_) => panic!("store_func"),
//             Err(_) => self.scopes[self.scope_pos as usize].store_f(key, func),
//         }
//     }

//     /**
//      *  Loads the variable.
//      *  Panic!: If variable dosen't exist in scope or one of the previous scopes.
//      */
//     pub fn load_var(&mut self, key: &str, numderef: i32) -> Result<Val> {
//         self.help_load_var(key, numderef, self.scope_pos)
//     }

//     /**
//      *  Helper function to load_var.
//      */
//     fn help_load_var(&mut self, key: &str, numderef: i32, pos: i32 ) -> Result<Val> {
//         if pos >= 0 {
//             let res = self.scopes[pos as usize].load_v(key);
//             match res {
//                 Ok(tup) => {
//                     if numderef > 0 {
//                         match tup.1 {
//                             Val::Ident(k, p) => return self.help_load_var(&k, numderef -1, p),
//                             _ => panic!("help_load_var"),
//                         };
//                     }
//                     return Ok(tup.1);
//                 },
//                 _ => {
//                     let p = self.scopes[pos as usize].get_prev();
//                     return self.help_load_var(key, numderef, p);
//                 },
//             }
//         }
//         Err(EnvError)
//     }

//     /**
//      *  Gets the value of var. value meaning a Val::Num or Val::Boolean.
//      */
//     pub fn get_var_value(&mut self, key: &str, scope: i32) -> Result<Val> {
//         let mut pos = scope;
//         while pos >= 0 {
//             let res = self.scopes[pos as usize].load_v(key);
//             match res {
//                 Ok(tup) => {
//                     match tup.1 {
//                         Val::Ident(i, n) => return self.get_var_value(&i, n),
//                         _ => return Ok(tup.1),
//                     }
//                 },
//                 _ => pos = self.scopes[pos as usize].get_prev(),
//             }
//         }
//         Err(EnvError)
//     }

//     /**
//      *  Loads function from current scope or one of it's previouse scopes.
//      */
//     pub fn load_func(&mut self, key: &'a str) -> Result<Expr<'a>> {
//         let mut pos = self.scope_pos;
//         while pos >= 0 {
//             let res = self.scopes[pos as usize].load_f(key);
//             match res {
//                 Ok(_) => {
//                     self.return_pos = self.scope_pos;
//                     self.scope_pos = pos; 
//                     return res
//                 },
//                 _ => {
//                     pos = self.scopes[pos as usize].get_prev();
//                 },
//             }
//         } 
//         Err(EnvError)
//     }

//     /**
//      *  Updates the value of a variable.
//      *  Panic!: If variable dosen't exists.
//      */
//     pub fn assign_var(&mut self, key: &str, val: Val, numderef: i32) -> Option<(Prefix, Val)> {
//         self.help_assign_var(key, val, self.scope_pos, numderef)
//     }

//     /**
//      *  Helper function for assign_var.
//      */
//     fn help_assign_var(&mut self, key: &str, val: Val, pos: i32,  numderef: i32) -> Option<(Prefix, Val)> {
//         if pos >= 0 {
//             let res = self.scopes[pos as usize].load_v(key);
//             match res {
//                 Ok(tup) => {
//                     if numderef > 0 {
//                         match tup.1 {
//                             Val::Ident(k, p) => return self.help_assign_var(&k, val, p, numderef -1),
//                             _ => panic!("help_assign_var"),
//                         };
//                     }
//                     match tup.0 {
//                         Prefix::Mut => return self.scopes[pos as usize].store_v(key, val, tup.0),
//                         _ => panic!("Can't help_assign_var none mut var"),
//                     }
//                 },
//                 _ => {
//                     let p = self.scopes[pos as usize].get_prev();
//                     return self.help_assign_var(key, val, p, numderef);
//                 },
//             }
//         }
//         panic!("help_assign_var");
//     }

//     /**
//      *  Gets the scope were a var is located.
//      *  Looks for the var in current scope and it's return scopes.
//      */
//     fn get_var_scope(&mut self, key: &str) -> i32 {
//         let mut pos = self.scope_pos;
//         while pos >= 0 {
//             let res = self.scopes[pos as usize].load_v(key);
//             match res {
//                 Ok(_) => return pos,
//                 _ => pos = self.scopes[pos as usize].get_return(),
//             }
//         } 
//         panic!("get_var_scope");
//     }
//     // pub fn get_scope_pos(&mut self) -> i32 {
//     //     self.scope_pos
//     // }
// }
