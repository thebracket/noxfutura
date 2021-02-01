use crate::{ground_z, Region, TileType};
use bengine::geometry::*;
use bengine::random::*;
use legion::*;
use nox_raws::*;
use nox_spatial::*;
use parking_lot::RwLock;

lazy_static! {
    static ref TREE_COUNTER: RwLock<usize> = RwLock::new(1);
}

fn random_tree_height(rng: &mut RandomNumberGenerator) -> f32 {
    let n = rng.range(40, 100);
    n as f32 / 100.0
}

pub fn plant_trees(
    region: &mut Region,
    biome: &BiomeType,
    rng: &mut RandomNumberGenerator,
    ecs: &mut World,
) {
    let mut d_chance = 0;
    let mut e_chance = 0;
    for t in biome.trees.iter() {
        if t.tree.to_lowercase() == "d" {
            d_chance = t.freq as i32;
        } else if t.tree.to_lowercase() == "e" {
            e_chance = t.freq as i32;
        }
    }
    //println!("{}, {}, {}", d_chance, e_chance, biome.name);

    for y in 10..REGION_HEIGHT - 10 {
        for x in 10..REGION_WIDTH - 10 {
            let z = ground_z(region, x, y);
            let crash_distance = DistanceAlg::Pythagoras.distance2d(
                Point::new(REGION_WIDTH / 2, REGION_HEIGHT / 2),
                Point::new(x, y),
            );
            let idx = mapidx(x, y, z);
            if crash_distance > 20.0
                && region.is_floor(idx)
                && region.water_level[idx] == 0
                && can_see_sky(region, x, y, z)
            {
                let mat_idx = region.material_idx[idx];
                let floor_material = &RAWS.read().materials.materials[mat_idx];
                let (can_plant, quality) = match floor_material.layer {
                    MaterialLayer::Sand => (true, 2.0),
                    MaterialLayer::Soil { quality } => (true, quality as f32),
                    _ => (false, 0.0),
                };

                const MAX_TREE: usize = 7;
                if can_plant && can_see_sky(region, x, y, z) {
                    if (rng.roll_dice(1, 10) as f32) <= quality {
                        let mut die_roll = rng.roll_dice(1, 1000);
                        if die_roll < d_chance {
                            crate::spawner::spawn_tree(
                                ecs,
                                x,
                                y,
                                z,
                                region.world_idx,
                                rng.range(0, MAX_TREE) as usize,
                                random_tree_height(rng),
                            )
                        } else {
                            die_roll = rng.roll_dice(1, 1000);
                            if die_roll < e_chance {
                                crate::spawner::spawn_tree(
                                    ecs,
                                    x,
                                    y,
                                    z,
                                    region.world_idx,
                                    rng.range(0, MAX_TREE) as usize,
                                    random_tree_height(rng),
                                )
                            }
                        }
                    }
                }
            }
        }
    }
}

fn can_see_sky(region: &Region, x: usize, y: usize, z: usize) -> bool {
    let mut sz = z + 1;
    while sz < REGION_DEPTH {
        if region.tile_types[mapidx(x, y, sz)] != TileType::Empty {
            return false;
        }
        sz += 1;
    }
    true
}
