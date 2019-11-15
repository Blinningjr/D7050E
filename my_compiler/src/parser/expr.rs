#![allow(dead_code)]

/**
 *  Imports.
 */
use super::{
    SpanExpr,
    SpanMyType,
    SpanOp,
    SpanPrefix,
};


/** 
 *  Defining all types of expr.
 */
#[derive(Debug, PartialEq, Clone)]
pub enum Expr<'a> {
    
    Num(i32),
    Bool(bool),
    Type(SpanMyType<'a>), // onödig?

    Prefixed(SpanPrefix<'a>, Box<SpanExpr<'a>>),

    Var(&'a str),
    VarWithType(Box<SpanExpr<'a>>, Box<SpanExpr<'a>>), // onödig?

    UnOp(SpanOp<'a>, Box<SpanExpr<'a>>),
    BinOp(Box<SpanExpr<'a>>, SpanOp<'a>, Box<SpanExpr<'a>>),
    
    Let(Box<SpanExpr<'a>>, Box<SpanExpr<'a>>),
    
    Assign(Box<SpanExpr<'a>>, Box<SpanExpr<'a>>),
    If(Box<SpanExpr<'a>>, Box<SpanExpr<'a>>,  Box<SpanExpr<'a>>),

    Body(Vec<SpanExpr<'a>>), 

    While(Box<SpanExpr<'a>>, Box<SpanExpr<'a>>),
    Func(Box<SpanExpr<'a>>, Vec<SpanExpr<'a>>, Box<SpanExpr<'a>>, Box<SpanExpr<'a>>),
    Return(Box<SpanExpr<'a>>),
    FuncCall(Box<SpanExpr<'a>>, Vec<SpanExpr<'a>>),
    Funcs(Vec<SpanExpr<'a>>), // onödig?
}
