use crate::planet::{REGION_DEPTH, REGION_HEIGHT, REGION_WIDTH};
use crate::utils::{idxmap, mapidx};
use std::collections::HashSet;

pub fn greedy_cubes(cube_index: &mut HashSet<usize>, layer : &mut Vec<f32>, element_count : &mut u32) {

    loop {
        let min_iter = cube_index.iter().min();
        if min_iter.is_none() {
            break;
        } else {
            let idx = *min_iter.unwrap();
            cube_index.remove(&idx);

            let (x, y, z) = idxmap(idx);
            let width = grow_right(cube_index, idx);
            let height = grow_down(cube_index, idx, width);
            //let depth = grow_in(&mut cube_index, idx, width, height);
            let depth = 1;

            crate::utils::add_cube_geometry(
                layer,
                element_count,
                x as f32,
                y as f32,
                z as f32,
                width as f32,
                height as f32,
                depth as f32
            );
        }
    }
}

fn grow_right(cube_index: &mut HashSet<usize>, idx: usize) -> usize {
    let mut width = 1;
    let mut candidate_idx = idx + 1;

    while cube_index.contains(&candidate_idx) {
        cube_index.remove(&candidate_idx);
        width += 1;
        candidate_idx += 1;
    }

    width
}

fn grow_down(cube_index: &mut HashSet<usize>, idx: usize, width: usize) -> usize {
    let mut height = 1;
    let mut candidate_idx = idx + REGION_WIDTH;
    'outer: loop {
        for cidx in candidate_idx..=candidate_idx + width {
            if !cube_index.contains(&cidx) {
                break 'outer;
            }
        }

        for cidx in candidate_idx..=candidate_idx + width {
            cube_index.remove(&cidx);
        }
        height += 1;
        candidate_idx += REGION_WIDTH;
    }
    height
}

fn grow_in(cube_index: &mut HashSet<usize>, idx: usize, width: usize, height: usize) -> usize {
    const LAYER_SIZE : usize = REGION_WIDTH * REGION_HEIGHT;
    let mut depth = 1;
    let mut candidate_idx = idx + LAYER_SIZE;
    'outer: loop {
        for y in 0..=height {
            for x in 0..=width {
                let cidx = candidate_idx + (y * REGION_WIDTH) + x;
                if !cube_index.contains(&cidx) {
                    break 'outer;
                }
            }
        }

        for y in 0..=height {
            for x in 0..=width {
                let cidx = candidate_idx + (y * REGION_WIDTH) + x;
                cube_index.remove(&cidx);
            }
        }

        depth += 1;
        candidate_idx += LAYER_SIZE;
    }
    depth
}