use serde::Deserialize;

use super::location::Location;

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct Var {
    pub text: String,
    pub location: Location,
}
