/**
 *  std imports.
 */
use std::{
    str::FromStr,
    fmt,
};


use super::syntaxerror::{Result, SyntaxError};


/** 
 *  Defining all of my types.
 */
#[derive(Debug, PartialEq, Clone)]
pub enum MyType {
    Int32,
    Boolean,
    Str,
    None,
}


/**
 *  Converts string to MyType.
 */
impl FromStr for MyType {
    type Err = SyntaxError;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "i32" => Ok(MyType::Int32),
            "bool" => Ok(MyType::Boolean),
            "Str" => Ok(MyType::Str),
            "None" => Ok(MyType::None),
            _ => Err(SyntaxError),
        }
    }
}


/**
 * to_string() for MyType.
 */
impl fmt::Display for MyType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MyType::Int32 =>  write!(f, "{}", "Int32"),
            MyType::Boolean =>  write!(f, "{}", "Boolean"),
            MyType::Str =>  write!(f, "{}", "Str"),
            MyType::None =>  write!(f, "{}", "None"),
        }

    }
}
