use crate::{ground_z, Region, TileType};
use bracket_geometry::prelude::*;
use bracket_random::prelude::*;
use legion::prelude::*;
use nox_raws::*;
use nox_spatial::*;
use parking_lot::RwLock;

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

                if can_plant {
                    if (rng.roll_dice(1, 10) as f32) <= quality {
                        let mut die_roll = rng.roll_dice(1, 1000);
                        if die_roll < d_chance {
                            plant_deciduous(x, y, z, rng, region);
                            //crate::spawner::spawn_tree(ecs, x, y, z, region.world_idx)
                        } else {
                            die_roll = rng.roll_dice(1, 1000);
                            if die_roll < e_chance {
                                //plant_evergreen(x, y, z, rng, region);
                                //crate::spawner::spawn_tree(ecs, x, y, z, region.world_idx)
                                plant_deciduous(x, y, z, rng, region);
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Trunk {
    x: usize,
    y: usize,
    z: usize,
    depth : usize,
    done: bool
}

fn check_collision(list: &[Trunk], x: usize, y:usize, z:usize) -> bool {
    for l in list.iter() {
        if l.x == x && l.y == y && l.z == z {
            return false;
        }
    }
    true
}

fn plant_deciduous(x: usize, y:usize, z:usize, rng: &mut RandomNumberGenerator, region: &mut Region) {
    let tree_id = {
        let mut tl = TREE_COUNTER.write();
        *tl += 1;
        tl
    };

    let tree_size = rng.roll_dice(3, 6) as usize;

    // Grow the tree
    let mut trunk = Vec::<Trunk>::new();
    trunk.push(Trunk{ x, y, z, depth: 1, done: true });
    region.tree_bases.insert(*tree_id, mapidx(x, y, z));
    trunk.push(Trunk{ x, y, z: z+1, depth: 2, done: false });
    let mut bailout_count = 0;
    while trunk.iter().filter(|t| t.done == false).count() > 0 && bailout_count < 50 {
        for i in 0..trunk.len() {
            if !trunk[i].done {
                trunk[i].done = true;

                if trunk[i].depth < tree_size {
                    let b = trunk[i].clone();
                    let mut added = false;
                    while !added && bailout_count < 50 {
                        for _ in 0 .. rng.range(0, 4) {
                            match rng.range(0, usize::max(4, 10 - b.depth)) {
                                0 => {
                                    if check_collision(&trunk, x-1, y, z) {
                                        trunk.push(Trunk{ x: b.x-1, y: b.y, z: b.z, depth: b.depth + 1, done: false });
                                        added = true;
                                    }
                                }
                                1 => {
                                    if check_collision(&trunk, x+1, y, z) {
                                        trunk.push(Trunk{ x: b.x+1, y: b.y, z: b.z, depth: b.depth + 1, done: false });
                                        added = true;
                                    }
                                }
                                2 => {
                                    if check_collision(&trunk, x, y-1, z) {
                                        trunk.push(Trunk{ x: b.x, y: b.y-1, z: b.z, depth: b.depth + 1, done: false });
                                        added = true;
                                    }
                                }
                                3 => {
                                    if check_collision(&trunk, x, y+1, z) {
                                        trunk.push(Trunk{ x: b.x, y: b.y+1, z: b.z, depth: b.depth + 1, done: false });
                                        added = true;
                                    }
                                }
                                _ => {
                                    if check_collision(&trunk, x-1, y, z+1) {
                                        trunk.push(Trunk{ x: b.x, y: b.y, z: b.z + 1, depth: b.depth + 1, done: false });
                                        added = true;
                                    }
                                }
                            }
                        }
                        bailout_count += 1;
                    }
                }
            }
        }
    }
    trunk.iter().for_each(|t| {
        if t.x > 0 && t.x < REGION_WIDTH-1 && t.y > 0 && t.y < REGION_HEIGHT-1 && t.z > 0 && t.z < REGION_DEPTH-1 {
            let idx = mapidx(t.x, t.y, t.z);
            region.tile_types[idx] = TileType::TreeTrunk{ tree_id: *tree_id };
        }
    });
    trunk.iter().for_each(|t| {
        if t.x > 1 && t.x < REGION_WIDTH-2 && t.y > 1 && t.y < REGION_HEIGHT-2 && t.z > 1 && t.z < REGION_DEPTH-2 {
            for fx in t.x - 1..= t.x + 1 {
                for fy in t.y - 1..= t.y + 1 {
                    for fz in t.z ..= t.z + 1 {
                        if t.depth > 2 && rng.roll_dice(1, 3) > 1 {
                            let idx = mapidx(fx, fy, fz);
                            if region.tile_types[idx] == TileType::Empty {
                                region.tile_types[idx] = TileType::TreeFoliage{ tree_id: *tree_id };
                            }
                        }
                    }
                }
            }
        }
    });
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
