use super::REGION;
use legion::world::SubWorld;
use legion::*;
use nox_components::*;
use nox_planet::{ConstructionMap, Region};
use nox_spatial::*;
use std::collections::VecDeque;

#[system]
#[read_component(Construction)]
#[read_component(Blueprint)]
#[read_component(Position)]
pub fn construction_map(ecs: &SubWorld, #[resource] map: &mut ConstructionMap) {
    if !map.is_dirty {
        return;
    }

    map.dijkstra.iter_mut().for_each(|n| *n = f32::MAX);

    // Build the tree starting points
    let rlock = REGION.read();
    let mut starts = Vec::new();
    <(&Construction, &Position)>::query()
        .filter(!component::<Blueprint>())
        .iter(ecs)
        .for_each(|(c, pos)| {
            let idx = pos.get_idx();
            match c.mode {
                1 => {
                    add_horizontally_adjacent_exists(&idx, &mut starts, &rlock);
                    let above = idx + (REGION_WIDTH * REGION_HEIGHT);
                    if rlock.flag(above, Region::CAN_STAND_HERE) {
                        starts.push(above);
                    }
                }
                2 => {
                    add_horizontally_adjacent_exists(&idx, &mut starts, &rlock);
                    let below = idx - (REGION_WIDTH * REGION_HEIGHT);
                    if rlock.flag(below, Region::CAN_STAND_HERE) {
                        starts.push(below);
                    }
                }
                3 => {
                    add_horizontally_adjacent_exists(&idx, &mut starts, &rlock);
                    let above = idx + (REGION_WIDTH * REGION_HEIGHT);
                    if rlock.flag(above, Region::CAN_STAND_HERE) {
                        starts.push(above);
                    }
                    let below = idx - (REGION_WIDTH * REGION_HEIGHT);
                    if rlock.flag(below, Region::CAN_STAND_HERE) {
                        starts.push(below);
                    }
                }
                _ => add_horizontally_adjacent_exists(&idx, &mut starts, &rlock)
            }
        });

    // Build the Dijkstra Map
    let mut open_list: VecDeque<(usize, f32)> = VecDeque::with_capacity(REGION_TILES_COUNT);

    for start in &starts {
        open_list.push_back((*start, 0.0));
        map.dijkstra[*start] = 0.0;
    }

    const MAX_DEPTH: f32 = 2048.0;
    while let Some((tile_idx, depth)) = open_list.pop_front() {
        let exits = rlock.get_available_exits(tile_idx);
        for (new_idx, add_depth) in exits {
            let new_depth = depth + add_depth;
            let prev_depth = map.dijkstra[new_idx];
            if new_depth >= prev_depth {
                continue;
            }
            if new_depth >= MAX_DEPTH {
                continue;
            }
            map.dijkstra[new_idx] = new_depth;
            open_list.push_back((new_idx, new_depth));
        }
    }

    println!("Construction Dijkstra Created");

    // Clear dirty flag
    map.is_dirty = false;
}

fn add_horizontally_adjacent_exists(idx: &usize, starts: &mut Vec<usize>, region: &Region) {
    if region.flag(idx - 1, Region::CAN_STAND_HERE) {
        starts.push(idx - 1);
    }
    if region.flag(idx + 1, Region::CAN_STAND_HERE) {
        starts.push(idx + 1);
    }
    if region.flag(idx - REGION_WIDTH, Region::CAN_STAND_HERE) {
        starts.push(idx - REGION_WIDTH);
    }
    if region.flag(idx + REGION_WIDTH, Region::CAN_STAND_HERE) {
        starts.push(idx + REGION_WIDTH);
    }
}
