/***  
 *  Tests for parser functions.
 *
 *  Too run: 'cargo test'
 */
#[path = "../src/parser/mod.rs"]
mod parser;
use crate::parser::{
    parse_expr, 
    parse,
    Span,
    };

/**
 *  Test parsing singel int.
 */
#[test]
fn test_parse_int() {
    let test1 = parse_expr(Span::new(" 2"));
    assert!(test1.is_ok());
    assert_eq!((test1.unwrap().0).fragment, "");

    let test2 = parse_expr(Span::new("1a"));
    assert!(test2.is_ok());
    assert_eq!((test2.unwrap().0).fragment, "a");

    let test3 = parse_expr(Span::new("931235"));
    assert!(test3.is_ok());
    assert_eq!((test3.unwrap().0).fragment, "");
}


/**
 *  Test parsing singel Bool.
 */
#[test]
fn test_parse_bool() {
    let test1 = parse_expr(Span::new(" true"));
    assert!(test1.is_ok());
    assert_eq!((test1.unwrap().0).fragment, "");

    let test2 = parse_expr(Span::new("false"));
    assert!(test2.is_ok());
    assert_eq!((test2.unwrap().0).fragment, "");
}


/**
 *  Test parsing singel ident.
 */
#[test]
fn test_parse_var() {
    let test1 = parse_expr(Span::new(" apa"));
    assert!(test1.is_ok());
    assert_eq!((test1.unwrap().0).fragment, "");

    let test3 = parse_expr(Span::new("koskos"));
    assert!(test3.is_ok());
    assert_eq!((test3.unwrap().0).fragment, "");

    let test4 = parse_expr(Span::new("koskos: i32"));
    assert!(test4.is_ok());
    assert_eq!((test4.unwrap().0).fragment, "");

    let test5 = parse_expr(Span::new("apa: bool"));
    assert!(test5.is_ok());
    assert_eq!((test5.unwrap().0).fragment, "");
}


/**
 *  Test parsing ident with prefix.
 */
#[test]
fn test_parse_var_with_prefix() {
    let test1 = parse_expr(Span::new(" &apa"));
    assert!(test1.is_ok());
    assert_eq!((test1.unwrap().0).fragment, "");

    let test3 = parse_expr(Span::new("mut koskos"));
    assert!(test3.is_ok());
    assert_eq!((test3.unwrap().0).fragment, "");

    let test4 = parse_expr(Span::new("&mut koskos"));
    assert!(test4.is_ok());
    assert_eq!((test4.unwrap().0).fragment, "");

    let test5 = parse_expr(Span::new("&apa: bool"));
    assert!(test5.is_ok());
    assert_eq!((test5.unwrap().0).fragment, "");

    let test6 = parse_expr(Span::new("mut apa: bool"));
    assert!(test6.is_ok());
    assert_eq!((test6.unwrap().0).fragment, "");
    
    let test7 = parse_expr(Span::new("&mut apa: bool"));
    assert!(test7.is_ok());
    assert_eq!((test7.unwrap().0).fragment, "");
}


/**
 *  Test parsing parentheses.
 */
#[test]
fn test_parse_parentheses() {
    let test1 = parse_expr(Span::new(" (true)"));
    assert!(test1.is_ok());
    assert_eq!((test1.unwrap().0).fragment, "");

    let test2 = parse_expr(Span::new("(false)"));
    assert!(test2.is_ok());
    assert_eq!((test2.unwrap().0).fragment, "");

    let test3 = parse_expr(Span::new(" (1 +2)"));
    assert!(test3.is_ok());
    assert_eq!((test3.unwrap().0).fragment, "");

    let test4 = parse_expr(Span::new("(10 - 2)* 2"));
    assert!(test4.is_ok());
    assert_eq!((test4.unwrap().0).fragment, "");
}


/**
 *  Test parsing function calls.
 */
#[test]
fn test_parse_func_call() {
    let test1 = parse_expr(Span::new(" apa(50)"));
    assert!(test1.is_ok());
    assert_eq!((test1.unwrap().0).fragment, "");

    let test2 = parse_expr(Span::new(" test(tjo, 20, true)"));
    assert!(test2.is_ok());
    assert_eq!((test2.unwrap().0).fragment, "");
}


/**
 *  Test parsing unary operations.
 */
#[test]
fn test_parse_unop() {
    let test1 = parse_expr(Span::new(" -20"));
    assert!(test1.is_ok());
    assert_eq!((test1.unwrap().0).fragment, "");

    let test2 = parse_expr(Span::new(" !true"));
    assert!(test2.is_ok());
    assert_eq!((test2.unwrap().0).fragment, "");
}


/**
 *  Test parsing binary operations.
 */
#[test]
fn test_parse_binop() {
    let test1 = parse_expr(Span::new("4 + 2"));
    assert!(test1.is_ok());
    assert_eq!((test1.unwrap().0).fragment, "");

    let test2 = parse_expr(Span::new("1-20"));
    assert!(test2.is_ok());
    assert_eq!((test2.unwrap().0).fragment, "");

    let test3 = parse_expr(Span::new("931235 /a"));
    assert!(test3.is_ok());
    assert_eq!((test3.unwrap().0).fragment, "");

    let test4 = parse_expr(Span::new(" a* b"));
    assert!(test4.is_ok());
    assert_eq!((test4.unwrap().0).fragment, "");

    let test5 = parse_expr(Span::new("(10 + 2) %5"));
    assert!(test5.is_ok());
    assert_eq!((test5.unwrap().0).fragment, "");

    let test6 = parse_expr(Span::new("true && false"));
    assert!(test6.is_ok());
    assert_eq!((test6.unwrap().0).fragment, "");

    let test7 = parse_expr(Span::new("true || false"));
    assert!(test7.is_ok());
    assert_eq!((test7.unwrap().0).fragment, "");

    let test8 = parse_expr(Span::new(" (10 + 2) == 12"));
    assert!(test8.is_ok());
    assert_eq!((test8.unwrap().0).fragment, "");

    let test9 = parse_expr(Span::new(" 10 != 12"));
    assert!(test9.is_ok());
    assert_eq!((test9.unwrap().0).fragment, "");

    let test10 = parse_expr(Span::new(" (10 + 2) < 12 * 2"));
    assert!(test10.is_ok());
    assert_eq!((test10.unwrap().0).fragment, "");

    let test11 = parse_expr(Span::new(" (10 + 2) > 12 * 2"));
    assert!(test11.is_ok());
    assert_eq!((test11.unwrap().0).fragment, "");

    let test12 = parse_expr(Span::new(" 4 <= -2"));
    assert!(test12.is_ok());
    assert_eq!((test12.unwrap().0).fragment, "");

    let test13 = parse_expr(Span::new(" 4 >= -2"));
    assert!(test13.is_ok());
    assert_eq!((test13.unwrap().0).fragment, "");
}


/**
 *  Test parsing let statments.
 */
#[test]
fn test_parse_let() {
    let test1 = parse_expr(Span::new(" let apa: i32 = 20;"));
    assert!(test1.is_ok());
    assert_eq!((test1.unwrap().0).fragment, "");

    let test3 = parse_expr(Span::new("let apa: bool = true;"));
    assert!(test3.is_ok());
    assert_eq!((test3.unwrap().0).fragment, "");

    let test4 = parse_expr(Span::new("let apa: bool = false;"));
    assert!(test4.is_ok());
    assert_eq!((test4.unwrap().0).fragment, "");

    let test5 = parse_expr(Span::new("let apa: i32=20 + 20- 2 * 20;"));
    assert!(test5.is_ok());
    assert_eq!((test5.unwrap().0).fragment, "");
}


/**
 *  Test parsing let statments with prefix.
 */
#[test]
fn test_parse_mut_let() {
    let test1 = parse_expr(Span::new(" let mut apa: i32 = 20;"));
    assert!(test1.is_ok());
    assert_eq!((test1.unwrap().0).fragment, "");

    let test3 = parse_expr(Span::new("let mut apa: bool = true;"));
    assert!(test3.is_ok());
    assert_eq!((test3.unwrap().0).fragment, "");
}


/**
 *  Test parsing if statments.
 */
#[test]
fn test_parse_if() {
    let test1 = parse_expr(Span::new("if false == true {1+2}"));
    assert!(test1.is_ok());
    assert_eq!((test1.unwrap().0).fragment, "");

    let test2 = parse_expr(Span::new("if false == true {1+2} else {1+2}"));
    assert!(test2.is_ok());
    assert_eq!((test2.unwrap().0).fragment, "");
}


/**
 *  Test parsing while statments.
 */
#[test]
fn test_parse_while() {
    let test1 = parse_expr(Span::new("while true {1+2}"));
    assert!(test1.is_ok());
    assert_eq!((test1.unwrap().0).fragment, "");

    let test2 = parse_expr(Span::new("while i < 10 {i}"));
    assert!(test2.is_ok());
    assert_eq!((test2.unwrap().0).fragment, "");
}


/**
 *  Test parsing Func statments.
 */
#[test]
fn test_parse_func() {
    let test1 = parse_expr(Span::new("fn apa(input: i32) -> bool { let apa = 10;}"));
    assert!(test1.is_ok());
    assert_eq!((test1.unwrap().0).fragment, "");

    let test2 = parse_expr(Span::new("fn apa(input: i32) -> i32 { let a = 10; let var = true;}"));
    assert!(test2.is_ok());
    assert_eq!((test2.unwrap().0).fragment, "");

    let test3 = parse_expr(Span::new("fn apor(input: bool, test: i32) -> bool { let apa = 10;}"));
    assert!(test3.is_ok());
    assert_eq!((test3.unwrap().0).fragment, "");
}


/**
 *  Test parsing Funcs statments.
 */
#[test]
fn test_parse_funcs() {
    let test1 = parse("
        fn tio(i: i32) -> i32 {
            let a = 200; 
            if i < 1000 {
                tio(i + 1)
                } 
            else{
                i
            }
        }

        fn main() {
            let a = 100; 
            tio(1);
        }"
    );
    assert!(test1.is_ok());
    assert_eq!((test1.unwrap().0).fragment, "");
}


/**
 *  Test return statments.
 */
#[test]
fn test_parse_return() {
    let test1 = parse_expr(Span::new("return tio(i + 1);"));
    assert!(test1.is_ok());
    assert_eq!((test1.unwrap().0).fragment, "");
}


/**
 *  Test assign statments.
 */
#[test]
fn test_parse_assign() {
    let test1 = parse_expr(Span::new("a = 10 +1;"));
    assert!(test1.is_ok());
    assert_eq!((test1.unwrap().0).fragment, "");

    let test1 = parse_expr(Span::new("a =a +1;"));
    assert!(test1.is_ok());
    assert_eq!((test1.unwrap().0).fragment, "");
}


/**
 *  Test var with type statments.
 */
#[test]
fn test_parse_var_with_type() {
    let test1 = parse_expr(Span::new("a: i32"));
    assert!(test1.is_ok());
    assert_eq!((test1.unwrap().0).fragment, "");

    let test1 = parse_expr(Span::new("sadsd: bool"));
    assert!(test1.is_ok());
    assert_eq!((test1.unwrap().0).fragment, "");
}
