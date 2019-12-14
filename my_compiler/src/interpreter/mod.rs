#![allow(dead_code)]

pub mod enverror;
use enverror::{Result}; //EnvError};

pub mod val;
pub use val::Val;

pub mod env;
pub use env::Env;


/**
 *  Imports from parser.
 */
#[path = "../parser/mod.rs"]
mod parser;
use crate::parser::{
    SpanExpr,
    expr::Expr,
    op::Op,
};

use crate::parser::varprefix::Prefix;

pub type SpanVal<'a> = (SpanExpr<'a>, Val);

#[path = "../typechecker/errormessage.rs"]
pub mod errormessage;
pub use errormessage::ErrorMessage;


/** 
 *  Get i32 value from Val.
*/
fn get_int<'a>(v: Val, env: &mut Env<'a>) -> Result<i32> {
    match v {
        Val::Num(i) => Ok(i),
        Val::Ident(k, p) => {
            let val = env.get_var_value(&k, p)?;
            return get_int(val, env);
        },
        _ => panic!("Error: Value {:?} should be of type i32.", v),
    }
}


/** 
 *  Get bool value from Val.
*/
fn get_bool<'a>(v: Val, env: &mut Env<'a>) -> Result<bool> {
    match v {
        Val::Bool(b) => Ok(b),
        Val::Ident(k, p) => {
            let val = env.get_var_value(&k, p)?;
            return get_bool(val, env);
        },
        _ => panic!("Error: Value {:?} should be of type bool.", v),
    }
}


/** 
 *  Interprets a ast.
*/
pub fn interp_ast<'a>(e: SpanExpr<'a>) -> Result<SpanVal<'a>> {
    let mut env = Env::new();
    env.crate_scope();
    // env.store_var(&"a", Val::Num(5), Prefix::Mut);
    let res = interp_expr(e, &mut env);
    // println!("{:#?}", res);
    // println!("{:#?}", env);
    res
}


/** 
 *  Interprets expresions in ast.
*/
fn interp_expr<'a>(e: SpanExpr<'a>, env: &mut Env<'a>) -> Result<SpanVal<'a>> {
    match (e.1).clone() {
        Expr::Num(i) => Ok((e, Val::Num(i))),
        Expr::Bool(i) => Ok((e, Val::Bool(i))),
        Expr::UnOp(_, _) => interp_unop(e, env),
        Expr::BinOp(_, _, _) => interp_binop(e, env),
        Expr::Let(_, _) => interp_let(e, env),
        Expr::Assign(_, _) => interp_assign(e, env),
        Expr::Var(_) => interp_var(e, env, Prefix::None),
        Expr::If(_, _, _) => interp_if(e, env),
        Expr::While(_, _) => interp_while(e, env),
        Expr::FuncCall(_, _) => interp_func_call(e, env),
        Expr::Func(v, _, _, _) => {env.store_func(match (*v).1 {Expr::Var(i) => i, _ => panic!("interp_func"),}, e.1.clone()); Ok((e, Val::Empty))},
        Expr::Funcs(_) => interp_funcs(e, env),
        Expr::Body(_) => interp_body(e, env),
        Expr::Prefixed(_, _) => interp_prefixed(e, env),
        _ => panic!("interp_expr"),
    }
}


/** 
 *  Interprets var expresion.
*/
fn interp_var<'a>(e: SpanExpr<'a>, env: &mut Env<'a>, p: Prefix) -> Result<SpanVal<'a>> {
    match (e.1).clone() {
        Expr::Var(s) => {
            match p {
                Prefix::Borrow => Ok((e, Val::Ident(s.to_string(), env.get_var_scope(s, env.get_current_scope_pos())?))),
                Prefix::BorrowMut => Ok((e, Val::Ident(s.to_string(),  env.get_var_scope(s, env.get_current_scope_pos())?))),
                Prefix::DeRef(n) => {
                    let t = env.load_var(s, n);
                    if t.is_err() {
                        let start = ((e.clone()).0).offset; 
                        env.error_panic(ErrorMessage{message: "Invalid dereference".to_string(), context: e.clone(), start: start,});
                    }
                    return Ok((e, t?));
                },
                _ => {
                    let t = env.load_var(s, 0);
                    if t.is_err() {
                        let start = ((e.clone()).0).offset; 
                        env.error_panic(ErrorMessage{message: "Var not in scope".to_string(), context: e.clone(), start: start,});
                    }
                    return Ok((e, t?));
                },
            }
        },
        _ => panic!("interp_expr"),
    }
}


/** 
 *  Interprets unary operations in ast.
*/
fn interp_unop<'a>(e: SpanExpr<'a>, env: &mut Env<'a>) -> Result<SpanVal<'a>> {
    match (e.1).clone() {
        Expr::UnOp(op, rv) => {
            match op.1 {
                Op::Sub => {
                    let res = interp_expr(*rv.clone(), env)?;
                    match res.1 {
                        Val::Num(i) => Ok((e, Val::Num(-i))),
                        _ => {
                            let start = ((op.clone()).0).offset; 
                            env.error_panic(ErrorMessage{message: "Unop Sub only works with type i32".to_string(), context: e.clone(), start: start,});
                            panic!("");
                        },
                    }
                }
                Op::Not => {
                    let res = interp_expr(*rv.clone(), env)?;
                    match res.1 {
                        Val::Bool(b) => Ok((e, Val::Bool(!b))),
                        _ => {
                            let start = ((op.clone()).0).offset; 
                            env.error_panic(ErrorMessage{message: "Unop Not only works with ype bool".to_string(), context: e.clone(), start: start,});
                            panic!("");
                        },
                    }
                }
                _ => {
                    let start = ((e.clone()).0).offset; 
                    env.error_panic(ErrorMessage{message: "Unop not supported".to_string(), context: e.clone(), start: start,});
                    panic!("");
                },
            }
        },
        _ => panic!("interp_unop"),
    }
}


/** 
 *  Interprets binary operations in ast.
*/
fn interp_binop<'a>(e: SpanExpr<'a>, env: &mut Env<'a>) -> Result<SpanVal<'a>> {
    match (e.1).clone() {
        Expr::BinOp(lv, op, rv) => {
            let lr = interp_expr(*lv, env)?.1;
            let rr = interp_expr(*rv, env)?.1;
            match op.1 {
                Op::Add => Ok((e, Val::Num(
                    get_int(lr, env)?
                    +
                    get_int(rr, env)?
                ))),
                Op::Sub => Ok((e, Val::Num(
                    get_int(lr, env)?
                    -
                    get_int(rr, env)?
                ))),
                Op::Div => Ok((e, Val::Num(
                    get_int(lr, env)?
                    /
                    get_int(rr, env)?
                ))),
                Op::Multi => Ok((e, Val::Num(
                    get_int(lr, env)?
                    *
                    get_int(rr, env)?
                ))),
                Op::Mod => Ok((e, Val::Num(
                    get_int(lr, env)?
                    %
                    get_int(rr, env)?
                ))),
                Op::LessEqThen => Ok((e, Val::Bool(
                    get_int(lr, env)?
                    <=
                    get_int(rr, env)?
                ))),
                Op::LargEqThen => Ok((e, Val::Bool(
                    get_int(lr, env)?
                    >=
                    get_int(rr, env)?
                ))),
                Op::LessThen => Ok((e, Val::Bool(
                    get_int(lr, env)?
                    <
                    get_int(rr, env)?
                ))),
                Op::LargThen => Ok((e, Val::Bool(
                    get_int(lr, env)?
                    >
                    get_int(rr, env)?
                ))),
                Op::Equal => Ok((e.clone(), Val::Bool(
                    match lr {
                        Val::Bool(b) => b == get_bool(rr, env)?,
                        Val::Num(v) => v == get_int(rr, env)?,
                        _ => {
                            let start = ((op.clone()).0).offset; 
                            env.error_panic(ErrorMessage{message: "The type is not supported by binop Equal".to_string(), context: e.clone(), start: start,});
                            panic!("");
                        },
                    }
                ))),
                Op::And => Ok((e, Val::Bool(
                    get_bool(lr, env)?
                    &&
                    get_bool(rr, env)?
                ))),
                Op::Or => Ok((e, Val::Bool(
                    get_bool(lr, env)?
                    ||
                    get_bool(rr, env)?
                ))),
                Op::NotEq => Ok((e.clone(), Val::Bool(
                    match lr {
                        Val::Bool(b) => b != get_bool(rr, env)?,
                        Val::Num(v) => v != get_int(rr, env)?,
                        _ => {
                            let start = ((op.clone()).0).offset; 
                            env.error_panic(ErrorMessage{message: "The type is not supported by binop NotEq".to_string(), context: e.clone(), start: start,});
                            panic!("");
                        },
                    }
                ))),
                _ => {
                    let start = ((op.clone()).0).offset; 
                    env.error_panic(ErrorMessage{message: "Op is not supported".to_string(), context: e.clone(), start: start,});
                    panic!("");
                },
            }
        },
        _ => panic!("interp_binop"),
    }
}


/** 
 *  Interprets let in ast.
*/
fn interp_let<'a>(e: SpanExpr<'a>, env: &mut Env<'a>) -> Result<SpanVal<'a>> {
    match (e.1).clone() {
        Expr::Let(s, value) => {
            let val = interp_expr(*value, env)?;
            let ident;
            let mut prefix = Prefix::None;
            match (*s).1 {
                Expr::Prefixed(p, v) => {
                    match p.1 {
                        Prefix::Mut => prefix = Prefix::Mut,
                        Prefix::None => (),
                        _ => {
                            let start = ((e.clone()).0).offset; 
                            env.error_panic(ErrorMessage{message: "That prefix is not allowed after let".to_string(), context: e.clone(), start: start,});
                            panic!("");
                        },
                    };
                    match (*v).1 {
                        Expr::VarWithType(var, _) => { 
                            match (*var).1 {
                                Expr::Var(i) => ident = i,
                                _ => {
                                    let start = ((e.clone()).0).offset; 
                                    env.error_panic(ErrorMessage{message: "Expected var".to_string(), context: e.clone(), start: start,});
                                    panic!("");
                                },
                            };
                        },
                        _ => {
                            let start = ((e.clone()).0).offset; 
                            env.error_panic(ErrorMessage{message: "Expected var with type".to_string(), context: e.clone(), start: start,});
                            panic!("");
                        },
                    };
                },
                Expr::VarWithType(v, _) => { 
                    match (*v).1 {
                        Expr::Var(i) => ident = i,
                        _ => {
                            let start = ((e.clone()).0).offset; 
                            env.error_panic(ErrorMessage{message: "Expected var".to_string(), context: e.clone(), start: start,});
                            panic!("");
                        },
                    };
                },
                _ => {
                    let start = ((e.clone()).0).offset; 
                    env.error_panic(ErrorMessage{message: "can't find var".to_string(), context: e.clone(), start: start,});
                    panic!("");
                },
            };
            env.store_var(ident, (val.1).clone(), prefix);
            return Ok(val);
        },
        _ => panic!("interp_let"),
    }
}


/** 
 *  Interprets assignments in ast.
*/
fn interp_assign<'a>(e: SpanExpr<'a>, env: &mut Env<'a>) -> Result<SpanVal<'a>> {
    match (e.1).clone() {
        Expr::Assign(s, value) => {
            let val = interp_expr(*value, env)?;
            match (*s).1 {
                Expr::Prefixed(p, v) => {
                    match (*v).1 {
                        Expr::Var(i) => {
                            match p.1 {
                                Prefix::DeRef(n) => env.assign_var(i, (val.1).clone(), n),
                                _ => env.assign_var(i, (val.1).clone(), 0),
                            };
                        },
                        _ => {
                            let start = ((e.clone()).0).offset; 
                            env.error_panic(ErrorMessage{message: "Expected var".to_string(), context: e.clone(), start: start,});
                        },
                    };
                },
                Expr::Var(i) => {
                    env.assign_var(i, (val.1).clone(), 0);
                },
                _ => {
                    let start = ((e.clone()).0).offset; 
                    env.error_panic(ErrorMessage{message: "Expected var".to_string(), context: e.clone(), start: start,});
                },
            };
            return Ok(val);
        },
        _ => panic!("interp_assign"),
    }
}


/** 
 *  Interprets if in ast.
*/
fn interp_if<'a>(e: SpanExpr<'a>, env: &mut Env<'a>) -> Result<SpanVal<'a>> {
    match (e.1).clone() {
        Expr::If(b, ib, eb) => {
            env.crate_scope();
            let res;
            if get_bool(interp_expr(*b, env)?.1, env)? {
                match ib.1.clone() {
                    Expr::Body(_) => res = interp_body(*ib, env),
                    _ => {
                        let start = ((ib.clone()).0).offset; 
                        env.error_panic(ErrorMessage{message: "Expected body".to_string(), context: e.clone(), start: start,});
                        panic!("");
                    },
                }
            } else {
                match eb.1.clone() {
                    Expr::Body(_) => res = interp_body(*eb, env),
                    _ => {
                        let start = ((eb.clone()).0).offset; 
                        env.error_panic(ErrorMessage{message: "Expected body".to_string(), context: e.clone(), start: start,});
                        panic!("");
                    },
                }
            }
            env.pop_scope();
            return res;
        },
        _ => panic!("interp_if"),
    }
}


/** 
 *  Interprets body in ast.
*/
fn interp_body<'a>(e: SpanExpr<'a>, env: &mut Env<'a>) -> Result<SpanVal<'a>> {
    match (e.1).clone() {
        Expr::Body(es) => {
            for e in es {
                match e.1 {
                    Expr::Return(v) => {
                        let val = interp_expr(*v, env)?;
                        return match val.1 {
                            Val::Num(v) => Ok((val.0, Val::ReturnNum(v))),
                            Val::Bool(b) => Ok((val.0, Val::ReturnBool(b))),
                            Val::Empty => Ok((val.0, Val::ReturnEmpty)),
                            _ => Ok(val),
                        };
                    },
                    _ => {
                        let res = interp_expr(e.clone(), env);
                        match res.clone()?.1 {
                            Val::ReturnBool(_) => return res,
                            Val::ReturnNum(_) => return res,
                            Val::ReturnEmpty => return res,
                            _ => (),
                        };
                    },
                }
            }
            return Ok((e, Val::Empty));
        },
        _ => panic!("interp_body"),
    }
}


/** 
 *  Interprets while in ast.
*/
fn interp_while<'a>(e: SpanExpr<'a>, env: &mut Env<'a>) -> Result<SpanVal<'a>> {
    match (e.1).clone() {
        Expr::While(expr, b) => {
            let mut res = Ok((e.clone(), Val::Empty));
            let mut w = get_bool(interp_expr(*expr.clone(), env)?.1, env)?;
            while w {
                env.crate_scope();
                res = interp_body( *b.clone(), env);
                w = get_bool(interp_expr(*expr.clone(), env)?.1, env)?;
                env.pop_scope();
            }
            return res;
        },
        _ => panic!("interp_while"),
    }
}


/** 
 *  Interprets function calls in ast.
*/
fn interp_func_call<'a>(e: SpanExpr<'a>, env: &mut Env<'a>) -> Result<SpanVal<'a>> {
    match (e.1).clone() {
        Expr::FuncCall(i,p) => {
            let mut v = Vec::new();
            for val in p {
                v.push(interp_expr(val, env)?.1);
            }
            let expr = env.load_func(match (*i).1 {
                Expr::Var(ident) => ident, 
                _ => {
                    let start = ((i.clone()).0).offset; 
                    env.error_panic(ErrorMessage{message: "Expected var".to_string(), context: e.clone(), start: start,});
                    panic!("");
                },
            })?;
            match expr {
                Expr::Func(_, _, _, _) => {
                    let res = interp_func(expr, v, env)?;
                    match res.1 {
                        Val::ReturnBool(b) => Ok((res.0, Val::Bool(b))),
                        Val::ReturnNum(v) => Ok((res.0, Val::Num(v))),
                        Val::ReturnEmpty => Ok((res.0, Val::Empty)),
                        _ => Ok(res),
                    }
                },
                _ => {
                    let start = ((e.clone()).0).offset; 
                    env.error_panic(ErrorMessage{message: "Can't load function".to_string(), context: e.clone(), start: start,});
                    panic!("");
                },
            }
        },
        _ => panic!("interp_func_call"),
    }
}


/** 
 *  Interprets function in ast.
*/
fn interp_func<'a>(e: Expr<'a>, pv: Vec<Val>, env: &mut Env<'a>) -> Result<SpanVal<'a>> {
    match (e).clone() {
        Expr::Func(_i, p, _t, b) => {
            env.crate_scope();
            let mut j = 0;
            for p_var in p.clone() { 
                match p_var.clone().1 {
                    Expr::VarWithType(i, _type) => {
                        env.store_var( match (*i).1 { 
                            Expr::Var(ident) => ident, 
                            _ => {
                                let start = ((i.clone()).0).offset; 
                                env.error_panic(ErrorMessage{message: "Expected var".to_string(), context: *i.clone(), start: start,});
                                panic!("");
                            },
                        }, 
                        pv[j].clone(), 
                        Prefix::None); 
                        ()
                    },
                    Expr::Prefixed(p, var) => {
                        match (*var).1 {
                            Expr::VarWithType(i, _type) => {
                                env.store_var( match (*i).1 { 
                                    Expr::Var(ident) => ident, 
                                    _ => {
                                        let start = ((i.clone()).0).offset; 
                                        env.error_panic(ErrorMessage{message: "Expected var".to_string(), context: *i.clone(), start: start,});
                                        panic!("");
                                    },
                                }, 
                                pv[j].clone(), 
                                Prefix::None); 
                                ()
                            },
                            Expr::Var(ident) => {env.store_var(ident, pv[j].clone(), p.1); ()},
                            _ => {
                                let start = ((var.clone()).0).offset; 
                                env.error_panic(ErrorMessage{message: "Expected var".to_string(), context: p_var.clone(), start: start,});
                                panic!("");
                            },
                        };
                    },
                    _ => {
                        let start = ((p_var.clone()).0).offset; 
                        env.error_panic(ErrorMessage{message: "Expected var with type".to_string(), context: p_var.clone(), start: start,});
                        panic!("");
                    },
                };
                j += 1;
            }
            let res;
            match b.1.clone() {
                Expr::Body(_) => res = interp_body(*b, env),
                _ => {
                    let start = ((b.clone()).0).offset; 
                    env.error_panic(ErrorMessage{message: "Expected body".to_string(), context: *b.clone(), start: start,});
                    panic!("");
                },
            }
            env.pop_scope();
            return res;
        },
        _ => panic!("interp_func"),
    }
}


/** 
 *  Interprets function in ast and store them in env.
*/
fn interp_funcs<'a>(e: SpanExpr<'a>, env: &mut Env<'a>) -> Result<SpanVal<'a>> {
    match (e.1).clone() {
        Expr::Funcs(funcs) => {
            for func in funcs {
                match (func.1).clone() {
                    Expr::Func(v, _, _, _) => {
                        env.store_func(match (*v).1 {
                            Expr::Var(i) => i, 
                            _ => {
                                let start = ((v.clone()).0).offset; 
                                env.error_panic(ErrorMessage{message: "Expected var".to_string(), context: *v.clone(), start: start,});
                                panic!("");
                            },
                        }, 
                        func.1); 
                        ()
                    },
                    _ => (),
                };
            }
            
            let expr = env.load_func(&"main")?;
            match expr.clone() {
                Expr::Func(_, _, _, _) => interp_func(expr, Vec::new(), env),
                _ =>  {
                    panic!("could not load function main");
                },
            }
        },
        _ => panic!("interp_funcs"),
    }
}


/** 
 *  Interprets prefixed expresion.
*/
fn interp_prefixed<'a>(e: SpanExpr<'a>, env: &mut Env<'a>) -> Result<SpanVal<'a>> {
    match (e.1).clone() {
        Expr::Prefixed(p, expr) => {
            match expr.1 {
                Expr::Var(_) => {
                    return interp_var(*expr, env, p.1);
                },
                Expr::Bool(b) => return Ok((e.clone(), Val::BorrowPrimitive(-1, Box::new(Val::Bool(b))))),
                Expr::Num(n) => return Ok((e.clone(), Val::BorrowPrimitive(-1, Box::new(Val::Num(n))))),
                _ => return interp_expr(*expr, env),
            }
        },
        _ => panic!("interp_prefixed"),
    }
}
