#![allow(dead_code)]

pub enum MyError {
    ErrorParseInt,

}

/** 
 *  Defines Errors.  
 */
pub struct Errors {
    error_mem: Vec<MyError>,
}
impl Errors {

    /**
     *  Creates a new Errors.
     */
    pub fn new() -> Errors {
        Errors {
            error_mem: Vec::new(),
        }
    }

    /**
     *  Add latest error
     */
    fn push(&mut self, e: MyError) -> () {
        self.error_mem.push(e)
    }

    /**
     *  Get latest error
     */
    fn pop(&mut self) -> Option<MyError> {
        self.error_mem.pop()
    }
}
