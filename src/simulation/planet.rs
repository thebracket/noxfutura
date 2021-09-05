use crate::raws::BlockType;
use bracket_noise::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::File;

#[derive(Clone, Serialize, Deserialize)]
pub struct Planet {
    pub rng_seed: u64,
    pub noise_seed: u64,
    pub landblocks: Vec<Landblock>,
    pub water_height: u32,
    pub plains_height: u32,
    pub hills_height: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Landblock {
    pub height: u32,
    pub variance: u32,
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

impl Planet {
    pub fn get_height_noise(&self) -> FastNoise {
        let mut noise = FastNoise::seeded(self.noise_seed);
        noise.set_noise_type(NoiseType::SimplexFractal);
        noise.set_fractal_type(FractalType::FBM);
        noise.set_fractal_octaves(5);
        noise.set_fractal_gain(0.5);
        noise.set_fractal_lacunarity(3.2);
        noise.set_frequency(0.01);
        noise
    }

    pub fn get_material_noise(&self) -> FastNoise {
        let mut cell_noise = FastNoise::seeded(self.noise_seed + 1);
        cell_noise.set_cellular_return_type(CellularReturnType::CellValue);
        cell_noise.set_noise_type(NoiseType::Cellular);
        cell_noise.set_frequency(0.08);
        cell_noise.set_cellular_distance_function(CellularDistanceFunction::Manhattan);
        cell_noise
    }
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

pub fn load_planet() -> Planet {
    use std::io::Read;
    use std::path::Path;
    let savepath = Path::new("savegame/world.dat");
    if !savepath.exists() {
        panic!("Saved game doesn't exist");
    }

    let mut f = File::open(&savepath).expect("Unable to open file");
    let mut buffer = Vec::<u8>::new();
    println!("Reading file");
    f.read_to_end(&mut buffer).expect("Unable to read file");
    let raw_bytes =
        miniz_oxide::inflate::decompress_to_vec(&buffer).expect("Unable to decompress file");
    println!("Decompressing file");

    println!("Deserializing");
    let saved: Planet = bincode::deserialize(&raw_bytes).expect("Unable to deserialize");
    println!("Done");
    saved
}
