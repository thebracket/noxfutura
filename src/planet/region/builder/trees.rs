use crate::planet::{Region, REGION_WIDTH, REGION_HEIGHT, REGION_DEPTH, TileType};
use crate::raws::*;
use crate::utils::{ground_z, mapidx};
use bracket_geometry::prelude::*;
use bracket_random::prelude::*;
use parking_lot::RwLock;

lazy_static! {
    static ref TREE_COUNTER: RwLock<usize> = RwLock::new(1);
}

pub fn plant_trees(region: &mut Region, biome: &BiomeType, rng: &mut RandomNumberGenerator) {
    reset_tree_id();

    let mut d_chance = 0;
    let mut e_chance = 0;
    for t in biome.trees.iter() {
        if t.tree.to_lowercase() == "d" {
            d_chance = (t.freq * 10.0) as i32;
        } else if t.tree.to_lowercase() == "e" {
            e_chance = (t.freq * 10.0) as i32;
        }
    }
    println!("{}, {}", d_chance, e_chance);

    for y in 10..REGION_HEIGHT-10 {
        for x in 10..REGION_WIDTH-10 {
            let z = ground_z(region, x, y);
            let crash_distance = DistanceAlg::Pythagoras.distance2d(
                Point::new(REGION_WIDTH/2, REGION_HEIGHT/2), 
                Point::new(x, y)
            );
            let idx = mapidx(x, y, z);
            if crash_distance > 20.0 && region.tile_types[idx] == TileType::Floor && region.water_level[idx]==0 && can_see_sky(region, x, y, z) {
                let mut die_roll = rng.roll_dice(1, 1000);
                if die_roll < d_chance {
                    plant_deciduous(x, y, z, rng, region);
                } else {
                    die_roll = rng.roll_dice(1, 1000);
                    if die_roll < e_chance {
                        plant_evergreen(x, y, z, rng, region);
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

fn plant_deciduous(x: usize, y: usize, z: usize, rng: &mut RandomNumberGenerator, region: &mut Region) {
    let tree_height = 1 + rng.roll_dice(2, 4) as usize;
    for i in 0..tree_height {
        set_tree_trunk(x, y, z+i, next_tree_id(), region);
        if i > tree_height/2 {
            let radius = (tree_height - i) + 1;
            for tx in x-radius .. x+radius {
                for ty in y-radius .. y+radius {
                    let distance = DistanceAlg::Pythagoras.distance2d(
                        Point::new(x, y),
                        Point::new(tx, ty)
                    );
                    if distance < radius as f32 && (tx != x || ty !=y) {
                        set_tree_foliage(tx, ty, z+i, next_tree_id(), region);
                    }
                }
            }
        }
    }
    inc_next_tree();
}

fn plant_evergreen(x: usize, y: usize, z: usize, rng: &mut RandomNumberGenerator, region: &mut Region) {
    let tree_height = 1 + rng.roll_dice(2, 3) as usize;
    for i in 0..tree_height {
        set_tree_trunk(x, y, z+i, next_tree_id(), region);
        if i > tree_height/2 {
            let radius = (tree_height - i) + 1;
            for tx in x-radius .. x+radius {
                for ty in y-radius .. y+radius {
                    let distance = DistanceAlg::Pythagoras.distance2d(
                        Point::new(x, y),
                        Point::new(tx, ty)
                    );
                    if distance < radius as f32 && (tx != x || ty !=y) {
                        set_tree_foliage(tx, ty, z+i, next_tree_id(), region);
                    }
                }
            }
        }
    }
    inc_next_tree();
}

fn set_tree_trunk(x: usize, y: usize, z: usize, tree_id: usize, region: &mut Region) {
    if x > 0 && y > 0 && z > 0 && x < REGION_WIDTH-1 && y < REGION_HEIGHT-1 && z < REGION_DEPTH-2 {
        let idx = mapidx(x, y, z);
        region.tile_types[idx] = TileType::TreeTrunk;
        region.tree_id[idx] = tree_id;
    }
}

fn set_tree_foliage(x: usize, y: usize, z: usize, tree_id: usize, region: &mut Region) {
    if x > 0 && y > 0 && z > 0 && x < REGION_WIDTH-1 && y < REGION_HEIGHT-1 && z < REGION_DEPTH-2 {
        let idx = mapidx(x, y, z);
        region.tile_types[mapidx(x,y,z)] = TileType::TreeFoliage;
        region.tree_id[idx] = tree_id;
    }
}

fn reset_tree_id() {
    *TREE_COUNTER.write() = 1;
}

fn inc_next_tree() {
    *TREE_COUNTER.write() += 1;
}

fn next_tree_id() -> usize {
    *TREE_COUNTER.read()
}