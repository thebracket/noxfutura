use crate::utils::{idxmap, mapidx};
use super::{Primitive, CHUNK_SIZE};
use std::collections::HashSet;
use std::iter::FromIterator;
use crate::region::{REGION_WIDTH};

pub fn greedy_cubes(mut cubes: Vec<usize>) -> Vec<Primitive> {
    cubes.sort_by(|a,b| b.cmp(a));
    let original_size = cubes.len();
    let mut p = Vec::new();
    let mut cube_index : HashSet<usize> = HashSet::from_iter(cubes.iter().cloned());
    while !cubes.is_empty() {
        let mut idx = cubes.pop().unwrap();
        if cube_index.contains(&idx) {
            let (mut x,y,z) = idxmap(idx);
            //println!("{} = {},{},{}", idx, x, y, z);
            let mut width = 1;

            // Grow right
            let mut candidate_idx = idx+1;
            while cube_index.contains(&candidate_idx) {
                cube_index.remove(&candidate_idx);
                candidate_idx += 1;
                width += 1;
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

            p.push(
                Primitive::Cube{
                    x, y, z,
                    w: width,
                    h: 1,
                    d: 1
                }
            )
        }
    }
    println!("Compressed {} cubes into {} primitives", original_size, p.len());
    p
}