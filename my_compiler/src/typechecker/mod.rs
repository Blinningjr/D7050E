#![allow(dead_code)]

/**
 *  Imports from parser.
 */
#[path = "../parser/mod.rs"]
mod parser;
use crate::parser::{
    SpanExpr,
    // SpanOp,
    expr::Expr,
    op::Op,
    mytype::MyType,
    IResult,
};

#[path = "../interpreter/enverror.rs"]
pub mod enverror;
// use enverror::{Result, EnvError};

pub mod env;
pub use env::Env;

pub mod errormessage;
pub use errormessage::ErrorMessage;

/** 
 *  Typecheck ast.
*/
pub fn typecheck_ast<'a>(e: SpanExpr<'a>) -> IResult<'a, SpanExpr<'a>, MyType> {
    let mut env = Env::new();
    env.crate_scope();
    let res = typecheck_expr(e, &mut env);
    
    if env.print_errormessages() {
        panic!("");
    } else {
        return res;
    }
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
        Expr::Let(_, _) => typecheck_let(e, env),
        Expr::Assign(_, _) => typecheck_assign(e, env),
        Expr::Var(_) => typecheck_var(e, env),
        Expr::If(_, _, _) => typecheck_if(e, env),
        Expr::While(_, _) => typecheck_while(e, env),
        Expr::Body(_) => typecheck_body(e, env),
        Expr::Func(_, _, _, _) => add_func_to_typechecking_list(e, env),
        Expr::FuncCall(_, _) => typecheck_func_call(e, env),
        Expr::Funcs(_) => typecheck_funcs(e, env),
        Expr::Prefixed(_, _) => typecheck_prefixed(e, env),
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
                    let start = (op.clone().0).offset;
                    return Ok((e.clone(), check_if_same_type(e.clone(), start, env, e_type.1, MyType::Boolean)));
                },
                Op::Sub => {
                    let start = (op.clone().0).offset;
                    return Ok((e.clone(), check_if_same_type(e.clone(), start, env, e_type.1, MyType::Int32)));
                },
                _ => panic!("typecheck_unop"),
            };
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
            let le_type = typecheck_expr(*le.clone(), env)?.1;
            let re_type = typecheck_expr(*re, env)?.1;
            let start = (le.clone().0).offset;
            match op.1 {
            Op::Add => Ok((e.clone(), check_if_same_type_and_type(e.clone(), start, env, le_type, re_type, MyType::Int32))),
            Op::Sub => Ok((e.clone(), check_if_same_type_and_type(e.clone(), start, env, le_type, re_type, MyType::Int32))),
            Op::Div => Ok((e.clone(), check_if_same_type_and_type(e.clone(), start, env, le_type, re_type, MyType::Int32))),
            Op::Multi => Ok((e.clone(), check_if_same_type_and_type(e.clone(), start, env, le_type, re_type, MyType::Int32))),
            Op::Mod => Ok((e.clone(), check_if_same_type_and_type(e.clone(), start, env, le_type, re_type, MyType::Int32))),
            Op::And => Ok((e.clone(), check_if_same_type_and_type(e.clone(), start, env, le_type, re_type, MyType::Boolean))),
            Op::Or => Ok((e.clone(), check_if_same_type_and_type(e.clone(), start, env, le_type, re_type, MyType::Boolean))),
            Op::NotEq => {check_if_same_type(e.clone(), start, env, le_type, re_type); return Ok((e, MyType::Boolean))},
            Op::Equal => {check_if_same_type(e.clone(), start, env, le_type, re_type); return Ok((e, MyType::Boolean))},
            Op::LessEqThen => {check_if_same_type_and_type(e.clone(), start, env, le_type, re_type, MyType::Int32); return Ok((e, MyType::Boolean))},
            Op::LargEqThen => {check_if_same_type_and_type(e.clone(), start, env, le_type, re_type, MyType::Int32); return Ok((e, MyType::Boolean))},
            Op::LessThen => {check_if_same_type_and_type(e.clone(), start, env, le_type, re_type, MyType::Int32); return Ok((e, MyType::Boolean))},
            Op::LargThen => {check_if_same_type_and_type(e.clone(), start, env, le_type, re_type, MyType::Int32); return Ok((e, MyType::Boolean))},
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
        Expr::Let(i, v) => {
            let vt = typecheck_expr(*v, env)?.1;
            let ident;
            let ty;
            match (*i.clone()).1 {
                Expr::VarWithType(var, typ) => {
                    ty = typ;
                    match (*var).1 {
                        Expr::Var(id) => ident = id,
                        _ => panic!("typecheck_let"),
                    };
                },
                Expr::Prefixed(_, varwt) => {
                    match (*varwt).1 {
                        Expr::VarWithType(var, typ) => {
                            ty = typ;
                            match (*var).1 {
                                Expr::Var(id) => ident = id,
                                _ => panic!("typecheck_let"),
                            };
                        },
                        _ => panic!("typecheck_let"),
                    };
                },
                _ => panic!("typecheck_let"),
            };

            let t;
            match (*ty).1 {
                Expr::Type(tt) => t = tt,
                Expr::Prefixed(_, ttt) => {
                    match (*ttt).1 {
                        Expr::Type(tt) => t = tt,
                        _ => panic!("typecheck_let"),
                    };
                },
                _ => panic!("typecheck_let"),
            };

            let tr = env.store_var(ident, t.clone().1);
            match tr {
                Ok(_) => (),
                Err(_) => {
                    let start = (i.clone().0).offset - 3;
                    env.add_errormessage(ErrorMessage{message: "Var already declared".to_string(), context: e.clone(), start: start,});
                },
            };
            let start = (i.clone().0).offset - 3;
            return Ok((e.clone(), check_if_same_type(e.clone(), start, env, t.1, vt)));
        },
        _ => panic!("typecheck_let"),
    }
}


/** 
 *  Typecheck assign in ast.
*/
fn typecheck_assign<'a>(e: SpanExpr<'a>, env: &mut Env<'a>) -> IResult<'a, SpanExpr<'a>, MyType> {
    match (e.1).clone() {
        Expr::Assign(var, v) => {
            let vt = typecheck_expr(*v, env)?.1;
            let ident;

            match (*var.clone()).1 {
                Expr::Var(i) => ident = i,
                Expr::Prefixed(_, va) => {
                    match (*va).1 {
                        Expr::Var(i) => ident = i,
                        _ => panic!("typecheck_assign"),
                    };
                },
                _ => panic!("typecheck_assign"),
            };

            let t = env.load_var(ident).unwrap();
            let start = (var.clone().0).offset;
            return Ok((e.clone(), check_if_same_type(e.clone(), start, env, t, vt)));
        },
        _ => panic!("typecheck_assign"),
    }
}


/** 
 *  Typecheck var in ast.
*/
fn typecheck_var<'a>(e: SpanExpr<'a>, env: &mut Env<'a>) -> IResult<'a, SpanExpr<'a>, MyType> {
    match (e.1).clone() {
        Expr::Var(i) => return Ok((e, env.load_var(i).unwrap())),
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
                        let _t1 = typecheck_funcs_in_list(e.clone(), env);
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
                                let _t2 = typecheck_funcs_in_list(e.clone(), env);
                                env.pop_scope();
                                return res;
                            },
                            _ => (),
                        };
                    },
                }
            }
            let _t3 = typecheck_funcs_in_list(e.clone(), env);
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
            if !check_if_bool(typecheck_expr(*i.clone(), env)?.1) {
                let start = (i.clone().0).offset - 2;
                env.add_errormessage(ErrorMessage{message: "Type missmatch, if needs to be of type bool".to_string(), context: e.clone(), start: start,})
            } 

            env.crate_scope();
            let ib_res = typecheck_body(*ib, env);
            
            env.crate_scope();
            let eb_res = typecheck_body(*eb, env);

            match ib_res.clone()?.1 {
                MyType::ReturnType(ibt) => {
                    match eb_res?.1 {
                        MyType::ReturnType(ebt) => {
                            let start = (i.clone().0).offset - 2;
                            return Ok((e.clone(), MyType::ReturnType(Box::new(check_if_same_type(e.clone(), start, env, *ibt, *ebt)))));
                        },
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
            if !check_if_bool(typecheck_expr(*i.clone(), env)?.1) {
                let start = (i.clone().0).offset - 5;
                env.add_errormessage(ErrorMessage{message: "Type missmatch, while needs to be of type bool".to_string(), context: e, start: start,})
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
                    Expr::VarWithType(var, typ) => {
                        let id;
                        let ty;
                        match (*var).1 {
                            Expr::Var(idd) => id = idd,
                            _ => panic!("add_func_to_typechecking_list"),
                        };
                        match (*typ).1 {
                            Expr::Type(t) => ty = t,
                            Expr::Prefixed(_, tt) => {
                                match (*tt).1 {
                                    Expr::Type(t) => ty = t,
                                    _ => panic!("add_func_to_typechecking_list"),
                                };
                            },
                            _ => panic!("add_func_to_typechecking_list"),
                        };
                        t_param.push(ty.clone().1);
                        t_var.push((id, ty.clone().1));
                    }
                    _ => panic!("add_func_to_typechecking_list"),
                }
            }
            let id;
            let rt;
            match (*ident).1 {
                Expr::Var(i) => id = i,
                _ => panic!("add_func_to_typechecking_list"),
            };
            match (*return_type).1 {
                Expr::Type(t) => rt = t,
                Expr::Prefixed(_, tt) => {
                    match (*tt).1 {
                        Expr::Type(t) => rt = t,
                        _ => panic!("add_func_to_typechecking_list"),
                    };
                },
                _ => panic!("add_func_to_typechecking_list"),
            };

            let tr = env.store_func(id, t_param, rt.clone().1, e.clone().1);
            match tr {
                Ok(_) => (),
                Err(_) => {
                    let start = (ident.0).offset - 3;
                    env.add_errormessage(ErrorMessage{message: "Func already declared".to_string(), context: e.clone(), start: start,});
                },
            };
            return Ok((e, rt.1));
        },
        _ => panic!("add_func_to_typechecking_list"),
    }
}


/** 
 *  Typecheck func call in ast.
*/
fn typecheck_func_call<'a>(e: SpanExpr<'a>, env: &mut Env<'a>) -> IResult<'a, SpanExpr<'a>, MyType> {
    match (e.1).clone() {
        Expr::FuncCall(var, param) => {
            let f_id;
            match (*var).1 {
                Expr::Var(i) => f_id = i,
                _ => panic!("add_func_to_typechecking_list"),
            };
            let temp = env.load_func(f_id);
            let param_t;
            let return_t;
            match temp {
                Ok(tup) => {param_t = tup.0; return_t = tup.1;},
                _ => {
                    let start = (var.0).offset;
                    env.add_errormessage(ErrorMessage{message: "No function with that name".to_string(), context: e.clone(), start: start});
                    return Ok((e, MyType::NoType));
                },
            };
            if param_t.len() != param.len() {
                let start = (var.0).offset;
                env.add_errormessage(ErrorMessage{message: "Parameters are not of equal length".to_string(), context: e.clone(), start: start});
            }
            let mut i = 0;
            for t in param_t {
                let tcheck_p = typecheck_expr(param[i].clone(), env)?.1;
                let start = (var.clone().0).offset;
                check_if_same_type(e.clone(), start, env, t, tcheck_p);
                i = i + 1;
                if i >= param.len() {
                    break;
                }
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
                let _t1 = add_func_to_typechecking_list(expr, env);
            }

            let _t2 = typecheck_funcs_in_list(e.clone(), env);
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
            Expr::Func(identt, param, return_type, body) => {
                env.crate_scope();
                for v in param {
                    match v.1 {
                        Expr::VarWithType(i, t) => {
                            let ident;
                            let typ;
                            match (*i).1 {
                                Expr::Var(id) => ident = id,
                                _ => panic!("add_func_to_typechecking_list"),
                            };
                            match (*t).1 {
                                Expr::Type(ty) => typ =ty,
                                Expr::Prefixed(_, vv) => {
                                    match (*vv).1 {
                                        Expr::Type(ty) => typ =ty,
                                        _ => panic!("add_func_to_typechecking_list"),
                                    };
                                },
                                _ => panic!("typecheck_funcs_in_list"),
                            };
                            let tr = env.store_var(ident, typ.clone().1);
                            match tr {
                                Ok(_) => (),
                                Err(_) => {
                                    let start = (identt.clone().0).offset - 3;
                                    env.add_errormessage(ErrorMessage{message: "Var already declared".to_string(), context: expr.clone(), start: start,});
                                },
                            };
                        }
                        _ => panic!("typecheck_funcs_in_list"),
                    }
                }
                let mut body_t = typecheck_body(*body, env)?.1;
                match body_t {
                    MyType::ReturnType(t) => body_t = *t,
                    _ => body_t = MyType::NoType,
                }
                let rt;
                match (*return_type).1 {
                    Expr::Type(t) => rt = t.1,
                    Expr::Prefixed(_, tt) => {
                        match (*tt).1 {
                            Expr::Type(t) => rt = t.1,
                            _ => panic!("typecheck_funcs_in_list"),
                        };
                    },
                    _ => panic!("typecheck_funcs_in_list"),
                };
                let start = (identt.clone().0).offset - 3;
                res = check_if_same_type(expr.clone(), start, env, rt, body_t);
            },
            _ => panic!("typecheck_funcs_in_list"),
        }
    }
    return Ok((expr, res));
}


/** 
 *  Typecheck prefixed in ast.
*/
fn typecheck_prefixed<'a>(e: SpanExpr<'a>, env: &mut Env<'a>) -> IResult<'a, SpanExpr<'a>, MyType> {
    match (e.1).clone() {
        Expr::Prefixed(_, var) => {
            return typecheck_expr(*var, env);
        },
        _ => panic!("typecheck_prefixed"),
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

fn check_if_notype(t: MyType) -> bool {
    match t {
        MyType::NoType => true,
        _ => false,
    }
}


fn check_if_same_type<'a>(e: SpanExpr<'a>, start: usize, env: &mut Env<'a>, lt: MyType, rt: MyType) -> MyType {
    if check_if_bool(lt.clone()) && check_if_bool(rt.clone()) {
        return MyType::Boolean;
    } else if check_if_num(lt.clone()) && check_if_num(rt.clone()) {
        return MyType::Int32;
    } else if check_if_notype(lt.clone()) && check_if_notype(rt.clone()) {
        return MyType::Int32;
    }
    env.add_errormessage(ErrorMessage{message: "Type missmatch".to_string(), context: e, start: start});
    return lt;
}


fn check_if_same_type_and_type<'a>(e: SpanExpr<'a>, start: usize, env: &mut Env<'a>, lt: MyType, rt: MyType, wanted: MyType) -> MyType {
    match wanted {
        MyType::Int32 => {
            if check_if_num(lt.clone()) {
                return check_if_same_type(e, start, env, lt,rt);
            }
        },
        MyType::Boolean => {
            if check_if_bool(lt.clone()) {
                return check_if_same_type(e, start, env, lt,rt);
            }
        },
        MyType::NoType => {
            if check_if_notype(lt.clone()) {
                return check_if_same_type(e, start, env, lt,rt);
            }
        },
        MyType::ReturnType(_) => panic!("ReturnType not implemented check_if_same_type_and_type"),
    };
    env.add_errormessage(ErrorMessage{message: "Type missmatch".to_string(), context: e, start: start,});
    return wanted;
}
