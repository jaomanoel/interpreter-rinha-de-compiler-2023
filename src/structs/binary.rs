use serde::Deserialize;

use crate::enums::{binary_op::BinaryOp, term::Term};

use super::location::Location;

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct Binary {
    pub lhs: Box<Term>,
    pub op: BinaryOp,
    pub rhs: Box<Term>,
    pub location: Location,
}
