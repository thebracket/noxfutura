use super::{set_worldgen_status, PLANET_BUILD};
use crate::planet::{planet_idx, Planet, River, RiverStep};
use nox_spatial::{WORLD_HEIGHT, WORLD_WIDTH};
use bengine::geometry::*;
use bengine::random::*;
use std::collections::HashSet;
use nox_raws::BlockType;

pub fn run_rivers() {
    set_worldgen_status("Running Rivers");

    let seed = PLANET_BUILD.lock().planet.rng_seed;
    let mut rng = RandomNumberGenerator::seeded(seed);

    let planet = PLANET_BUILD.lock().planet.clone();
    let n_rivers = WORLD_WIDTH / 2;
    let mut used_starts: HashSet<usize> = HashSet::new();
    let mut used_steps: HashSet<usize> = HashSet::new();

    let mut rivers = Vec::new();
    for _ in 0..n_rivers {
        let mut river = River::new();
        let mut start_ok = false;
        while !start_ok {
            river.start = Point::new(
                rng.roll_dice(1, WORLD_WIDTH as i32 - 1),
                rng.roll_dice(1, WORLD_HEIGHT as i32 - 1),
            );
            let pidx = planet_idx(river.start.x as usize, river.start.y as usize);
            if (planet.landblocks[pidx].btype == BlockType::Mountains
                || planet.landblocks[pidx].btype == BlockType::Hills)
                && !used_starts.contains(&pidx)
            {
                start_ok = true;
            }
        }
        used_starts.insert(planet_idx(river.start.x as usize, river.start.y as usize));

        let mut done = false;
        let mut x = river.start.x;
        let mut y = river.start.y;

        while !done {
            let mut candidates: Vec<(u8, usize)> = Vec::new();
            candidate(
                &used_starts,
                &used_steps,
                x - 1,
                y,
                &planet,
                &mut candidates,
            );
            candidate(
                &used_starts,
                &used_steps,
                x + 1,
                y,
                &planet,
                &mut candidates,
            );
            candidate(
                &used_starts,
                &used_steps,
                x,
                y - 1,
                &planet,
                &mut candidates,
            );
            candidate(
                &used_starts,
                &used_steps,
                x,
                y + 1,
                &planet,
                &mut candidates,
            );
            if candidates.is_empty() {
                done = true;
            } else {
                candidates.sort_by(|(h, _), (h2, _)| h.cmp(&h2));
                used_steps.insert(candidates[0].1);
                let sx = candidates[0].1 % WORLD_WIDTH as usize;
                let sy = candidates[0].1 / WORLD_WIDTH as usize;
                river.steps.push(RiverStep {
                    pos: Point::new(sx, sy),
                });
                x = sx as i32;
                y = sy as i32;
            }
        }
        rivers.push(river);
    }
    PLANET_BUILD.lock().planet.rivers = rivers;
}

fn candidate(
    used_starts: &HashSet<usize>,
    used_steps: &HashSet<usize>,
    x: i32,
    y: i32,
    planet: &Planet,
    candidates: &mut Vec<(u8, usize)>,
) {
    if x < 0 || x > WORLD_WIDTH as i32 - 1 || y < 0 || y > WORLD_HEIGHT as i32 - 1 {
        return;
    }
    let pidx = planet_idx(x as usize, y as usize);
    if used_starts.contains(&pidx) {
        return;
    }
    if used_steps.contains(&pidx) {
        return;
    }
    if planet.landblocks[pidx].btype == BlockType::Water {
        return;
    }
    candidates.push((planet.landblocks[pidx].height, pidx));
}
