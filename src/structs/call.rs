use serde::Deserialize;

use crate::enums::term::Term;

use super::location::Location;

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct Call {
    pub callee: Box<Term>,
    pub arguments: Vec<Term>,
    pub location: Location,
}
