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
// // use crate::parser::expr::Expr;


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


// /**
//  *  Calculates the value of an math expression.
//  *  Needed for tests.
//  */
// pub fn math_expr_eval(e: Expr) -> Result<i32> {
//     match e {
//         Num(i) => Ok(i),
//         BinOp(l, op, r) => {
//             let left_value = math_expr_eval(*l).unwrap();
//             let right_value = math_expr_eval(*r).unwrap();
//             match op {
//                 Add => Ok(left_value + right_value),
//                 Sub => Ok(left_value - right_value),
//                 Div => Ok(left_value / right_value),
//                 Multi => Ok(left_value * right_value),
//                 Mod => Ok(left_value % right_value),
//                 _ => Err(SyntaxError),
//             }
//         }
//         UnOp(op, r) => {
//             let right_value = math_expr_eval(*r).unwrap();
//             match op {
//                 Sub => Ok(-right_value),
//                 _ => Err(SyntaxError),
//             }
//         }
//         _ => Err(SyntaxError),
//     }
// }


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
fn test_parse_ident() {
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

    let test6 = parse_expr(Span::new("apa: None"));
    assert!(test6.is_ok());
    assert_eq!((test6.unwrap().0).fragment, "");

    let test7 = parse_expr(Span::new("koskos: Str"));
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


// /**
//  *  Test parsing let statments.
//  */
#[test]
fn test_parse_let() {
    let test1 = parse_expr(Span::new(" let apa = 20;"));
    assert!(test1.is_ok());
    assert_eq!((test1.unwrap().0).fragment, "");

    let test3 = parse_expr(Span::new("let apa = true;"));
    assert!(test3.is_ok());
    assert_eq!((test3.unwrap().0).fragment, "");

    let test4 = parse_expr(Span::new("let apa = false;"));
    assert!(test4.is_ok());
    assert_eq!((test4.unwrap().0).fragment, "");

    let test5 = parse_expr(Span::new("let apa=20 + 20- 2 * 20;"));
    assert!(test5.is_ok());
    assert_eq!((test5.unwrap().0).fragment, "");
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

    let test2 = parse_expr(Span::new("while i < 10 {i = i + 1;}"));
    assert!(test2.is_ok());
    assert_eq!((test2.unwrap().0).fragment, "");
}


/**
 *  Test parsing Func statments.
 */
#[test]
fn test_parse_func() {
    let test1 = parse_expr(Span::new("fn apa(input) -> bool { let apa = 10;}"));
    assert!(test1.is_ok());
    assert_eq!((test1.unwrap().0).fragment, "");

    let test2 = parse_expr(Span::new("fn apa(input: i32) -> None { let a = 10; let var = true;}"));
    assert!(test2.is_ok());
    assert_eq!((test2.unwrap().0).fragment, "");

    let test3 = parse_expr(Span::new("fn apor(input: Str, test) -> bool { let apa = 10;}"));
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

        fn main() -> None {
            let a = 100; 
            tio(1);
        }"
    );
    assert!(test1.is_ok());
    assert_eq!((test1.unwrap().0).fragment, "");
}
