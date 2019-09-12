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
    character::complete::{ digit1, alpha1, multispace0, multispace1},
    sequence::{preceded, tuple},
    bytes::complete::tag,
    combinator::map_res,
    multi::fold_many0,
};


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


/** 
 *  Defining all of my types.
 */
#[derive(Debug, PartialEq, Clone)]
pub enum MyType {
    Int32,
    Bool,
    Str,
}


/**
 *  Converts string to MyType.
 */
impl FromStr for MyType {
    type Err = SyntaxError;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "i32" => Ok(MyType::Int32),
            "bool" => Ok(MyType::Bool),
            "Str" => Ok(MyType::Str),
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
            MyType::Bool =>  write!(f, "{}", "Bool"),
            MyType::Str =>  write!(f, "{}", "Str"),
        }

    }
}


/** 
 *  Defining all types of expr.
 */
#[derive(Debug, PartialEq, Clone)]
pub enum Expr<'a> {
    Empty,
    Num(i32),
    Bool(bool),
    Ident(&'a str),
    Type(MyType),
    Assign(Box<Expr<'a>>, Box<Expr<'a>>),
    UnOp(Op, Box<Expr<'a>>),
    BinOp(Box<Expr<'a>>, Op, Box<Expr<'a>>),
    Body(Vec<Expr<'a>>),
    If(Box<Expr<'a>>, Box<Expr<'a>>,  Box<Expr<'a>>),
    While(Box<Expr<'a>>, Box<Expr<'a>>),
}
use Expr::Num;


/**
 * to_string() for expr.
 */
impl fmt::Display for Expr <'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Num(i) =>  write!(f, "{}", i),
            Expr::BinOp(l, op, r) => write!(f, "({} {:?} {})", l.to_string(), op,  r.to_string()),
            Expr::UnOp(op, r) => write!(f, "({:?} {})", op,  r.to_string()),
            Expr::Bool(b) =>  write!(f, "{}", b),
            Expr::Ident(s) =>  write!(f, "{}", s),
            Expr::Type(s) =>  write!(f, ":{:?} =", s.to_string()),
            Expr::Assign(l, r) => write!(f, "({:?} {:?})", l.to_string(),  r.to_string()),
            Expr::Body(s) =>  write!(f, "{}", "Not implemented"),
            Expr::If(l, m, r) =>  write!(f, "if {} ({}) else ({})", l.to_string(), m.to_string(), r.to_string()),
            Expr::Empty =>  write!(f, "{}", "Empty"),
            Expr::While(l, r) =>  write!(f, "while {} ({})", l.to_string(), r.to_string()),
        }
    }
}


/**
 *  Parse a I32 expresion from string.
 */
fn parse_i32(input: &str) -> IResult<&str, Expr> {
    map(
        preceded(
            multispace0, 
            digit1
        ), 
        |s: &str| Num(i32::from_str(s).unwrap())
    )(input)
}


/**
 *  Parse a Bool expresion from string.
 */
fn parse_bool(input: &str) -> IResult<&str, Expr> {
    map(
        preceded(
            multispace0, 
            alt((
                tag("false"),
                tag("true"),
            ))
        ),
        |v| Expr::Bool(bool::from_str(v).unwrap())
    )(input)
}


/**
 *  Parse a Binary operand from string.
 */
fn parse_binoperand(input: &str) -> IResult<&str, Op> {
    map_res(
        preceded(
            multispace0, 
            alt((
                tag("+"),
                tag("-"),
                tag("/"),
                tag("*"),
                tag("%"),
                tag("&&"),
                tag("||"),
                tag("!="),
                tag("=="),
                tag("<="),
                tag(">="),
                tag("<"),
                tag(">"),
                tag("="),
            ))
        ),
        |op| Op::from_str(op)
    )(input)
}


/**
 *  Parse the unary operator from string.
 */
fn parse_unoperand(input: &str) -> IResult<&str, Op> {
    map_res(
        preceded(
            multispace0, 
            alt((
                tag("!"),
                tag("-"),
            ))
        ),
        |op| Op::from_str(op)
    )(input)
}


/**
 *  Parse a Ident expresion from string.
 */
fn parse_ident(input: &str) -> IResult<&str, Expr> {
    map(
        preceded(multispace1, alpha1),
        |v| Expr::Ident(v)
    )(input)
}


/**
 *  Parse a MyType expresion from string.
 */
fn parse_mytype(input: &str) -> IResult<&str, MyType> {
    map_res(preceded(
        multispace1, 
        alt((
            tag("i32"),
            tag("bool"),
            tag("str"),
        ))
    ),
    |v| MyType::from_str(v) 
    )(input)
}


/**
 *  Parse a let expresion from string.
 */
fn parse_let(input: &str) -> IResult<&str, Expr>{
    alt((
        map(
            tuple((
                preceded(multispace0, tag("let")), 
                parse_ident, 
                tag(":"),
                parse_mytype,
                preceded(multispace0, tag("=")), 
                preceded(multispace0, parse_expr), 
                preceded(multispace0, tag(";")))),
                |(_, i, _, t, _, r, _)| Expr::Assign(Box::new(Expr::Assign(Box::new(i), Box::new(Expr::Type(t)))), Box::new(r))
        ),
        map(
            tuple((
                preceded(multispace0, tag("let")), 
                parse_ident, 
                preceded(multispace0, tag("=")), 
                preceded(multispace0, parse_expr), 
                preceded(multispace0, tag(";")))),
                |(_, i, _, r, _)| Expr::Assign(Box::new(i), Box::new(r))
        ),
    ))(input)
}


/**
 *  Parse singel expresions from string.
 */
fn parse_singel_expr(input: &str) -> IResult<&str, Expr> {
    alt((
        parse_i32,
        parse_bool,
        parse_ident,
    ))(input)
}


/**
 *  Parse a string into a Box<Expr>.
 */
pub fn parse_expr(input: &str) -> IResult<&str, Expr> {
    alt((
        parse_while,
        parse_if,
        parse_let,
        map(
            tuple((
                parse_singel_expr, 
                parse_binoperand, 
                parse_expr
            )),
            |(l, op, r)| Expr::BinOp(Box::new(l), op, Box::new(r)),
        ),
        map(
            tuple((
                preceded(multispace0, parse_unoperand), 
                parse_expr
            )),
            |(l, r)| Expr::UnOp(l, Box::new(r)),
        ),
        parse_singel_expr,
    ))(input)
}


/**
 *  Parse a Body expresion from string.
 */
fn parse_body(input: &str) -> IResult<&str, Expr> {
    map(
        tuple((
            preceded(multispace0, tag("{")), 
            fold_many0(
                parse_expr,
                Vec::new(),
                |mut acc: Vec<_>, item| {
                    acc.push(item);
                    acc
                }
            ),
            preceded(multispace0, tag("}")),
        )),
        |(_, v, _)| Expr::Body(v)
    )(input)
}


/**
 *  Parse a If expresion from string.
 */
fn parse_if(input: &str) -> IResult<&str, Expr> {
    alt((
        map(
            tuple((
                preceded(multispace0, tag("if")), 
                parse_expr,
                parse_body,
                preceded(multispace0, tag("else")),
                parse_body,
            )),
            |(_, i,lb, _, rb)| Expr::If(Box::new(i), Box::new(lb), Box::new(rb))
        ),
        map(
            tuple((
                preceded(multispace0, tag("if")), 
                parse_expr,
                parse_body,
            )),
            |(_, i, b)| Expr::If(Box::new(i), Box::new(b), Box::new(Expr::Empty))
        ),
    ))(input)
}


/**
 *  Parse a While expresion from string.
 */
fn parse_while(input: &str) -> IResult<&str, Expr> {
    map(
        tuple((
            preceded(multispace0, tag("while")), 
            parse_expr,
            parse_body,
        )),
        |(_, i, b)| Expr::While(Box::new(i), Box::new(b))
    )(input)
}


/**
 *  Calculates the value of an math expression.
 */
pub fn math_expr_eval(e: Expr) -> Result<i32> {
    match e {
        Expr::Num(i) => Ok(i),
        Expr::BinOp(l, op, r) => {
            let left_value = math_expr_eval(*l).unwrap();
            let right_value = math_expr_eval(*r).unwrap();
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
            let right_value = math_expr_eval(*r).unwrap();
            match op {
                Op::Sub => Ok(-right_value),
                _ => Err(SyntaxError),
            }
        }
        _ => Err(SyntaxError),
    }
}
