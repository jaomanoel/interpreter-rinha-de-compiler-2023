use serde::Deserialize;

use crate::enums::term::Term;

use super::location::Location;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct If {
    pub condition: Box<Term>,
    pub then: Box<Term>,
    pub otherwise: Box<Term>,
    pub location: Location,
}
