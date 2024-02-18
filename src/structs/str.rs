use serde::Deserialize;

use super::location::Location;

#[derive(Deserialize, Debug)]
pub struct Str {
    pub value: String,
    pub location: Location,
}
