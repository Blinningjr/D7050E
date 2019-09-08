mod test_parser;

extern crate nom;

/**
 *  std imports.
 */
use std::{
    str::FromStr,
    error,
    fmt,
};

/** 
 *  Needed for creating SyntaxError. 
 *  src: https://doc.rust-lang.org/std/str/trait.FromStr.html
 */
type Result<T> = std::result::Result<T, SyntaxError>;
#[derive(Debug, Clone)]
pub struct SyntaxError;

/** 
 * 
 */
impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error something is wrong")
    }
}

/** 
 *  This is important for other errors to wrap this one.
 */ 
impl error::Error for SyntaxError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

/**
 *  nom imports.
 */
use nom::{
    branch::alt,
    IResult,
    combinator::map,
    character::complete::digit1,
    character::complete::multispace0,
    sequence::{preceded, tuple},
    bytes::complete::tag,
    Err::Error,
    error::ErrorKind::Tag,
};

/**
 *  All binary operators.
 */
#[derive(Debug, PartialEq)]
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
 *  Converts string to Op for op.
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

/** 
 *  A tree of Op and leafs of type i32.
 */
#[derive(Debug, PartialEq)]
pub enum Expr {
    Num(i32),
    BinOp(Box<Expr>, Op, Box<Expr>),
}
use Expr::Num;

/**
 *  Parse a string to get the first i32 in the string.
 *   
 *  :param input: A string.
 *
 *  :returns &str: The rest of the string that wansen't parsed.
 *  :returns i32: A i32 that was in the beging of the string.
 */
fn parse_i32(input: &str) -> IResult<&str, Expr> {
    map(preceded(multispace0, digit1), |s: &str| Num(i32::from_str(s).unwrap()))(input)
}

/**
 *  Parse a string to get the first operator in the string.
 * 
 *  :param input: A string.
 *
 *  :returns &str: The rest of the string that wansen't parsed.
 *  :returns Op: A operator that was in the beging of the string.
 */
fn parse_operand(input: &str) -> IResult<&str, Op> {
    let result: IResult<&str, &str> = preceded(multispace0, alt((
        tag("+"),
        tag("-"),
        tag("/"),
        tag("*"),
        tag("%"),
        tag("&&"),
        tag("||"),
        tag("!="),
        tag("!"),
        tag("=="),
        tag("<="),
        tag(">="),
        tag("<"),
        tag(">"),
    )))(input);
    if result.is_err() {
       return Err(Error((input, Tag)));
    }
    let (i, t) = result.unwrap();
    Ok((
    i,
    Op::from_str(t).unwrap(),
    ))
}

/**
 *  Parse a string into a Box<Expr>.
 *
 *  :return IResult<&str, Box<Expr>>: A IResult with the rest of the string that coulden't be parsed
 *  and a Box<Expr> with the parsed result.
 */
pub fn parse_expr(input: &str) -> IResult<&str, Expr> {
    alt((
        map(
            tuple((parse_i32, parse_operand, parse_expr)),
            |(l, m, r)| Expr::BinOp(Box::new(l), m, Box::new(r)),
        ),
        parse_i32,
    ))(input)
}

