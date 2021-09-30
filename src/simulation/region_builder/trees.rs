use bevy::math::Vec3;
use bracket_random::prelude::RandomNumberGenerator;

use crate::components::PlanetLocation;
use crate::raws::{MaterialLayer, RAWS};
use crate::simulation::spawner::spawn_tree;
use crate::simulation::terrain::PLANET_STORE;
use crate::simulation::terrain::{get_material_idx, ground_z, is_tile_floor};
use crate::simulation::{mapidx, REGION_HEIGHT, REGION_WIDTH};

pub(crate) fn plant_trees(region_id: PlanetLocation) {
    let mut rng = RandomNumberGenerator::seeded(
        PLANET_STORE.read().planet.as_ref().unwrap().noise_seed
            + region_id.to_region_index() as u64,
    );
    let biome_idx = PLANET_STORE.read().planet.as_ref().unwrap().landblocks
        [region_id.to_region_index()]
    .biome_idx;
    let biome = &RAWS.read().biomes.areas[biome_idx];

    let mut deciduous_chance = 0;
    let mut evergreen_chance = 0;
    for t in biome.trees.iter() {
        if t.tree.to_lowercase() == "d" {
            deciduous_chance = t.freq as i32;
        } else if t.tree.to_lowercase() == "e" {
            evergreen_chance = t.freq as i32;
        }
    }

    for y in 1..REGION_HEIGHT - 1 {
        for x in 1..REGION_WIDTH - 1 {
            let z = ground_z(region_id, x, y);
            let crash_distance =
                Vec3::new(REGION_WIDTH as f32 / 2.0, REGION_HEIGHT as f32 / 2.0, 0.0)
                    .distance(Vec3::new(x as f32, y as f32, 0.0));
            let tile_idx = mapidx(x, y, z);
            if crash_distance > 20.0 && is_tile_floor(region_id, tile_idx) {
                let mat_idx = get_material_idx(region_id, tile_idx);
                let floor_material = &RAWS.read().materials.materials[mat_idx];
                let (can_plant, quality) = match floor_material.layer {
                    MaterialLayer::Sand => (true, 2.0),
                    MaterialLayer::Soil { quality } => (true, quality as f32),
                    _ => (false, 0.0),
                };

                if can_plant {
                    if (rng.roll_dice(1, 10) as f32) < quality {
                        let die_roll = rng.roll_dice(1, 1000);
                        if die_roll < deciduous_chance {
                            spawn_tree(region_id, tile_idx);
                        } else if die_roll < evergreen_chance {
                            spawn_tree(region_id, tile_idx);
                        }
                    }
                }
            }
        }
    }
}
