use crate::raws::BlockType;

pub struct Planet {
    pub rng_seed: u64,
    pub noise_seed: u64,
    pub landblocks: Vec<Landblock>,
    pub water_height: u8,
    pub plains_height: u8,
    pub hills_height: u8,
}

#[derive(Clone, Debug)]
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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Direction {
    North,
    South,
    East,
    West,
    None,
}
