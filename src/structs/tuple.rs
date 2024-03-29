use super::location::Location;
use crate::enums::term::Term;
use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct Tuple {
    pub first: Box<Term>,
    pub second: Box<Term>,
    pub location: Location,
}
