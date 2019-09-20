extern crate nom;


/**
 *  nom imports.
 */
use nom::{
    IResult,
    combinator::map,
};


/**
 *  Imports from parser.
 */
#[path = "../parser/mod.rs"]
mod parser;
use crate::parser::Expr;
use crate::parser::Op;
use crate::parser::MyType;
use crate::parser::SyntaxError;
use crate::parser::Result;


/**
 *  Calculates the value of an math expression.
 */
pub fn interp_math_expr(e: Expr) -> Result<i32> {
    match e {
        Expr::Num(i) => Ok(i),
        Expr::BinOp(l, op, r) => {
            let left_value = interp_math_expr(*l).unwrap();
            let right_value = interp_math_expr(*r).unwrap();
            match op {
                Op::Add => Ok(left_value + right_value),
                Op::Sub => Ok(left_value - right_value),
                Op::Div => Ok(left_value / right_value),
                Op::Multi => Ok(left_value * right_value),
                Op::Mod => Ok(left_value % right_value),
                _ => Err(SyntaxError),
            }
        }
        Expr::UnOp(op, r) => {
            let right_value = interp_math_expr(*r).unwrap();
            match op {
                Op::Sub => Ok(-right_value),
                _ => Err(SyntaxError),
            }
        }
        _ => Err(SyntaxError),
    }
}
