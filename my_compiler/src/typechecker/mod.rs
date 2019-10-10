/**
 *  Imports from parser.
 */
#[path = "../parser/mod.rs"]
mod parser;
use crate::parser::{
    SpanExpr,
    SpanOp,
    expr::Expr,
    op::Op,
    mytype::MyType,
    IResult,
};


/** 
 *  Typecheck ast.
*/
pub fn typecheck_ast<'a>(e: SpanExpr<'a>) -> IResult<SpanExpr, MyType> {
    typecheck_expr(e)
}


/** 
 *  Typecheck expresions in ast.
*/
fn typecheck_expr<'a>(e: SpanExpr<'a>) -> IResult<SpanExpr, MyType> {
    match (e.1).clone() {
        Expr::Num(_) => Ok((e, MyType::Int32)),
        Expr::Bool(_) => Ok((e, MyType::Boolean)),
        Expr::UnOp(_, _) => typecheck_unop(e),
        Expr::BinOp(_, _, _) => typecheck_binop(e),
        Expr::Let(_, _, _, _, _) => typecheck_let(e),
        _ => panic!("typecheck_expr {:#?}", e),
    }
}


/** 
 *  Typecheck unop in ast.
*/
fn typecheck_unop<'a>(e: SpanExpr<'a>) -> IResult<SpanExpr, MyType> {
    match (e.1).clone() {
        Expr::UnOp(op, expr) => {
            let e_type = typecheck_expr(*expr)?;
            match op.1 {
                Op::Not => {
                    match e_type.1 {
                        MyType::Boolean => return Ok((e, MyType::Boolean)),
                        _ => panic!("typecheck_unop"),
                    }
                },
                Op::Sub => {
                    match e_type.1 {
                        MyType::Int32 => return Ok((e, MyType::Int32)),
                        _ => panic!("typecheck_unop"),
                    }
                },
                _ => panic!("typecheck_unop"),
            }
        },
        _ => panic!("typecheck_unop"),
    }
}


/** 
 *  Typecheck binop in ast.
*/
fn typecheck_binop<'a>(e: SpanExpr<'a>) -> IResult<SpanExpr, MyType> {
    match (e.1).clone() {
        Expr::BinOp(le, op, re) => {
            let le_type = typecheck_expr(*le)?.1;
            let re_type = typecheck_expr(*re)?.1;
            match op.1 {
            Op::Add => Ok((e, check_if_same_type_and_type(le_type, re_type, MyType::Int32))),
            Op::Sub => Ok((e, check_if_same_type_and_type(le_type, re_type, MyType::Int32))),
            Op::Div => Ok((e, check_if_same_type_and_type(le_type, re_type, MyType::Int32))),
            Op::Multi => Ok((e, check_if_same_type_and_type(le_type, re_type, MyType::Int32))),
            Op::Mod => Ok((e, check_if_same_type_and_type(le_type, re_type, MyType::Int32))),
            Op::And => Ok((e, check_if_same_type_and_type(le_type, re_type, MyType::Boolean))),
            Op::Or => Ok((e, check_if_same_type_and_type(le_type, re_type, MyType::Boolean))),
            Op::NotEq => {check_if_same_type(le_type, re_type); return Ok((e, MyType::Boolean))},
            Op::Equal => {check_if_same_type(le_type, re_type); return Ok((e, MyType::Boolean))},
            Op::LessEqThen => {check_if_same_type_and_type(le_type, re_type, MyType::Int32); return Ok((e, MyType::Boolean))},
            Op::LargEqThen => {check_if_same_type_and_type(le_type, re_type, MyType::Int32); return Ok((e, MyType::Boolean))},
            Op::LessThen => {check_if_same_type_and_type(le_type, re_type, MyType::Int32); return Ok((e, MyType::Boolean))},
            Op::LargThen => {check_if_same_type_and_type(le_type, re_type, MyType::Int32); return Ok((e, MyType::Boolean))},
                _ => panic!("typecheck_binop"),
            }
        },
        _ => panic!("typecheck_binop"),
    }
}


/** 
 *  Typecheck let in ast.
*/
fn typecheck_let<'a>(e: SpanExpr<'a>) -> IResult<SpanExpr, MyType> {
    match (e.1).clone() {
        Expr::Let(_, _, _, t, v) => {
            let vt = typecheck_expr(*v)?.1;
            return Ok((e, check_if_same_type(t.1, vt)));
        },
        _ => panic!("typecheck_unop"),
    }
}


fn check_if_num(t: MyType) -> bool {
    match t {
        MyType::Int32 => true,
        _ => false,
    }
}


fn check_if_bool(t: MyType) -> bool {
    match t {
        MyType::Boolean => true,
        _ => false,
    }
}


fn check_if_same_type(lt: MyType, rt: MyType) -> MyType {
    if check_if_bool(lt.clone()) && check_if_bool(rt.clone()) {
        return MyType::Boolean;
    } else if check_if_num(lt) && check_if_num(rt) {
        return MyType::Int32;
    }
    panic!("check_if_same_type");
}


fn check_if_same_type_and_type(lt: MyType, rt: MyType, wanted: MyType) -> MyType {
    match wanted {
        MyType::Int32 => {
            if check_if_num(lt.clone()) {
                return check_if_same_type(lt,rt);
            }
            panic!("check_if_same_type_and_type");
        },
        MyType::Boolean => {
            if check_if_bool(lt.clone()) {
                return check_if_same_type(lt,rt);
            }
            panic!("check_if_same_type_and_type");
        },
        MyType::NoType => panic!("NoType not implemented check_if_same_type_and_type"),
    }
}
