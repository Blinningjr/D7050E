// use std::collections::HashMap;

// use super::enverror::{EnvError, Result};
// use crate::parser::expr::Expr;
// use crate::parser::varprefix::Prefix;

// use super::VarInfo;



// /** 
//  *  Defines Scope. 
//  *  Scope stores the variables and functions declarde in scope. 
//  *  It allso stores the previous scope and the scope to return to.  
//  */
// #[derive(Debug, PartialEq, Clone)]
// struct Scope<'a> {
//     mem_var: HashMap<String, usize>,
//     mem: Vec<VarInfo>,

//     mem_func: HashMap<String, (Vec<Prefix>, Prefix)>,
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
//             mem: Vec::new(),
//             prev: prev_pos,
//         }
//     }

//     /**
//      *  Loads variable with name "key" form scope.
//      */
//     fn load_v(&mut self, key: &str) -> Result<VarInfo> {
//         match self.mem_var.get(key) {
//             Some(val) => Ok(self.mem[*val].clone()),
//             _ => Err(EnvError),
//         }
//     }

//     /**
//      *  Loads function with name "key" form scope.
//      */
//     fn load_f(&mut self, key: &'a str) -> Result<(Vec<Prefix>, Prefix)> {
//         match self.mem_func.get(key) {
//             Some(ts) => Ok(ts.clone()),
//             _ => Err(EnvError),
//         }
//     }

//     /**
//      *  Stores variable to scope.
//      */
//     fn store_v(&mut self, val: VarInfo) -> Option<usize> {
//         self.mem.push(val.clone());
//         return self.mem_var.insert(key.to_string(), self.mem.len() - 1);
//     }

//     fn store_v_mem(&mut self, val: VarInfo) -> () {
//         self.mem.push(val.clone());
//     }

//     /**
//      *  Stores function to scope.
//      */
//     fn store_f(&mut self, key: &'a str, ts: Vec<Prefix>, t: Prefix, expr: Expr<'a>) -> Option<(Vec<Prefix>, Prefix)> {
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

//     fn get_pos(&mut self, key: &str) -> Result<usize> {
//         match self.mem_var.get(key) {
//             Some(val) => Ok((*val).clone()),
//             _ => Err(EnvError),
//         }
//     }
//     fn get_val(&mut self, pos: usize) -> Result<VarInfo> {
//         Ok(self.mem[pos].clone())
//     }
//     fn update_val(&mut self, pos: usize, val: VarInfo) -> () {
//         self.mem[pos] = val;
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
//     pub fn store_var(&mut self, val: VarInfo) -> () {
//         match val {
//             VarInfo::Ident(_, i, _, _) => {

//             },
//             _ => 
//         };
//     }

//     /**
//      *  Stores a function in the current scope. 
//      *  Panic!: If there already exists a function with the same name in the current scope 
//      *  or one of it's previouse scopes.
//      */
//     pub fn store_func(&mut self, key: &'a str, ts: Vec<Prefix>, t: Prefix, expr: Expr<'a>) -> Option<(Vec<Prefix>, Prefix)> {
//         let res = self.load_func(key);
//         match res {
//             Ok(_) => panic!("store_func"),
//             Err(_) => self.scopes[self.scope_pos as usize].store_f(key, ts, t, expr),
//         }
//     }

//     /**
//      *  Loads the variable.
//      *  Panic!: If variable dosen't exist in scope or one of the previous scopes.
//      */
//     pub fn load_var(&mut self, key: &str, numderef: i32) -> Result<VarInfo> {
//         let mem_pos;
//         match self.get_var_pos(key.clone()) {
//             Ok(p) => mem_pos = p,
//             Err(e) => return Err(e),
//         };
//         let pos;
//         match self.get_var_scope(key.clone()) {
//             Ok(p) => pos = p,
//             Err(e) => return Err(e),
//         };
//         self.help_load_var(mem_pos, numderef, pos)
//     }

//     /**
//      *  Helper function to load_var.
//      */
//     pub fn help_load_var(&mut self, mem_pos: usize, numderef: i32, pos: i32 ) -> Result<VarInfo> {
//         if pos >= 0 {
//             let res = self.scopes[pos as usize].get_val(mem_pos);
//             match res {
//                 Ok(tup) => {
//                     if numderef > 0 {
//                         match tup {
//                             VarInfo::Pointer(_, pos_mem, p, _, _) => {
//                                 self.help_load_var(pos_mem, numderef- 1, p);
//                             },
//                             _ => panic!("help_load_var"),
//                         };
//                     }
//                     return Ok(tup);
//                 },
//                 _ => {
//                     let p = self.scopes[pos as usize].get_prev();
//                     return self.help_load_var(mem_pos, numderef, p);
//                 },
//             }
//         }
//         Err(EnvError)
//     }

//     /**
//      *  Loads function from current scope or one of it's previouse scopes.
//      */
//     pub fn load_func(&mut self, key: &'a str) -> Result<(Vec<Prefix>, Prefix)> {
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

//     // /**
//     //  *  Gets the value of var. value meaning a Val::Num or Val::Boolean.
//     //  */
//     // pub fn get_var_value(&mut self, key: &str, scope: i32) -> Result<VarInfo> {
//     //     let mut pos = scope;
//     //     while pos >= 0 {
//     //         let res = self.scopes[pos as usize].load_v(key);
//     //         match res {
//     //             Ok(tup) => {
//     //                 match tup {
//     //                     Val::Ident(i, n) => return self.get_var_value(&i, n),
//     //                     _ => return Ok(tup.1),
//     //                 }
//     //             },
//     //             _ => pos = self.scopes[pos as usize].get_prev(),
//     //         }
//     //     }
//     //     Err(EnvError)
//     // }

//     /**
//      *  Updates the value of a variable.
//      *  Panic!: If variable dosen't exists.
//      */
//     pub fn assign_var(&mut self, key: &str, val: VarInfo, numderef: i32) -> () {
//         let mem_pos;
//         match self.get_var_pos(key.clone()) {
//             Ok(p) => mem_pos = p,
//             Err(_) => panic!("assign_var"),
//         };
//         let pos;
//         match self.get_var_scope(key.clone()) {
//             Ok(p) => pos = p,
//             Err(_) => panic!("assign_var"),
//         };
//         self.help_assign_var(mem_pos, pos, val, numderef)
//     }

//     /**
//      *  Helper function for assign_var.
//      */
//     fn help_assign_var(&mut self, mem_pos: usize, pos: i32, val: VarInfo, numderef: i32) -> () {
//         if pos >= 0 {
//             let res = self.scopes[pos as usize].get_val(mem_pos);
//             match res {
//                 Ok(tup) => {
//                     if numderef > 0 {
//                         match tup {
//                             VarInfo::Pointer(_, pos_mem, p, _, _) => {
//                                 self.help_assign_var(pos_mem, p, val, numderef- 1);
//                             },
//                             _ => panic!("help_load_var"),
//                         };
//                     }
//                     self.scopes[pos as usize].update_val(mem_pos, val);
//                 },
//                 _ => {
//                     let prev = self.scopes[pos as usize].get_prev();
//                     return self.help_assign_var(mem_pos, prev, val, numderef)
//                 },
//             }
//         }
//         panic!("help_assign_var");
//     }

//     /**
//      *  Gets the scope were a var is located.
//      *  Looks for the var in current scope and it's return scopes.
//      */
//     fn get_var_scope(&mut self, key: &str) -> Result<i32> {
//         let mut pos = self.scope_pos;
//         while pos >= 0 {
//             let res = self.scopes[pos as usize].load_v(key);
//             match res {
//                 Ok(_) => return Ok(pos),
//                 _ => pos = self.scopes[pos as usize].get_prev(),
//             }
//         } 
//         return Err(EnvError);
//     }

//     /**
//      *  Gets the mem pos of were a var is located.
//      *  Looks for the var in current scope and it's return scopes.
//      */
//     fn get_var_pos(&mut self, key: &str) -> Result<usize> {
//         let s;
//         match self.get_var_scope(key) {
//             Ok(v) => s = v,
//             Err(e) => return Err(e),
//         };
//         self.scopes[s as usize].get_pos(key)
//     }

//     pub fn getPos(&mut self) -> i32 {
//         return self.scope_pos;
//     }
// }
