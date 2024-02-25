use crate::structs::{call::Call, closure::Closure};

#[derive(Debug, PartialEq, Clone)]
pub enum ReturnTypes {
    Void,
    Str(String),
    Int(i32),
    Bool(bool),
    Tuple((Box<ReturnTypes>, Box<ReturnTypes>)),
    Closure(Closure),
    Call(Call),
}
