use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Location {
    pub start: i32,
    pub end: i32,
    pub filename: String,
}
