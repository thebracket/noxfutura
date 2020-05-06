use super::BlockType;
#[derive(Clone)]
pub struct Block {
    pub height: u8,
    pub variance: u8,
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
            biome_idx: 0,
        }
    }
}
