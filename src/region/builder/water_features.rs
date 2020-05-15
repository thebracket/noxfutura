use crate::planet::{Planet, WORLD_WIDTH, REGION_WIDTH, REGION_HEIGHT, REGION_DEPTH};
use super::Region;
use bracket_geometry::prelude::*;
use bracket_random::prelude::RandomNumberGenerator;

pub fn just_add_water(
    planet: &Planet, 
    region: &Region, 
    water: &mut Vec<u8>, 
    hm: &mut Vec<u8>,
    rng: &mut RandomNumberGenerator
) {
    let center_point = Point::new(REGION_WIDTH/2, REGION_HEIGHT/2);
    //let mut river_starts_here = false;
    let mut river_terminates_here = false;
    let mut has_river = false;

    let mut river_entry = [0,0,0,0];
    let mut river_exit = 0;
    let region_loc = Point::new(
        region.world_idx % WORLD_WIDTH as usize,
        region.world_idx / WORLD_WIDTH as usize
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

                if last_pos.x < region_loc.x { river_entry[0] += 1 }
                if last_pos.x > region_loc.x { river_entry[1] += 1 }
                if last_pos.y < region_loc.y { river_entry[2] += 1 }
                if last_pos.y > region_loc.y { river_entry[3] += 1 }

                if i+1 < river.steps.len() {
                    let next = river.steps[i+1].pos;
                    if next.x < region_loc.x { river_exit = 1; }
                    if next.x > region_loc.x { river_exit = 2; }
                    if next.y < region_loc.y { river_exit = 3; }
                    if next.y > region_loc.y { river_exit = 4; }
                } else {
                    river_terminates_here = true;
                }
            }
            last_pos = river.steps[i].pos;
        }
    }

    if !has_river { return; }

    println!("River start: {:?}, end: {}", river_entry, river_terminates_here);

    // Determine a mid-point
    let mut mid_ok = false;
    let mut midpoint = Point::zero();
    while !mid_ok {
        midpoint = Point::new(
            rng.roll_dice(1, REGION_WIDTH/2) + REGION_WIDTH/4,
            rng.roll_dice(1, REGION_HEIGHT/2) + REGION_HEIGHT/4,
        );
        let d = DistanceAlg::Pythagoras.distance2d(center_point, midpoint);
        if d > 15.0 { mid_ok = true }
    }

    // Run rivers to the confluence
    let oh = hm.clone();
    for _ in 0..river_entry[0] {
        let start = Point::new(
            0, 
            rng.roll_dice(1, REGION_HEIGHT/2) + REGION_HEIGHT/4 -1
        );
        for point in line2d_vector(start, midpoint) {
            add_dig_target(point, 2, 2, &oh, hm, water);
        }
    }
    for _ in 0..river_entry[1] {
        let start = Point::new(
            REGION_WIDTH-1, 
            rng.roll_dice(1, REGION_HEIGHT/2) + REGION_HEIGHT/4 -1
        );
        for point in line2d_vector(start, midpoint) {
            add_dig_target(point, 2, 2, &oh, hm, water);
        }
    }
    for _ in 0..river_entry[2] {
        let start = Point::new(
            rng.roll_dice(1, REGION_WIDTH/2) + REGION_WIDTH/4 -1,
            0
        );
        for point in line2d_vector(start, midpoint) {
            add_dig_target(point, 2, 2, &oh, hm, water);
        }
    }
    for _ in 0..river_entry[3] {
        let start = Point::new(
            rng.roll_dice(1, REGION_WIDTH/2) + REGION_WIDTH/4 -1,
            REGION_HEIGHT-2
        );
        for point in line2d_vector(start, midpoint) {
            add_dig_target(point, 2, 2, &oh, hm, water);
        }
    }

    if !river_terminates_here {
        let end = match river_exit {
            1 => Point::new(0, rng.roll_dice(1, REGION_HEIGHT/2) + REGION_HEIGHT/4 -1),
            2 => Point::new(REGION_WIDTH-1, rng.roll_dice(1, REGION_HEIGHT/2) + REGION_HEIGHT/4 -1),
            3 => Point::new(rng.roll_dice(1, REGION_WIDTH/2) + REGION_WIDTH/4 -1, 0),
            _ => Point::new(rng.roll_dice(1, REGION_WIDTH/2) + REGION_WIDTH/4 -1, REGION_HEIGHT-1),
        };
        for point in line2d_vector(midpoint, end) {
            add_dig_target(point, 2, 2, &oh, hm, water);
        }
    }
}

fn add_dig_target(pt: Point, radius: i32, depth: u8, orig_height:&[u8], hm: &mut Vec<u8>, water: &mut Vec<u8>) {
    //println!("{:?}", pt);
    let mut min_altitude = std::u8::MAX;
    for y in 0-radius..radius {
        for x in 0-radius..radius {
            let apt = Point::new(x, y) + pt;
            if apt.x >= 0 && apt.x < REGION_WIDTH && apt.y >= 0 && apt.y < REGION_HEIGHT {
                let idx = (apt.y * REGION_WIDTH) + apt.x;
                //println!("{}", orig_height[idx as usize]);
                if orig_height[idx as usize] < min_altitude { min_altitude = hm[idx as usize] }
            }
        }
    }
    //println!("Min: {}", min_altitude);
    if min_altitude < depth+1 {
        min_altitude = depth+1;
    }
    //println!("Min: {}", min_altitude);

    for y in 0-radius..radius {
        for x in 0-radius..radius {
            let apt = Point::new(x, y) + pt;
            if apt.x > 0 && apt.x < REGION_WIDTH-1 && apt.y > 0 && apt.y < REGION_HEIGHT-1 {
                let idx = (apt.y * REGION_WIDTH) + apt.x;
                hm[idx as usize] = min_altitude - depth;
                water[idx as usize] = hm[idx as usize] + 1;
            }
        }
    }
}