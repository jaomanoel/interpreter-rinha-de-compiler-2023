use serde::Deserialize;

use crate::structs::{
    binary::Binary, bool::Bool, call::Call, first::First, function::Function, if_struct::If,
    int::Int, let_struct::Let, print::Print, second::Second, str::Str, tuple::Tuple, var::Var,
};

#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(tag = "kind")]
pub enum Term {
    Int(Int),
    Str(Str),
    Print(Print),
    Bool(Bool),
    Binary(Binary),
    Tuple(Tuple),
    First(First),
    Second(Second),
    If(If),
    Function(Function),
    Let(Let),
    Var(Var),
    Call(Call),
}
