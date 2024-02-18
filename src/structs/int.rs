use serde::Deserialize;

use super::location::Location;

#[derive(Deserialize, Debug)]
pub struct Int {
    pub value: i32,
    pub location: Location,
}
