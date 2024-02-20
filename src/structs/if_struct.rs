use serde::Deserialize;

use crate::enums::term::Term;

use super::location::Location;

#[derive(Debug, Deserialize)]
pub struct If {
    pub condition: Box<Term>,
    pub then: Box<Term>,
    pub otherwise: Box<Term>,
    pub location: Location,
}
