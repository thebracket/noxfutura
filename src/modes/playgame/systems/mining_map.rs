use legion::*;
use super::super::MiningMap;
use super::REGION;
use crate::planet::Region;
use crate::spatial::*;
use std::collections::VecDeque;

#[system]
pub fn mining_map(#[resource] map : &mut MiningMap) {
    if !map.is_dirty {
        return;
    }

    map.dijkstra.iter_mut().for_each(|n| *n = f32::MAX);

    let rlock = REGION.read();
    if rlock.jobs_board.mining_designations.is_empty() {
        map.is_dirty = false;
        return;
    }
    // Build starting points for Dijkstra
    let mut starts = Vec::with_capacity(rlock.jobs_board.mining_designations.len()*4);
    rlock.jobs_board.mining_designations.iter().for_each(|(idx, t)| {
        // TODO: Adjust this
        match t {
            _ => add_horizontally_adjacent_exists(idx, &mut starts, &rlock),
        }
    });

    // Build the Dijkstra Map
    let mut open_list: VecDeque<(usize, f32)> = VecDeque::with_capacity(REGION_TILES_COUNT);

    for start in &starts {
        open_list.push_back((*start, 0.0));
        map.dijkstra[*start] = 0.0;
    }

    const MAX_DEPTH : f32 = 2048.0;
    while let Some((tile_idx, depth)) = open_list.pop_front() {
        let exits = rlock.get_available_exits(tile_idx);
        for (new_idx, add_depth) in exits {
            let new_depth = depth + add_depth;
            let prev_depth = map.dijkstra[new_idx];
            if new_depth >= prev_depth { continue; }
            if new_depth >= MAX_DEPTH { continue; }
            map.dijkstra[new_idx] = new_depth;
            open_list.push_back((new_idx, new_depth));
        }
    }

    println!("Mining Dijkstra Created");

    // Clear dirty flag
    map.is_dirty = false;
}

fn add_horizontally_adjacent_exists(idx: &usize, starts: &mut Vec<usize>, region: &Region) {
    if region.flag(idx-1, Region::CAN_STAND_HERE) {
        starts.push(idx-1);
    }
    if region.flag(idx+1, Region::CAN_STAND_HERE) {
        starts.push(idx+1);
    }
    if region.flag(idx - REGION_WIDTH, Region::CAN_STAND_HERE) {
        starts.push(idx - REGION_WIDTH);
    }
    if region.flag(idx + REGION_WIDTH, Region::CAN_STAND_HERE) {
        starts.push(idx + REGION_WIDTH);
    }
}