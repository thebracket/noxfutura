use super::{set_worldgen_status, PLANET_BUILD, WORLDGEN_RENDER};
use crate::planet::{planet_idx, BlockType, Planet, WORLD_HEIGHT, WORLD_TILES_COUNT, WORLD_WIDTH};
use bracket_random::prelude::*;
use bracket_geometry::prelude::*;
use std::collections::HashMap;

pub fn build_biomes() {
    set_worldgen_status("Growing Biomes");

    let seed = PLANET_BUILD.lock().planet.rng_seed;
    let mut rng = RandomNumberGenerator::seeded(seed);
    let n_biomes = WORLD_TILES_COUNT / 64 + rng.roll_dice(1, 32) as u16;

    let mut centroids : Vec<(i32,i32)> = Vec::new();
    for _ in 0..n_biomes {
        centroids.push(
            (
                rng.roll_dice(1, WORLD_WIDTH as i32 -1),
                rng.roll_dice(1, WORLD_HEIGHT as i32 -1)
            )
        );
    }

    set_worldgen_status(format!("Scanning {} Biomes.", n_biomes));

    for y in 0..WORLD_HEIGHT {
        for x in 0..WORLD_WIDTH {
            let mut distance = std::i32::MAX;
            let mut closest_index = -1;

            for (i,biome) in centroids.iter().enumerate() {
                let biome_distance = DistanceAlg::Pythagoras.distance2d(Point::new(x, y), Point::new(biome.0, biome.1));
                if (biome_distance as i32) < distance {
                    distance = biome_distance as i32;
                    closest_index = i as i32;
                }
            }

            let pidx = planet_idx(x as i32, y as i32);
            PLANET_BUILD.lock().planet.landblocks[pidx].biome_idx = closest_index as usize;
        }
    }

    set_worldgen_status("Hand-crafting Fjords");
    let mut count = 0;
    let mut no_match = 0;
    let mut planet = PLANET_BUILD.lock().planet.clone();
    while count < planet.biomes.len() {
        let membership_count = biome_membership(&mut planet, count);
        if !membership_count.is_empty() {

        } else {
            no_match += 1;
        }

        count += 1;
    }

    set_worldgen_status("Biomes are cooked.");
    println!("Biomes that didn't match: {}", no_match);
}

fn biome_membership(planet : &mut Planet, idx: usize) -> HashMap<u8, f32> {
    let mut percents : HashMap<u8, f32> = HashMap::new();
    let mut counts : HashMap<u8, i32> = HashMap::new();
    let mut n_cells = 0;
    let mut total_temperature = 0i32;
    let mut total_rainfall = 0i32;
    let mut total_height = 0i32;
    let mut total_variance = 0i32;
    let mut total_x = 0i32;
    let mut total_y = 0i32;

    for y in 0..WORLD_HEIGHT as i32 {
        for x in 0..WORLD_WIDTH as i32 {
            let block_idx = planet_idx(x, y);

            if planet.landblocks[block_idx].biome_idx == idx {
                let b = &planet.landblocks[block_idx];
                n_cells += 1;
                total_temperature += b.temperature as i32;
                total_rainfall += b.rainfall as i32;
                total_height += b.height as i32;
                total_variance += b.variance as i32;
                total_x += x;
                total_y += y;

                // Increment counts by cell type
                let c_index = b.btype as u8;
                if counts.contains_key(&c_index) {
                    let old_c = counts[&c_index];
                    counts.insert(c_index, old_c + 1);
                } else {
                    counts.insert(c_index, 1);
                }
            }
        }
    }

    // Calculate the averages
    if n_cells == 0 { n_cells = 1}
    let counter = n_cells as f32;
    planet.biomes[idx].mean_altitude = (total_height as f32 / counter) as u8;
    planet.biomes[idx].mean_rainfall = (total_rainfall as f32 / counter) as i8;
    planet.biomes[idx].mean_temperature = (total_temperature as f32 / counter) as i8;
    planet.biomes[idx].mean_variance = (total_variance as f32 / counter) as u8;
    planet.biomes[idx].center = Point::new(total_x / n_cells, total_y / n_cells);

    let distance_from_pole = f32::min(
        DistanceAlg::Pythagoras.distance2d(planet.biomes[idx].center, Point::new(WORLD_WIDTH/2, 0)),
        DistanceAlg::Pythagoras.distance2d(planet.biomes[idx].center, Point::new(WORLD_WIDTH/2, WORLD_HEIGHT-1))
    );
    let distance_from_center = DistanceAlg::Pythagoras.distance2d(planet.biomes[idx].center, Point::new(planet.biomes[idx].center.x, WORLD_HEIGHT as i32/2));

    // Warp mutation and Savageness
    if distance_from_pole > 200.0 {
        planet.biomes[idx].warp_mutation = 0;
    } else {
        planet.biomes[idx].warp_mutation = (200 - distance_from_pole as u8)/2;
    }
    planet.biomes[idx].savagery = u8::min(100, distance_from_center as u8);

    // Percentage counts - 10 is the max blocktype
    for i in 0..10 {
        if !counts.contains_key(&i) {
            percents.insert(i, 0.0);
        } else {
            let pct = counts[&i] as f32 / counter;
            percents.insert(i, pct);
        }
    }
    percents
}