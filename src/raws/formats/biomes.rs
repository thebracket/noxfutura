use crate::raws::BlockType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Biomes {
    pub areas: Vec<BiomeType>,
}

impl Biomes {
    pub fn new() -> Self {
        Self { areas: Vec::new() }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BiomeType {
    pub name: String,
    pub min_temp: i8,
    pub max_temp: i8,
    pub min_rain: i8,
    pub max_rain: i8,
    pub min_mutation: u8,
    pub max_mutation: u8,
    pub occurs: Vec<BlockType>,
    pub soils: SoilTypes,
    pub trees: Vec<TreeType>,
    pub nouns: Vec<String>,
    pub color: Vec<f32>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SoilTypes {
    pub soil: i32,
    pub sand: i32,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TreeType {
    pub tree: String,
    pub freq: f32,
}
