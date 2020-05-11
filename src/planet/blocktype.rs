use serde::{Serialize, Deserialize};
use std::slice::Iter;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
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

impl BlockType {
    pub fn iter() -> Iter<'static, BlockType> {
        static BTYPES: [BlockType; 10] = [
            BlockType::None,
            BlockType::Water,
            BlockType::Plains,
            BlockType::Hills,
            BlockType::Mountains,
            BlockType::Marsh,
            BlockType::Plateau,
            BlockType::Highlands,
            BlockType::Coastal,
            BlockType::SaltMarsh
        ];
        BTYPES.iter()
    }
}