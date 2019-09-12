mod parser;

pub use crate::parser::parse_expr;
pub use crate::parser::math_expr_eval;

fn main() {
    println!("{:?}", parse_expr("if false == true {1+2}"));
    let v = parse_expr("1 + 2 let apa = 20").unwrap();
    println!("{:?} : {:?}" , v.0, math_expr_eval(v.1));

    println!("{:?}", 2-15+11);
}