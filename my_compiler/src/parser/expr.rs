/**
 *  Imports.
 */
use super::{
    SpanExpr,
    SpanMyType,
    SpanOp,
};


/** 
 *  Defining all types of expr.
 */
#[derive(Debug, PartialEq, Clone)]
pub enum Expr<'a> {
    Empty,
    Num(i32),
    Bool(bool),
    Ident(&'a str),
    Type(SpanMyType<'a>),
    Assign(Box<SpanExpr<'a>>, Box<SpanExpr<'a>>),
    UnOp(SpanOp<'a>, Box<SpanExpr<'a>>),
    BinOp(Box<SpanExpr<'a>>, SpanOp<'a>, Box<SpanExpr<'a>>),
    Body(Vec<SpanExpr<'a>>),
    If(Box<SpanExpr<'a>>, Box<SpanExpr<'a>>,  Box<SpanExpr<'a>>),
    While(Box<SpanExpr<'a>>, Box<SpanExpr<'a>>),
    Func(Box<SpanExpr<'a>>, Box<SpanExpr<'a>>, SpanMyType<'a>, Box<SpanExpr<'a>>),
    Param(Vec<SpanExpr<'a>>),
    Funcs(Vec<SpanExpr<'a>>),
    FuncCall(Box<SpanExpr<'a>>, Box<SpanExpr<'a>>),
    UpdateVar(Box<SpanExpr<'a>>, Box<SpanExpr<'a>>),
    Return(Box<SpanExpr<'a>>),
}
