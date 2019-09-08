mod parser;

pub use crate::parser::parse_expr;

fn main() {
    println!("{:?}", parse_expr(" 1 +2 / 4 * 2 /3 &&2 ||9 == 2"));
}