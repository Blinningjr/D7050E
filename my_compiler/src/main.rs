mod parser;

pub use crate::parser::parse_expr;
pub use crate::parser::math_expr_eval;

fn main() {
    println!("{:#?}", parse_expr(" 1 +2 / 4 * 2 /3 &&2 ||9 == 2"));
    println!("{:?}", math_expr_eval(parse_expr("10 / 2 * 3 - 2 + 4 % 6").unwrap().1));

    println!("{:?}", parse_expr("false"));
}