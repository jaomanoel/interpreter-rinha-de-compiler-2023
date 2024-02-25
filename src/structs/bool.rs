use serde::Deserialize;

use super::location::Location;

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct Bool {
    pub value: bool,
    pub location: Location,
}
