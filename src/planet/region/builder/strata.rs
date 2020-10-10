use crate::planet::{Region, TileType};
use nox_raws::*;
use nox_spatial::{mapidx, REGION_DEPTH, REGION_HEIGHT, REGION_WIDTH};
use bracket_noise::prelude::*;
use bengine::random::RandomNumberGenerator;

fn get_strata_indices(st: MaterialLayer) -> Vec<usize> {
    let mlock = RAWS.read();
    mlock
        .materials
        .materials
        .iter()
        .enumerate()
        .filter(|(_, m)| m.layer == st)
        .map(|(i, _)| i)
        .collect()
}

fn get_soil_indices() -> Vec<usize> {
    let mlock = RAWS.read();
    mlock
        .materials
        .materials
        .iter()
        .enumerate()
        .filter(|(_, m)| match m.layer {
            MaterialLayer::Soil { .. } => true,
            _ => false,
        })
        .map(|(i, _)| i)
        .collect()
}

fn get_strata_materials() -> (Vec<usize>, Vec<usize>, Vec<usize>, Vec<usize>) {
    (
        get_soil_indices(),
        get_strata_indices(MaterialLayer::Sand),
        get_strata_indices(MaterialLayer::Sedimentary),
        get_strata_indices(MaterialLayer::Igneous),
    )
}

pub struct Strata {
    pub map: Vec<usize>,
    pub material_idx: Vec<usize>,
    pub counts: Vec<(usize, usize, usize, usize)>,
}

pub fn build_strata(
    rng: &mut RandomNumberGenerator,
    hm: &[u8],
    biome: &BiomeType,
    perlin_seed: u64,
) -> Strata {
    const REGION_TILES_COUNT: usize = REGION_WIDTH * REGION_HEIGHT * REGION_DEPTH;
    let (soils, sands, sedimentaries, igeneouses) = get_strata_materials();
    let n_strata = 1000;
    let mut result = Strata {
        map: vec![1; REGION_TILES_COUNT],
        material_idx: vec![1; n_strata],
        counts: vec![(0, 0, 0, 0); n_strata],
    };

    let mut cell_noise = FastNoise::seeded(perlin_seed + 4);
    cell_noise.set_cellular_return_type(CellularReturnType::CellValue);
    cell_noise.set_noise_type(NoiseType::Cellular);
    cell_noise.set_frequency(0.08);
    cell_noise.set_cellular_distance_function(CellularDistanceFunction::Manhattan);
    for z in 0..REGION_DEPTH {
        for y in 0..REGION_HEIGHT {
            for x in 0..REGION_WIDTH {
                let noise = cell_noise.get_noise3d(x as f32, y as f32, z as f32);
                let biome_idx = (((noise + 1.0) / 2.0) * n_strata as f32) as usize;

                result.counts[biome_idx].0 += 1;
                result.counts[biome_idx].1 += x;
                result.counts[biome_idx].2 += y;
                result.counts[biome_idx].3 += z;
                let map_idx = mapidx(x, y, z);
                result.map[map_idx] = biome_idx;
            }
        }
    }

    //let mut count_used = 0;
    for i in 0..n_strata {
        if result.counts[i].0 > 0 {
            //count_used += 1;
            result.counts[i].1 /= result.counts[i].0;
            result.counts[i].2 /= result.counts[i].0;
            result.counts[i].3 /= result.counts[i].0;

            let (_n, x, y, z) = result.counts[i];
            let altitude_at_point = hm[(y * REGION_WIDTH) + x];
            let mat_idx = if z as u8 > altitude_at_point - (1 + rng.roll_dice(1, 4) as u8) {
                // Soil
                let roll = rng.roll_dice(1, 100);
                if roll < biome.soils.soil {
                    rng.random_slice_entry(&soils)
                } else {
                    rng.random_slice_entry(&sands)
                }
            } else if z as u8 > ((altitude_at_point - 10) / 2) {
                // Sedimentary
                rng.random_slice_entry(&sedimentaries)
            } else {
                // Igneous
                rng.random_slice_entry(&igeneouses)
            };
            result.material_idx[i] = *mat_idx.unwrap();
        } else {
            result.material_idx[i] = *rng.random_slice_entry(&igeneouses).unwrap();
        }
    }

    result
}

pub fn layer_cake(
    hm: &[u8],
    region: &mut Region,
    strata: &Strata,
    rng: &mut RandomNumberGenerator,
) {
    // Clear it
    region
        .tile_types
        .iter_mut()
        .for_each(|tt| *tt = TileType::Empty);

    let soils = get_soil_indices();

    // Build layered tiles
    //let x = 4;
    for x in 0..REGION_WIDTH {
        for y in 0..REGION_HEIGHT {
            let mut altitude = hm[(y * REGION_WIDTH) + x] as usize;
            if altitude > REGION_DEPTH - 10 {
                altitude = REGION_DEPTH - 1
            };

            // Bottom layer is always SMR
            region.tile_types[mapidx(x, y, 0)] = TileType::SemiMoltenRock;

            // Add lava above the bottom
            let mut z = 1;
            while z < altitude / 3 {
                let cell_idx = mapidx(x, y, z);
                if x == 0 || x == REGION_WIDTH - 1 || y == 0 || y == REGION_HEIGHT - 1 {
                    region.tile_types[cell_idx] = TileType::SemiMoltenRock;
                } else {
                    region.tile_types[cell_idx] = TileType::Empty;
                    // Just add magma
                    region.material_idx[cell_idx] = 0;
                }
                z += 1;
            }

            // Next is rock until the soil layer
            while z < altitude - 1 {
                let cell_idx = mapidx(x, y, z);
                region.tile_types[cell_idx] = TileType::Solid;
                let mat_idx = strata.map[cell_idx];
                region.material_idx[cell_idx] = strata.material_idx[mat_idx];
                z += 1;
            }

            // Add a top floor
            let cell_idx = mapidx(x, y, z);
            region.tile_types[cell_idx] = TileType::Floor;
            let mat_idx = strata.map[cell_idx];
            region.material_idx[cell_idx] = *rng
                .random_slice_entry(&soils)
                .unwrap_or(&strata.material_idx[mat_idx]);

            // Temporary reveal code
            //z -= 1;
            while z < REGION_DEPTH {
                let cell_idx = mapidx(x, y, z);
                region.revealed[cell_idx] = true;

                if x > 1 && x < REGION_WIDTH - 2 && y > 1 && y < REGION_HEIGHT - 2 {
                    for oy in -1..=1 {
                        for ox in -1..=1 {
                            let cell_idx =
                                mapidx((x as i32 + ox) as usize, (y as i32 + oy) as usize, z);
                            region.revealed[cell_idx] = true;
                        }
                    }
                }

                z += 1;
            }
        }
    }
}
