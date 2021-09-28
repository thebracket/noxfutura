use crate::geometry::constants::CUBE_NORMALS;

use super::VoxTemplate;
use std::collections::{HashMap, HashSet};
pub type VoxMap = HashMap<usize, u8>;

pub fn greedy_cubes(
    template: &VoxTemplate,
    vertices: &mut Vec<[f32; 3]>,
    normals: &mut Vec<[f32; 3]>,
    uvs: &mut Vec<[f32; 2]>,
) {
    let mut cube_index = template.voxels.clone();

    let invisible = cube_index
        .iter()
        .filter(|(idx, _)| {
            cube_index.contains_key(&(*idx - 1))
                && cube_index.contains_key(&(*idx + 1))
                && cube_index.contains_key(&(*idx - template.width as usize))
                && cube_index.contains_key(&(*idx + template.width as usize))
                && cube_index
                    .contains_key(&(*idx + (template.width as usize * template.width as usize)))
                && cube_index
                    .contains_key(&(*idx - (template.width as usize * template.width as usize)))
        })
        .map(|(idx, _)| *idx)
        .collect::<HashSet<usize>>();
    //println!("Invisibility cull: {}", invisible.len());
    cube_index.retain(|idx, _| !invisible.contains(idx));

    loop {
        let min_iter = cube_index.keys().min();
        if min_iter.is_none() {
            break;
        } else {
            let idx = *min_iter.unwrap();
            let mat_idx = cube_index.remove(&idx).unwrap();

            let (x, y, z) = template.idxmap(idx);
            let width = grow_right(&mut cube_index, idx, mat_idx);
            let height = grow_down(&mut cube_index, idx, width, mat_idx, template);
            let depth = grow_in(&mut cube_index, idx, width, height, mat_idx, template);

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

fn grow_right(cube_index: &mut VoxMap, idx: usize, mat: u8) -> usize {
    let mut width = 1;
    let mut candidate_idx = idx + 1;

    while cube_index.contains_key(&candidate_idx) && cube_index[&candidate_idx] == mat {
        cube_index.remove(&candidate_idx);
        width += 1;
        candidate_idx += 1;
    }

    width
}

fn grow_down(
    cube_index: &mut VoxMap,
    idx: usize,
    width: usize,
    mat: u8,
    template: &VoxTemplate,
) -> usize {
    let mut height = 1;
    let mut candidate_idx = idx + template.width as usize;
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
        candidate_idx += template.width as usize;
    }
    height
}

fn grow_in(
    cube_index: &mut VoxMap,
    idx: usize,
    width: usize,
    height: usize,
    mat: u8,
    template: &VoxTemplate,
) -> usize {
    let mut depth = 1;
    let layer_size = template.width as usize * template.height as usize;
    let mut candidate_idx = idx + layer_size;
    'outer: loop {
        for x in 0..width {
            for y in 0..height {
                let cidx = candidate_idx + x + (y * template.width as usize);
                if !cube_index.contains_key(&cidx) {
                    break 'outer;
                }
                if cube_index[&cidx] != mat {
                    break 'outer;
                }
            }
        }

        for x in 0..width {
            for y in 0..height {
                let cidx = candidate_idx + x + (y * template.width as usize);
                cube_index.remove(&cidx);
            }
        }
        depth += 1;
        candidate_idx += layer_size;
    }
    depth
}

const GEOMETRY_SIZE : f32 = 1.0 / 32.0;

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

    let mat_x = (mat_idx % 8) as f32 / 32.0;
    let mat_y = (mat_idx / 8) as f32 / 32.0;

    let uv_base = [[mat_x, mat_y]; 36];
    uvs.extend_from_slice(&uv_base);
}
