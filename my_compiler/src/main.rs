#![allow(dead_code)]

/**
 * Required for reading files.
 */
use std::fs;
use std::env;

#[path = "parser/mod.rs"]
mod parser;
#[path = "interpreter/mod.rs"]
mod interpreter;
#[path = "typechecker/mod.rs"]
mod typechecker;
#[path = "borrowchecker/mod.rs"]
mod borrowchecker;

pub use crate::parser::parse;
pub use crate::interpreter::{interp_ast, Val};
pub use crate::typechecker::typecheck_ast;
pub use crate::borrowchecker::borrowcheck_ast;


fn main() {
    let args: Vec<String> = env::args().collect();

    let mut filename = "";
    let mut typecheck = true;
    let mut borrowcheck = true;
    for i in 1..args.len() {
        if args[i] == "t" {
            typecheck = false;
        } else if args[i] == "b" {
            borrowcheck = false;
        } else {
            filename = &args[i];
        }
    }


    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let parsed = parse(&contents).unwrap();
    if (parsed.0).fragment != "" {
        println!("Could not parse file: \n {:#?}", parsed.0);
    }
    
    if typecheck {
        let _tres = typecheck_ast(parsed.1.clone());
    }

    if borrowcheck {
        let _bres = borrowcheck_ast(parsed.1.clone());
    }
    let result = interp_ast(parsed.1).unwrap().1;
    match result {
        Val::ReturnBool(b) => println!("{:?}", b),
        Val::ReturnNum(n) => println!("{:?}", n),
        Val::Bool(b) => println!("{:?}", b),
        Val::Num(n) => println!("{:?}", n),
        Val::BorrowPrimitive(_, v) => {
            match *v {
                Val::Bool(b) => println!("{:?}", b),
                Val::Num(n) => println!("{:?}", n),
                _ =>  return,
            }
        },
        _ => return,
    }
}
