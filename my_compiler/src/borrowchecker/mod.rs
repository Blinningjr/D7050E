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
        Expr::VarWithType(_, _) => borrowcheck_varWithType(e, env),
        Expr::Let(_, _) => borrowcheck_let(e, env),
        Expr::Assign(_, _) => borrowcheck_assign(e, env),
        Expr::Var(_) => borrowcheck_var(e, env),
        Expr::Body(_) => borrowcheck_body(e, env),
        // Expr::If(_, _, _) => borrowcheck_if(e, env),
        // Expr::While(_, _) => borrowcheck_while(e, env),
        // Expr::Func(_, _, _, _) => add_func_to_borrowchecking_list(e, env),
        // Expr::FuncCall(_, _) => borrowcheck_func_call(e, env),
        // Expr::Funcs(_) => borrowcheck_funcs(e, env),
        Expr::Prefixed(_, _) => borrowcheck_prefixed(e, env),
        _ => panic!("borrowcheck_expr {:#?}", e),
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

                                scope: -1,
                                mem_pos: 0,

                                num_borrows: 0, 
                                num_borrowmuts: 0}, false);
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

                                scope: -1,
                                mem_pos: 0,


                                num_borrows: 0, 
                                num_borrowmuts: 0}, false);
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
                BorrowInfo::Value(v, _) => {
                    p = v.prefix;
                },
                BorrowInfo::Var(v, _) => {
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
                        BorrowInfo::Value(v, _) => p1 = v.prefix,
                        BorrowInfo::Var(v, _) => p1 = v.prefix,
                    };
                    match p1 {
                        Prefix::BorrowMut => panic!("borrowcheck_binop"),
                        _ => (),
                    };
                    let p2; 
                    match rp {
                        BorrowInfo::Value(v, _) => p2 = v.prefix,
                        BorrowInfo::Var(v, _) => p2 = v.prefix,
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

                scope: -1,
                mem_pos: 0,


                num_borrows: 0, 
                num_borrowmuts: 0}, false)));
        },
        _ => panic!("borrowcheck_binop"),
    }
}


fn borrowcheck_varWithType<'a>(e: SpanExpr<'a>, env: &mut Env<'a>) -> IResult<'a, SpanExpr<'a>, BorrowInfo> {
    match e.1.clone() {
        Expr::VarWithType(var, typ) => {
            let res;
            
            match (*var).1 {
                Expr::Var(id) => {
                    match (*typ).1 {
                        Expr::Type(_) => {
                            res = BorrowInfo::Var(VarInfo {
                                mutable: false,
                                prefix: Prefix::None,
                                ident: id.to_string(),

                                scope: -1,
                                mem_pos: 0,

                                pointer_scope_pos: -1,
                                pointer_mem_pos: 0,

                                num_borrows: 0,
                                num_borrowmuts: 0,
                            }, false);
                        },
                        Expr::Prefixed(p, t) => {
                            match (*t).1 {
                                Expr::Type(_) => {
                                    res = BorrowInfo::Var(VarInfo {
                                        mutable: false,
                                        prefix: p.1,
                                        ident: id.to_string(),

                                        scope: -1,
                                        mem_pos: 0,
        
                                        pointer_scope_pos: -1,
                                        pointer_mem_pos: 0,
        
                                        num_borrows: 0,
                                        num_borrowmuts: 0,
                                    }, false);
                                },
                                _ => panic!("borrowcheck_varWithType"),
                            };
                        },
                        _ => panic!("borrowcheck_varWithType"),
                    };
                },
                _ => panic!("borrowcheck_varWithType"),
            };

            return Ok((e, res));
        },
        _ => panic!("borrowcheck_varWithType"),
    }
}

/** 
 *  Borrowcheck let in ast.
*/
fn borrowcheck_let<'a>(e: SpanExpr<'a>, env: &mut Env<'a>) -> IResult<'a, SpanExpr<'a>, BorrowInfo> {
    match (e.1).clone() {
        Expr::Let(var, value) => {
            let mut var_res = borrowcheck_expr(*var, env)?.1;
            let value_res = borrowcheck_expr(*value, env)?.1;
            let pointer;

            let p2;
            match value_res.clone() {
                BorrowInfo::Value(v, _) => {
                    p2 = v.prefix;
                    pointer = (v.scope, v.mem_pos);
                },
                BorrowInfo::Var(v, _) => {
                    p2 = v.prefix;
                    pointer = (v.scope, v.mem_pos);
                },
            };

            let p1;
            match var_res.clone() {
                BorrowInfo::Value(_, _) => panic!("borrowcheck_let"),
                BorrowInfo::Var(mut v, _) => {
                    p1 = v.clone().prefix;
                    v.pointer_scope_pos = pointer.0;
                    v.pointer_mem_pos = pointer.1;
                    var_res = BorrowInfo::Var(v, false);
                },
            };

            if p1 != p2 {
                panic!("borrowcheck_let");
            }

            let store = env.store_var(var_res.clone());

            var_res = env.load_val(store.1, 0, store.0).unwrap().0;

            // match var_res.clone() {
            //     BorrowInfo::Value(_, _) => panic!("borrowcheck_let"),
            //     BorrowInfo::Var(mut v, _) => {
            //         v.scope = store.0;
            //         v.mem_pos = store.1;
            //         var_res = BorrowInfo::Var(v, false);
            //     },
            // };

            // println!("{:#?}", env);

            return Ok((e, var_res));
        },
        _ => panic!("borrowcheck_let"),
    }
}


/** 
 *  Borrowcheck assign in ast.
*/
fn borrowcheck_assign<'a>(e: SpanExpr<'a>, env: &mut Env<'a>) -> IResult<'a, SpanExpr<'a>, BorrowInfo> {
    match (e.1).clone() {
        Expr::Assign(variable, value) => {
            let mut var;
            let ident;
            match variable.clone().1 {
                Expr::Prefixed(p, ev) => {
                    match ev.1 {
                        Expr::Var(i) => ident = i,
                        _ => panic!("borrowcheck_assign 2"),
                    };
                    match p.clone().1 {
                        Prefix::DeRef(n) => {
                            var = env.load_var(ident, n).unwrap().0;
                        },
                        _ => panic!("borrowcheck_assign 1 {:?}", p.1),
                    };
                },
                Expr::Var(i) => {
                    ident = i;
                    var = env.load_var(ident, 0).unwrap().0;
                },
                _ => panic!("borrowcheck_assign 2"),
            };

            let val = borrowcheck_expr(*value, env)?;
            let p_var;
            let scope;
            let mem_pos;
            match var.clone() {
                BorrowInfo::Value(v, _) => {
                    p_var = v.prefix;
                    scope = v.scope;
                    mem_pos = v.mem_pos;
                },
                BorrowInfo::Var(v, _) => {
                    p_var = v.prefix;
                    scope = v.pointer_scope_pos;
                    mem_pos = v.pointer_mem_pos;
                },
            };
            let p_val;
            match val.clone().1 {
                BorrowInfo::Value(v, _) => p_val = v.prefix,
                BorrowInfo::Var(v, _) => p_val = v.prefix, 
            };

            if p_var != p_val {
                panic!("borrowcheck_assign 3 {:?} {:?}", p_var, p_val);
            }

            let res = env.load_var(ident, 0).unwrap().0;

            // println!("{:#?}", env);

            return Ok((e, res));
        },
        _ => panic!("borrowcheck_assign 0"),
    }
}

/** 
 *  Borrowcheck var in ast.
*/
fn borrowcheck_var<'a>(e: SpanExpr<'a>, env: &mut Env<'a>) -> IResult<'a, SpanExpr<'a>, BorrowInfo> {
    match (e.1).clone() {
        Expr::Var(ident) => {
            let res;
            let stored = env.load_var(ident, 0).unwrap();
            match stored.clone().0 {
                BorrowInfo::Value(mut v, _) => {
                    v.scope = (stored.1).0;
                    v.mem_pos = (stored.1).1;
                    res = BorrowInfo::Value(v, false);
                },
                BorrowInfo::Var(mut v, _) => {
                    v.scope = (stored.1).0;
                    v.mem_pos = (stored.1).1;
                    res = BorrowInfo::Var(v, false);
                },
            };
            return Ok((e, res));
        },
        _ => panic!("borrowcheck_var"),
    }
}


/** 
 *   Borrowcheck body in ast.
*/
fn borrowcheck_body<'a>(e: SpanExpr<'a>, env: &mut Env<'a>) -> IResult<'a, SpanExpr<'a>, BorrowInfo> {
    match (e.1).clone() {
        Expr::Body(es) => {
            let mut res = BorrowInfo::Value(ValueInfo {
                mutable: false, 
                prefix: Prefix::None, 

                scope: -1,
                mem_pos: 0,

                num_borrows: 0, 
                num_borrowmuts: 0}, false);
            for expr in es {
                match expr.1 {
                    Expr::Return(v) => {
                        let val = borrowcheck_expr(*v, env)?;
                        borrowcheck_funcs_in_list(e.clone(), env);
                        env.pop_scope();

                        match val.1 {
                            BorrowInfo::Value(v, _) => {
                                res = BorrowInfo::Value(v, true);
                            },
                            BorrowInfo::Var(v, _) => {
                                res = BorrowInfo::Var(v, true);
                            },
                        };
                        return Ok((e, res));
                    },
                    _ => {
                        res = borrowcheck_expr(expr, env)?.1;

                        match res.clone(){
                            BorrowInfo::Value(_, b) => {
                                if b {
                                    borrowcheck_funcs_in_list(e.clone(), env);
                                    env.pop_scope();
                                    return Ok((e, res));
                                }
                            },
                            BorrowInfo::Var(_, b) => {
                                if b {
                                    borrowcheck_funcs_in_list(e.clone(), env);
                                    env.pop_scope();
                                    return Ok((e, res));
                                }
                            },
                        };
                    },
                };
            }
            borrowcheck_funcs_in_list(e.clone(), env);
            env.pop_scope();
            return Ok((e, res));
        },
        _ => panic!("borrowcheck_body"),
    }
}


// /** 
//  *  Borrowcheck if in ast.
// */
// fn borrowcheck_if<'a>(e: SpanExpr<'a>, env: &mut Env<'a>) -> IResult<'a, SpanExpr<'a>, BorrowInfo> {
//     match (e.1).clone() {
//         Expr::If(b, ib, eb) => {
//             let val = borrowcheck_expr(*b, env)?;
//             match val.1 {
//                 BorrowInfo::Value(v) => {
//                     match v.prefix {
//                         Prefix::DeRef(_) => panic!("borrowcheck_if"),
//                         Prefix::Mut => panic!("borrowcheck_if"),
//                         Prefix::ReturnPrefix(_) => panic!("borrowcheck_if"),
//                         _ => (),
//                     };
//                 },
//                 _ => panic!("borrowcheck_if"),
//             };
//             let ib_r = borrowcheck_body(*ib, env)?;
//             let eb_r = borrowcheck_body(*eb, env)?;
//             let p_ib;
//             match ib_r.1 {
//                 BorrowInfo::Value(v) => p_ib = v.prefix,
//                 BorrowInfo::Var(v) => p_ib = v.prefix,
//             };
//             let p_eb;
//             match eb_r.1 {
//                 BorrowInfo::Value(v) => p_eb = v.prefix,
//                 BorrowInfo::Var(v) => p_eb = v.prefix,
//             };

//             match p_ib {
//                 Prefix::ReturnPrefix(ip) => {
//                     match p_eb {
//                         Prefix::ReturnPrefix(ep) => {
//                                 if ip != ep {
//                                     panic!("borrowcheck_if");
//                                 }
//                             },
//                         _ => (),
//                     }
//                 },
//                 _ => (),
//             };
//             return Ok(eb_r);
//         },
//         _ => panic!("borrowcheck_if"),
//     }
// }


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


/** 
 *  Adds to list of func that need borrowchecking in ast.
*/
fn borrowcheck_funcs_in_list<'a>(expr: SpanExpr<'a>, env: &mut Env<'a>) -> () {
    while env.get_funcs_len() > 0 {
        let e;
        match env.get_func() {
            Some(expr) => e = expr,
            _ => panic!("borrowcheck_funcs_in_list"),
        }

        match e.clone() {
            Expr::Func(ident, param, t, body) => {
                let typ = borrowcheck_expr(*t, env).unwrap().1;
                let pt;
                match typ {
                    BorrowInfo::Value(v, _) => pt = v.prefix,
                    BorrowInfo::Var(v, _) => pt = v.prefix,
                };

                env.crate_scope();
                for v in param {
                    let val = borrowcheck_varWithType(v, env).unwrap().1;
                    env.store_var(val);
                }
                let body_p = borrowcheck_body(*body, env).unwrap().1;
                let pb;
                match body_p {
                    BorrowInfo::Value(v, _) => pb = v.prefix,
                    BorrowInfo::Var(v, _) => pb = v.prefix,
                };
                if pt != pb {
                    panic!("borrowcheck_funcs_in_list");
                }

            },
            _ => panic!("borrowcheck_funcs_in_list"),
        }
    }
    return ();
}


/** 
 *  Borrowcheck prefixed in ast.
*/
fn borrowcheck_prefixed<'a>(e: SpanExpr<'a>, env: &mut Env<'a>) -> IResult<'a, SpanExpr<'a>, BorrowInfo> {
    match (e.1).clone() {
        Expr::Prefixed(p, v) => {
            let mut val = borrowcheck_expr(*v.clone(), env)?.1;
            match val.clone() {
                BorrowInfo::Value(mut v, _) => {
                    match p.clone().1 {
                        Prefix::DeRef(_) => panic!("borrowcheck_prefixed"),
                        Prefix::Borrow => {
                            let pointer = env.store_var(val.clone());
                            env.add_borrow(pointer.0, pointer.1);
                            v.prefix = p.clone().1;
                            v.scope = pointer.0;
                            v.mem_pos = pointer.1;
                            val = BorrowInfo::Value(v, false);
                        },
                        Prefix::BorrowMut => {
                            let pointer = env.store_var(val.clone());
                            env.add_borrowmut(pointer.0, pointer.1);
                            v.prefix = p.clone().1;
                            v.scope = pointer.0;
                            v.mem_pos = pointer.1;
                            val = BorrowInfo::Value(v, false);
                        },
                        Prefix::Mut => panic!("borrowcheck_prefixed"),
                        Prefix::None => (),
                    };
                },
                BorrowInfo::Var(mut v, _) => {
                    match p.clone().1 {
                        Prefix::DeRef(n) => {
                            let lval = env.load_var(&v.ident, n).unwrap();
                            match lval.0 {
                                BorrowInfo::Value(mut v, _) => {
                                    v.scope = (lval.1).0;
                                    v.mem_pos = (lval.1).1;
                                    val = BorrowInfo::Value(v, false);
                                },
                                BorrowInfo::Var(mut v, _) => {
                                    v.scope = (lval.1).0;
                                    v.mem_pos = (lval.1).1;
                                    val = BorrowInfo::Var(v, false);
                                },
                            };
                        },
                        Prefix::Borrow => {
                            let pointer = env.store_var(val.clone());
                            env.add_borrow(pointer.0, pointer.1);
                            v.prefix = p.clone().1;
                            v.scope = pointer.0;
                            v.mem_pos = pointer.1;
                            val = BorrowInfo::Var(v, false);
                        },
                        Prefix::BorrowMut => {
                            let pointer = env.store_var(val.clone());
                            env.add_borrowmut(pointer.0, pointer.1);
                            v.prefix = p.clone().1;
                            v.scope = pointer.0;
                            v.mem_pos = pointer.1;
                            val = BorrowInfo::Var(v, false);
                        },
                        Prefix::Mut =>{
                            v.mutable = true;
                            val = BorrowInfo::Var(v, false);
                        },
                        Prefix::None => (),
                    };
                },
            };
            
            return Ok((e, val));
        },
        _ => panic!("borrowcheck_prefixed"),
    }
}
