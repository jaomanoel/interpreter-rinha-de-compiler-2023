use serde::Deserialize;

use crate::enums::term::Term;

use super::{location::Location, parameter::Parameter};

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Let {
    pub name: Parameter,
    pub value: Box<Term>,
    pub next: Box<Term>,
    pub location: Location,
}
