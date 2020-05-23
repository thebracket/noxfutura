use super::BlockType;
use serde::{Deserialize, Serialize};
use crate::region::HeightType;

#[derive(Clone, Serialize, Deserialize)]
pub struct Block {
    pub height: HeightType,
    pub variance: HeightType,
    pub btype: BlockType,
    pub temperature: i8,
    pub rainfall: i8,
    pub biome_idx: usize,
}

impl Block {
    pub fn blank() -> Self {
        Self {
            height: 0,
            variance: 0,
            btype: BlockType::None,
            temperature: 0,
            rainfall: 0,
            biome_idx: std::usize::MAX,
        }
    }
}
