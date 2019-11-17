#![allow(dead_code)]

use crate::parser::{
    SpanExpr,
};

#[derive(Debug, PartialEq, Clone)]
pub struct ErrorMessage<'a> {
    pub message: String,
    pub context: SpanExpr<'a>,
    pub start: usize,
}
