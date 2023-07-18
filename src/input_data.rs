use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Manusia {
    name: String,
    age: u8,
    is_male: bool,
}

pub fn run() {}
