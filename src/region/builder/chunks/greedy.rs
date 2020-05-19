use crate::utils::{idxmap, mapidx};
use super::{Primitive, CHUNK_SIZE};
use std::collections::HashSet;
use std::iter::FromIterator;
use crate::region::{REGION_WIDTH, REGION_HEIGHT, REGION_DEPTH};

pub fn greedy_cubes(mut cubes: Vec<usize>) -> Vec<Primitive> {
    cubes.sort_unstable_by(|a,b| b.cmp(a));
    let original_size = cubes.len();
    let mut p = Vec::new();
    let mut cube_index : HashSet<usize> = HashSet::from_iter(cubes.iter().cloned());
    while !cubes.is_empty() {
        let mut idx = cubes.pop().unwrap();
        if cube_index.contains(&idx) {
            let (mut x,y,z) = idxmap(idx);
            //println!("{} = {},{},{}", idx, x, y, z);
            let mut width = 1;

            let mut candidate_idx;

            // Grow right
            if idx < REGION_WIDTH-2 {
                candidate_idx = idx+1;
                while cube_index.contains(&candidate_idx) {
                    cube_index.remove(&candidate_idx);
                    candidate_idx += 1;
                    width += 1;
                }
            }

            // Grow left
            if idx > 1 {
                candidate_idx = idx-1;
                while cube_index.contains(&candidate_idx) {
                    cube_index.remove(&candidate_idx);
                    candidate_idx -= 1;
                    width += 1;
                    idx -= 1;
                    x -= 1;
                }
            }

            // Grow down
            let mut y_grow = true;
            let mut height = 1;
            candidate_idx = idx;
            while y_grow {
                if candidate_idx < (REGION_WIDTH*REGION_HEIGHT*REGION_DEPTH)-REGION_WIDTH {
                    let base_y_index = candidate_idx + REGION_WIDTH;
                    let mut can_grow = true;
                    for y_candidate in base_y_index .. base_y_index + width {
                        if !cube_index.contains(&y_candidate) {
                            can_grow = false;
                        }
                    }
                    if can_grow {
                        height += 1;
                        for y_candidate in base_y_index .. base_y_index + width {
                            cube_index.remove(&y_candidate);
                        }
                        candidate_idx += REGION_HEIGHT;
                    } else {
                        y_grow = false;
                    }
                } else {
                    y_grow = false;
                }
            }

            // Grow deeper
            let mut z_grow = true;
            let mut depth = 1;
            candidate_idx = idx;
            while z_grow {
                if candidate_idx + (depth * REGION_WIDTH * REGION_HEIGHT) < (REGION_WIDTH*REGION_HEIGHT*REGION_DEPTH)-(REGION_WIDTH*REGION_HEIGHT) {
                    let mut can_grow = true;
                    for candidate_y in y.. y+height {
                        for candidate_x in x .. x+width {
                            let cidx = mapidx(candidate_x, candidate_y, z + depth);
                            if !cube_index.contains(&cidx) {
                                can_grow = false;
                            }
                        }
                    }

                    if can_grow {
                        for candidate_y in y.. y+height {
                            for candidate_x in x .. x+width {
                                let cidx = mapidx(candidate_x, candidate_y, z + depth);
                                cube_index.remove(&cidx);
                            }
                        }
                        depth += 1;
                    } else {
                        z_grow = false;
                    }
                } else {
                    z_grow = false;
                }
            }

            p.push(
                Primitive::Cube{
                    x, y, z,
                    w: width,
                    h: height,
                    d: depth
                }
            )
        }
    }
    //println!("Compressed {} cubes into {} primitives", original_size, p.len());
    p
}