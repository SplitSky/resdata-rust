use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug)]
pub struct Dataset {
    pub id: u32,
    pub name: String,
    pub data: String,
}
