/***  
 *  Tests for parser functions.
 *
 *  Too run: 'cargo test'
 */
#[path = "../src/parser/mod.rs"]
mod parser;
#[allow(unused_imports)]
use crate::parser::{parse_expr, parse_funcs};


/**
 *  Import enum Expr.
 */
#[allow(unused_imports)]
use crate::parser::expr::Expr::{
    Num,
    Bool,
    BinOp,
    UnOp,
    Ident,
    Type,
    Assign,
    Empty,
    If,
    Body,
    While,
    Func,
    Param,
    Funcs,
};
use crate::parser::expr::Expr;


/**
 *  Import enum Op.
 */
#[allow(unused_imports)]
use crate::parser::op::Op::{
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
};


/**
 *  Import enum MyType.
 */
#[allow(unused_imports)]
use crate::parser::mytype::MyType::{
    Int32,
    Boolean,
    Str,
    None,
};

use crate::parser::error::{SyntaxError, Result};


/**
 *  Calculates the value of an math expression.
 *  Needed for tests.
 */
pub fn math_expr_eval(e: Expr) -> Result<i32> {
    match e {
        Num(i) => Ok(i),
        BinOp(l, op, r) => {
            let left_value = math_expr_eval(*l).unwrap();
            let right_value = math_expr_eval(*r).unwrap();
            match op {
                Add => Ok(left_value + right_value),
                Sub => Ok(left_value - right_value),
                Div => Ok(left_value / right_value),
                Multi => Ok(left_value * right_value),
                Mod => Ok(left_value % right_value),
                _ => Err(SyntaxError),
            }
        }
        UnOp(op, r) => {
            let right_value = math_expr_eval(*r).unwrap();
            match op {
                Sub => Ok(-right_value),
                _ => Err(SyntaxError),
            }
        }
        _ => Err(SyntaxError),
    }
}


/**
 *  Test parsing singel int.
 */
#[test]
fn test_parse_int() {
    assert_eq!(parse_expr("2"), Ok(("", Num(2))));
    assert!(parse_expr("1a").is_ok());
}


/**
 *  Test parsing addition.
 */
#[test]
fn test_parse_add() {
    let mut expec = Ok(("", BinOp(Box::new(Num(4)), Add, Box::new(Num(2)))));
    let mut expr = parse_expr("4 + 2");
    assert_eq!(expr, expec);
    assert_eq!(math_expr_eval(expr.unwrap().1).unwrap(), 6);

    expec = Ok(("", BinOp(Box::new(Ident("a")), Add, Box::new(Ident("b")))));
    expr = parse_expr(" a + b");
    assert_eq!(expr, expec);
}


/**
 *  Test parsing subtraction.
 */
#[test]
fn test_parse_sub() {
    let test_val = "4 - 2";
    let expec = Ok(("", BinOp(Box::new(Num(4)), Sub, Box::new(Num(2)))));
    let expr = parse_expr(test_val);
    
    assert_eq!(expr, expec);
    assert_eq!(math_expr_eval(expr.unwrap().1).unwrap(), 2);
    assert_eq!(parse_expr(" -2"), Ok(("", UnOp(Sub, Box::new(Num(2))))));
    assert_eq!(math_expr_eval(parse_expr(" -2").unwrap().1).unwrap(), -2);
}


/**
 *  Test parsing divition.
 */
#[test]
fn test_parse_div() {
    let test_val = "4 / 2";
    let expec = Ok(("", BinOp(Box::new(Num(4)), Div, Box::new(Num(2)))));
    let expr = parse_expr(test_val);

    assert_eq!(expr, expec);
    assert_eq!(math_expr_eval(expr.unwrap().1).unwrap(), 2);
}


/**
 *  Test parsing multiplication.
 */
#[test]
fn test_parse_multi() {
    let test_val = "4 * 2";
    let expec = Ok(("", BinOp(Box::new(Num(4)), Multi, Box::new(Num(2)))));
    let expr = parse_expr(test_val);

    assert_eq!(expr, expec);
    assert_eq!(math_expr_eval(expr.unwrap().1).unwrap(), 8);
}


/**
 *  Test parsing modulus.
 */
#[test]
fn test_parse_mod() {
    let test_val = "4 % 2";
    let expec = Ok(("", BinOp(Box::new(Num(4)), Mod, Box::new(Num(2)))));
    let expr = parse_expr(test_val);

    assert_eq!(expr, expec);
    assert_eq!(math_expr_eval(expr.unwrap().1).unwrap(), 0);
}


/**
 *  Test parsing and.
 */
#[test]
fn test_parse_and() {
    let expec = Ok(("", BinOp(Box::new(Num(4)), And, Box::new(Num(2)))));
    assert_eq!(parse_expr("4 && 2"), expec);
}


/**
 *  Test parsing or.
 */
#[test]
fn test_parse_or() {
    let expec = Ok(("", BinOp(Box::new(Num(4)), Or, Box::new(Num(2)))));
    assert_eq!(parse_expr("4 || 2"), expec);
}


/**
 *  Test parsing not.
 */
#[test]
fn test_parse_not() {
    let expec = Ok(("", UnOp( Not, Box::new(Num(2)))));
    assert_eq!(parse_expr(" ! 2"), expec);
}


/**
 *  Test parsing equal.
 */
#[test]
fn test_parse_equal() {
    let expec = Ok(("", BinOp(Box::new(Num(4)), Equal, Box::new(Num(2)))));
    assert_eq!(parse_expr("4 == 2"), expec);
}


/**
 *  Test parsing not equal.
 */
#[test]
fn test_parse_not_eq() {
    let expec = Ok(("", BinOp(Box::new(Num(4)), NotEq, Box::new(Num(2)))));
    assert_eq!(parse_expr("4 != 2"), expec);
}


/**
 *  Test parsing lesser then.
 */
#[test]
fn test_parse_less_then() {
    let expec = Ok(("", BinOp(Box::new(Num(4)), LessThen, Box::new(Num(2)))));
    assert_eq!(parse_expr("4 < 2"), expec);
}


/**
 *  Test parsing larger then.
 */
#[test]
fn test_parse_larg_then() {
    let expec = Ok(("", BinOp(Box::new(Num(4)), LargThen, Box::new(Num(2)))));
    assert_eq!(parse_expr("4 > 2"), expec);
}


/**
 *  Test parsing lesser equal then.
 */
#[test]
fn test_parse_less_eq_then() {
    let expec = Ok(("", BinOp(Box::new(Num(4)), LessEqThen, Box::new(Num(2)))));
    assert_eq!(parse_expr("4 <= 2"), expec);
}


/**
 *  Test parsing larger equal then.
 */
#[test]
fn test_parse_larg_eq_then() {
    let expec = Ok(("", BinOp(Box::new(Num(4)), LargEqThen, Box::new(Num(2)))));
    assert_eq!(parse_expr("4 >= 2"), expec);
}


/**
 *  Test parsing singel boolean.
 */
#[test]
fn test_parse_bool() {
    assert_eq!(parse_expr(" false"), Ok(("", Bool(false))));
    assert_eq!(parse_expr("false"), Ok(("", Bool(false))));
    assert_eq!(parse_expr(" true"), Ok(("", Bool(true))));
    assert_eq!(parse_expr("true"), Ok(("", Bool(true))));
    assert_eq!(parse_expr("true == false"), Ok(("", BinOp(Box::new(Bool(true)), Equal, Box::new(Bool(false))))));
    assert_eq!(parse_expr(" true  2"), Ok(("  2", Bool(true))));
    assert!(parse_expr(" true  2").is_ok());
}


/**
 *  Test parsing let statments.
 */
#[test]
fn test_parse_let() {
    let expec = Ok(("", Assign(Box::new(Ident("apa")), Box::new(Num(20)))));
    assert_eq!(parse_expr(" let apa = 20;"), expec);

    let expec = Ok((" let apa = 20;", BinOp(Box::new(Num(1)), Add, Box::new(Num(2)))));
    assert_eq!(parse_expr("1 + 2 let apa = 20;"), expec);

    assert!(parse_expr("let apa = true;").is_ok());
    assert!(parse_expr("let apa = false;").is_ok());
    assert!(parse_expr("let apa=20 + 20- 2 * 20;").is_ok());
}


/**
 *  Test parsing if statments.
 */
#[test]
fn test_parse_if() {
    let expec = Ok(("", If(Box::new(BinOp(Box::new(Bool(false)), Equal, Box::new(Bool(true)))), 
    Box::new(Body([BinOp(Box::new(Num(1)), Add, Box::new(Num(2)))].to_vec())), 
    Box::new(Empty))));
    assert_eq!(parse_expr("if false == true {1+2}"), expec);

    let expec = Ok(("", If(Box::new(BinOp(Box::new(Bool(false)), Equal, Box::new(Bool(true)))), 
        Box::new(Body([BinOp(Box::new(Num(1)), Add, Box::new(Num(2)))].to_vec())), 
        Box::new(Body([BinOp(Box::new(Num(1)), Add, Box::new(Num(2)))].to_vec())))));
    assert_eq!(parse_expr("if false == true {1+2} else {1+2}"), expec);
}


/**
 *  Test parsing while statments.
 */
#[test]
fn test_parse_while() {
    let expec = Ok(("", While(Box::new(Bool(true)), 
        Box::new(Body([BinOp(Box::new(Num(1)), Add, Box::new(Num(2)))].to_vec())))));
    assert_eq!(parse_expr("while true {1+2}"), expec);
}


/**
 *  Test parsing Func statments.
 */
#[test]
fn test_parse_func() {
    let expec = Ok(("", 
        Funcs([Func(Box::new(Ident("apa")), 
        Box::new(Param([Ident("input")].to_vec())), 
        Boolean, 
        Box::new(Body([Assign(Box::new(Ident("apa")), 
        Box::new(Num(10)))].to_vec())))].to_vec())));
    assert_eq!(parse_funcs("fn apa(input) -> bool { let apa = 10;}"), expec);

    let expec = Ok(("", 
        Funcs([Func(Box::new(Ident("apa")), 
        Box::new(Param([Assign(Box::new(Ident("input")), Box::new(Type(Int32)))].to_vec())), 
        None, 
        Box::new(Body([Assign(Box::new(Ident("a")), Box::new(Num(10))), 
            Assign(Box::new(Ident("var")), Box::new(Bool(true)))].to_vec())))].to_vec())));
    assert_eq!(parse_funcs("fn apa(input: i32) -> None { let a = 10; let var = true;}"), expec);

    let expec = Ok(("", 
        Funcs([Func(Box::new(Ident("apor")), 
        Box::new(Param([Assign(Box::new(Ident("input")), Box::new(Type(Str))), Ident("test")].to_vec())), 
        Boolean, 
        Box::new(Body([Assign(Box::new(Ident("apa")), Box::new(Num(10)))].to_vec())))].to_vec())));
    assert_eq!(parse_funcs("fn apor(input: Str, test) -> bool { let apa = 10;}"), expec);
}


/**
 *  Test multiple math expretions.
 */
#[test]
fn test_math_expr() {
    assert_eq!(math_expr_eval(parse_expr(" 10 * (2+3)").unwrap().1).unwrap(), 50);
    assert_eq!(math_expr_eval(parse_expr(" 10 / (2-4)").unwrap().1).unwrap(), -5);
}
