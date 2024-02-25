use serde::Deserialize;

use crate::enums::term::Term;

use super::location::Location;

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct Print {
    pub value: Box<Term>,
    pub location: Location,
}
