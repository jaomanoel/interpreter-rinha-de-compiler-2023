use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
pub enum ReturnTypes {
    Void,
    Str(String),
    Int(i32),
    Bool(bool),
    Tuple((Box<ReturnTypes>, Box<ReturnTypes>)),
}
