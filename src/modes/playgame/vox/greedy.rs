use super::modelsize::ModelSize;
use std::collections::HashMap;

pub type VoxMap = HashMap<u32, u8>;

pub fn greedy_cubes(cube_index: &mut VoxMap, output: &mut Vec<f32>, size: &ModelSize) {
    loop {
        let min_iter = cube_index.keys().min();
        if min_iter.is_none() {
            break;
        } else {
            let idx = *min_iter.unwrap();
            let mat_idx = cube_index.remove(&idx).unwrap();

            let (x, y, z) = size.idxmap(idx);
            let width = grow_right(cube_index, idx, mat_idx);
            let height = grow_down(cube_index, idx, width, mat_idx, size);
            //let depth = grow_in(&mut cube_index, idx, width, height);
            let depth = 1;

            super::cube::add_cube_geometry(
                output,
                x as f32,
                y as f32,
                z as f32,
                width as f32,
                height as f32,
                depth as f32,
                mat_idx,
            );
        }
    }
}

fn grow_right(cube_index: &mut VoxMap, idx: u32, mat: u8) -> u32 {
    let mut width = 1;
    let mut candidate_idx = idx + 1;

    while cube_index.contains_key(&candidate_idx) && cube_index[&candidate_idx] == mat {
        cube_index.remove(&candidate_idx);
        width += 1;
        candidate_idx += 1;
    }

    width
}

fn grow_down(cube_index: &mut VoxMap, idx: u32, width: u32, mat: u8, size: &ModelSize) -> u32 {
    let mut height = 1;
    let mut candidate_idx = idx + size.x;
    'outer: loop {
        for cidx in candidate_idx..candidate_idx + width {
            if !cube_index.contains_key(&cidx) {
                break 'outer;
            }
            if cube_index[&cidx] != mat {
                break 'outer;
            }
        }

        for cidx in candidate_idx..candidate_idx + width {
            cube_index.remove(&cidx);
        }
        height += 1;
        candidate_idx += size.x;
    }
    height
}
