use super::{set_worldgen_status, PLANET_BUILD};
use crate::{planet_idx, Biome, BlockType, Planet};
use bracket_geometry::prelude::*;
use bracket_random::prelude::*;
use nox_spatial::{WORLD_HEIGHT, WORLD_TILES_COUNT, WORLD_WIDTH};
use std::collections::HashMap;

type BiomeCounts = HashMap<BlockType, i32>;

pub fn build_biomes() {
    set_worldgen_status("Growing Biomes");

    let mut planet = PLANET_BUILD.lock().planet.clone();
    let seed = PLANET_BUILD.lock().planet.rng_seed;
    let mut rng = RandomNumberGenerator::seeded(seed);
    let n_biomes = WORLD_TILES_COUNT / 64 + rng.roll_dice(1, 32) as usize;

    let mut centroids: Vec<(i32, i32)> = Vec::new();
    for _ in 0..n_biomes {
        centroids.push((
            rng.roll_dice(1, WORLD_WIDTH as i32 - 1),
            rng.roll_dice(1, WORLD_HEIGHT as i32 - 1),
        ));
    }

    set_worldgen_status(format!("Scanning {} Biomes.", n_biomes));
    planet.biomes = vec![Biome::new(); n_biomes as usize];

    for y in 0..WORLD_HEIGHT {
        for x in 0..WORLD_WIDTH {
            let mut distance = std::i32::MAX;
            let mut closest_index = -1;

            for (i, biome) in centroids.iter().enumerate() {
                let biome_distance = DistanceAlg::Pythagoras
                    .distance2d(Point::new(x, y), Point::new(biome.0, biome.1));
                if (biome_distance as i32) < distance {
                    distance = biome_distance as i32;
                    closest_index = i as i32;
                }
            }

            let pidx = planet_idx(x, y);
            planet.landblocks[pidx].biome_idx = closest_index as usize;
        }
    }

    set_worldgen_status("Hand-crafting Fjords");
    let mut count = 0;
    while count < planet.biomes.len() {
        let membership_count = biome_membership(&mut planet, count);
        if !membership_count.is_empty() {
            let possible_types = find_possible_biomes(&membership_count, &planet.biomes[count]);
            let biome_index = pick_random_biome(&possible_types, &mut rng);
            if let Some(biome_index) = biome_index {
                planet.biomes[count].biome_type = biome_index;
                planet.biomes[count].name = name_biome(&planet.biomes[count]);
            }
        }

        count += 1;
    }

    PLANET_BUILD.lock().planet.landblocks = planet.landblocks;
    PLANET_BUILD.lock().planet.biomes = planet.biomes;

    set_worldgen_status("Biomes are cooked.");
}

fn biome_membership(planet: &mut Planet, idx: usize) -> BiomeCounts {
    let mut counts: BiomeCounts = HashMap::new();
    let mut n_cells = 0;
    let mut total_temperature = 0i32;
    let mut total_rainfall = 0i32;
    let mut total_height = 0i32;
    let mut total_variance = 0i32;
    let mut total_x = 0i32;
    let mut total_y = 0i32;

    for (i, lb) in planet
        .landblocks
        .iter()
        .filter(|lb| lb.biome_idx == idx)
        .enumerate()
    {
        n_cells += 1;
        total_temperature += lb.temperature as i32;
        total_rainfall += lb.rainfall as i32;
        total_height += lb.height as i32;
        total_variance = lb.variance as i32;
        total_x += (i % WORLD_WIDTH as usize) as i32;
        total_y += (i / WORLD_WIDTH as usize) as i32;

        if counts.contains_key(&lb.btype) {
            let old_count = counts[&lb.btype];
            counts.insert(lb.btype, old_count + 1);
        } else {
            counts.insert(lb.btype, 1);
        }
    }

    // Calculate the averages
    if n_cells == 0 {
        n_cells = 1
    }
    let counter = n_cells as f32;
    planet.biomes[idx].mean_altitude = (total_height as f32 / counter) as u8;
    planet.biomes[idx].mean_rainfall = (total_rainfall as f32 / counter) as i8;
    planet.biomes[idx].mean_temperature = (total_temperature as f32 / counter) as i8;
    planet.biomes[idx].mean_variance = (total_variance as f32 / counter) as u8;
    planet.biomes[idx].center = Point::new(total_x / n_cells, total_y / n_cells);

    let distance_from_pole = f32::min(
        DistanceAlg::Pythagoras
            .distance2d(planet.biomes[idx].center, Point::new(WORLD_WIDTH / 2, 0)),
        DistanceAlg::Pythagoras.distance2d(
            planet.biomes[idx].center,
            Point::new(WORLD_WIDTH / 2, WORLD_HEIGHT - 1),
        ),
    );
    let distance_from_center = DistanceAlg::Pythagoras.distance2d(
        planet.biomes[idx].center,
        Point::new(planet.biomes[idx].center.x, WORLD_HEIGHT as i32 / 2),
    );

    // Warp mutation and Savageness
    if distance_from_pole > 200.0 {
        planet.biomes[idx].warp_mutation = 0;
    } else {
        planet.biomes[idx].warp_mutation = (200 - distance_from_pole as u8) / 2;
    }
    planet.biomes[idx].savagery = u8::min(100, distance_from_center as u8);

    counts
}

fn find_possible_biomes(membership: &BiomeCounts, biome: &Biome) -> Vec<(usize, i32)> {
    use nox_raws::RAWS;
    let mut result: Vec<(usize, i32)> = Vec::new();

    let raws = RAWS.read();
    for (i, biome) in raws.biomes.areas.iter().enumerate().filter(|(_, b)| {
        biome.mean_temperature >= b.min_temp
            && biome.mean_temperature <= b.max_temp
            && biome.mean_rainfall >= b.min_rain
            && biome.mean_rainfall <= b.max_rain
            && biome.warp_mutation >= b.min_mutation
            && biome.warp_mutation <= b.max_mutation
    }) {
        for bt in biome.occurs.iter() {
            if membership.contains_key(bt) && membership[&bt] > 0 {
                result.push((i, membership[bt]));
            }
        }
    }

    result
}

fn pick_random_biome(
    distribution: &Vec<(usize, i32)>,
    rng: &mut RandomNumberGenerator,
) -> Option<usize> {
    if distribution.len() == 1 {
        return Some(distribution[0].0);
    }
    if distribution.is_empty() {
        return None;
    }

    let sum: usize = distribution.iter().map(|(_, pct)| (*pct) as usize).sum();
    if sum == 0 {
        return Some(distribution[0].0);
    }
    let roll = rng.range(0, sum);
    let mut cumulative = 0;
    for item in distribution.iter() {
        cumulative += item.1 as usize;
        if (roll as usize) < cumulative {
            return Some(item.0);
        }
    }
    Some(distribution[distribution.len() - 1].0)
}

fn name_biome(biome: &Biome) -> String {
    // TODO: Incomplete
    let result = String::from("Nameless");
    let mut adjectives: Vec<String> = Vec::new();

    // Location-based
    if i32::abs(biome.center.x - WORLD_WIDTH as i32 / 2) < WORLD_WIDTH as i32 / 10
        && i32::abs(biome.center.y - WORLD_HEIGHT as i32 / 2) < WORLD_HEIGHT as i32 / 10
    {
        adjectives.push(String::from("Central"));
    } else {
        if biome.center.x < (WORLD_WIDTH as i32 / 2) {
            adjectives.push(String::from("Western"))
        }
    }

    result
}
