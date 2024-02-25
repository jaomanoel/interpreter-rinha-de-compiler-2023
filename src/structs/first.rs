use super::location::Location;
use crate::enums::term::Term;
use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct First {
    pub value: Box<Term>,
    pub location: Location,
}
