/**
 *  Imports.
 */
// use std::fmt;
// use super::mytype::MyType;
// use super::op::Op;
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
}


// /**
//  * to_string() for expr.
//  */
// impl fmt::Display for Expr <'_> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             Expr::Num(i) =>  write!(f, "{}", i),
//             Expr::BinOp(l, op, r) => write!(f, "({} {:?} {})", l.to_string(), op,  r.to_string()),
//             Expr::UnOp(op, r) => write!(f, "({:?} {})", op,  r.to_string()),
//             Expr::Bool(b) =>  write!(f, "{}", b),
//             Expr::Ident(s) =>  write!(f, "{}", s),
//             Expr::Type(s) =>  write!(f, ":{:?} =", s.to_string()),
//             Expr::Assign(l, r) => write!(f, "({:?} {:?})", l.to_string(),  r.to_string()),
//             Expr::Body(s) =>  write!(f, "{:?}", s),
//             Expr::If(l, m, r) =>  write!(f, "if {} ({}) else ({})", l.to_string(), m.to_string(), r.to_string()),
//             Expr::Empty =>  write!(f, "{}", "Empty"),
//             Expr::While(l, r) =>  write!(f, "while {} ({})", l.to_string(), r.to_string()),
//             Expr::Func(i, e, t, r) =>  write!(f, "fn {}({}) -> {} ({})", i.to_string(), e.to_string(), t.to_string(), r.to_string()),
//             Expr::Param(s) =>  write!(f, "{:?}", s),
//             Expr::Funcs(s) =>  write!(f, "{:?}", s),
//             Expr::FuncCall(i, p) =>  write!(f, "({:?} {:?})", i.to_string(),  p.to_string()),
//         }
//     }
// }