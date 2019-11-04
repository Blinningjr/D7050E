// /***  
//  *  Tests for typechecker functions.
//  *
//  *  Too run: 'cargo test'
//  */
// #[path = "../src/parser/mod.rs"]
// mod parser;
// use crate::parser::{
//     parse_expr, 
//     parse,
//     Span,
//     mytype::MyType,
// };

// #[path = "../src/typechecker/mod.rs"]
// mod typechecker;
// use crate::typechecker::{
//     typecheck_ast,
// };


// /**
//  *  Test typecheck singel int.
//  */
// #[test]
// fn test_typecheck_int() {
//     let test1 = typecheck_ast(parse_expr(Span::new(" 2")).unwrap().1);
//     assert_eq!(test1.unwrap().1, MyType::Int32);
// }


// /**
//  *  Test typecheck singel bool.
//  */
// #[test]
// fn test_typecheck_bool() {
//     let test1 = typecheck_ast(parse_expr(Span::new("false")).unwrap().1);
//     assert_eq!(test1.unwrap().1, MyType::Boolean);

//     let test2 = typecheck_ast(parse_expr(Span::new(" true")).unwrap().1);
//     assert_eq!(test2.unwrap().1, MyType::Boolean);
// }


// /**
//  *  Test typecheck unop.
//  */
// #[test]
// fn test_typecheck_unop() {
//     let test1 = typecheck_ast(parse_expr(Span::new(" -2")).unwrap().1);
//     assert_eq!(test1.unwrap().1, MyType::Int32);

//     let test2 = typecheck_ast(parse_expr(Span::new(" !true")).unwrap().1);
//     assert_eq!(test2.unwrap().1, MyType::Boolean);
// }


// /**
//  *  Test typecheck binop.
//  */
// #[test]
// fn test_typecheck_binop() {
//     let test1 = typecheck_ast(parse_expr(Span::new("4 + 2")).unwrap().1);
//     assert_eq!(test1.unwrap().1, MyType::Int32);

//     let test2 = typecheck_ast(parse_expr(Span::new("1-20")).unwrap().1);
//     assert_eq!(test2.unwrap().1, MyType::Int32);

//     let test3 = typecheck_ast(parse_expr(Span::new("200 / 4")).unwrap().1);
//     assert_eq!(test3.unwrap().1, MyType::Int32);

//     let test4 = typecheck_ast(parse_expr(Span::new("10 * 3")).unwrap().1);
//     assert_eq!(test4.unwrap().1, MyType::Int32);

//     let test5 = typecheck_ast(parse_expr(Span::new("13 % 5")).unwrap().1);
//     assert_eq!(test5.unwrap().1, MyType::Int32);

//     let test6 = typecheck_ast(parse_expr(Span::new("true && false")).unwrap().1);
//     assert_eq!(test6.unwrap().1, MyType::Boolean);

//     let test7 = typecheck_ast(parse_expr(Span::new("true || false")).unwrap().1);
//     assert_eq!(test7.unwrap().1, MyType::Boolean);

//     let test8 = typecheck_ast(parse_expr(Span::new("12 == 12")).unwrap().1); 
//     assert_eq!(test8.unwrap().1, MyType::Boolean);

//     let test9 = typecheck_ast(parse_expr(Span::new("true == false")).unwrap().1);  
//     assert_eq!(test9.unwrap().1, MyType::Boolean);

//     let test10 = typecheck_ast(parse_expr(Span::new("10 != 12")).unwrap().1); 
//     assert_eq!(test10.unwrap().1, MyType::Boolean);

//     let test11 = typecheck_ast(parse_expr(Span::new("true != false")).unwrap().1); 
//     assert_eq!(test11.unwrap().1, MyType::Boolean);

//     let test12 = typecheck_ast(parse_expr(Span::new("(10 + 2) < 12 * 2")).unwrap().1); 
//     assert_eq!(test12.unwrap().1, MyType::Boolean); 

//     let test13 = typecheck_ast(parse_expr(Span::new("(10 + 2) > 12 * 2")).unwrap().1); 
//     assert_eq!(test13.unwrap().1, MyType::Boolean); 

//     let test14 = typecheck_ast(parse_expr(Span::new("4 <= -2")).unwrap().1); 
//     assert_eq!(test14.unwrap().1, MyType::Boolean); 

//     let test15 = typecheck_ast(parse_expr(Span::new("4 >= 4")).unwrap().1); 
//     assert_eq!(test15.unwrap().1, MyType::Boolean); 
// }


// /**
//  *  Test typecheck let.
//  */
// #[test]
// fn test_typecheck_let() {
//     let test1 = typecheck_ast(parse_expr(Span::new(" let apa: i32 = 20;")).unwrap().1);
//     assert_eq!(test1.unwrap().1, MyType::Int32);

//     let test2 = typecheck_ast(parse_expr(Span::new("let apa: bool = true;")).unwrap().1);
//     assert_eq!(test2.unwrap().1, MyType::Boolean);

//     let test1 = typecheck_ast(parse_expr(Span::new("let apa: bool = false;")).unwrap().1);
//     assert_eq!(test1.unwrap().1, MyType::Boolean);

//     let test2 = typecheck_ast(parse_expr(Span::new("let apa: i32=20 + 20- 2 * 20;")).unwrap().1);
//     assert_eq!(test2.unwrap().1, MyType::Int32);
// }


// /**
//  *  Test typecheck assign.
//  */
// #[test]
// fn test_typecheck_assign() {
//     let test1 = typecheck_ast(parse_expr(Span::new("{let mut a: i32 = 10; a = a + 2;}")).unwrap().1);
//     assert_eq!(test1.unwrap().1, MyType::Int32);

//     let test2 = typecheck_ast(parse_expr(Span::new("{let mut a: bool = true;}")).unwrap().1);
//     assert_eq!(test2.unwrap().1, MyType::Boolean);
// }


// /**
//  *  Test typecheck var.
//  */
// #[test]
// fn test_typecheck_var() {
//     let test1 = typecheck_ast(parse_expr(Span::new("{let mut a: i32 = 10; a = a + 2; a}")).unwrap().1);
//     assert_eq!(test1.unwrap().1, MyType::Int32);

//     let test2 = typecheck_ast(parse_expr(Span::new("{let mut a: bool = true; a}")).unwrap().1);
//     assert_eq!(test2.unwrap().1, MyType::Boolean);
// }


// /**
//  *  Test typecheck if.
//  */
// #[test]
// fn test_typecheck_if() {
//     let test1 = typecheck_ast(parse_expr(Span::new("if 1 < 10 {12} else {false}")).unwrap().1);
//     assert_eq!(test1.unwrap().1, MyType::NoType);

//     let test2 = typecheck_ast(parse_expr(Span::new("if 1 > 10 {true} else {false}")).unwrap().1);
//     assert_eq!(test2.unwrap().1, MyType::NoType);
// }


// /**
//  *  Test typecheck while.
//  */
// #[test]
// fn test_typecheck_while() {
//     let test1 = typecheck_ast(parse_expr(Span::new("{let mut a: i32 = 0; while a < 10 {a = a +1;} a}")).unwrap().1);
//     assert_eq!(test1.unwrap().1, MyType::Int32);

//     let test2 = typecheck_ast(parse_expr(Span::new("{let mut a: bool = true; while a {a = false;} a}")).unwrap().1);
//     assert_eq!(test2.unwrap().1, MyType::Boolean);
// }


// /**
//  *  Test typecheck func.
//  */
// #[test]
// fn test_typecheck_func() {
//     let test1 = typecheck_ast(parse_expr(Span::new(
//         "   {
//             fn tio(i: i32) -> i32 {
//                 return i;
//             }
//             tio(2)
//             }
//         ")).unwrap().1);
//     assert_eq!(test1.unwrap().1, MyType::Int32);

//     let test2 = typecheck_ast(parse_expr(Span::new(
//         "   {
//             fn tio(i: i32) -> i32 {
//                 return 10;
//                 i
//             }
//             tio(2)
//             }
//         ")).unwrap().1);
//     assert_eq!(test2.unwrap().1, MyType::Int32);
// }


// /**
//  *  Test typecheck recursiv func.
//  */
// #[test]
// fn test_interp_recursiv_func() {
//     let test1 = typecheck_ast(parse_expr(Span::new(
//         "   {
//             fn tio(i: i32) -> i32 {
//                 if i < 50 {
//                     return tio(i + 1);
//                 } 
//                 else{
//                     return i;       
//                 }
//             }
//             tio(2)
//             }
//         ")).unwrap().1);
//     assert_eq!(test1.unwrap().1, MyType::Int32);
// }


// /**
//  *  Test typechecking funcs.
//  */
// #[test]
// fn test_typecheck_funcs() {
//     let test1 = typecheck_ast(parse(
//         "  
//         fn tio(i: i32) -> i32 {
//             if i < 50 {
//                 return tio(i + 1);
//             } 
//             else{
//                 return i;       
//             }
//         }

//         fn main() {
//             let a: i32 = 2; 
//             tio(2);
//         }
//         ").unwrap().1);
//     assert_eq!(test1.unwrap().1, MyType::NoType);
// }
