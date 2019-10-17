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

#[path = "../interpreter/enverror.rs"]
pub mod enverror;
use enverror::{Result, EnvError};

pub mod env;
pub use env::Env;

/** 
 *  Typecheck ast.
*/
pub fn typecheck_ast<'a>(e: SpanExpr<'a>) -> IResult<'a, SpanExpr<'a>, MyType> {
    let mut env = Env::new();
    env.crate_scope();
    typecheck_expr(e, &mut env)
}


/** 
 *  Typecheck expresions in ast.
*/
fn typecheck_expr<'a>(e: SpanExpr<'a>, env: &mut Env<'a>) -> IResult<'a, SpanExpr<'a>, MyType> {
    match (e.1).clone() {
        Expr::Num(_) => Ok((e, MyType::Int32)),
        Expr::Bool(_) => Ok((e, MyType::Boolean)),
        Expr::UnOp(_, _) => typecheck_unop(e, env),
        Expr::BinOp(_, _, _) => typecheck_binop(e, env),
        Expr::Let(_, _, _, _, _) => typecheck_let(e, env),
        Expr::Assign(_, _, _) => typecheck_assign(e, env),
        Expr::Var(_, _) => typecheck_var(e, env),
        Expr::If(_, _, _) => typecheck_if(e, env),
        Expr::While(_, _) => typecheck_while(e, env),
        Expr::Body(_) => typecheck_body(e, env),
        Expr::Func(_, _, _, _) => add_func_to_typechecking_list(e, env),
        Expr::FuncCall(_, _) => typecheck_func_call(e, env),
        Expr::Funcs(_) => typecheck_funcs(e, env),
        _ => panic!("typecheck_expr {:#?}", e),
    }
}


/** 
 *  Typecheck unop in ast.
*/
fn typecheck_unop<'a>(e: SpanExpr<'a>, env: &mut Env<'a>) -> IResult<'a, SpanExpr<'a>, MyType> {
    match (e.1).clone() {
        Expr::UnOp(op, expr) => {
            let e_type = typecheck_expr(*expr, env)?;
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
fn typecheck_binop<'a>(e: SpanExpr<'a>, env: &mut Env<'a>) -> IResult<'a, SpanExpr<'a>, MyType> {
    match (e.1).clone() {
        Expr::BinOp(le, op, re) => {
            let le_type = typecheck_expr(*le, env)?.1;
            let re_type = typecheck_expr(*re, env)?.1;
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
fn typecheck_let<'a>(e: SpanExpr<'a>, env: &mut Env<'a>) -> IResult<'a, SpanExpr<'a>, MyType> {
    match (e.1).clone() {
        Expr::Let(_, i, _, t, v) => {
            let vt = typecheck_expr(*v, env)?.1;
            env.store_var(i, t.clone().1);
            return Ok((e, check_if_same_type(t.1, vt)));
        },
        _ => panic!("typecheck_let"),
    }
}


/** 
 *  Typecheck assign in ast.
*/
fn typecheck_assign<'a>(e: SpanExpr<'a>, env: &mut Env<'a>) -> IResult<'a, SpanExpr<'a>, MyType> {
    match (e.1).clone() {
        Expr::Assign(_, i, v) => {
            let vt = typecheck_expr(*v, env)?.1;
            let t = env.load_var(i).unwrap();
            return Ok((e, check_if_same_type(t, vt)));
        },
        _ => panic!("typecheck_assign"),
    }
}


/** 
 *  Typecheck var in ast.
*/
fn typecheck_var<'a>(e: SpanExpr<'a>, env: &mut Env<'a>) -> IResult<'a, SpanExpr<'a>, MyType> {
    match (e.1).clone() {
        Expr::Var(_, i) => return Ok((e, env.load_var(i).unwrap())),
        _ => panic!("typecheck_var"),
    }
}


/** 
 *  Typecheck body in ast.
*/
fn typecheck_body<'a>(e: SpanExpr<'a>, env: &mut Env<'a>) -> IResult<'a, SpanExpr<'a>, MyType> {
    match (e.1).clone() {
        Expr::Body(es) => {
            let mut res = Ok((e.clone(), MyType::NoType));
            for expr in es {
                match expr.1 {
                    Expr::Return(v) => {
                        let val = typecheck_expr(*v, env)?;
                        typecheck_funcs_in_list(e.clone(), env);
                        env.pop_scope();
                        return match val.1 {
                            MyType::Int32 => Ok((val.0, MyType::ReturnType(Box::new(MyType::Int32)) )),
                            MyType::Boolean => Ok((val.0, MyType::ReturnType(Box::new(MyType::Boolean)) )),
                            MyType::NoType => Ok((val.0, MyType::ReturnType(Box::new(MyType::NoType)) )),
                            _ => Ok(val),
                        };
                    },
                    _ => {
                        res = typecheck_expr(expr, env);
                        match res.clone()?.1 {
                            MyType::ReturnType(_) => {
                                typecheck_funcs_in_list(e.clone(), env);
                                env.pop_scope();
                                return res;
                            },
                            _ => (),
                        };
                    },
                }
            }
            typecheck_funcs_in_list(e.clone(), env);
            env.pop_scope();
            return res;
        },
        _ => panic!("typecheck_body"),
    }
}


/** 
 *  Typecheck if in ast.
*/
fn typecheck_if<'a>(e: SpanExpr<'a>, env: &mut Env<'a>) -> IResult<'a, SpanExpr<'a>, MyType> {
    match (e.1).clone() {
        Expr::If(i, ib, eb) => {
            if !check_if_bool(typecheck_expr(*i, env)?.1) {
                panic!("typecheck_if if statment is not of type bool");
            } 

            env.crate_scope();
            let ib_res = typecheck_body(*ib, env);
            
            env.crate_scope();
            let eb_res = typecheck_body(*eb, env);

            match ib_res.clone()?.1 {
                MyType::ReturnType(ibt) => {
                    match eb_res?.1 {
                        MyType::ReturnType(ebt) => return Ok((e, MyType::ReturnType(Box::new(check_if_same_type(*ibt, *ebt))))),
                        _ => return ib_res,
                    };
                },
                _ => {
                    match eb_res.clone()?.1 {
                        MyType::ReturnType(_) => return eb_res,
                        _ => return Ok((e, MyType::NoType)),
                    };
                },
            };
        },
        _ => panic!("typecheck_if"),
    }
}


/** 
 *  Typecheck while in ast.
*/
fn typecheck_while<'a>(e: SpanExpr<'a>, env: &mut Env<'a>) -> IResult<'a, SpanExpr<'a>, MyType> {
    match (e.1).clone() {
        Expr::While(i, b) => {
            if !check_if_bool(typecheck_expr(*i, env)?.1) {
                panic!("typecheck_while while statment is not of type bool");
            }
            env.crate_scope();
            let res = typecheck_body(*b, env);
            return res;
        },
        _ => panic!("typecheck_while"),
    }
}


/** 
 *  Typecheck func in ast.
*/
fn add_func_to_typechecking_list<'a>(e: SpanExpr<'a>, env: &mut Env<'a>) -> IResult<'a, SpanExpr<'a>, MyType> {
    match (e.1).clone() {
        Expr::Func(ident, param, return_type, _) => {
            let mut t_param = Vec::new();
            let mut t_var = Vec::new();
            for v in param {
                match v.1 {
                    Expr::VarWithType(_, i, t) => {
                        t_param.push(t.clone().1);
                        t_var.push((i, t.clone().1));
                    }
                    _ => panic!("add_func_to_typechecking_list"),
                }
            }
            env.store_func(ident, t_param, return_type.clone().1, e.clone().1);
            return Ok((e, return_type.1));
        },
        _ => panic!("add_func_to_typechecking_list"),
    }
}


/** 
 *  Typecheck func call in ast.
*/
fn typecheck_func_call<'a>(e: SpanExpr<'a>, env: &mut Env<'a>) -> IResult<'a, SpanExpr<'a>, MyType> {
    match (e.1).clone() {
        Expr::FuncCall(i, param) => {
            let temp = env.load_func(i);
            let param_t;
            let return_t;
            match temp {
                Ok(tup) => {param_t = tup.0; return_t = tup.1;},
                _ => panic!("typecheck_func_call"),
            };
            if param_t.len() != param.len() {
                panic!("typecheck_func_call");
            }
            let mut i = 0;
            for t in param_t {
                check_if_same_type(t, typecheck_expr(param[i].clone(), env)?.1);
                i = i + 1;
            }
            return Ok((e, return_t));
        },
        _ => panic!("typecheck_func_call"),
    }
}


/** 
 *  Typecheck funcs in ast.
*/
fn typecheck_funcs<'a>(e: SpanExpr<'a>, env: &mut Env<'a>) -> IResult<'a, SpanExpr<'a>, MyType> {
    match (e.1).clone() {
        Expr::Funcs(es) => {
            for expr in es {
                add_func_to_typechecking_list(expr, env);
            }

            typecheck_funcs_in_list(e.clone(), env);
            env.pop_scope();
            return Ok((e, MyType::NoType));
        },
        _ => panic!("typecheck_funcs"),
    }
}


/** 
 *  Adds to list of func that need typechecking in ast.
*/
fn typecheck_funcs_in_list<'a>(expr: SpanExpr<'a>, env: &mut Env<'a>) -> IResult<'a, SpanExpr<'a>, MyType> {
    let mut res = MyType::NoType;
    while env.get_funcs_len() > 0 {
        let e;
        match env.get_func() {
            Some(expr) => e = expr,
            _ => panic!("typecheck_funcs_in_list"),
        }
        match e.clone() {
            Expr::Func(ident, param, return_type, body) => {
                env.crate_scope();
                for v in param {
                    match v.1 {
                        Expr::VarWithType(_, i, t) => {
                            env.store_var(i, t.clone().1);
                        }
                        _ => panic!("typecheck_funcs_in_list"),
                    }
                }
                let mut body_t = typecheck_body(*body, env)?.1;
                match body_t {
                    MyType::ReturnType(t) => body_t = *t,
                    _ => body_t = MyType::NoType,
                }
                res = check_if_same_type(return_type.1, body_t);
            },
            _ => panic!("typecheck_funcs_in_list"),
        }
    }
    return Ok((expr, res));
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

fn check_if_notype(t: MyType) -> bool {
    match t {
        MyType::NoType => true,
        _ => false,
    }
}


fn check_if_same_type(lt: MyType, rt: MyType) -> MyType {
    if check_if_bool(lt.clone()) && check_if_bool(rt.clone()) {
        return MyType::Boolean;
    } else if check_if_num(lt.clone()) && check_if_num(rt.clone()) {
        return MyType::Int32;
    } else if check_if_notype(lt.clone()) && check_if_notype(rt.clone()) {
        return MyType::Int32;
    }
    panic!("check_if_same_type ({:?} != {:?})", lt, rt)
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
        MyType::ReturnType(_) => panic!("ReturnType not implemented check_if_same_type_and_type"),
    }
}
