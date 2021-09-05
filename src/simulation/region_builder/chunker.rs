use crate::{
    geometry::Degrees,
    raws::RAWS,
    simulation::{
        chunk_idx, noise_lat, noise_lon, noise_to_planet_height, planet_idx, sphere_vertex, Planet,
        CHUNK_DEPTH, CHUNK_HEIGHT, CHUNK_SIZE, CHUNK_WIDTH,
    },
};
use bracket_noise::prelude::*;
use bracket_random::prelude::RandomNumberGenerator;

use super::strata::StrataMaterials;

#[derive(Clone, Copy, Debug)]
pub enum TileType {
    Empty,
    SemiMoltenRock,
    Solid { material: usize },
}

#[derive(Clone, Copy, Debug)]
pub enum ChunkType {
    Empty,
    Populated,
}

#[derive(Clone, Debug)]
pub struct Chunk {
    pub world: (usize, usize),
    pub base: (usize, usize, usize),
    pub chunk_type: ChunkType,
    pub tiles: Option<Vec<TileType>>,
}

impl Chunk {
    pub fn empty(
        tile_x: usize,
        tile_y: usize,
        region_x: usize,
        region_y: usize,
        region_z: usize,
    ) -> Self {
        Self {
            world: (tile_x, tile_y),
            base: (region_x, region_y, region_z),
            chunk_type: ChunkType::Empty,
            tiles: None,
        }
    }

    pub fn generate(
        planet: &Planet,
        strata: &StrataMaterials,
        tile_x: usize,
        tile_y: usize,
        region_x: usize,
        region_y: usize,
        region_z: usize,
    ) -> Self {
        let mut chunk = Chunk::empty(tile_x, tile_y, region_x, region_y, region_z);
        let lb_idx = planet_idx(tile_x, tile_y);
        let biome_idx = planet.landblocks[lb_idx].biome_idx;
        let biome = &RAWS.read().biomes.areas[biome_idx];

        // Determine the altitudes for this chunk
        let mut altitudes = vec![0; CHUNK_SIZE * CHUNK_SIZE];
        let noise = planet.get_height_noise();
        for y in region_y..region_y + CHUNK_HEIGHT {
            for x in region_x..region_x + CHUNK_WIDTH {
                let altitude = cell_altitude(&noise, tile_x, tile_y, x, y);
                let altitude_idx = ((y - region_y) * CHUNK_WIDTH) + (x - region_x);
                altitudes[altitude_idx] = altitude;
            }
        }

        let max_altitude = altitudes.iter().max().unwrap();
        if region_z > *max_altitude as usize {
            // There's nothing here - it's an empty cell
            chunk.chunk_type = ChunkType::Empty;
        } else {
            chunk.chunk_type = ChunkType::Populated;
            let mut tiles: Vec<TileType> =
                vec![TileType::Empty; CHUNK_HEIGHT * CHUNK_WIDTH * CHUNK_DEPTH];
            let cell_noise = planet.get_material_noise();
            let mut rng = RandomNumberGenerator::seeded(
                planet.rng_seed + (tile_x + tile_y + region_x + region_y + region_z) as u64,
            );
            for ry in region_y..region_y + CHUNK_HEIGHT {
                let cy = ry - region_y;
                for rx in region_x..region_x + CHUNK_WIDTH {
                    let cx = rx - region_x;
                    let altitude_idx = ((ry - region_y) * CHUNK_WIDTH) + (rx - region_x);
                    for cz in 0..CHUNK_DEPTH {
                        let rz = cz + region_z;
                        let altitude = altitudes[altitude_idx] as usize;
                        let idx = chunk_idx(cx, cy, cz);
                        if rz < altitude {
                            if rz < 1 {
                                // Semi molten rock
                                tiles[idx] = TileType::SemiMoltenRock;
                            } else if rz < altitude / 4 {
                                // Lava
                                tiles[idx] = TileType::Empty;
                                //tiles[idx].lava_level = 10;
                            } else if rz < altitude / 2 {
                                // Igneous only
                                let n = cell_noise.get_noise3d(
                                    noise_lon(tile_y, ry * 2),
                                    noise_lat(tile_x, rx * 2),
                                    rz as f32,
                                );
                                tiles[idx] = TileType::Solid {
                                    material: pick_material(&strata.igneous, n),
                                };
                            } else if rz < altitude - 4 {
                                // Igneouus or sedimentary
                                let n = cell_noise.get_noise3d(
                                    noise_lon(tile_y, ry * 2),
                                    noise_lat(tile_x, rx * 2),
                                    rz as f32,
                                );
                                if rng.range(0, 100) < 50 {
                                    tiles[idx] = TileType::Solid {
                                        material: pick_material(&strata.igneous, n),
                                    };
                                } else {
                                    tiles[idx] = TileType::Solid {
                                        material: pick_material(&strata.sedimentary, n),
                                    };
                                }
                            } else {
                                // Soil or sand
                                let n = cell_noise.get_noise3d(
                                    noise_lon(tile_y, ry * 2),
                                    noise_lat(tile_x, rx * 2),
                                    rz as f32,
                                );
                                if rng.roll_dice(1, 100) < biome.soils.soil {
                                    tiles[idx] = TileType::Solid {
                                        material: pick_material(&strata.soils, n),
                                    };
                                } else {
                                    tiles[idx] = TileType::Solid {
                                        material: pick_material(&strata.sand, n),
                                    };
                                }
                            }
                        }
                    }
                }
            }
            chunk.tiles = Some(tiles);
        }

        chunk
    }
}

fn cell_altitude(noise: &FastNoise, tile_x: usize, tile_y: usize, x: usize, y: usize) -> u32 {
    let lat = noise_lat(tile_y, y);
    let lon = noise_lon(tile_x, x);
    let sphere_coords = sphere_vertex(100.0, Degrees::new(lat), Degrees::new(lon));
    let noise_height = noise.get_noise3d(sphere_coords.0, sphere_coords.1, sphere_coords.2);
    noise_to_planet_height(noise_height)
}

fn pick_material(materials: &[usize], noise: f32) -> usize {
    let noise_normalized = (noise + 1.0) / 2.0;
    let n = 1.0 / materials.len() as f32;
    (noise_normalized * n) as usize
}
