/**
 *  Tests for borrowchecker functions.
 *
 *  Too run: 'cargo test'
 */
#[path = "../src/parser/mod.rs"]
mod parser;
use crate::parser::{
    parse_expr, 
    parse,
    Span,
    varprefix::Prefix,
};

#[path = "../src/borrowchecker/mod.rs"]
mod borrowchecker;
use crate::borrowchecker::{
    borrowcheck_ast,
};


/**
 *  Test borrwocheck singel int.
 */
#[test]
fn test_borrowcheck_int() {
    let test1 = borrowcheck_ast(parse_expr(Span::new(" 2")).unwrap().1);
    assert_eq!(test1.unwrap().1, Prefix::None);

    let test1 = borrowcheck_ast(parse_expr(Span::new(" &2")).unwrap().1);
    assert_eq!(test1.unwrap().1, Prefix::Borrow);

    let test1 = borrowcheck_ast(parse_expr(Span::new(" &mut 2")).unwrap().1);
    assert_eq!(test1.unwrap().1, Prefix::BorrowMut);
}

/**
 *  Test borrwocheck singel int panic.
 */
#[test]
#[should_panic]
fn test_borrowcheck_int_panic_1() {
    let test1 = borrowcheck_ast(parse_expr(Span::new(" mut 2")).unwrap().1);
}


/**
 *  Test borrwocheck singel int panic.
 */
#[test]
#[should_panic]
fn test_borrowcheck_int_panic_2() {
    let test1 = borrowcheck_ast(parse_expr(Span::new(" *2")).unwrap().1);
}


/**
 *  Test borrwocheck singel bool.
 */
#[test]
fn test_borrowcheck_bool() {
    let test1 = borrowcheck_ast(parse_expr(Span::new(" true")).unwrap().1);
    assert_eq!(test1.unwrap().1, Prefix::None);

    let test1 = borrowcheck_ast(parse_expr(Span::new(" &false")).unwrap().1);
    assert_eq!(test1.unwrap().1, Prefix::Borrow);

    let test1 = borrowcheck_ast(parse_expr(Span::new(" &mut true")).unwrap().1);
    assert_eq!(test1.unwrap().1, Prefix::BorrowMut);
}


/**
 *  Test borrwocheck singel bool panic.
 */
#[test]
#[should_panic]
fn test_borrowcheck_bool_panic_1() {
    let test1 = borrowcheck_ast(parse_expr(Span::new(" mut false")).unwrap().1);
}


/**
 *  Test borrwocheck singel bool panic.
 */
#[test]
#[should_panic]
fn test_borrowcheck_bool_panic_2() {
    let test1 = borrowcheck_ast(parse_expr(Span::new(" *2")).unwrap().1);
}


/**
 *  Test borrwocheck unop.
 */
#[test]
fn test_borrwocheck_unop() {
    let test1 = borrowcheck_ast(parse_expr(Span::new(" & -2")).unwrap().1);
    assert_eq!(test1.unwrap().1, Prefix::Borrow);

    let test2 = borrowcheck_ast(parse_expr(Span::new(" &mut !true")).unwrap().1);
    assert_eq!(test2.unwrap().1, Prefix::BorrowMut);
}


/**
 *  Test borrwocheck unop panic.
 */
#[test]
#[should_panic]
fn test_borrowcheck_unop_panic_1() {
    let test1 = borrowcheck_ast(parse_expr(Span::new(" mut !false")).unwrap().1);
}


/**
 *  Test borrwocheck unop panic.
 */
#[test]
#[should_panic]
fn test_borrowcheck_unop_panic_2() {
    let test1 = borrowcheck_ast(parse_expr(Span::new(" * -2")).unwrap().1);
}


/**
 *  Test borrwocheck binop.
 */
#[test]
fn test_borrwocheck_binop() {
    let test1 = borrowcheck_ast(parse_expr(Span::new(" & 1-2")).unwrap().1);
    assert_eq!(test1.unwrap().1, Prefix::None);

    let test2 = borrowcheck_ast(parse_expr(Span::new(" &1 + &12")).unwrap().1);
    assert_eq!(test2.unwrap().1, Prefix::None);
}


/**
 *  Test borrwocheck binop panic.
 */
#[test]
#[should_panic]
fn test_borrowcheck_binop_panic_1() {
    let test1 = borrowcheck_ast(parse_expr(Span::new(" &mut 1-2")).unwrap().1);
}


/**
 *  Test borrwocheck binop panic.
 */
#[test]
#[should_panic]
fn test_borrowcheck_binop_panic_2() {
    let test1 = borrowcheck_ast(parse_expr(Span::new("  &mut 10- &mut 2")).unwrap().1);
}


/**
 *  Test borrwocheck binop panic.
 */
#[test]
#[should_panic]
fn test_borrowcheck_binop_panic_3() {
    let test1 = borrowcheck_ast(parse_expr(Span::new(" * 1-2")).unwrap().1);
}


/**
 *  Test borrwocheck binop panic.
 */
#[test]
#[should_panic]
fn test_borrowcheck_binop_panic_4() {
    let test1 = borrowcheck_ast(parse_expr(Span::new("  * 10- * 2")).unwrap().1);
}


/**
 *  Test borrwocheck binop panic.
 */
#[test]
#[should_panic]
fn test_borrowcheck_binop_panic_5() {
    let test1 = borrowcheck_ast(parse_expr(Span::new(" mut 1-2")).unwrap().1);
}


/**
 *  Test borrwocheck binop panic.
 */
#[test]
#[should_panic]
fn test_borrowcheck_binop_panic_6() {
    let test1 = borrowcheck_ast(parse_expr(Span::new("  mut 10-  mut 2")).unwrap().1);
}


/**
 *  Test borrwocheck let.
 */
#[test]
fn test_borrwocheck_let() {
    let test1 = borrowcheck_ast(parse_expr(Span::new("let a: &i32 = &10;")).unwrap().1);
    assert_eq!(test1.unwrap().1, Prefix::Borrow);

    let test2 = borrowcheck_ast(parse_expr(Span::new("let a: &mut bool = &mut true;")).unwrap().1);
    assert_eq!(test2.unwrap().1, Prefix::None);
}


/**
 *  Test borrwocheck let panic.
 */
#[test]
#[should_panic]
fn test_borrowcheck_let_panic_1() {
    let test1 = borrowcheck_ast(parse_expr(Span::new("let a: bool = &true;")).unwrap().1);
}


/**
 *  Test borrwocheck let panic.
 */
#[test]
#[should_panic]
fn test_borrowcheck_let_panic_2() {
    let test1 = borrowcheck_ast(parse_expr(Span::new("let a: &bool = true;")).unwrap().1);
}


/**
 *  Test borrwocheck let panic.
 */
#[test]
#[should_panic]
fn test_borrowcheck_let_panic_3() {
    let test1 = borrowcheck_ast(parse_expr(Span::new("let a: &bool = &mut true;")).unwrap().1);
}


/**
 *  Test borrwocheck let panic.
 */
#[test]
#[should_panic]
fn test_borrowcheck_let_panic_4() {
    let test1 = borrowcheck_ast(parse_expr(Span::new("let a: &mut bool = &true;")).unwrap().1);
}


/**
 *  Test borrwocheck assign.
 */
#[test]
fn test_borrwocheck_assign() {
    let test1 = borrowcheck_ast(parse_expr(Span::new("{let a: &i32 = &10; a = &4;}")).unwrap().1);
    assert_eq!(test1.unwrap().1, Prefix::Borrow);

    let test2 = borrowcheck_ast(parse_expr(Span::new("{let a: &mut i32 = &mut 10; *a = 4;}")).unwrap().1);
    assert_eq!(test2.unwrap().1, Prefix::None);
}


/**
 *  Test borrwocheck assign panic.
 */
#[test]
#[should_panic]
fn test_borrowcheck_assign_panic_1() {
    let test1 = borrowcheck_ast(parse_expr(Span::new("{let a: &i32 = &10; a = 4;}")).unwrap().1);
}


/**
 *  Test borrwocheck assign panic.
 */
#[test]
#[should_panic]
fn test_borrowcheck_assign_panic_2() {
    let test1 = borrowcheck_ast(parse_expr(Span::new("{let a: &i32 = &10; *a = 4;}")).unwrap().1);
}


/**
 *  Test borrwocheck assign panic.
 */
#[test]
#[should_panic]
fn test_borrowcheck_assign_panic_3() {
    let test1 = borrowcheck_ast(parse_expr(Span::new("{let a: &mut i32 = &mut 10; a = &4;}")).unwrap().1);
}


/**
 *  Test borrwocheck assign panic.
 */
#[test]
#[should_panic]
fn test_borrowcheck_assign_panic_4() {
    let test1 = borrowcheck_ast(parse_expr(Span::new("{let a: &mut i32 = &mut 10; a = 4;}")).unwrap().1);
}


/**
 *  Test borrwocheck var.
 */
#[test]
fn test_borrwocheck_var() {
    let test1 = borrowcheck_ast(parse_expr(Span::new("{let a: i32 = 10; let b: &i32 = &a;}")).unwrap().1);
    assert_eq!(test1.unwrap().1, Prefix::Borrow);

    let test2 = borrowcheck_ast(parse_expr(Span::new("{let mut a: i32 = 10; let b: &mut i32 = &mut a;}")).unwrap().1);
    assert_eq!(test2.unwrap().1, Prefix::None);

    let test3 = borrowcheck_ast(parse_expr(Span::new("{let a: &i32 = &10; let b: i32 = *a;}")).unwrap().1);
    assert_eq!(test3.unwrap().1, Prefix::Borrow);

    let test4 = borrowcheck_ast(parse_expr(Span::new("{let a: &i32 = &10; let b: &i32 = &a; let c: i32 = **b;}")).unwrap().1);
    assert_eq!(test4.unwrap().1, Prefix::None);
}


/**
 *  Test borrwocheck var panic.
 */
#[test]
#[should_panic]
fn test_borrowcheck_var_panic_1() {
    let test1 = borrowcheck_ast(parse_expr(Span::new("{let a: &i32 = &10; let b: i32 = **a;}")).unwrap().1);
}


/**
 *  Test borrwocheck var panic.
 */
#[test]
#[should_panic]
fn test_borrowcheck_var_panic_2() {
    let test1 = borrowcheck_ast(parse_expr(Span::new("{let a: &i32 = &10; let b: i32 = a;}")).unwrap().1);
}


/**
 *  Test borrwocheck var panic.
 */
#[test]
#[should_panic]
fn test_borrowcheck_var_panic_3() {
    let test1 = borrowcheck_ast(parse_expr(Span::new("{let a: &mut i32 = &mut 10; let b: &i32 = a;}")).unwrap().1);
}


/**
 *  Test borrwocheck var panic.
 */
#[test]
#[should_panic]
fn test_borrowcheck_var_panic_4() {
    let test1 = borrowcheck_ast(parse_expr(Span::new("{let a: &i32 = &10; let b: &mut i32 = a;}")).unwrap().1);
}
