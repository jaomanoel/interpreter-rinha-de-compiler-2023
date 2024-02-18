use serde::Deserialize;

use super::location::Location;

#[derive(Deserialize, Debug)]
pub struct Bool {
    pub value: bool,
    pub location: Location,
}
