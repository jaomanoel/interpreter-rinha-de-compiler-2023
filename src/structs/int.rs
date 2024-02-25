use serde::Deserialize;

use super::location::Location;

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct Int {
    pub value: i32,
    pub location: Location,
}
