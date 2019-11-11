use crate::parser::varprefix::Prefix;


/** 
 *  Defins VarInfo
*/
#[derive(Debug, PartialEq, Clone)]
pub enum BorrowInfo {
    Value(ValueInfo),
    Var(VarInfo),
}


/** 
 *  Defins VarInfo
*/
#[derive(Debug, PartialEq, Clone)]
pub struct VarInfo {
    pub mutable: bool,
    pub prefix: Prefix,
    pub ident: String,

    pub pointer_scope_pos: i32,
    pub pointer_mem_pos: usize,

    pub num_borrows: i32,
    pub num_borrowmuts: i32,
}

/** 
 *  Defins ValueInfo
*/
#[derive(Debug, PartialEq, Clone)]
pub struct ValueInfo {
    pub mutable: bool,
    pub prefix: Prefix,

    pub num_borrows: i32,
    pub num_borrowmuts: i32,
}
