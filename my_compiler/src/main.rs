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
// #[path = "borrowchecker/mod.rs"]
// mod borrowchecker;

pub use crate::parser::parse;
pub use crate::interpreter::interp_ast;
// pub use crate::borrowchecker::borrowcheck_ast;


fn main() {
    // let contents = fs::read_to_string("src/test_code.rs")
    //     .expect("Something went wrong reading the file");
    // println!("{}", contents);

    // let f = parse("{
    //     let mut a: bool = true; 
    //     let mut b: &mut bool = &mut a; 
    //     let c: &mut bool = &mut b; 
    //     **c = false; 
    //     return a;
    //     }");
    let f = parse(" & 1-2");
    // let f = parse(contents.as_str());
    println!("Output = {:#?}" , f); // print parsed ast.
    // println!("{:#?}", interp_ast(f.unwrap().1)); // Print interp and env.
    // interp_ast(f.unwrap().1);
    // let mut a = 10;
    // let b = &mut a;
    // *b = 1 ;
    
    // mut and borrow tests.
    // let mut a: i32 = 10; a = a + 2; // funka
    // let mut a: bool = true; a =!a; // funka 
    // let a: i32 = 10; a = a + 2; // funka inte
    // let mut a: i32 = 10; let b: &i32 = &a; let c: &i32 = b; //c //funka
    // let mut a: i32 = 10; let b: &mut i32 = &mut a; *b = 12; //funka 
    // let mut a: bool = true; let b: &bool = &a; // funka
    // let mut a: bool = true; let mut b: &mut bool = &mut a; let c = &mut b; **c = false; // funka
    // let mut a: bool = true; let mut b: &mut bool = &mut a; let c = &mut b; &**c = &false; // funka inte
    // let a = 10; let b = &a; let c = b +10; //funka
    // print!("\n {:?} \n", c);

    // let a = &mut 10; *a = 20; // funkar
    // let mut b = &mut a;
    // let mut c = &mut a;
    // let v = &mut a + &mut b;

    // print!("\n {:?} \n", a);
}
