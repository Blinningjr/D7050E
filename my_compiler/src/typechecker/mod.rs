/**
 *  Imports from parser.
 */
#[path = "../parser/mod.rs"]
mod parser;
use crate::parser::{
    SpanExpr,
    expr::Expr,
    op::Op,
    mytype::MyType,
};


#[path = "../interpreter/mod.rs"]
mod interpreter;
use crate::interpreter::{
    Val,
    SpanVal,
};


// /** 
//  *  Typecheck ast.
// */
// pub fn typecheck_ast<'a>(e: SpanExpr<'a>) -> Result<SpanVal<'a>> {

// }


// /** 
//  *  Typecheck ast.
// */
// pub fn typecheck_num<'a>(e: SpanExpr<'a>) -> Result<SpanVal<'a>> {
    
// }
