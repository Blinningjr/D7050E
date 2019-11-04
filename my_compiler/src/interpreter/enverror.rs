// /**
//  *  std imports.
//  */
// use std::{
//     error,
//     fmt,
// };


// /** 
//  *  Needed for creating EnvError. 
//  *  src: https://doc.rust-lang.org/std/str/trait.FromStr.html
//  */
// pub type Result<T> = std::result::Result<T, EnvError>;
// #[derive(Debug, PartialEq, Clone)]
// pub struct EnvError;


// /** 
//  * 
//  */
// impl fmt::Display for EnvError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "Error something is wrong")
//     }
// }


// /** 
//  *  This is important for other errors to wrap this one.
//  */ 
// impl error::Error for EnvError {
//     fn source(&self) -> Option<&(dyn error::Error + 'static)> {
//         // Generic error, underlying cause isn't tracked.
//         None
//     }
// }
