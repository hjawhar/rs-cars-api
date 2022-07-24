use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Car {
    pub name: String,
    pub model: String,
}
