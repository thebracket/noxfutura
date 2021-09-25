use super::{PlanetLocation, TileType};
use crate::raws::RAWS;
use crate::simulation::terrain::PLANET_STORE;
use crate::{
    geometry::Degrees,
    simulation::{
        mapidx, noise_lat, noise_lon, noise_to_planet_height, sphere_vertex, REGION_DEPTH,
        REGION_HEIGHT, REGION_TILES_COUNT, REGION_WIDTH,
    },
};
use bracket_noise::prelude::FastNoise;
use bracket_random::prelude::RandomNumberGenerator;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum RegionStatus {
    NotLoaded,
    CreatingTiles,
    CreatedTiles,
}

#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum RegionRequirement {
    Unimportant,
    Peek,
    Required,
}

#[derive(Clone)]
pub struct Region {
    pub location: PlanetLocation,
    pub required: RegionRequirement,
    pub status: RegionStatus,
    pub should_rechunk: bool,

    pub tile_types: Vec<TileType>,
    pub material: Vec<usize>,
    pub revealed: Vec<bool>,
}

impl Region {
    pub fn new(location: PlanetLocation, required: RegionRequirement) -> Self {
        Self {
            location,
            required,
            status: RegionStatus::NotLoaded,
            tile_types: vec![TileType::Empty; REGION_TILES_COUNT],
            material: vec![0; REGION_TILES_COUNT],
            revealed: vec![false; REGION_TILES_COUNT],
            should_rechunk: false,
        }
    }

    pub fn build_tiles(&mut self) {
        // Sanity check
        if self.status != RegionStatus::CreatingTiles {
            panic!("Region status invalid");
        }

        // Obtain resources
        let plock = PLANET_STORE.read();
        let planet = plock.planet.as_ref().unwrap();
        let strata = plock.strata.as_ref().unwrap();
        let noise = plock.height_noise.as_ref().unwrap();
        let cell_noise = plock.material_noise.as_ref().unwrap();
        let tile_x = self.location.x;
        let tile_y = self.location.y;
        let biome_idx = planet.landblocks[self.location.to_region_index()].biome_idx;
        let biome = &RAWS.read().biomes.areas[biome_idx];

        // Determine base altitudes for the region
        let mut altitudes = vec![0; REGION_WIDTH * REGION_HEIGHT];
        for y in 0..REGION_HEIGHT {
            for x in 0..REGION_WIDTH {
                let altitude = cell_altitude(&noise, tile_x, tile_y, x, y);
                let altitude_idx = (y * REGION_WIDTH) + x;
                altitudes[altitude_idx] = altitude;
            }
        }

        // Build basic terrain based on noise patterns
        let mut rng = RandomNumberGenerator::seeded(
            planet.noise_seed + ((tile_y * REGION_WIDTH) + tile_x) as u64,
        );
        for y in 0..REGION_HEIGHT {
            for x in 0..REGION_WIDTH {
                let altitude_idx = (y * REGION_WIDTH) + x;
                let altitude = altitudes[altitude_idx] as usize;
                for z in 0..REGION_DEPTH {
                    let idx = mapidx(x, y, z);

                    // Reveal if above ground
                    if z > altitude - 2 {
                        self.revealed[idx] = true;
                    } else {
                        self.revealed[idx] = false;
                    }

                    // Layers
                    if z < 2 {
                        self.tile_types[idx] = TileType::SemiMoltenRock;
                    } else if z < altitude / 4 {
                        // Lava
                        self.tile_types[idx] = TileType::Empty;
                        //tiles[idx].lava_level = 10;
                    } else if z < altitude / 2 {
                        // Igneous only
                        let n = cell_noise.get_noise3d(
                            noise_lon(tile_y, y * 2),
                            noise_lat(tile_x, x * 2),
                            z as f32,
                        );
                        self.tile_types[idx] = TileType::Solid;
                        self.material[idx] = pick_material(&strata.igneous, n);
                    } else if z < altitude - 4 {
                        // Igneous or sedimentary
                        let n = cell_noise.get_noise3d(
                            noise_lon(tile_y, y * 2),
                            noise_lat(tile_x, x * 2),
                            z as f32,
                        );
                        if rng.range(0, 100) < 50 {
                            self.tile_types[idx] = TileType::Solid;
                            self.material[idx] = pick_material(&strata.igneous, n);
                        } else {
                            self.tile_types[idx] = TileType::Solid;
                            self.material[idx] = pick_material(&strata.sedimentary, n);
                        }
                    } else if z < altitude {
                        // Soil or sand
                        let n = cell_noise.get_noise3d(
                            noise_lon(tile_y, y * 2),
                            noise_lat(tile_x, x * 2),
                            z as f32,
                        );
                        if rng.roll_dice(1, 100) < biome.soils.soil {
                            self.tile_types[idx] = TileType::Solid;
                            self.material[idx] = pick_material(&strata.soils, n);
                        } else {
                            self.tile_types[idx] = TileType::Solid;
                            self.material[idx] = pick_material(&strata.sand, n);
                        }
                    }
                }
            }
        }

        // Finish up
        self.status = RegionStatus::CreatedTiles;
        self.should_rechunk = true;
        println!("Made region");
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
    let n = materials.len() as f32 / 1.0;
    materials[(noise_normalized * n) as usize]
}
