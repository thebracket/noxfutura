use crate::simulation::{idxmap, REGION_WIDTH};
use std::collections::HashMap;

use super::chunk_mesh::add_cube_geometry;

pub type CubeMap = HashMap<usize, (usize, bool)>;

pub fn greedy_cubes(
    cube_index: &mut CubeMap,
    vertices: &mut Vec<[f32; 3]>,
    normals: &mut Vec<[f32; 3]>,
    uv: &mut Vec<[f32; 2]>,
) {
    loop {
        let min_iter = cube_index.keys().min();
        if min_iter.is_none() {
            break;
        } else {
            let idx = *min_iter.unwrap();
            let mat_idx = cube_index.remove(&idx).unwrap();

            let (x, y, z) = idxmap(idx);
            let width = grow_right(cube_index, idx, mat_idx);
            let height = grow_down(cube_index, idx, width, mat_idx);
            //let depth = grow_in(&mut cube_index, idx, width, height);
            let depth = 1;

            add_cube_geometry(
                vertices,
                normals,
                uv,
                x as f32,
                y as f32,
                z as f32,
                width as f32,
                height as f32,
                depth as f32,
            );
        }
    }
}

/*pub fn greedy_floors(cube_index: &mut CubeMap, layer: &mut Vec<f32>, element_count: &mut u32) {
    loop {
        let min_iter = cube_index.keys().min();
        if min_iter.is_none() {
            break;
        } else {
            let idx = *min_iter.unwrap();
            let mat_idx = cube_index.remove(&idx).unwrap();

            let (x, y, z) = idxmap(idx);
            let width = grow_right(cube_index, idx, mat_idx);
            let height = grow_down(cube_index, idx, width, mat_idx);

            nox_utils::add_floor_geometry(
                layer,
                element_count,
                x as f32,
                y as f32,
                z as f32,
                width as f32,
                height as f32,
                mat_idx,
            );
        }
    }
}*/

fn grow_right(cube_index: &mut CubeMap, idx: usize, mat: (usize, bool)) -> usize {
    let mut width = 1;
    let mut candidate_idx = idx + 1;

    while cube_index.contains_key(&candidate_idx) && cube_index[&candidate_idx] == mat {
        cube_index.remove(&candidate_idx);
        width += 1;
        candidate_idx += 1;
    }

    width
}

fn grow_down(cube_index: &mut CubeMap, idx: usize, width: usize, mat: (usize, bool)) -> usize {
    let mut height = 1;
    let mut candidate_idx = idx + REGION_WIDTH;
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
        candidate_idx += REGION_WIDTH;
    }
    height
}
