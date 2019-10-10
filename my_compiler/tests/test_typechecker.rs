/***  
 *  Tests for typechecker functions.
 *
 *  Too run: 'cargo test'
 */
#[path = "../src/parser/mod.rs"]
mod parser;
use crate::parser::{
    parse_expr, 
    parse,
    Span,
    mytype::MyType,
};

#[path = "../src/typechecker/mod.rs"]
mod typechecker;
use crate::typechecker::{
    typecheck_ast,
};


/**
 *  Test tycpecheck singel int.
 */
#[test]
fn test_typecheck_int() {
    let test1 = typecheck_ast(parse_expr(Span::new(" 2")).unwrap().1);
    assert_eq!(test1.unwrap().1, MyType::Int32);
}


/**
 *  Test tycpecheck singel bool.
 */
#[test]
fn test_typecheck_bool() {
    let test1 = typecheck_ast(parse_expr(Span::new("false")).unwrap().1);
    assert_eq!(test1.unwrap().1, MyType::Boolean);

    let test2 = typecheck_ast(parse_expr(Span::new(" true")).unwrap().1);
    assert_eq!(test2.unwrap().1, MyType::Boolean);
}


/**
 *  Test tycpecheck unop.
 */
#[test]
fn test_typecheck_unop() {
    let test1 = typecheck_ast(parse_expr(Span::new(" -2")).unwrap().1);
    assert_eq!(test1.unwrap().1, MyType::Int32);

    let test2 = typecheck_ast(parse_expr(Span::new(" !true")).unwrap().1);
    assert_eq!(test2.unwrap().1, MyType::Boolean);
}


/**
 *  Test tycpecheck binop.
 */
#[test]
fn test_typecheck_binop() {
    let test1 = typecheck_ast(parse_expr(Span::new("4 + 2")).unwrap().1);
    assert_eq!(test1.unwrap().1, MyType::Int32);

    let test2 = typecheck_ast(parse_expr(Span::new("1-20")).unwrap().1);
    assert_eq!(test2.unwrap().1, MyType::Int32);

    let test3 = typecheck_ast(parse_expr(Span::new("200 / 4")).unwrap().1);
    assert_eq!(test3.unwrap().1, MyType::Int32);

    let test4 = typecheck_ast(parse_expr(Span::new("10 * 3")).unwrap().1);
    assert_eq!(test4.unwrap().1, MyType::Int32);

    let test5 = typecheck_ast(parse_expr(Span::new("13 % 5")).unwrap().1);
    assert_eq!(test5.unwrap().1, MyType::Int32);

    let test6 = typecheck_ast(parse_expr(Span::new("true && false")).unwrap().1);
    assert_eq!(test6.unwrap().1, MyType::Boolean);

    let test7 = typecheck_ast(parse_expr(Span::new("true || false")).unwrap().1);
    assert_eq!(test7.unwrap().1, MyType::Boolean);

    let test8 = typecheck_ast(parse_expr(Span::new("12 == 12")).unwrap().1); 
    assert_eq!(test8.unwrap().1, MyType::Boolean);

    let test9 = typecheck_ast(parse_expr(Span::new("true == false")).unwrap().1);  
    assert_eq!(test9.unwrap().1, MyType::Boolean);

    let test10 = typecheck_ast(parse_expr(Span::new("10 != 12")).unwrap().1); 
    assert_eq!(test10.unwrap().1, MyType::Boolean);

    let test11 = typecheck_ast(parse_expr(Span::new("true != false")).unwrap().1); 
    assert_eq!(test11.unwrap().1, MyType::Boolean);

    let test12 = typecheck_ast(parse_expr(Span::new("(10 + 2) < 12 * 2")).unwrap().1); 
    assert_eq!(test12.unwrap().1, MyType::Boolean); 

    let test13 = typecheck_ast(parse_expr(Span::new("(10 + 2) > 12 * 2")).unwrap().1); 
    assert_eq!(test13.unwrap().1, MyType::Boolean); 

    let test14 = typecheck_ast(parse_expr(Span::new("4 <= -2")).unwrap().1); 
    assert_eq!(test14.unwrap().1, MyType::Boolean); 

    let test15 = typecheck_ast(parse_expr(Span::new("4 >= 4")).unwrap().1); 
    assert_eq!(test15.unwrap().1, MyType::Boolean); 
}
