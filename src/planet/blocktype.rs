use serde::{Serialize, Deserialize};
#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
pub enum BlockType {
    None,
    Water,
    Plains,
    Hills,
    Mountains,
    Marsh,
    Plateau,
    Highlands,
    Coastal,
    SaltMarsh,
}
