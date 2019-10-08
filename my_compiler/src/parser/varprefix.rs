/** 
 *  Defining all of my var prefix.
 */
#[derive(Debug, PartialEq, Clone)]
pub enum Prefix {
    Mut,
    Borrow,
    BorrowMut,
    None,
}
