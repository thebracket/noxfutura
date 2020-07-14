use crate::{ground_z, Region, TileType};
use bracket_geometry::prelude::*;
use bracket_random::prelude::*;
use legion::prelude::*;
use nox_raws::*;
use parking_lot::RwLock;
use nox_spatial::*;

lazy_static! {
    static ref TREE_COUNTER: RwLock<usize> = RwLock::new(1);
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
    println!("{}, {}, {}", d_chance, e_chance, biome.name);

    for y in 10..REGION_HEIGHT - 10 {
        for x in 10..REGION_WIDTH - 10 {
            let z = ground_z(region, x, y);
            let crash_distance = DistanceAlg::Pythagoras.distance2d(
                Point::new(REGION_WIDTH / 2, REGION_HEIGHT / 2),
                Point::new(x, y),
            );
            let idx = mapidx(x, y, z);
            if crash_distance > 20.0
                && region.tile_types[idx] == TileType::Floor
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

                if can_plant {
                    if (rng.roll_dice(1, 10) as f32) <= quality {
                        let mut die_roll = rng.roll_dice(1, 1000);
                        if die_roll < d_chance {
                            //plant_deciduous(x, y, z, rng, region);
                            crate::spawner::spawn_tree(ecs, x, y, z, region.world_idx)
                        } else {
                            die_roll = rng.roll_dice(1, 1000);
                            if die_roll < e_chance {
                                //plant_evergreen(x, y, z, rng, region);
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
