use crate::raws::BlockType;
use serde::{Deserialize, Serialize};
use std::fs::File;

#[derive(Clone, Serialize, Deserialize)]
pub struct Planet {
    pub rng_seed: u64,
    pub noise_seed: u64,
    pub landblocks: Vec<Landblock>,
    pub water_height: u8,
    pub plains_height: u8,
    pub hills_height: u8,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Landblock {
    pub height: u8,
    pub variance: u8,
    pub btype: BlockType,
    pub temperature_c: f32,
    pub rainfall_mm: i32,
    pub air_pressure_kpa: f32,
    pub prevailing_wind: Direction,
    pub biome_idx: usize,
    pub neighbors: [(Direction, usize); 4],
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub enum Direction {
    North,
    South,
    East,
    West,
    None,
}

pub fn save_planet(state: Planet) {
    use std::io::Write;
    let mut world_file = File::create("savegame/world.dat").unwrap();
    let mem_vec = bincode::serialize(&state).expect("Unable to binary serialize");
    let compressed_bytes = miniz_oxide::deflate::compress_to_vec(&mem_vec, 6);
    world_file
        .write_all(&compressed_bytes)
        .expect("Unable to write file data");
}
