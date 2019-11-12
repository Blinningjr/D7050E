use std::collections::HashMap;

use super::enverror::{EnvError, Result};
use crate::parser::expr::Expr;
use crate::parser::varprefix::Prefix;

use super::{
    ValueInfo,
    VarInfo,
    BorrowInfo,
};



/** 
 *  Defines Scope. 
 *  Scope stores the variables and functions declarde in scope. 
 *  It allso stores the previous scope and the scope to return to.  
 */
#[derive(Debug, PartialEq, Clone)]
struct Scope<'a> {
    mem_var: HashMap<String, usize>,
    mem: Vec<BorrowInfo>,

    mem_func: HashMap<String, (Vec<Prefix>, Prefix)>,
    mem_func_to_check: Vec<Expr<'a>>,

    prev: i32,
    
}

impl<'a> Scope<'a> {
        
    /**
     *  Creates a new scope.
     */
    fn new(prev_pos: i32) -> Scope<'a> {
        Scope {
            mem_var: HashMap::new(),
            mem_func: HashMap::new(),
            mem_func_to_check: Vec::new(),
            mem: Vec::new(),
            prev: prev_pos,
        }
    }

    /**
     *  Loads variable with name "key" form scope.
     */
    fn load_var(&mut self, key: &str) -> Result<BorrowInfo> {
        match self.mem_var.get(key) {
            Some(val) => Ok(self.mem[*val].clone()),
            _ => Err(EnvError),
        }
    }

    /**
     *  Loads function with name "key" form scope.
     */
    fn load_f(&mut self, key: &'a str) -> Result<(Vec<Prefix>, Prefix)> {
        match self.mem_func.get(key) {
            Some(ts) => Ok(ts.clone()),
            _ => Err(EnvError),
        }
    }

    /**
     *  Stores variable to scope.
     */
    fn store_var(&mut self, key: String, val: BorrowInfo) -> usize {
        self.mem.push(val.clone());
        self.mem_var.insert(key.to_string(), self.mem.len() - 1);
        return self.mem.len() - 1;
    }

    fn store_val(&mut self, val: BorrowInfo) -> usize {
        self.mem.push(val.clone());
        return self.mem.len() - 1;
    }

    /**
     *  Stores function to scope.
     */
    fn store_f(&mut self, key: &'a str, ts: Vec<Prefix>, t: Prefix, expr: Expr<'a>) -> Option<(Vec<Prefix>, Prefix)> {
        self.mem_func_to_check.push(expr);
        self.mem_func.insert(key.to_string(), (ts, t))
    }

    /**
     *  Gets previous scope.
     */
    fn get_prev(&mut self) -> i32 {
        self.prev
    }

    fn get_f(&mut self) -> Option<Expr<'a>> {
        self.mem_func_to_check.pop()
    }

    fn get_fs_len(&mut self) -> i32 {
        self.mem_func_to_check.len() as i32
    }

    fn get_pos(&mut self, key: &str) -> Result<usize> {
        match self.mem_var.get(key) {
            Some(val) => Ok((*val).clone()),
            _ => Err(EnvError),
        }
    }
    fn get_val(&mut self, pos: usize) -> Result<BorrowInfo> {
        Ok(self.mem[pos].clone())
    }
    fn update_val(&mut self, pos: usize, val: BorrowInfo) -> () {
        self.mem[pos] = val;
    }
    
}


/**
 *  Defines Env.
 *  Env stores all the scopes in to a vector.
 *  Env also stores the current scope in use. 
 *  And the scope that will be saved as return scope for the next created scope.
 */
#[derive(Debug, PartialEq, Clone)]
pub struct Env<'a> {
    scopes: Vec<Scope<'a>>,
    scope_pos: i32,
}

impl<'a> Env<'a> {
    
    /**
     *  Creates a new Env.
     *  Ops! It dose not create a initial scope.
     */
    pub fn new() -> Env<'a> {
        Env {
            scopes: Vec::new(),
            scope_pos: -1,
        }
    }

    /** 
     *  Creates a new Scope and stores it in the vec scopes.
     *  Sets the new scope position and return scope position.
     */
    pub fn crate_scope(&mut self) -> () {
        self.scopes.push(Scope::new(self.scope_pos));
        self.scope_pos = (self.scopes.len() as i32) -1;
    }

    /**
     *  Removes top scope.
     *  Also sets the correct scope position and return scope position.
     */
    pub fn pop_scope(&mut self) -> () {
        self.scope_pos = self.scopes[self.scope_pos as usize].prev;
        self.scopes.pop();
    }

    /**
     *  Stores a variable in the current scope. 
     *  Panic!: If there already exists a variable with the same name in the current scope 
     *  or one of it's previouse scopes.
     */
    pub fn store_var(&mut self, val: BorrowInfo) -> (i32, usize) {
        let pointer;
        let res;
        match val.clone() {
            BorrowInfo::Var(mut v, f) => {
                let lres = self.scopes[self.scope_pos as usize].load_var(&v.ident);
                match lres {
                    Ok(_) => panic!("store_var"),
                    Err(_) => (),
                };

                pointer = (self.scope_pos, self.scopes[self.scope_pos as usize].store_var(v.clone().ident, val));

                v.mem_pos = pointer.1;
                v.scope = pointer.0;
                res = BorrowInfo::Var(v, f);
            },
            BorrowInfo::Value(mut v, f) => {
                pointer = (self.scope_pos, self.scopes[self.scope_pos as usize].store_val(val));
                v.mem_pos = pointer.1;
                v.scope = pointer.0;
                res = BorrowInfo::Value(v, f);
            },
        };
        self.scopes[pointer.0 as usize].update_val(pointer.1, res);
        return pointer;
    }

    /**
     *  Stores a function in the current scope. 
     *  Panic!: If there already exists a function with the same name in the current scope 
     *  or one of it's previouse scopes.
     */
    pub fn store_func(&mut self, key: &'a str, ts: Vec<Prefix>, t: Prefix, expr: Expr<'a>) -> Option<(Vec<Prefix>, Prefix)> {
        let res = self.load_func(key);
        match res {
            Ok(_) => panic!("store_func"),
            Err(_) => self.scopes[self.scope_pos as usize].store_f(key, ts, t, expr),
        }
    }

    /**
     *  Loads the variable.
     *  Panic!: If variable dosen't exist in scope or one of the previous scopes.
     */
    pub fn load_var(&mut self, key: &str, numderef: i32) -> Result<(BorrowInfo, (i32, usize))> {
        let mem_pos;
        match self.get_var_pos(key.clone()) {
            Ok(p) => mem_pos = p,
            Err(e) => return Err(e),
        };
        let pos;
        match self.get_var_scope(key.clone()) {
            Ok(p) => pos = p,
            Err(e) => return Err(e),
        };
        return self.load_val(mem_pos, numderef, pos)
    }

    /**
     *  Helper function to load_var.
     */
    pub fn load_val(&mut self, mem_pos: usize, numderef: i32, pos: i32 ) -> Result<(BorrowInfo, (i32, usize))> {
        if pos >= 0 {
            let res = self.scopes[pos as usize].get_val(mem_pos);
            match res {
                Ok(bi) => {
                    if numderef > 0 {
                        match bi.clone() {
                            BorrowInfo::Var(v, false) => {
                                return self.load_val(v.pointer_mem_pos, numderef - 1, v.pointer_scope_pos);
                            },
                            _ => panic!("load_val"),
                        };
                    }
                    return Ok((bi, (pos, mem_pos)));
                },
                _ => {
                    let p = self.scopes[pos as usize].get_prev();
                    return self.load_val(mem_pos, numderef, p);
                },
            }
        }
        Err(EnvError)
    }

    /**
     *  Loads function from current scope or one of it's previouse scopes.
     */
    pub fn load_func(&mut self, key: &'a str) -> Result<(Vec<Prefix>, Prefix)> {
        let mut pos = self.scope_pos;
        while pos >= 0 {
            let res = self.scopes[pos as usize].load_f(key);
            match res {
                Ok(_) => {
                    // self.scope_pos = pos; 
                    return res
                },
                _ => {
                    pos = self.scopes[pos as usize].get_prev();
                },
            }
        } 
        Err(EnvError)
    }

    pub fn get_funcs_len(&mut self) -> i32 {
        self.scopes[self.scope_pos as usize].get_fs_len()
    }

    pub fn get_func(&mut self) -> Option<Expr<'a>> {
        self.scopes[self.scope_pos as usize].get_f()
    }

    /**
     *  Updates the value of a variable.
     *  Panic!: If variable dosen't exists.
     */
    pub fn assign_var(&mut self, pos: i32, mem_pos: usize, val: BorrowInfo) -> BorrowInfo {
        let value;
        match val {
            BorrowInfo::Value(mut v, _) => {
                v.mem_pos = mem_pos;
                v.scope = pos;
                value = BorrowInfo::Value(v, false);
            },
            BorrowInfo::Var(mut v, _) => {
                v.mem_pos = mem_pos;
                v.scope = pos;
                value = BorrowInfo::Var(v, false);
            },
        };
        
        if pos < 0 {
            panic!("assign_var");
        }
        self.scopes[pos as usize].update_val(mem_pos, value.clone());
        return value;
    }

    // /**
    //  *  Helper function for assign_val.
    //  */
    // pub fn assign_val(&mut self, mem_pos: usize, pos: i32, val: BorrowInfo, numderef: i32) -> () {
    //     if pos >= 0 {
    //         let res = self.scopes[pos as usize].get_val(mem_pos);
    //         match res {
    //             Ok(bi) => {
    //                 let vv;
    //                 if numderef > 0 {
    //                     match bi {
    //                         BorrowInfo::Var(v, _) => {
    //                             vv = self.load_val(v.pointer_mem_pos, numderef- 1, v.pointer_scope_pos)?.1;
    //                         },
    //                         _ => panic!("assign_val"),
    //                     };
    //                 }
    //                 let vall;
    //                 match val {
    //                     BorrowInfo::Value(v, b) => {

    //                     },
    //                     BorrowInfo::Value(v, b) => {
                            
    //                     },
    //                 };

    //                 self.scopes[(vv.1).0 as usize].update_val((vv.1).1, vall.clone());
    //                 return ();
    //             },
    //             _ => {
    //                 let prev = self.scopes[pos as usize].get_prev();
    //                 return self.assign_val(mem_pos, prev, val, numderef)
    //             },
    //         }
    //     }
    //     panic!("assign_val");
    // }

    /**
     *  Gets the scope were a var is located.
     *  Looks for the var in current scope and it's return scopes.
     */
    pub fn get_var_scope(&mut self, key: &str) -> Result<i32> {
        let mut pos = self.scope_pos;
        while pos >= 0 {
            let res = self.scopes[pos as usize].load_var(key);
            match res {
                Ok(_) => return Ok(pos),
                _ => pos = self.scopes[pos as usize].get_prev(),
            }
        } 
        return Err(EnvError);
    }

    /**
     *  Gets the mem pos of were a var is located.
     *  Looks for the var in current scope and it's return scopes.
     */
    pub fn get_var_pos(&mut self, key: &str) -> Result<usize> {
        let s;
        match self.get_var_scope(key) {
            Ok(v) => s = v,
            Err(e) => return Err(e),
        };
        self.scopes[s as usize].get_pos(key)
    }

    pub fn get_pos(&mut self) -> i32 {
        return self.scope_pos;
    }

    pub fn get_value(&mut self, pos: i32, mem_pos: usize) -> Result<BorrowInfo> {
        return self.scopes[pos as usize].get_val(mem_pos);
    }

    pub fn add_borrow(&mut self, pos: i32, mem_pos: usize) -> () {
        let old = self.get_value(pos, mem_pos);
        let mut val = match old {Ok(v) => v, Err(_) =>panic!("add_borrow"),};
        match val {
            BorrowInfo::Value(mut v, _) => {
                v.num_borrows += 1;
                val = BorrowInfo::Value(v, false);
            },
            BorrowInfo::Var(mut v, _) => {
                v.num_borrows += 1;
                val = BorrowInfo::Var(v, false);
            },
        };
        self.check_borrow(val.clone());
        self.scopes[pos as usize].update_val(mem_pos, val);
    }

    pub fn add_borrowmut(&mut self, pos: i32, mem_pos: usize) -> () {
        let old = self.get_value(pos, mem_pos);
        let mut val = match old {Ok(v) => v, Err(_) =>panic!("add_borrow"),};
        match val {
            BorrowInfo::Value(mut v, _) => {
                v.num_borrowmuts += 1;
                val = BorrowInfo::Value(v, false);
            },
            BorrowInfo::Var(mut v, _) => {
                v.num_borrowmuts += 1;
                val = BorrowInfo::Var(v, false);
            },
        };
        self.check_borrow(val.clone());
        self.scopes[pos as usize].update_val(mem_pos, val);
    }

    pub fn check_borrow(&mut self, val: BorrowInfo) -> () {
        match val {
            BorrowInfo::Value(v, _) => {
                if v.num_borrows > 0 {
                    if v.num_borrowmuts != 0 {
                        panic!("check_borrow");
                    }
                } else if v.num_borrowmuts > 1 {
                    panic!("check_borrow");
                }
            },
            BorrowInfo::Var(v, _) => {
                if v.num_borrows > 0 {
                    if v.num_borrowmuts != 0 {
                        panic!("check_borrow");
                    }
                } else if v.num_borrowmuts > 1 {
                    panic!("check_borrow");
                }
            },
        };
    }

    pub fn load_borowinfo(&mut self, val: BorrowInfo, numderef: i32) -> Result<(BorrowInfo, (i32, usize))> {
        match val {
            BorrowInfo::Value(v, _) => {
                return self.load_val(v.mem_pos, numderef, v.scope);
            },
            BorrowInfo::Var(v, _) => {
                return self.load_var(&v.ident, numderef);
            },
        };
    }
}
