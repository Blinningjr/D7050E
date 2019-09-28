/**
 *  std imports.
 */
use std::{
    error,
    fmt,
};


/** 
 *  Needed for creating SyntaxError. 
 *  src: https://doc.rust-lang.org/std/str/trait.FromStr.html
 */
pub type Result<T> = std::result::Result<T, SyntaxError>;
#[derive(Debug, Clone)]
pub struct SyntaxError;


/** 
 * 
 */
impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error something is wrong")
    }
}


/** 
 *  This is important for other errors to wrap this one.
 */ 
impl error::Error for SyntaxError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}
