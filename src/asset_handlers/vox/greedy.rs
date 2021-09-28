use crate::geometry::constants::CUBE_NORMALS;

use super::{VoxTemplate, model_size::ModelSize};
use std::collections::{HashMap, HashSet};
pub type VoxMap = HashMap<i32, u8>;

pub fn greedy_cubes(
    template: &VoxTemplate,
    vertices: &mut Vec<[f32; 3]>,
    normals: &mut Vec<[f32; 3]>,
    uvs: &mut Vec<[f32; 2]>,
) {
    let mut cube_index = template.voxels.clone();
    let size = &template.size;

    /*let invisible = cube_index
        .iter()
        .filter(|(idx, _)| {
            cube_index.contains_key(&(*idx - 1))
                && cube_index.contains_key(&(*idx + 1))
                && cube_index.contains_key(&(*idx - size.x as i32))
                && cube_index.contains_key(&(*idx + size.x as i32))
                && cube_index.contains_key(&(*idx + (size.x * size.y) as i32))
                && cube_index.contains_key(&(*idx - (size.x * size.y) as i32))
        })
        .map(|(idx, _)| *idx)
        .collect::<HashSet<i32>>();
    //println!("Invisibility cull: {}", invisible.len());
    cube_index.retain(|idx, _| !invisible.contains(idx));*/

    loop {
        let min_iter = cube_index.keys().min();
        if min_iter.is_none() {
            break;
        } else {
            let idx = *min_iter.unwrap();
            let mat_idx = cube_index.remove(&idx).unwrap();

            let (x, y, z) = size.idxmap(idx as u32);
            //let width = grow_right(&mut cube_index, idx as u32, mat_idx);
            //let height = grow_down(&mut cube_index, idx as u32, width, mat_idx, size);
            //let depth = grow_in(&mut cube_index, idx as u32, width, height, mat_idx, size);
            let width = 1;
            let height = 1;
            let depth = 1;

            add_vox_cube(
                vertices,
                normals,
                uvs,
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

    while cube_index.contains_key(&(candidate_idx as i32))
        && cube_index[&(candidate_idx as i32)] == mat
    {
        cube_index.remove(&(candidate_idx as i32));
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
            if !cube_index.contains_key(&(cidx as i32)) {
                break 'outer;
            }
            if cube_index[&(cidx as i32)] != mat {
                break 'outer;
            }
        }

        for cidx in candidate_idx..candidate_idx + width {
            cube_index.remove(&(cidx as i32));
        }
        height += 1;
        candidate_idx += size.x;
    }
    height
}

fn grow_in(
    cube_index: &mut VoxMap,
    idx: u32,
    width: u32,
    height: u32,
    mat: u8,
    size: &ModelSize,
) -> u32 {
    let mut depth = 1;
    let layer_size = size.x * size.y;
    let mut candidate_idx = idx + layer_size;
    'outer: loop {
        for x in 0..width {
            for y in 0..height {
                let cidx = candidate_idx + x + (y * size.x);
                if !cube_index.contains_key(&(cidx as i32)) {
                    break 'outer;
                }
                if cube_index[&(cidx as i32)] != mat {
                    break 'outer;
                }
            }
        }

        for x in 0..width {
            for y in 0..height {
                let cidx = candidate_idx + x + (y * size.x);
                cube_index.remove(&(cidx as i32));
            }
        }
        depth += 1;
        candidate_idx += layer_size;
    }
    depth
}

//const GEOMETRY_SIZE : f32 = 1.0 / 32.0;
const GEOMETRY_SIZE : f32 = 1.0 / 1.0;

fn add_vox_cube(
    vertices: &mut Vec<[f32; 3]>,
    normals: &mut Vec<[f32; 3]>,
    uvs: &mut Vec<[f32; 2]>,
    x: f32,
    y: f32,
    z: f32,
    w: f32,
    h: f32,
    d: f32,
    mat_idx: u8,
) {
    let x0 = x * GEOMETRY_SIZE;
    let x1 = (x0 + w) * GEOMETRY_SIZE;
    let y0 = y * GEOMETRY_SIZE;
    let y1 = (y0 + h) * GEOMETRY_SIZE;
    let z0 = z * GEOMETRY_SIZE;
    let z1 = (z0 + d) * GEOMETRY_SIZE;

    //println!("Cube at: {},{},{}", x0, y0, z0);

    #[rustfmt::skip]
    let cube_geometry = [
        [x0, y0, z0,],
        [x1, y1, z0,],
        [x1, y0, z0,],
        [x1, y1, z0,],
        [x0, y0, z0,],
        [x0, y1, z0,],

        [x0, y0, z1,],
        [x1, y0, z1,],
        [x1, y1, z1,],
        [x1, y1, z1,],
        [x0, y1, z1,],
        [x0, y0, z1,],

        [x0, y1, z1,],
        [x0, y1, z0,],
        [x0, y0, z0,],
        [x0, y0, z0,],
        [x0, y0, z1,],
        [x0, y1, z1,],

        [x1, y1, z1,],
        [x1, y0, z0,],
        [x1, y1, z0,],
        [x1, y0, z0,],
        [x1, y1, z1,],
        [x1, y0, z1,],

        [x0, y0, z0,],
        [x1, y0, z0,],
        [x1, y0, z1,],
        [x1, y0, z1,],
        [x0, y0, z1,],
        [x0, y0, z0,],

        [x1, y1, z1,],
        [x1, y1, z0,],
        [x0, y1, z0,],
        [x0, y1, z0,],
        [x0, y1, z1,],
        [x1, y1, z1,],
    ];
    vertices.extend_from_slice(&cube_geometry);

    #[rustfmt::skip]
    const NORMAL_GEOMETRY: [[f32; 3]; 36] = [
        CUBE_NORMALS[0],
        CUBE_NORMALS[0],
        CUBE_NORMALS[0],
        CUBE_NORMALS[0],
        CUBE_NORMALS[0],
        CUBE_NORMALS[0],

        CUBE_NORMALS[1],
        CUBE_NORMALS[1],
        CUBE_NORMALS[1],
        CUBE_NORMALS[1],
        CUBE_NORMALS[1],
        CUBE_NORMALS[1],

        CUBE_NORMALS[2],
        CUBE_NORMALS[2],
        CUBE_NORMALS[2],
        CUBE_NORMALS[2],
        CUBE_NORMALS[2],
        CUBE_NORMALS[2],

        CUBE_NORMALS[3],
        CUBE_NORMALS[3],
        CUBE_NORMALS[3],
        CUBE_NORMALS[3],
        CUBE_NORMALS[3],
        CUBE_NORMALS[3],

        CUBE_NORMALS[4],
        CUBE_NORMALS[4],
        CUBE_NORMALS[4],
        CUBE_NORMALS[4],
        CUBE_NORMALS[4],
        CUBE_NORMALS[4],

        CUBE_NORMALS[5],
        CUBE_NORMALS[5],
        CUBE_NORMALS[5],
        CUBE_NORMALS[5],
        CUBE_NORMALS[5],
        CUBE_NORMALS[5],
    ];
    normals.extend_from_slice(&NORMAL_GEOMETRY);

    let mx = ((mat_idx % 16) * 4)+2;
    let my = ((mat_idx / 16) * 4)+2;
    println!("Mat: {}={},{}",mat_idx, mx, my);

    let mat_x = 1.0 - (mx as f32 / 64.0);
    let mat_y = my as f32 / 64.0;

    let uv_base = [[mat_x, mat_y]; 36];
    uvs.extend_from_slice(&uv_base);
}
