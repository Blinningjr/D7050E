#![allow(dead_code)]

pub mod varinfo;
pub use varinfo::{
    ValueInfo,
    VarInfo,
    BorrowInfo,
};

/**
 *  Imports from parser.
 */
#[path = "../parser/mod.rs"]
mod parser;
use crate::parser::{
    SpanExpr,
    expr::Expr,
    op::Op,
    IResult,
    varprefix::Prefix,
};

#[path = "../interpreter/enverror.rs"]
pub mod enverror;
use enverror::{Result, EnvError};

pub mod env;
pub use env::Env;

/** 
 *  Borrowcheckast.
*/
pub fn borrowcheck_ast<'a>(e: SpanExpr<'a>) -> IResult<'a, SpanExpr<'a>, BorrowInfo> {
    let mut env = Env::new();
    env.crate_scope();
    borrowcheck_expr(e, &mut env)
}


/** 
 *  Borrowcheck expresions in ast.
*/
fn borrowcheck_expr<'a>(e: SpanExpr<'a>, env: &mut Env<'a>) -> IResult<'a, SpanExpr<'a>, BorrowInfo> {
    match (e.1).clone() {
        Expr::Num(_) => borrowcheck_num(e, env),
        Expr::Bool(_) => borrowcheck_bool(e, env),
        Expr::UnOp(_, _) => borrowcheck_unop(e, env),
        Expr::BinOp(_, _, _) => borrowcheck_binop(e, env),
        // Expr::VarWithType(_, _) => borrowcheck_varWithType(e, env),
        // Expr::Let(_, _) => borrowcheck_let(e, env),
        // Expr::Assign(_, _) => borrowcheck_assign(e, env),
        // Expr::Var(_) => borrowcheck_var(e, env),
        // Expr::Body(_) => borrowcheck_body(e, env),
        // Expr::If(_, _, _) => borrowcheck_if(e, env),
        // Expr::While(_, _) => borrowcheck_while(e, env),
        // Expr::Func(_, _, _, _) => add_func_to_borrowchecking_list(e, env),
        // Expr::FuncCall(_, _) => borrowcheck_func_call(e, env),
        // Expr::Funcs(_) => borrowcheck_funcs(e, env),
        Expr::Prefixed(_, _) => borrowcheck_prefixed(e, env),
        _ => panic!("borrowcheck_expr"),
    }
}


/** 
 *  Borrowcheck num in ast.
 */
fn borrowcheck_num<'a>(e: SpanExpr<'a>, _env: &mut Env<'a>) -> IResult<'a, SpanExpr<'a>, BorrowInfo> {
    match (e.1).clone() {
        Expr::Num(_) => {
            let res = BorrowInfo::Value(ValueInfo {
                                mutable: false, 
                                prefix: Prefix::None, 
                                num_borrows: 0, 
                                num_borrowmuts: 0});
            return Ok((e, res));
        },
        _ => panic!("borrowcheck_num"),
    }
}

/** 
 *  Borrowcheck bool in ast.
 */
fn borrowcheck_bool<'a>(e: SpanExpr<'a>, _env: &mut Env<'a>) -> IResult<'a, SpanExpr<'a>, BorrowInfo> {
    match (e.1).clone() {
        Expr::Bool(_) => {
            let res = BorrowInfo::Value(ValueInfo {
                                mutable: false, 
                                prefix: Prefix::None, 
                                num_borrows: 0, 
                                num_borrowmuts: 0});
            return Ok((e, res));
        },
        _ => panic!("borrowcheck_num"),
    }
}


/** 
 *  Borrowcheck unop in ast.
*/
fn borrowcheck_unop<'a>(e: SpanExpr<'a>, env: &mut Env<'a>) -> IResult<'a, SpanExpr<'a>, BorrowInfo> {
    match (e.1).clone() {
        Expr::UnOp(_, expr) => {
            let val = borrowcheck_expr(*expr, env)?.1;
            let p;
            match val.clone() {
                BorrowInfo::Value(v) => {
                    p = v.prefix;
                },
                BorrowInfo::Var(v) => {
                    p = v.prefix;
                },
            };
            match p {
                Prefix::BorrowMut => panic!("borrowcheck_unop"),
                _ => (),
            };

            return Ok((e, val));
        },
        _ => panic!("borrowcheck_unop"),
    }
}


/** 
 *  Borrowcheck binop in ast.
*/
fn borrowcheck_binop<'a>(e: SpanExpr<'a>, env: &mut Env<'a>) -> IResult<'a, SpanExpr<'a>, BorrowInfo> {
    match (e.1).clone() {
        Expr::BinOp(le, op, re) => {
            let lp = borrowcheck_expr(*le, env)?.1;
            let rp = borrowcheck_expr(*re, env)?.1;
            match op.1 {
                Op::Equal => (),
                Op::NotEq => (),
                _ => {
                    let p1;
                    match lp {
                        BorrowInfo::Value(v) => p1 = v.prefix,
                        BorrowInfo::Var(v) => p1 = v.prefix,
                    };
                    match p1 {
                        Prefix::BorrowMut => panic!("borrowcheck_binop"),
                        _ => (),
                    };
                    let p2; 
                    match rp {
                        BorrowInfo::Value(v) => p2 = v.prefix,
                        BorrowInfo::Var(v) => p2 = v.prefix,
                    };
                    match p2 {
                        Prefix::BorrowMut => panic!("borrowcheck_binop"),
                        _ => (),
                    };
                },
            };
            return Ok((e, BorrowInfo::Value(ValueInfo {
                mutable: false, 
                prefix: Prefix::None, 
                num_borrows: 0, 
                num_borrowmuts: 0})));
        },
        _ => panic!("borrowcheck_binop"),
    }
}


// // fn borrowcheck_varWithType<'a>(e: SpanExpr<'a>, env: &mut Env<'a>) -> IResult<'a, SpanExpr<'a>, VarInfo> {
// //     match e.1 {
// //         Expr::VarWithType(var, typ) => {
// //             let res;
            
// //             match (*var).1 {
// //                 Expr::Var(ident) => {
// //                     match (*typ).1 {
// //                         Expr::Type(_) => {
// //                             res = VarInfo::Ident(Prefix::None, ident.to_string(), 0, 0);
// //                         },
// //                         Expr::Prefixed(p, t) => {
// //                             match (*t).1 {
// //                                 Expr::Type(_) => {
// //                                     res = VarInfo::Ident(p, ident.to_string(), 0, 0);
// //                                 },
// //                                 _ => panic!("borrowcheck_varWithType"),
// //                             };
// //                         },
// //                         _ => panic!("borrowcheck_varWithType"),
// //                     };
// //                 },
// //                 _ => panic!("borrowcheck_varWithType"),
// //             };

// //             return Ok((e, VarInfo::None));
// //         },
// //         _ => panic!("borrowcheck_varWithType"),
// //     }
// // }

// // /** 
// //  *  Borrowcheck let in ast.
// // */
// // fn borrowcheck_let<'a>(e: SpanExpr<'a>, env: &mut Env<'a>) -> IResult<'a, SpanExpr<'a>, VarInfo> {
// //     match (e.1).clone() {
// //         Expr::Let(var, value) => {
// //             let var_res = borrowcheck_expr(*var, env)?.1;
// //             let value_res = borrowcheck_expr(*value, env)?.1;
// //             match value_res {
// //                 VarInfo::Ident(p1, _, _, _) => {
// //                     match var_res.clone() {
// //                         VarInfo::Ident(p2, i, _, _) => {
// //                             if p1 != p2 {
// //                                 panic!("borrowcheck_let");
// //                             }
// //                             env.store_var(&i, var_res);
// //                         },
// //                         _ => panic!("borrowcheck_let"),
// //                     };
// //                 },
// //                 VarInfo::Pointer(p1, _, _, _) => {
// //                     match var_res.clone() {
// //                         VarInfo::Ident(p2, i, _, _) => {
// //                             if p1 != p2 {
// //                                 panic!("borrowcheck_let");
// //                             }
// //                             env.store_var(&i, var_res);
// //                         },
// //                         _ => panic!("borrowcheck_let"),
// //                     };
// //                 },
// //                 VarInfo::Value(p1, _, _) => {
// //                     match var_res.clone() {
// //                         VarInfo::Ident(p2, i, _, _) => {
// //                             if p1 != p2 {
// //                                 panic!("borrowcheck_let");
// //                             }
// //                             env.store_var(&i, var_res);
// //                         },
// //                         _ => panic!("borrowcheck_let"),
// //                     };
// //                 },
// //                 _ => panic!("borrowcheck_let"),
// //             };
// //             return Ok((e, VarInfo::None));
// //         },
// //         _ => panic!("borrowcheck_let"),
// //     }
// // }


// // /** 
// //  *  Borrowcheck assign in ast.
// // */
// // fn borrowcheck_assign<'a>(e: SpanExpr<'a>, env: &mut Env<'a>) -> IResult<'a, SpanExpr<'a>, Prefix> {
// //     match (e.1).clone() {
// //         Expr::Assign(variable, value) => {
// //             let var = borrowcheck_expr(*variable, env)?;
// //             let val = borrowcheck_expr(*value, env)?;
// //             if var.1 != val.1 {
// //                 panic!("borrowcheck_assign");
// //             }
// //             match var.1 {
// //                 Prefix::Borrow => panic!("borrowcheck_assign Prefix::Borrow"),
// //                 Prefix::BorrowMut => panic!("borrowcheck_assign Prefix::BorrowMut"),
// //                 Prefix::Mut => panic!("borrowcheck_assign Prefix::Mut"),
// //                 Prefix::DeRef(_n) => {
// //                     // if stored_prefix != Prefix::BorrowMut {
// //                     //     panic!("borrowcheck_assign Prefix::Mut");
// //                     // }
// //                 },
// //                 Prefix::None => (),
// //                 Prefix::ReturnPrefix(_) => panic!("borrowcheck_assign"),
// //             };
// //             return Ok(val);
// //         },
// //         _ => panic!("borrowcheck_assign"),
// //     }
// // }

// // /** 
// //  *  Borrowcheck var in ast.
// // */
// // fn borrowcheck_var<'a>(e: SpanExpr<'a>, env: &mut Env<'a>) -> IResult<'a, SpanExpr<'a>, Prefix> {
// //     match (e.1).clone() {
// //         Expr::Var(ident) => {
// //             let stored_prefix = env.load_var(ident, 0).unwrap().0;
// //             return Ok((e, stored_prefix));
// //         },
// //         _ => panic!("borrowcheck_var"),
// //     }
// // }


// // /** 
// //  *   Borrowcheck body in ast.
// // */
// // fn borrowcheck_body<'a>(e: SpanExpr<'a>, env: &mut Env<'a>) -> IResult<'a, SpanExpr<'a>, Prefix> {
// //     match (e.1).clone() {
// //         Expr::Body(es) => {
// //             for expr in es {
// //                 match expr.1 {
// //                     Expr::Return(v) => {
// //                         let val = borrowcheck_expr(*v, env)?;
// //                         borrowcheck_funcs_in_list(e.clone(), env);
// //                         env.pop_scope();
// //                         return match val.1 {
// //                             Prefix::Borrow => Ok((e, Prefix::ReturnPrefix(Box::new(Prefix::Borrow)))),
// //                             Prefix::BorrowMut => Ok((e, Prefix::ReturnPrefix(Box::new(Prefix::BorrowMut)))),
// //                             Prefix::DeRef(_) => panic!("borrowcheck_body"),
// //                             Prefix::Mut => panic!("borrowcheck_body"),
// //                             Prefix::None =>  Ok((e, Prefix::ReturnPrefix(Box::new(Prefix::None)))),
// //                             Prefix::ReturnPrefix(_) =>  Ok(val),
// //                         };
// //                     },
// //                     _ => {
// //                         let res = borrowcheck_expr(expr, env);
// //                         match res.clone()?.1 {
// //                             Prefix::ReturnPrefix(_) => {
// //                                 borrowcheck_funcs_in_list(e.clone(), env);
// //                                 env.pop_scope();
// //                                 return res;
// //                             },
// //                             _ => (),
// //                         };
// //                     },
// //                 }
// //             }
// //             borrowcheck_funcs_in_list(e.clone(), env);
// //             env.pop_scope();
// //             return Ok((e, Prefix::None));
// //         },
// //         _ => panic!("borrowcheck_body"),
// //     }
// // }


// // /** 
// //  *  Borrowcheck if in ast.
// // */
// // fn borrowcheck_if<'a>(e: SpanExpr<'a>, env: &mut Env<'a>) -> IResult<'a, SpanExpr<'a>, Prefix> {
// //     match (e.1).clone() {
// //         Expr::If(b, ib, eb) => {
// //             let val = borrowcheck_expr(*b, env)?;
// //             match val.1 {
// //                 Prefix::DeRef(_) => panic!("borrowcheck_if"),
// //                 Prefix::Mut => panic!("borrowcheck_if"),
// //                 Prefix::ReturnPrefix(_) => panic!("borrowcheck_if"),
// //                 _ => (),
// //             }
// //             let ib_r = borrowcheck_body(*ib, env)?;
// //             let eb_r = borrowcheck_body(*eb, env)?;
// //             match ib_r.1 {
// //                 Prefix::ReturnPrefix(ip) => {
// //                     match eb_r.1 {
// //                         Prefix::ReturnPrefix(ep) => {
// //                                 if ip != ep {
// //                                     panic!("borrowcheck_if");
// //                                 }
// //                                 return Ok((e, *ip));
// //                             },
// //                         _ => return Ok((e, *ip)),
// //                     }
// //                 },
// //                 _ => (),
// //             };
// //             match eb_r.1 {
// //                 Prefix::ReturnPrefix(_) => return Ok(eb_r),
// //                 _ => return Ok((e, Prefix::None)),
// //             }
// //         },
// //         _ => panic!("borrowcheck_if"),
// //     }
// // }


// // /** 
// //  *  Borrowcheck while in ast.
// // */
// // fn borrowcheck_while<'a>(e: SpanExpr<'a>, env: &mut Env<'a>) -> IResult<'a, SpanExpr<'a>, Prefix> {
// //     match (e.1).clone() {
// //         Expr::While(b, body)=> {
// //             let val = borrowcheck_expr(*b, env)?;
// //             match val.1 {
// //                 Prefix::DeRef(_) => panic!("borrowcheck_while"),
// //                 Prefix::Mut => panic!("borrowcheck_while"),
// //                 Prefix::ReturnPrefix(_) => panic!("borrowcheck_while"),
// //                 _ => (),
// //             }
// //             let body_r = borrowcheck_body(*body, env)?;
// //             match body_r.1 {
// //                 Prefix::ReturnPrefix(_) => return Ok(body_r),
// //                 _ => return Ok((e, Prefix::None)),
// //             }
// //         },
// //         _ => panic!("borrowcheck_while"),
// //     }
// //     return Ok((e, Prefix::None));
// // }


// // /** 
// //  *  Borrowcheck func in ast.
// // */
// // fn add_func_to_borrowchecking_list<'a>(e: SpanExpr<'a>, env: &mut Env<'a>) -> IResult<'a, SpanExpr<'a>, Prefix> {
// //     match (e.1).clone() {
// //         Expr::Func(var, param, _, _) => {
// //             let mut t_param = Vec::new();
// //             let mut t_var = Vec::new();
// //             for v in param {
// //                 match v.1 {
// //                     Expr::VarWithType(v,t) => {
// //                         match (*v).1 {
// //                             Expr::Var(i) => {
// //                                 let p = borrowcheck_expr(*t, env)?;
// //                                 t_param.push(p.clone().1);
// //                                 t_var.push((i, p.clone().1));
// //                             },
// //                             _ => panic!("add_func_to_borrowchecking_list"),
// //                         };
// //                     }
// //                     _ => panic!("add_func_to_borrowchecking_list"),
// //                 }
// //             }
// //             let ident;
// //             match (*var).1 {
// //                 Expr::Var(i) => {
// //                     ident = i;
// //                 },
// //                 _ => panic!("add_func_to_borrowchecking_list"),
// //             };
// //             env.store_func(&ident, t_param, Prefix::None, e.clone().1);
// //             return Ok((e, Prefix::None));
// //         },
// //         _ => panic!("add_func_to_borrowchecking_list"),
// //     }
// // }


// // /** 
// //  *  Borrowcheck func call in ast.
// // */
// // fn borrowcheck_func_call<'a>(e: SpanExpr<'a>, env: &mut Env<'a>) -> IResult<'a, SpanExpr<'a>, Prefix> {
// //     match (e.1).clone() {
// //         Expr::FuncCall(i, param) => {
// //             let ident;
// //             match (*i).1 {
// //                 Expr::Var(id) => ident = id,
// //                 _ => panic!("borrowcheck_func_call"),
// //             };
// //             let temp = env.load_func(ident);
// //             let param_p;
// //             let return_p;
// //             match temp {
// //                 Ok(tup) => {param_p = tup.0; return_p = tup.1;},
// //                 _ => panic!("borrowcheck_func_call"),
// //             };
// //             if param_p.len() != param.len() {
// //                 panic!("borrowcheck_func_call");
// //             }
// //             let mut i = 0;
// //             for p in param_p {
// //                 if p != borrowcheck_expr(param[i].clone(), env)?.1 {
// //                     panic!("borrowcheck_func_call");
// //                 }
// //                 i = i + 1;
// //             }
// //             return Ok((e, return_p));
// //         },
// //         _ => panic!("borrowcheck_func_call"),
// //     }
// // }


// // /** 
// //  *  Borrowcheck funcs in ast.
// // */
// // fn borrowcheck_funcs<'a>(e: SpanExpr<'a>, env: &mut Env<'a>) -> IResult<'a, SpanExpr<'a>, Prefix> {
// //     match (e.1).clone() {
// //         Expr::Funcs(es) => {
// //             for expr in es {
// //                 add_func_to_borrowchecking_list(expr, env);
// //             }

// //             borrowcheck_funcs_in_list(e.clone(), env);
// //             env.pop_scope();
// //             return Ok((e, Prefix::None));
// //         },
// //         _ => panic!("borrowcheck_funcs"),
// //     }
// // }


// // /** 
// //  *  Adds to list of func that need borrowchecking in ast.
// // */
// // fn borrowcheck_funcs_in_list<'a>(expr: SpanExpr<'a>, env: &mut Env<'a>) -> IResult<'a, SpanExpr<'a>, Prefix> {
// //     let mut res = Prefix::None;
// //     while env.get_funcs_len() > 0 {
// //         let e;
// //         match env.get_func() {
// //             Some(expr) => e = expr,
// //             _ => panic!("borrowcheck_funcs_in_list"),
// //         }
// //         match e.clone() {
// //             Expr::Func(ident, param, _, body) => {
// //                 env.crate_scope();
// //                 for v in param {
// //                     match v.1 {
// //                         Expr::VarWithType(i , t) => {
// //                             let ident;
// //                             let p = borrowcheck_expr(*t, env)?;
// //                             match (*i).1 {
// //                                 Expr::Var(id) => ident = id,
// //                                 _ => panic!("borrowcheck_funcs_in_list"),
// //                             } 
// //                             env.store_var(ident, Val::Empty, p.clone().1);
// //                         }
// //                         _ => panic!("borrowcheck_funcs_in_list"),
// //                     }
// //                 }
// //                 let mut body_p = borrowcheck_body(*body, env)?.1;
// //                 match body_p {
// //                     Prefix::ReturnPrefix(p) => body_p = *p,
// //                     _ => body_p = Prefix::None,
// //                 }
// //                 res = body_p;
// //             },
// //             _ => panic!("borrowcheck_funcs_in_list"),
// //         }
// //     }
// //     return Ok((expr, res));
// // }


/** 
 *  Borrowcheck prefixed in ast.
*/
fn borrowcheck_prefixed<'a>(e: SpanExpr<'a>, env: &mut Env<'a>) -> IResult<'a, SpanExpr<'a>, BorrowInfo> {
    match (e.1).clone() {
        Expr::Prefixed(p, v) => {
            let mut val = borrowcheck_expr(*v.clone(), env)?.1;
            let mutable;
            match p.clone().1 {
                Prefix::Mut => mutable = true,
                _ => mutable = false,
            };
            match val.clone() {
                BorrowInfo::Value(mut v) => {
                    match p.clone().1 {
                        Prefix::DeRef(_) => panic!("borrowcheck_prefixed"),
                        _ => (),
                    };
                    if mutable {
                        panic!("borrowcheck_prefixed");
                    } else {
                        v.prefix = p.1;
                    }
                    val = BorrowInfo::Value(v);
                },
                BorrowInfo::Var(mut v) => {
                    if mutable {
                        v.mutable = mutable;
                    } else {
                        v.prefix = p.1;
                    }
                    val = BorrowInfo::Var(v);
                },
            };
            
            return Ok((e, val));
        },
        _ => panic!("borrowcheck_prefixed"),
    }
}
