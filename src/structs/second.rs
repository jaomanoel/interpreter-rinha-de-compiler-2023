use super::location::Location;
use crate::Term;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Second {
    pub value: Box<Term>,
    pub location: Location,
}
