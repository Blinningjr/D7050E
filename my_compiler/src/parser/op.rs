/**
 *  std imports.
 */
use std::{
    str::FromStr,
    fmt,
};


use super::syntaxerror::{Result, SyntaxError};


/**
 *  All binary operators.
 */
#[derive(Debug, PartialEq, Clone)]
pub enum Op {
    Add,        // "+"
    Sub,        // "-"
    Div,        // "/"
    Multi,      // "*"
    Mod,        // "%"
    And,        // "&&"
    Or,         // "||"
    Not,        // "!"
    Equal,      // "=="
    NotEq,      // "!="
    LessThen,   // "<"
    LargThen,   // ">"
    LessEqThen, // "<="
    LargEqThen, // ">="  
}


/**
 *  to_string() for Op.
 */
impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Op::Add => write!(f, "{}", "+"),
            Op::Sub => write!(f, "{}", "-"),
            Op::Div => write!(f, "{}", "/"),
            Op::Multi => write!(f, "{}", "*"),
            Op::Mod => write!(f, "{}", "%"),
            Op::And => write!(f, "{}", "&&"),
            Op::Or => write!(f, "{}", "||"),
            Op::Not => write!(f, "{}", "!"),
            Op::Equal => write!(f, "{}", "=="),
            Op::NotEq => write!(f, "{}", "!="),
            Op::LessThen => write!(f, "{}", "<"),
            Op::LargThen => write!(f, "{}", ">"),
            Op::LessEqThen => write!(f, "{}", "<="),
            Op::LargEqThen => write!(f, "{}", ">="),
        }
    }
}


/**
 *  Converts string to Op.
 */
impl FromStr for Op {
    type Err = SyntaxError;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "+" => Ok(Op::Add),
            "-" => Ok(Op::Sub),
            "/" => Ok(Op::Div),
            "*" => Ok(Op::Multi),
            "%" => Ok(Op::Mod),
            "&&" => Ok(Op::And),
            "||" => Ok(Op::Or),
            "!=" => Ok(Op::NotEq),
            "!" => Ok(Op::Not),
            "==" => Ok(Op::Equal),
            "<=" => Ok(Op::LessEqThen),
            ">=" => Ok(Op::LargEqThen),
            "<" => Ok(Op::LessThen),
            ">" => Ok(Op::LargThen),
            _ => Err(SyntaxError),
        }
    }
}
