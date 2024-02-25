use serde::Deserialize;

use crate::enums::term::Term;

use super::{location::Location, parameter::Parameter};

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct Function {
    pub parameters: Vec<Parameter>,
    pub value: Box<Term>,
    pub location: Location,
}
