#[path = "parser/mod.rs"]
mod parser;
#[path = "interpreter/mod.rs"]
mod interpreter;


pub use crate::parser::parse_expr;
pub use crate::parser::parse_funcs;
pub use crate::interpreter::interp_ast;


fn main() {
    // println!("{:#?}", parse_funcs("fn apor(input: Str, test) -> bool { let apa = 10; while apa >= 2 { let apa = 10 /2 * 5 -2;}} fn test(inp) -> None { if inp == false { let inp = true; } else { let inp = false;} if inp { let apa = 5 % 2; }}")); 
    let v = parse_expr("if true {let apa = 0; while apa < 20 {let apa = apa +1;}} ").unwrap();
    println!("{:?} : {:?}" , v.0, interp_ast(v.1));

    // println!("{:#?}",  parse_funcs("fn test(inp) -> None { if inp == false { let inp = true; } else { let inp = false;} if inp { let apa = 5 % 2; }}"));
}