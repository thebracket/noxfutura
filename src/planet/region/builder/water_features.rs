use super::Region;
use crate::planet::{Planet, TileType};
use bracket_geometry::prelude::*;
use bracket_random::prelude::RandomNumberGenerator;
use crate::spatial::*;
use std::collections::HashSet;

pub fn just_add_water(
    planet: &Planet,
    region: &Region,
    water: &mut Vec<u8>,
    hm: &mut Vec<u8>,
    rng: &mut RandomNumberGenerator,
) {
    let center_point = Point::new(REGION_WIDTH / 2, REGION_HEIGHT / 2);
    //let mut river_starts_here = false;
    let mut river_terminates_here = false;
    let mut has_river = false;

    let mut river_entry = [0, 0, 0, 0];
    let mut river_exit = 0;
    let region_loc = Point::new(
        region.world_idx % WORLD_WIDTH as usize,
        region.world_idx / WORLD_WIDTH as usize,
    );

    for river in planet.rivers.iter() {
        if river.start == region_loc {
            //river_starts_here = true;
            has_river = true;
        }

        let mut last_pos = river.start;
        for i in 0..river.steps.len() {
            if river.steps[i].pos == region_loc {
                has_river = true;

                if last_pos.x < region_loc.x {
                    river_entry[0] += 1
                }
                if last_pos.x > region_loc.x {
                    river_entry[1] += 1
                }
                if last_pos.y < region_loc.y {
                    river_entry[2] += 1
                }
                if last_pos.y > region_loc.y {
                    river_entry[3] += 1
                }

                if i + 1 < river.steps.len() {
                    let next = river.steps[i + 1].pos;
                    if next.x < region_loc.x {
                        river_exit = 1;
                    }
                    if next.x > region_loc.x {
                        river_exit = 2;
                    }
                    if next.y < region_loc.y {
                        river_exit = 3;
                    }
                    if next.y > region_loc.y {
                        river_exit = 4;
                    }
                } else {
                    river_terminates_here = true;
                }
            }
            last_pos = river.steps[i].pos;
        }
    }

    if !has_river {
        return;
    }

    println!(
        "River start: {:?}, end: {}",
        river_entry, river_terminates_here
    );

    // Determine a mid-point
    let mut mid_ok = false;
    let mut midpoint = Point::zero();
    while !mid_ok {
        midpoint = Point::new(
            rng.roll_dice(1, REGION_WIDTH as i32 / 2) + REGION_WIDTH as i32 / 4,
            rng.roll_dice(1, REGION_HEIGHT as i32 / 2) + REGION_HEIGHT as i32 / 4,
        );
        let d = DistanceAlg::Pythagoras.distance2d(center_point, midpoint);
        if d > 15.0 {
            mid_ok = true
        }
    }

    let mut dig_targets: HashSet<usize> = HashSet::new();

    // Run rivers to the confluence
    for _ in 0..river_entry[0] {
        let start = Point::new(
            0,
            rng.roll_dice(1, REGION_HEIGHT as i32 / 2) + REGION_HEIGHT as i32 / 4 - 1,
        );
        for point in line2d_vector(start, midpoint) {
            add_dig_target(point, 2, &mut dig_targets);
        }
    }
    for _ in 0..river_entry[1] {
        let start = Point::new(
            REGION_WIDTH as i32 - 1,
            rng.roll_dice(1, REGION_HEIGHT as i32 / 2) + REGION_HEIGHT as i32 / 4 - 1,
        );
        for point in line2d_vector(start, midpoint) {
            add_dig_target(point, 2, &mut dig_targets);
        }
    }
    for _ in 0..river_entry[2] {
        let start = Point::new(
            rng.roll_dice(1, REGION_WIDTH as i32 / 2) + REGION_WIDTH as i32 / 4 - 1,
            0,
        );
        for point in line2d_vector(start, midpoint) {
            add_dig_target(point, 2, &mut dig_targets);
        }
    }
    for _ in 0..river_entry[3] {
        let start = Point::new(
            rng.roll_dice(1, REGION_WIDTH as i32 / 2) + REGION_WIDTH as i32 / 4 - 1,
            REGION_HEIGHT as i32 - 2,
        );
        for point in line2d_vector(start, midpoint) {
            add_dig_target(point, 2, &mut dig_targets);
        }
    }

    if !river_terminates_here {
        let end = match river_exit {
            1 => Point::new(
                0,
                rng.roll_dice(1, REGION_HEIGHT as i32 / 2) + REGION_HEIGHT as i32 / 4 - 1,
            ),
            2 => Point::new(
                REGION_WIDTH as i32 - 1,
                rng.roll_dice(1, REGION_HEIGHT as i32 / 2) + REGION_HEIGHT as i32 / 4 - 1,
            ),
            3 => Point::new(
                rng.roll_dice(1, REGION_WIDTH as i32 / 2) + REGION_WIDTH as i32 / 4 - 1,
                0,
            ),
            _ => Point::new(
                rng.roll_dice(1, REGION_WIDTH as i32 / 2) + REGION_WIDTH as i32 / 4 - 1,
                REGION_HEIGHT as i32 - 1,
            ),
        };
        for point in line2d_vector(midpoint, end) {
            add_dig_target(point, 2, &mut dig_targets);
        }
    }

    // Do the digging
    let orig_height = hm.clone();
    for idx in dig_targets.iter() {
        let dig_at = Point::new(idx % REGION_WIDTH as usize, idx / REGION_WIDTH as usize);
        let mut min_altitude = std::u8::MAX;
        for off_y in -2..2 {
            for off_x in -2..=2 {
                let pt = Point::new(off_x, off_y) + dig_at;
                let idx = (pt.y * REGION_WIDTH as i32) + pt.x;
                if idx > 0 && idx < REGION_TILES_COUNT as i32
                {
                    let pt_alt = orig_height[idx as usize];
                    if pt_alt < min_altitude {
                        min_altitude = pt_alt;
                    }
                }
            }
        }
        if min_altitude > 4 {
            hm[*idx] = min_altitude - 3;
            water[*idx] = min_altitude - 2;
        }
    }
}

fn add_dig_target(pt: Point, radius: i32, dig_targets: &mut HashSet<usize>) {
    for y in 0 - radius..radius {
        for x in 0 - radius..radius {
            let apt = Point::new(x, y) + pt;
            if apt.x > 0
                && apt.x < REGION_WIDTH as i32 - 1
                && apt.y > 0
                && apt.y < REGION_HEIGHT as i32 - 1
            {
                let idx = (apt.y * REGION_WIDTH as i32) + apt.x;
                dig_targets.insert(idx as usize);
            }
        }
    }
}

pub fn set_water_tiles(region: &mut Region, water: &Vec<u8>, planet_water_level: usize) {
    for z in 0..REGION_DEPTH {
        for y in 0..REGION_HEIGHT {
            for x in 0..REGION_WIDTH {
                let idx = mapidx(x, y, z);
                if region.is_floor(idx) || region.tile_types[idx] == TileType::Empty {
                    let pool_idx = (y * REGION_WIDTH) + x;
                    if z <= water[pool_idx] as usize || z <= planet_water_level {
                        region.water_level[idx] = 10;
                    }
                }
            }
        }
    }
}
