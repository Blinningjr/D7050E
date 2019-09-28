extern crate nom;

use std::str::FromStr;

pub mod error;
#[allow(unused_imports)]
use error::{Result, SyntaxError};

pub mod op;
use op::Op;

pub mod mytype;
use mytype::MyType;

pub mod expr;
use expr::Expr;


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
    multi::separated_list,
};


/**
 *  Parse a I32 expresion from string.
 */
fn parse_i32(input: &str) -> IResult<&str, Expr> {
    map(
        preceded(
            multispace0, 
            digit1
        ), 
        |s: &str| Expr::Num(i32::from_str(s).unwrap())
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
    alt((
        map(
            tuple((
                preceded(multispace0, alpha1),
                tag(":"),
                parse_mytype,
            )),
            |(i, _, t)| Expr::Assign(Box::new(Expr::Ident(i)), Box::new(Expr::Type(t)))
        ),
        map(
            preceded(multispace0, alpha1),
            |v| Expr::Ident(v)
        ),
    ))(input)
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
            tag("Str"),
            tag("None"),
        ))
    ),
    |v| MyType::from_str(v) 
    )(input)
}


/**
 *  Parse a let expresion from string.
 */
fn parse_let(input: &str) -> IResult<&str, Expr>{
    map(
        tuple((
            preceded(multispace0, tag("let")), 
            parse_ident, 
            preceded(multispace0, tag("=")), 
            preceded(multispace0, parse_expr), 
            preceded(multispace0, tag(";")),
        )),
        |(_, i, _, r, _)| Expr::Assign(Box::new(i), Box::new(r))
    )(input)
}


/**
 *  Parse singel expresions from string.
 */
fn parse_singel_expr(input: &str) -> IResult<&str, Expr> {
    alt((

        map(
            tuple((
                preceded(multispace0, tag("(")),
                parse_expr,
                preceded(multispace0, tag(")")),
            )),
            |(_, e, _)| e
        ),
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
        parse_func,
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
        parse_func_call,
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
 *  Parse a Func expresion from string.
 */
fn parse_func(input: &str) -> IResult<&str, Expr> {
    map(
        tuple((
            preceded(multispace0, tag("fn")),
            parse_ident, 
            parse_param,
            preceded(multispace0, tag("->")),
            parse_mytype,
            parse_body,
        )),
        |(_, i, p, _, t, b)| Expr::Func(Box::new(i), Box::new(p), t, Box::new(b))
    )(input)
}


/**
 *  Parse a Parameter expresion from string.
 */
fn parse_param(input: &str) -> IResult<&str, Expr> {
    alt ((
        map(
            tuple((
                preceded(multispace0, tag("(")), 
                separated_list(tag(","), parse_ident),
                preceded(multispace0, tag(")")), 
            )),
            |(_, v, _)| Expr::Param(v)
        ),
        map(
            tuple((
                preceded(multispace0, tag("(")), 
                separated_list(tag(","), parse_expr),
                preceded(multispace0, tag(")")), 
            )),
            |(_, v, _)| Expr::Param(v)
        ),
    ))(input)
}


/**
 *  Parse a Functions into expresions from string.
 */
pub fn parse_funcs(input: &str) -> IResult<&str, Expr> {
    map(
        preceded(multispace0,  
            fold_many0(
                parse_func,
                Vec::new(),
                |mut acc: Vec<_>, item| {
                    acc.push(item);
                    acc
                }
            )),
        |v| Expr::Funcs(v)
    )(input)
}


/**
 *  Parse a Function call into expresion from string.
 */
pub fn parse_func_call(input: &str) -> IResult<&str, Expr> {
    alt ((
        map(
            preceded(multispace0,
                tuple((
                    parse_ident,
                    parse_param,
                    tag(";"),
                )),
            ),
            |(i, p, _)| Expr::FuncCall(Box::new(i), Box::new(p))
        ),
        map(
            preceded(multispace0,
                tuple((
                    parse_ident,
                    parse_param,
                )),
            ),
            |(i, p)| Expr::FuncCall(Box::new(i), Box::new(p))
        ),
    ))(input)
}
