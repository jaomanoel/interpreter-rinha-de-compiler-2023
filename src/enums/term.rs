use serde::Deserialize;

use crate::structs::{
    binary::Binary, bool::Bool, first::First, int::Int, print::Print, second::Second, str::Str,
    tuple::Tuple,
};

#[derive(Deserialize, Debug)]
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
}
