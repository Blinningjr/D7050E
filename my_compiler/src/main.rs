#[path = "parser/mod.rs"]
mod parser;
#[path = "interpreter/mod.rs"]
mod interpreter;


pub use crate::parser::parse_expr;
pub use crate::parser::math_expr_eval;


fn main() {
    // println!("{:?}", parse_expr("fn apa(input: i32) -> bool { let apa = 10; 10 + 2}"));
    let v = parse_expr(" 10 / (2-4)").unwrap();
    println!("{:?} : {:?}" , v.0, math_expr_eval(v.1));

    // println!("{:?}", 2-15+11);
}