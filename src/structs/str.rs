use serde::Deserialize;

use super::location::Location;

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct Str {
    pub value: String,
    pub location: Location,
}
