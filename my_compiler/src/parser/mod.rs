#![allow(dead_code)]

extern crate nom;

pub mod op;
use op::Op;

pub mod mytype;
use mytype::MyType;

pub mod varprefix;
use varprefix::Prefix;

pub mod expr;
use expr::Expr;

/**
 *  nom imports.
 */
use nom::{
    branch::alt,
    combinator::{map, opt},
    character::complete::{ digit1, alpha1, multispace0, multispace1},
    sequence::{preceded, tuple},
    bytes::complete::tag,
    multi::fold_many0,
    multi::separated_list,
    error,
    Err,
};


use nom_locate::LocatedSpan;
pub type Span<'a> = LocatedSpan<&'a str>;

#[derive(Debug, PartialEq, Clone)]
pub struct Error<'a>(Span<'a>, Option<Span<'a>>, ErrorKind);
pub type IResult<'a, I, O, E = Error<'a>> = Result<(I, O), Err<E>>;

impl<'a> error::ParseError<Span<'a>> for Error<'a> {
    fn from_error_kind(input: Span<'a>, kind: error::ErrorKind) -> Self {
        Error(input, None, ErrorKind::Nom(kind))
    }

    fn append(_: Span<'a>, _: error::ErrorKind, other: Self) -> Self {
        other
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ErrorKind {
    ParseIntError(std::num::ParseIntError),
    Nom(error::ErrorKind),
}

pub type SpanOp<'a> = (Span<'a>, Op);

pub type SpanExpr<'a> = (Span<'a>, Expr<'a>);

pub type SpanMyType<'a> = (Span<'a>, MyType);
pub type SpanPrefix<'a> = (Span<'a>, Prefix);


/**
 *  Parse a string into a Box<Expr>.
 */
pub fn parse_expr(input: Span) -> IResult<Span, SpanExpr> {
    alt((
        parse_func,
        parse_while,
        parse_if,
        parse_body,
        parse_let,
        parse_return,
        parse_assign,
        parse_binop,
        parse_unop,
        parse_func_call,
        parse_var_with_type,
        parse_prefixed,
        parse_parentheses,
        parse_mytype,
        parse_i32,
        parse_bool,
        parse_var,
    ))(input)
}


/**
 *  Parse a I32 expresion from string.
 *  Note: Taken from teachers example
 */
fn parse_i32(input: Span) -> IResult<Span, SpanExpr> {
    let (i, digits) = preceded(multispace0, digit1)(input)?;
    match digits.fragment.parse() {
        Ok(int) => Ok((i, (digits, Expr::Num(int)))),
        Err(e) => Err(Err::Failure(Error(i, Some(digits), ErrorKind::ParseIntError(e)))),
    }
}


/**
 *  Parse a Bool expresion from string.
 */
fn parse_bool(input: Span) -> IResult<Span, SpanExpr> {
    preceded(
        multispace0, 
        alt((
            map(tag("false"), |s| (s, Expr::Bool(false))),
            map(tag("true"), |s| (s, Expr::Bool(true))),
        ))
    )(input)
}


/**
 *  Parse the unary operator from string.
 */
fn parse_unoperand(input: Span) -> IResult<Span, SpanOp> {
    preceded(
        multispace0, 
        alt((
            map(tag("-"), |s| (s, Op::Sub)),
            map(tag("!"), |s| (s, Op::Not)),
        ))
    )(input)
}


/**
 *  Parse a Binary operand from string.
 */
fn parse_binoperand(input: Span) -> IResult<Span, SpanOp> {
    preceded(
        multispace0, 
        alt((
            map(tag("+"), |s| (s, Op::Add)),
            map(tag("-"), |s| (s, Op::Sub)),
            map(tag("/"), |s| (s, Op::Div)),
            map(tag("*"), |s| (s, Op::Multi)),
            map(tag("%"), |s| (s, Op::Mod)),
            map(tag("&&"), |s| (s, Op::And)),
            map(tag("||"), |s| (s, Op::Or)),
            map(tag("!="), |s| (s, Op::NotEq)),
            map(tag("=="), |s| (s, Op::Equal)),
            map(tag("<="), |s| (s, Op::LessEqThen)),
            map(tag(">="), |s| (s, Op::LargEqThen)),
            map(tag("<"), |s| (s, Op::LessThen)),
            map(tag(">"), |s| (s, Op::LargThen)),
        ))
    )(input)
}

/**
 *  Parse a Ident expresion from string.
 */
fn parse_var(input: Span) -> IResult<Span, SpanExpr> {
    map (
        preceded(multispace0, alpha1),
        |s: Span| (s, Expr::Var(s.fragment))
    )(input)
}


/**
 *  Parse a var prefix expresion from string.
 */
fn parse_var_prefix(input: Span) -> IResult<Span, SpanPrefix> {
    alt((
        map(
            preceded(multispace0, tag("&mut")),
            |_| (input, Prefix::BorrowMut)
        ),
        map(
            preceded(multispace0, tag("mut")),
            |_| (input, Prefix::Mut)
        ),
        map(
            preceded(multispace0, tag("&")),
            |_| (input, Prefix::Borrow)
        ),
        parse_deref,
        map(
            tag(""),
            |_| (input, Prefix::None)
        ),
    ))(input)
}


fn parse_deref(input: Span) -> IResult<Span, SpanPrefix> {
    let v = map(
        preceded(multispace0, tag("*")),
        |s| (s, Prefix::DeRef(1))
    )(input);
    match v {
        Ok((s,_)) => {
            let r = parse_deref(s);
            match r {
                Ok((s2, p2)) => {
                    return Ok((s2, 
                        match p2.1 {
                            Prefix::DeRef(val) => (p2.0, Prefix::DeRef(val + 1)),
                            _ => panic!("parse_deref"),
                        }
                    ));
                },
                Err(_) => return v,
            }
        },
        Err(_) => return v,
    }
}


/**
 *  Parse a MyType expresion from string.
 */
fn parse_mytype(input: Span) -> IResult<Span, SpanExpr> {
    preceded(
        multispace0, 
        alt((
            map(tag("i32"), |s| (s, Expr::Type((s, MyType::Int32)))),
            map(tag("bool"), |s| (s, Expr::Type((s, MyType::Boolean)))),
        ))
    )(input)
}


/**
 *  Parse a let expresion from string.
 */
fn parse_let(input: Span) -> IResult<Span, SpanExpr> {
    map(
        tuple((
            preceded(multispace0, tag("let")), 
            alt ((
                map(
                    parse_prefixed,
                    |p| p
                ),
                map(
                    parse_var_with_type,
                    |p| p
                ),
            )),
            preceded(multispace0, tag("=")), 
            parse_expr, 
            preceded(multispace0, tag(";")), // onödig?
        )),
        |(_, i, _, r, _)| (input, Expr::Let(Box::new(i), Box::new(r)))
    )(input)
}


/**
 *  Parse a assign expresion from string.
 */
fn parse_assign(input: Span) -> IResult<Span, SpanExpr> {
    map(
        tuple((
            preceded(multispace0, alt((parse_var, parse_prefixed,))), 
            preceded(multispace0, tag("=")), 
            parse_expr, 
            preceded(multispace0, tag(";")),
        )),
        |(i, _, r, _)| (input, Expr::Assign(Box::new(i), Box::new(r)))
    )(input)
}


/**
 *  Parse parentheses form string.
 */
fn parse_parentheses(input: Span) -> IResult<Span, SpanExpr> {
    map(
        tuple((
            preceded(multispace0, tag("(")),
            parse_expr,
            preceded(multispace0, tag(")")),
        )),
        |(_, e, _)| e
    )(input)
}


/**
 *  Parse a string into a Binop.
 */
pub fn parse_binop(input: Span) -> IResult<Span, SpanExpr> {
    fn help_parse(i: Span) -> IResult<Span, SpanExpr>{
        alt ((
            parse_prefixed,
            parse_func_call,
            parse_parentheses,
            parse_i32,
            parse_bool,
            parse_var,
        ))(i)
    }
    map(
        tuple((
            help_parse, 
            parse_binoperand, 
            parse_expr,
        )),
        |(l, op, r)| (input, Expr::BinOp(Box::new(l), op, Box::new(r))),
    )(input)
}


/**
 *  Parse a string into a Binop.
 */
pub fn parse_unop(input: Span) -> IResult<Span, SpanExpr> {
    map(
        tuple((
            parse_unoperand, 
            parse_expr,
        )),
        |(l, r)| (input, Expr::UnOp(l, Box::new(r))),
    )(input)
}


/**
 *  Parse a Body expresion from string.
 */
fn parse_body(input: Span) -> IResult<Span, SpanExpr> {
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
        |(_, v, _)| (input, Expr::Body(v))
    )(input)
}


/**
 *  Parse a If expresion from string.
 */
fn parse_if(input: Span) -> IResult<Span, SpanExpr> {
    alt((
        map(
            tuple((
                preceded(multispace0, tag("if")), 
                parse_expr,
                parse_body,
                preceded(multispace0, tag("else")),
                parse_body,
            )),
            |(_, i,lb, _, rb)| (input, Expr::If(Box::new(i), Box::new(lb), Box::new(rb)))
        ),
        map(
            tuple((
                preceded(multispace0, tag("if")), 
                parse_expr,
                parse_body,
            )),
            |(_, i, b)| (input, Expr::If(Box::new(i), Box::new(b), Box::new((input ,Expr::Body(Vec::new())))))
        ),
    ))(input)
}


/**
 *  Parse a While expresion from string.
 */
fn parse_while(input: Span) -> IResult<Span, SpanExpr> {
    map(
        tuple((
            preceded(multispace0, tag("while")), 
            parse_expr,
            parse_body,
        )),
        |(_, i, b)| (input, Expr::While(Box::new(i), Box::new(b)))
    )(input)
}


/**
 *  Parse a Func expresion from string.
 */
fn parse_func(input: Span) -> IResult<Span, SpanExpr> {
    alt ((
        map(
            tuple((
                preceded(multispace0, tag("fn")),
                preceded(multispace0, parse_var),
                tag("("),
                separated_list(tag(","), alt((parse_prefixed, parse_var_with_type))),
                tag(")"),
                preceded(multispace0, tag("->")),
                alt ((
                    parse_prefixed,
                    parse_mytype,
                    map(preceded(multispace0,tag("()")), |s| (s, Expr::Type((s, MyType::NoType)))),
                )),
                parse_body,
            )),
            |(_, i, _, p, _, _, t, b)| (input, Expr::Func(Box::new(i), p, Box::new(t), Box::new(b)))
        ),
        map(
            tuple((
                preceded(multispace0, tag("fn")),
                preceded(multispace0, parse_var),
                tag("("),
                separated_list(tag(","), alt((parse_prefixed, parse_var_with_type))),
                tag(")"),
                parse_body,
            )),
            |(_, i, _, p, _, b)| (input, Expr::Func(Box::new(i), p, Box::new((input, Expr::Type((input, MyType::NoType)))), Box::new(b)))
        ),
    ))(input)
}


/**
 *  Parse a Functions into expresions from string.
 */
pub fn parse_funcs(input: Span) -> IResult<Span, SpanExpr> {
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
        |v| (input, Expr::Funcs(v))
    )(input)
}


/**
 *  Parse a Function call into expresion from string.
 */
fn parse_func_call(input: Span) -> IResult<Span, SpanExpr> {
    alt ((
        map(
            preceded(multispace0,
                tuple((
                    parse_var,
                    tag("("),
                    separated_list(tag(","), parse_expr),
                    tag(")"),
                    tag(";"), // TODO onödig?
                )),
            ),
            |(i, _, p, _, _)| (input, Expr::FuncCall(Box::new(i), p))
        ),
        map(
            preceded(multispace0,
                tuple((
                    parse_var,
                    tag("("),
                    separated_list(tag(","), parse_expr),
                    tag(")"),
                )),
            ),
            |(i, _, p, _)| (input, Expr::FuncCall(Box::new(i), p))
        ),
    ))(input)
}


/**
 *  Parse return.
 */
fn parse_return(input: Span) -> IResult<Span, SpanExpr> {
    map(
        preceded(multispace0,
            tuple((
                preceded(tag("return"), multispace1),
                alt ((parse_parentheses, parse_expr)),
                opt(tag(";")),
            )),
        ),
        |(_, e, _)| (input, Expr::Return(Box::new(e)))
    )(input)
}


/**
 *  Parse a Function call into expresion from string.
 */
pub fn parse<'a>(input: &'a str) -> IResult<Span, SpanExpr> {
    let i_span = Span::new(input);
    // parse_expr(i_span)
    parse_funcs(i_span)
}


/**
 *  Parse ident with type.
 */
fn parse_var_with_type(input: Span) -> IResult<Span, SpanExpr> {
    map(
        preceded(multispace0,
            tuple((
                parse_var,
                tag(":"),
                alt ((
                    map(
                        parse_prefixed,
                        |p| p
                    ),
                    map(
                        parse_mytype,
                        |t| t
                    ),
                )),
            )),
        ),
        |(e, _, t)| (input, Expr::VarWithType(Box::new(e), Box::new(t)))
    )(input)
}


/**
 *  Parse expr with prefix.
 */
fn parse_prefixed(input: Span) -> IResult<Span, SpanExpr> {
    map(
        preceded(multispace0,
            tuple((
                alt((
                    map(
                        preceded(multispace0, tag("&mut")),
                        |_| (input, Prefix::BorrowMut)
                    ),
                    map(
                        preceded(multispace0, tag("mut")),
                        |_| (input, Prefix::Mut)
                    ),
                    map(
                        preceded(multispace0, tag("&")),
                        |_| (input, Prefix::Borrow)
                    ),
                    parse_deref,
                )),
                alt((
                    parse_var_with_type,
                    parse_func_call,
                    parse_unop,
                    parse_mytype,
                    parse_parentheses,
                    parse_i32,
                    parse_bool,
                    parse_var,
                )),
            )),
        ),
        |(p, val)| (input, Expr::Prefixed(p, Box::new(val)))
    )(input)
}
