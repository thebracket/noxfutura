use crate::planet::BlockType;
use ron::de::from_reader;
use serde::{Deserialize, Serialize};
use std::fs::File;

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

pub fn load_biomes() -> Biomes {
    let biome_path = "resources/raws/biomes.ron";
    let f = File::open(&biome_path).expect("Failed opening file");
    let biomes: Biomes = match from_reader(f) {
        Ok(x) => x,
        Err(e) => {
            println!("Failed to load biomes: {}", e);
            std::process::exit(1);
        }
    };
    biomes
}
