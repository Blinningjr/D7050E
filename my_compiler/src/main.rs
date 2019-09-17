#[path = "parser/mod.rs"]
mod parser;
#[path = "interpreter/mod.rs"]
mod interpreter;


pub use crate::parser::parse_expr;
pub use crate::parser::parse_funcs;
pub use crate::parser::math_expr_eval;


fn main() {
    println!("{:#?}", parse_funcs("fn apor(input: Str, test) -> bool { let apa = 10; while apa >= 2 { let apa = 10 /2 * 5 -2;}} fn test(inp) -> None { if inp == false { let inp = true; } else { let inp = false;} if inp { let apa = 5 % 2; }}")); 
    // let v = parse_expr(" 10 / (2-4)").unwrap();
    // println!("{:?} : {:?}" , v.0, math_expr_eval(v.1));

    // println!("{:#?}",  parse_funcs("fn test(inp) -> None { if inp == false { let inp = true; } else { let inp = false;} if inp { let apa = 5 % 2; }}"));
}