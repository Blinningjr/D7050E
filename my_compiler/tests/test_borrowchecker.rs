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
    VarInfo,
    BorrowInfo,
    ValueInfo,
};


/**
 *  Test borrowcheck singel int.
 */
#[test]
fn test_borrowcheck_int() {
    let test1 = borrowcheck_ast(parse_expr(Span::new(" 2")).unwrap().1);
    assert_eq!(test1.unwrap().1, BorrowInfo::Value(ValueInfo {
        mutable: false, 
        prefix: Prefix::None, 
        scope: -1,
        mem_pos: 0,
        num_borrows: 0, 
        num_borrowmuts: 0
    }, false));

    let test1 = borrowcheck_ast(parse_expr(Span::new(" &2")).unwrap().1);
    assert_eq!(test1.unwrap().1, BorrowInfo::Value(ValueInfo {
        mutable: false, 
        prefix: Prefix::Borrow, 
        scope: 0,
        mem_pos: 0,
        num_borrows: 0, 
        num_borrowmuts: 0
    }, false));

    let test1 = borrowcheck_ast(parse_expr(Span::new(" &mut 2")).unwrap().1);
    assert_eq!(test1.unwrap().1, BorrowInfo::Value(ValueInfo {
        mutable: false, 
        prefix: Prefix::BorrowMut, 
        scope: 0,
        mem_pos: 0,
        num_borrows: 0, 
        num_borrowmuts: 0
    }, false));
}

/**
 *  Test borrowcheck singel int panic.
 */
#[test]
#[should_panic]
fn test_borrowcheck_int_panic_1() {
    let test1 = borrowcheck_ast(parse_expr(Span::new(" mut 2")).unwrap().1);
}


/**
 *  Test borrowcheck singel int panic.
 */
#[test]
#[should_panic]
fn test_borrowcheck_int_panic_2() {
    let test1 = borrowcheck_ast(parse_expr(Span::new(" *2")).unwrap().1);
}


/**
 *  Test borrowcheck singel bool.
 */
#[test]
fn test_borrowcheck_bool() {
    let test1 = borrowcheck_ast(parse_expr(Span::new(" true")).unwrap().1);
    assert_eq!(test1.unwrap().1, BorrowInfo::Value(ValueInfo {
        mutable: false, 
        prefix: Prefix::None, 
        scope: -1,
        mem_pos: 0,
        num_borrows: 0, 
        num_borrowmuts: 0
    }, false));

    let test1 = borrowcheck_ast(parse_expr(Span::new(" &false")).unwrap().1);
    assert_eq!(test1.unwrap().1, BorrowInfo::Value(ValueInfo {
        mutable: false, 
        prefix: Prefix::Borrow, 
        scope: 0,
        mem_pos: 0,
        num_borrows: 0, 
        num_borrowmuts: 0
    }, false));

    let test1 = borrowcheck_ast(parse_expr(Span::new(" &mut true")).unwrap().1);
    assert_eq!(test1.unwrap().1, BorrowInfo::Value(ValueInfo {
        mutable: false, 
        prefix: Prefix::BorrowMut, 
        scope: 0,
        mem_pos: 0,
        num_borrows: 0, 
        num_borrowmuts: 0
    }, false));
}


/**
 *  Test borrowcheck singel bool panic.
 */
#[test]
#[should_panic]
fn test_borrowcheck_bool_panic_1() {
    let test1 = borrowcheck_ast(parse_expr(Span::new(" mut false")).unwrap().1);
}


/**
 *  Test borrowcheck singel bool panic.
 */
#[test]
#[should_panic]
fn test_borrowcheck_bool_panic_2() {
    let test1 = borrowcheck_ast(parse_expr(Span::new(" *2")).unwrap().1);
}


/**
 *  Test borrowcheck unop.
 */
#[test]
fn test_borrowcheck_unop() {
    let test1 = borrowcheck_ast(parse_expr(Span::new(" & -2")).unwrap().1);
    assert_eq!(test1.unwrap().1, BorrowInfo::Value(ValueInfo {
        mutable: false, 
        prefix: Prefix::Borrow, 
        scope: 0,
        mem_pos: 0,
        num_borrows: 0, 
        num_borrowmuts: 0
    }, false));
}


/**
 *  Test borrowcheck unop panic.
 */
#[test]
#[should_panic]
fn test_borrowcheck_unop_panic_1() {
    let test1 = borrowcheck_ast(parse_expr(Span::new(" mut !false")).unwrap().1);
}


/**
 *  Test borrowcheck unop panic.
 */
#[test]
#[should_panic]
fn test_borrowcheck_unop_panic_2() {
    let test1 = borrowcheck_ast(parse_expr(Span::new(" * -2")).unwrap().1);
}


/**
 *  Test borrowcheck binop.
 */
#[test]
fn test_borrowcheck_binop() {
    let test1 = borrowcheck_ast(parse_expr(Span::new(" & 1-2")).unwrap().1);
    assert_eq!(test1.unwrap().1, BorrowInfo::Value(ValueInfo {
        mutable: false, 
        prefix: Prefix::None, 
        scope: -1,
        mem_pos: 0,
        num_borrows: 0, 
        num_borrowmuts: 0
    }, false));

    let test2 = borrowcheck_ast(parse_expr(Span::new(" &1 + &12")).unwrap().1);
    assert_eq!(test2.unwrap().1, BorrowInfo::Value(ValueInfo {
        mutable: false, 
        prefix: Prefix::None, 
        scope: -1,
        mem_pos: 0,
        num_borrows: 0, 
        num_borrowmuts: 0
    }, false));
}


/**
 *  Test borrowcheck binop panic.
 */
#[test]
#[should_panic]
fn test_borrowcheck_binop_panic_1() {
    let test1 = borrowcheck_ast(parse_expr(Span::new(" &mut 1-2")).unwrap().1);
}


/**
 *  Test borrowcheck binop panic.
 */
#[test]
#[should_panic]
fn test_borrowcheck_binop_panic_2() {
    let test1 = borrowcheck_ast(parse_expr(Span::new("  &mut 10- &mut 2")).unwrap().1);
}


/**
 *  Test borrowcheck binop panic.
 */
#[test]
#[should_panic]
fn test_borrowcheck_binop_panic_3() {
    let test1 = borrowcheck_ast(parse_expr(Span::new(" * 1-2")).unwrap().1);
}


/**
 *  Test borrowcheck binop panic.
 */
#[test]
#[should_panic]
fn test_borrowcheck_binop_panic_4() {
    let test1 = borrowcheck_ast(parse_expr(Span::new("  * 10- * 2")).unwrap().1);
}


/**
 *  Test borrowcheck binop panic.
 */
#[test]
#[should_panic]
fn test_borrowcheck_binop_panic_5() {
    let test1 = borrowcheck_ast(parse_expr(Span::new(" mut 1-2")).unwrap().1);
}


/**
 *  Test borrowcheck binop panic.
 */
#[test]
#[should_panic]
fn test_borrowcheck_binop_panic_6() {
    let test1 = borrowcheck_ast(parse_expr(Span::new("  mut 10-  mut 2")).unwrap().1);
}


/**
 *  Test borrowcheck let.
 */
#[test]
fn test_borrowcheck_let() {
    let test1 = borrowcheck_ast(parse_expr(Span::new("let a: &i32 = &10;")).unwrap().1);
    assert_eq!(test1.unwrap().1, BorrowInfo::Var(VarInfo {
        mutable: false,
        prefix: Prefix::Borrow,
        ident: "a".to_string(),

        scope: 0,
        mem_pos: 1,

        pointer_scope_pos: 0,
        pointer_mem_pos: 0,

        num_borrows: 0,
        num_borrowmuts: 0,
    }, false));


    let test2 = borrowcheck_ast(parse_expr(Span::new("let a: &mut bool = &mut true;")).unwrap().1);
    assert_eq!(test2.unwrap().1, BorrowInfo::Var(VarInfo {
        mutable: false,
        prefix: Prefix::BorrowMut,
        ident: "a".to_string(),

        scope: 0,
        mem_pos: 1,

        pointer_scope_pos: 0,
        pointer_mem_pos: 0,

        num_borrows: 0,
        num_borrowmuts: 0,
    }, false));
}


// /**
//  *  Test borrowcheck let panic.
//  */
// #[test]
// #[should_panic]
// fn test_borrowcheck_let_panic_1() {
//     let test1 = borrowcheck_ast(parse_expr(Span::new("let a: bool = &true;")).unwrap().1);
// }


// /**
//  *  Test borrowcheck let panic.
//  */
// #[test]
// #[should_panic]
// fn test_borrowcheck_let_panic_2() {
//     let test1 = borrowcheck_ast(parse_expr(Span::new("let a: &bool = true;")).unwrap().1);
// }


// /**
//  *  Test borrowcheck let panic.
//  */
// #[test]
// #[should_panic]
// fn test_borrowcheck_let_panic_3() {
//     let test1 = borrowcheck_ast(parse_expr(Span::new("let a: &bool = &mut true;")).unwrap().1);
// }


// /**
//  *  Test borrowcheck let panic.
//  */
// #[test]
// #[should_panic]
// fn test_borrowcheck_let_panic_4() {
//     let test1 = borrowcheck_ast(parse_expr(Span::new("let a: &mut bool = &true;")).unwrap().1);
// }


// /**
//  *  Test borrowcheck assign.
//  */
// #[test]
// fn test_borrowcheck_assign() {
//     let test1 = borrowcheck_ast(parse_expr(Span::new("{let a: &i32 = &10; a = &4;}")).unwrap().1);
//     assert_eq!(test1.unwrap().1, BorrowInfo::Var(VarInfo {
//         mutable: false,
//         prefix: Prefix::Borrow,
//         ident: "a".to_string(),

//         scope: 0,
//         mem_pos: 0,

//         pointer_scope_pos: 0,
//         pointer_mem_pos: 0,

//         num_borrows: 0,
//         num_borrowmuts: 0,
//     }, false));

//     let test2 = borrowcheck_ast(parse_expr(Span::new("{let a: &mut i32 = &mut 10; *a = 4;}")).unwrap().1);
//     assert_eq!(test2.unwrap().1, BorrowInfo::Var(VarInfo {
//         mutable: false,
//         prefix: Prefix::DeRef(1),
//         ident: "a".to_string(),

//         scope: 0,
//         mem_pos: 0,

//         pointer_scope_pos: 0,
//         pointer_mem_pos: 0,

//         num_borrows: 0,
//         num_borrowmuts: 0,
//     }, false));
// }


// /**
//  *  Test borrowcheck assign panic.
//  */
// #[test]
// #[should_panic]
// fn test_borrowcheck_assign_panic_1() {
//     let test1 = borrowcheck_ast(parse_expr(Span::new("{let a: &i32 = &10; a = 4;}")).unwrap().1);
// }


// /**
//  *  Test borrowcheck assign panic.
//  */
// #[test]
// #[should_panic]
// fn test_borrowcheck_assign_panic_2() {
//     let test1 = borrowcheck_ast(parse_expr(Span::new("{let a: &i32 = &10; *a = 4;}")).unwrap().1);
// }


// /**
//  *  Test borrowcheck assign panic.
//  */
// #[test]
// #[should_panic]
// fn test_borrowcheck_assign_panic_3() {
//     let test1 = borrowcheck_ast(parse_expr(Span::new("{let a: &mut i32 = &mut 10; a = &4;}")).unwrap().1);
// }


// /**
//  *  Test borrowcheck assign panic.
//  */
// #[test]
// #[should_panic]
// fn test_borrowcheck_assign_panic_4() {
//     let test1 = borrowcheck_ast(parse_expr(Span::new("{let a: &mut i32 = &mut 10; a = 4;}")).unwrap().1);
// }


// /**
//  *  Test borrowcheck var.
//  */
// #[test]
// fn test_borrowcheck_var() {
//     let test1 = borrowcheck_ast(parse_expr(Span::new("{let a: i32 = 10; let b: &i32 = &a;}")).unwrap().1);
//     assert_eq!(test1.unwrap().1, Prefix::Borrow);

//     let test2 = borrowcheck_ast(parse_expr(Span::new("{let mut a: i32 = 10; let b: &mut i32 = &mut a;}")).unwrap().1);
//     assert_eq!(test2.unwrap().1, Prefix::BorrowMut);

//     let test3 = borrowcheck_ast(parse_expr(Span::new("{let a: &i32 = &10; let b: i32 = *a;}")).unwrap().1);
//     assert_eq!(test3.unwrap().1, Prefix::Borrow);

//     let test4 = borrowcheck_ast(parse_expr(Span::new("{let a: &i32 = &10; let b: &i32 = &a; let c: i32 = **b;}")).unwrap().1);
//     assert_eq!(test4.unwrap().1, Prefix::None);
// }


// /**
//  *  Test borrowcheck var panic.
//  */
// #[test]
// #[should_panic]
// fn test_borrowcheck_var_panic_1() {
//     let test1 = borrowcheck_ast(parse_expr(Span::new("{let a: &i32 = &10; let b: i32 = **a;}")).unwrap().1);
// }


// /**
//  *  Test borrowcheck var panic.
//  */
// #[test]
// #[should_panic]
// fn test_borrowcheck_var_panic_2() {
//     let test1 = borrowcheck_ast(parse_expr(Span::new("{let a: &i32 = &10; let b: i32 = a;}")).unwrap().1);
// }


// /**
//  *  Test borrowcheck var panic.
//  */
// #[test]
// #[should_panic]
// fn test_borrowcheck_var_panic_3() {
//     let test1 = borrowcheck_ast(parse_expr(Span::new("{let a: &mut i32 = &mut 10; let b: &i32 = a;}")).unwrap().1);
// }


// /**
//  *  Test borrowcheck var panic.
//  */
// #[test]
// #[should_panic]
// fn test_borrowcheck_var_panic_4() {
//     let test1 = borrowcheck_ast(parse_expr(Span::new("{let a: &i32 = &10; let b: &mut i32 = a;}")).unwrap().1);
// }


// /**
//  *  Test borrowcheck if.
//  */
// #[test]
// fn test_borrowcheck_if() {
//     let test1 = borrowcheck_ast(parse_expr(Span::new("{let a: i32 = 10; if a == 10 {}}")).unwrap().1);
//     assert_eq!(test1.unwrap().1, Prefix::None);

//     let test2 = borrowcheck_ast(parse_expr(Span::new("{let a: &i32 = &10; if *a == 10 {}}")).unwrap().1);
//     assert_eq!(test2.unwrap().1, Prefix::None);

//     let test3 = borrowcheck_ast(parse_expr(Span::new("{let a: &mut i32 = &mut 10; if *a == 10 {}}")).unwrap().1);
//     assert_eq!(test3.unwrap().1, Prefix::None);
// }


// /**
//  *  Test borrowcheck while.
//  */
// #[test]
// fn test_borrowcheck_while() {
//     let test1 = borrowcheck_ast(parse_expr(Span::new("{let a: i32 = 10; while a == 10 {}}")).unwrap().1);
//     assert_eq!(test1.unwrap().1, Prefix::None);

//     let test2 = borrowcheck_ast(parse_expr(Span::new("{let a: &i32 = &10; while *a == 10 {}}")).unwrap().1);
//     assert_eq!(test2.unwrap().1, Prefix::None);

//     let test3 = borrowcheck_ast(parse_expr(Span::new("{let a: &mut i32 = &mut 10; while *a == 10 {}}")).unwrap().1);
//     assert_eq!(test3.unwrap().1, Prefix::None);
// }


// /**
//  *  Test borrowcheck func.
//  */
// #[test]
// fn test_borrowcheck_func() {
//     let test1 = borrowcheck_ast(parse_expr(Span::new("{
//             fn tio(i: &i32) -> i32 {
//                 return *i;
//             }
//             tio(&2)
//             }")).unwrap().1);
//     assert_eq!(test1.unwrap().1, Prefix::None);

//     let test2 = borrowcheck_ast(parse_expr(Span::new("{
//             fn tio(i: &mut i32) -> () {
//                 *i = 10
//             }
//             tio(&mut 2)
//             }")).unwrap().1);
//     assert_eq!(test2.unwrap().1, Prefix::None);

//     let test3 = borrowcheck_ast(parse_expr(Span::new("{
//             fn tio(i: & i32) -> () {
//                 let a: & i32 = i;
//                 let b: & i32 = i;
//                 let c: & i32 = a;
//             }
//             tio(& 2)
//             }")).unwrap().1);
//     assert_eq!(test3.unwrap().1, Prefix::None);
// }

// /**
//  *  Test borrowcheck funcs.
//  */
// #[test]
// fn test_borrowcheck_funcs() {
//     let test1 = borrowcheck_ast(parse_expr(Span::new("
//         fn tio(i: &i32) -> i32 {
//             if i < 50 {
//                 return tio(&(i + 1));
//             } 
//             else{
//                 return *i;       
//             }
//         }

//         fn main() {
//             let a: i32 = 2; 
//             tio(&a);
//         }
//         ")).unwrap().1);
//     assert_eq!(test1.unwrap().1, Prefix::None);
// }
