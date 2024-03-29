#![allow(dead_code)]

/** 
 *  Defins Val so bool and i32 can be returnd.
*/
#[derive(Debug, PartialEq, Clone)]
pub enum Val {
    Num(i32),
    ReturnNum(i32),
    Bool(bool),
    ReturnBool(bool),
    Empty,
    ReturnEmpty,
    Ident(String, i32),
    Borrow(usize, i32),
    BorrowPrimitive(i32, Box<Val>)
}
