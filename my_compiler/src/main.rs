mod parser;

pub use crate::parser::parse_expr;
pub use crate::parser::math_expr_eval;

fn main() {
    println!("{:?}", parse_expr("let apa = 20"));
    println!("{:?}", math_expr_eval(parse_expr("10 / 2 * 3 - 2 + 4 % 6").unwrap().1));

    println!("{:?}", 2-15+11);
}