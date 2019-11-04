// /***  
//  *  Tests for interpreter functions.
//  *
//  *  Too run: 'cargo test'
//  */
// #[path = "../src/parser/mod.rs"]
// mod parser;
// use crate::parser::{
//     parse_expr, 
//     parse,
//     Span,
// };

// #[path = "../src/interpreter/mod.rs"]
// mod interpreter;
// use crate::interpreter::{
//     Val, 
//     interp_ast,
// };

// /**
//  *  Test interpreting singel int.
//  */
// #[test]
// fn test_interp_int() {
//     let test1 = interp_ast(parse_expr(Span::new(" 2")).unwrap().1);
//     assert!(test1.is_ok());
//     assert_eq!(test1.unwrap().1, Val::Num(2));

//     let test2 = interp_ast(parse_expr(Span::new("931235")).unwrap().1);
//     assert!(test2.is_ok());
//     assert_eq!(test2.unwrap().1, Val::Num(931235));
// }


// /**
//  *  Test interpreting singel bool.
//  */
// #[test]
// fn test_interp_bool() {
//     let test1 = interp_ast(parse_expr(Span::new("false")).unwrap().1);
//     assert!(test1.is_ok());
//     assert_eq!(test1.unwrap().1, Val::Bool(false));

//     let test2 = interp_ast(parse_expr(Span::new("true")).unwrap().1);
//     assert!(test2.is_ok());
//     assert_eq!(test2.unwrap().1, Val::Bool(true));
// }


// /**
//  *  Test interpreting singel unop.
//  */
// #[test]
// fn test_interp_unop() {
//     let test1 = interp_ast(parse_expr(Span::new("-1")).unwrap().1);
//     assert!(test1.is_ok());
//     assert_eq!(test1.unwrap().1, Val::Num(-1));

//     let test2 = interp_ast(parse_expr(Span::new("!true")).unwrap().1);
//     assert!(test2.is_ok());
//     assert_eq!(test2.unwrap().1, Val::Bool(false));
// }


// /**
//  *  Test interpreting singel binop.
//  */
// #[test]
// fn test_interp_binop() {
//     let test1 = interp_ast(parse_expr(Span::new("4 + 2")).unwrap().1);
//     assert!(test1.is_ok());
//     assert_eq!(test1.unwrap().1, Val::Num(6));

//     let test2 = interp_ast(parse_expr(Span::new("1-20")).unwrap().1);
//     assert!(test2.is_ok());
//     assert_eq!(test2.unwrap().1, Val::Num(-19));

//     let test3 = interp_ast(parse_expr(Span::new("200 / 4")).unwrap().1);
//     assert!(test3.is_ok());
//     assert_eq!(test3.unwrap().1, Val::Num(50));

//     let test4 = interp_ast(parse_expr(Span::new("10 * 3")).unwrap().1);
//     assert!(test4.is_ok());
//     assert_eq!(test4.unwrap().1, Val::Num(30));

//     let test5 = interp_ast(parse_expr(Span::new("13 % 5")).unwrap().1);
//     assert!(test5.is_ok());
//     assert_eq!(test5.unwrap().1, Val::Num(3));

//     let test6 = interp_ast(parse_expr(Span::new("true && false")).unwrap().1);
//     assert!(test6.is_ok());
//     assert_eq!(test6.unwrap().1, Val::Bool(false));

//     let test7 = interp_ast(parse_expr(Span::new("true || false")).unwrap().1);
//     assert!(test7.is_ok());
//     assert_eq!(test7.unwrap().1, Val::Bool(true));

//     let test8 = interp_ast(parse_expr(Span::new("12 == 12")).unwrap().1);
//     assert!(test8.is_ok());
//     assert_eq!(test8.unwrap().1, Val::Bool(true));

//     let test9 = interp_ast(parse_expr(Span::new("10 != 12")).unwrap().1);
//     assert!(test9.is_ok());
//     assert_eq!(test9.unwrap().1, Val::Bool(true));

//     let test10 = interp_ast(parse_expr(Span::new("(10 + 2) < 12 * 2")).unwrap().1);
//     assert!(test10.is_ok());
//     assert_eq!(test10.unwrap().1, Val::Bool(true));

//     let test11 = interp_ast(parse_expr(Span::new("(10 + 2) > 12 * 2")).unwrap().1);
//     assert!(test11.is_ok());
//     assert_eq!(test11.unwrap().1, Val::Bool(false));

//     let test12 = interp_ast(parse_expr(Span::new("4 <= -2")).unwrap().1);
//     assert!(test12.is_ok());
//     assert_eq!(test12.unwrap().1, Val::Bool(false));

//     let test13 = interp_ast(parse_expr(Span::new("4 >= 4")).unwrap().1);
//     assert!(test13.is_ok());
//     assert_eq!(test13.unwrap().1, Val::Bool(true));

//     let test14 = interp_ast(parse_expr(Span::new("false != true")).unwrap().1);
//     assert!(test14.is_ok());
//     assert_eq!(test14.unwrap().1, Val::Bool(true));

//     let test15 = interp_ast(parse_expr(Span::new("true == true")).unwrap().1);
//     assert!(test15.is_ok());
//     assert_eq!(test15.unwrap().1, Val::Bool(true));
// }


// /**
//  *  Test interpreting let, assign and var.
//  */
// #[test]
// fn test_interp_let_assign_var() {
//     let test1 = interp_ast(parse_expr(Span::new("{let mut a: i32 = 10; a = a + 2; a}")).unwrap().1);
//     assert!(test1.is_ok());
//     assert_eq!(test1.unwrap().1, Val::Num(12));

//     let test2 = interp_ast(parse_expr(Span::new("{let mut a: bool = true; !a}")).unwrap().1);
//     assert!(test2.is_ok());
//     assert_eq!(test2.unwrap().1, Val::Bool(false));
// }


// /**
//  *  Test interpreting if.
//  */
// #[test]
// fn test_interp_if() {
//     let test1 = interp_ast(parse_expr(Span::new("if 1 < 10 {12} else {false}")).unwrap().1);
//     assert!(test1.is_ok());
//     assert_eq!(test1.unwrap().1, Val::Num(12));

//     let test2 = interp_ast(parse_expr(Span::new("if 1 > 10 {true} else {false}")).unwrap().1);
//     assert!(test2.is_ok());
//     assert_eq!(test2.unwrap().1, Val::Bool(false));
// }


// /**
//  *  Test interpreting if. Should panic
//  */
// #[test]
// #[should_panic]
// fn test_interp_if_panic() {
//     let test3 = interp_ast(parse_expr(Span::new("{if 1 > 10 {true} else {let a: i32 = 12;} a}")).unwrap().1);
//     assert!(test3.is_err());
// }


// /**
//  *  Test interpreting while.
//  */
// #[test]
// fn test_interp_while() {
//     let test1 = interp_ast(parse_expr(Span::new("{let mut a: i32 = 0; while a < 10 {a = a +1;} a}")).unwrap().1);
//     assert!(test1.is_ok());
//     assert_eq!(test1.unwrap().1, Val::Num(10));

//     let test2 = interp_ast(parse_expr(Span::new("{let mut a: bool = true; while a {a = false;} a}")).unwrap().1);
//     assert!(test2.is_ok());
//     assert_eq!(test2.unwrap().1, Val::Bool(false));
// }


// /**
//  *  Test interpreting func, return and funcCall.
//  */
// #[test]
// fn test_interp_func() {
//     let test1 = interp_ast(parse_expr(Span::new(
//         "   {
//             fn tio(i: i32) -> i32 {
//                 i
//             }
//             tio(2)
//             }
//         "
//     )).unwrap().1);
//     assert!(test1.is_ok());
//     assert_eq!(test1.unwrap().1, Val::Num(2));

//     let test2 = interp_ast(parse_expr(Span::new(
//         "   {
//             fn tio(i: i32) -> i32 {
//                 return 10;
//                 i
//             }
//             tio(2)
//             }
//         "
//     )).unwrap().1);
//     assert!(test2.is_ok());
//     assert_eq!(test2.unwrap().1, Val::Num(10));
// }


// /**
//  *  Test interpreting recursiv func.
//  */
// #[test]
// fn test_interp_recursiv_func() {
//     let test1 = interp_ast(parse_expr(Span::new(
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
//         "
//     )).unwrap().1);
//     assert!(test1.is_ok());
//     assert_eq!(test1.unwrap().1, Val::Num(50));
// }


// /**
//  *  Test interpreting recursiv ast.
//  */
// #[test]
// fn test_interp_ast() {
//     let test1 = interp_ast(parse_expr(Span::new(
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
//         "
//     )).unwrap().1);
//     assert!(test1.is_ok());
//     assert_eq!(test1.unwrap().1, Val::Empty);
// }


// /**
//  *  Test interpreting mutabilety.
//  */
// #[test]
// fn test_interp_mut() {
//     let test1 = interp_ast(parse_expr(Span::new("{let mut a: i32 = 10; a = a + 2;}")).unwrap().1);
//     assert!(test1.is_ok());
//     assert_eq!(test1.unwrap().1, Val::Num(12));

//     let test2 = interp_ast(parse_expr(Span::new("{let mut a: bool = true; a =!a; a}")).unwrap().1);
//     assert!(test2.is_ok());
//     assert_eq!(test2.unwrap().1, Val::Bool(false));
// }


// /**
//  *  Test interpreting mutabilety.
//  */
// #[test]
// #[should_panic]
// fn test_interp_mut_panic1() {
//     let test1 = interp_ast(parse_expr(Span::new("{let a: i32 = 10; a = a + 2;}")).unwrap().1);
//     assert!(test1.is_ok());
//     assert_eq!(test1.unwrap().1, Val::Num(12));
// }


// /**
//  *  Test interpreting borrow.
//  */
// #[test]
// fn test_interp_borrow() {
//     let test1 = interp_ast(parse_expr(Span::new("{let mut a: i32 = 10; let b: &i32 = &a; b }")).unwrap().1);
//     assert!(test1.is_ok());
//     assert_eq!(test1.unwrap().1, Val::Ident("a".to_string(), 0));

//     let test2 = interp_ast(parse_expr(Span::new("{let mut a: bool = true; let b: &bool = &a; *b}")).unwrap().1);
//     assert!(test2.is_ok());
//     assert_eq!(test2.unwrap().1, Val::Bool(true));

//     let test3 = interp_ast(parse_expr(Span::new("{let mut a: i32 = 10; let b: &i32 = &a; let c: &i32 = b; c}")).unwrap().1);
//     assert_eq!(test3.unwrap().1, Val::Ident("a".to_string(), 0));
// }


// /**
//  *  Test interpreting borrow mut.
//  */
// #[test]
// fn test_interp_borrow_mut() {
//     let test1 = interp_ast(parse_expr(Span::new("{let mut a: i32 = 10; let b: &mut i32 = &mut a; *b = 12; a}")).unwrap().1);
//     assert!(test1.is_ok());
//     assert_eq!(test1.unwrap().1, Val::Num(12)); 

//     let test2 = interp_ast(parse_expr(Span::new("{let mut a: bool = true; let mut b: &mut bool = &mut a; let c: &mut bool = &mut b; **c = false; a}")).unwrap().1);
//     assert!(test2.is_ok());
//     assert_eq!(test2.unwrap().1, Val::Bool(false));
// }

// /**
//  *  Test interpreting borrow mut for func.
//  */
// #[test]
// fn test_interp_func_borrow_mut() {
//     let test1 = interp_ast(parse_expr(Span::new(
//         " 
//         {
//         fn test(i: &mut i32) -> () {
//             *i = 10;
//         }
//         let mut a: i32 = 0;
//         test(&mut a);
//         a
//         }
//         ")).unwrap().1);
//     assert_eq!(test1.unwrap().1, Val::Num(10)); 

//     let test2 = interp_ast(parse_expr(Span::new(
//         " 
//         {
//         fn test(i: &mut i32) -> () {
//             **i = 10;
//         }
//         let mut a: i32 = 0;
//         let mut b:  &mut i32 = &mut a;
//         test(&mut b);
//         a
//         }
//         ")).unwrap().1);
//     assert_eq!(test2.unwrap().1, Val::Num(10)); 
// }
