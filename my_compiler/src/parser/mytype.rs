/** 
 *  Defining all of my types.
 */
#[derive(Debug, PartialEq, Clone)]
pub enum MyType {
    Int32,
    Boolean,
    NoType,
    ReturnType(Box<MyType>),
}
