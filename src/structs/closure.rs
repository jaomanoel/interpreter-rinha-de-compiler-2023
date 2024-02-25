use crate::{enums::term::Term, Scope};

use super::parameter::Parameter;

#[derive(Debug, PartialEq, Clone)]
pub struct Closure {
    pub body: Term,
    pub parameters: Vec<Parameter>,
    pub env: Scope,
}
