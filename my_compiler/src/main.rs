/**
 * Required for reading files.
 */
use std::fs;

#[path = "parser/mod.rs"]
mod parser;
#[path = "interpreter/mod.rs"]
mod interpreter;


pub use crate::parser::parse_expr;
pub use crate::parser::parse_funcs;
pub use crate::interpreter::interp_ast;


fn main() {

    let contents = fs::read_to_string("src/test_code.rs")
        .expect("Something went wrong reading the file");
    // println!("{}", contents);

    let f = parse_funcs(contents.as_str()).unwrap();
    // println!("{:#?}" , f); // print parsed ast.
    println!("{:?} : {:#?}" , f.0, interp_ast(f.1)); // Print interp and env.
}