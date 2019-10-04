extern crate nom;

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
    combinator::map,
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

#[derive(Debug)]
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

#[derive(Debug)]
pub enum ErrorKind {
    ParseIntError(std::num::ParseIntError),
    Nom(error::ErrorKind),
}

pub type SpanOp<'a> = (Span<'a>, Op);

pub type SpanExpr<'a> = (Span<'a>, Expr<'a>);

pub type SpanMyType<'a> = (Span<'a>, MyType);


/**
 *  Parse a I32 expresion from string.
 *  Note: Taken from teachers example
 */
#[allow(dead_code)]
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
#[allow(dead_code)]
fn parse_bool(input: Span) -> IResult<Span, SpanExpr> {
    preceded(
        multispace0, 
        alt((
            map(tag("false"), |s| (s, Expr::Bool(false))),
            map(tag("true"), |s| (s, Expr::Bool(true))),
        ))
    )(input)
    // match res {
    //     Ok(_) => res,
    //     Err(_) => Err(Err::Failure(Error(input, None, ErrorKind::ParseBooleanError))),
    // }
}


/**
 *  Parse a Binary operand from string.
 */
#[allow(dead_code)]
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
 *  Parse the unary operator from string.
 */
#[allow(dead_code)]
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
 *  Parse a Ident expresion from string.
 */
#[allow(dead_code)]
fn parse_ident(input: Span) -> IResult<Span, SpanExpr> {
    alt((
        map(
            tuple((
                preceded(multispace0, alpha1),
                tag(":"),
                parse_mytype,
            )),
            |(i, _, t)| (input, (Expr::Assign( Box::new((i, Expr::Ident(i.fragment))), Box::new((t.0, Expr::Type(t))))))
        ),
        map(
            preceded(multispace0, alpha1),
            |s: Span| (s, Expr::Ident(s.fragment))
        ),
    ))(input)
}


/**
 *  Parse a MyType expresion from string.
 */
#[allow(dead_code)]
fn parse_mytype(input: Span) -> IResult<Span, SpanMyType> {
    preceded(
        multispace1, 
        alt((
            map(tag("i32"), |s| (s, MyType::Int32)),
            map(tag("bool"), |s| (s, MyType::Boolean)),
            map(tag("Str"), |s| (s, MyType::Str)),
            map(tag("None"), |s| (s, MyType::None)),
        ))
    )(input)
}


/**
 *  Parse a let expresion from string.
 */
#[allow(dead_code)]
fn parse_let(input: Span) -> IResult<Span, SpanExpr> {
    alt ((
        map(
            tuple((
                preceded(multispace0, tag("let")), 
                parse_ident, 
                preceded(multispace0, tag("=")), 
                preceded(multispace0, parse_expr), 
                preceded(multispace0, tag(";")),
            )),
            |(_, i, _, r, _)| (input, Expr::Assign(Box::new(i), Box::new(r)))
        ),
        map(
            tuple((
                parse_ident, 
                preceded(multispace0, tag("=")), 
                preceded(multispace0, parse_expr), 
                preceded(multispace0, tag(";")),
            )),
            |(i, _, r, _)| (input, Expr::UpdateVar(Box::new(i), Box::new(r)))
        ),
    ))(input)
}


/**
 *  Parse singel expresions from string.
 */
#[allow(dead_code)]
fn parse_singel_expr(input: Span) -> IResult<Span, SpanExpr> {
    alt((
        parse_parentheses,
        parse_i32,
        parse_bool,
        parse_ident,
    ))(input)
}


/**
 *  Parse parentheses form string.
 */
#[allow(dead_code)]
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
 *  Parse a string into a Box<Expr>.
 */
#[allow(dead_code)]
pub fn parse_expr(input: Span) -> IResult<Span, SpanExpr> {
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
            |(l, op, r)| (input, Expr::BinOp(Box::new(l), op, Box::new(r))),
        ),
        map(
            tuple((
                preceded(multispace0, parse_unoperand), 
                parse_expr
            )),
            |(l, r)| (input, Expr::UnOp(l, Box::new(r))),
        ),
        parse_func_call,
        parse_return,
        parse_singel_expr,
    ))(input)
}


/**
 *  Parse a Body expresion from string.
 */
#[allow(dead_code)]
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
#[allow(dead_code)]
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
            |(_, i, b)| (input, Expr::If(Box::new(i), Box::new(b), Box::new((input ,Expr::Empty))))
        ),
    ))(input)
}


/**
 *  Parse a While expresion from string.
 */
#[allow(dead_code)]
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
#[allow(dead_code)]
fn parse_func(input: Span) -> IResult<Span, SpanExpr> {
    map(
        tuple((
            preceded(multispace0, tag("fn")),
            parse_ident, 
            parse_param,
            preceded(multispace0, tag("->")),
            parse_mytype,
            parse_body,
        )),
        |(_, i, p, _, t, b)| (input, Expr::Func(Box::new(i), Box::new(p), t, Box::new(b)))
    )(input)
}


/**
 *  Parse a Parameter expresion from string.
 */
#[allow(dead_code)]
fn parse_param(input: Span) -> IResult<Span, SpanExpr> {
    alt ((
        map(
            tuple((
                preceded(multispace0, tag("(")), 
                separated_list(tag(","), parse_ident),
                preceded(multispace0, tag(")")), 
            )),
            |(_, v, _)| (input, Expr::Param(v))
        ),
        map(
            tuple((
                preceded(multispace0, tag("(")), 
                separated_list(tag(","), parse_expr),
                preceded(multispace0, tag(")")), 
            )),
            |(_, v, _)| (input, Expr::Param(v))
        ),
    ))(input)
}


/**
 *  Parse a Functions into expresions from string.
 */
#[allow(dead_code)]
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
#[allow(dead_code)]
fn parse_func_call(input: Span) -> IResult<Span, SpanExpr> {
    alt ((
        map(
            preceded(multispace0,
                tuple((
                    parse_ident,
                    parse_param,
                    tag(";"),
                )),
            ),
            |(i, p, _)| (input, Expr::FuncCall(Box::new(i), Box::new(p)))
        ),
        map(
            preceded(multispace0,
                tuple((
                    parse_ident,
                    parse_param,
                )),
            ),
            |(i, p)| (input, Expr::FuncCall(Box::new(i), Box::new(p)))
        ),
    ))(input)
}


/**
 *  Parse return
 */
#[allow(dead_code)]
fn parse_return(input: Span) -> IResult<Span, SpanExpr> {
    map(
        preceded(multispace0,
            tuple((
                tag("return"),
                parse_expr,
                tag(";"),
            )),
        ),
        |(_, e, _)| (input, Expr::Return(Box::new(e)))
    )(input)
}


/**
 *  Parse a Function call into expresion from string.
 */
#[allow(dead_code)]
pub fn parse<'a>(input: &'a str) -> IResult<Span, SpanExpr> {
    let i_span = Span::new(input);
    parse_funcs(i_span)
}
