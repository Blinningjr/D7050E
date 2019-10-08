/**
 * Required for reading files.
 */
use std::fs;

#[path = "parser/mod.rs"]
mod parser;
#[path = "interpreter/mod.rs"]
mod interpreter;
#[path = "typechecker/mod.rs"]
mod typechecker;

pub use crate::parser::parse;
pub use crate::interpreter::interp_ast;


fn main() {
    let contents = fs::read_to_string("src/test_code.rs")
        .expect("Something went wrong reading the file");
    // println!("{}", contents);

    // let f = parse(" while a < 10 { a = a + 2;}");
    let f = parse(contents.as_str());
    // println!("Output = {:#?}" , f); // print parsed ast.
    println!("{:#?}", interp_ast(f.unwrap().1)); // Print interp and env.
    // interp_ast(f.unwrap().1);
}
