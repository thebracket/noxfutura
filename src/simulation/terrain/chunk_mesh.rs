use std::collections::HashMap;

use crate::simulation::{chunk_idx, mapidx, CHUNK_SIZE};
use bevy::{prelude::Mesh, render::mesh::VertexAttributeValues};

use super::{
    chunker::{Chunk, ChunkType, TileType},
    greedy::{greedy_cubes, CubeMap},
};

pub fn chunk_to_mesh(chunk: &Chunk) -> Option<Vec<(usize, Mesh)>> {
    match chunk.chunk_type {
        ChunkType::Populated => populated_chunk_to_mesh(chunk),
        _ => None,
    }
}

pub type MaterialCubeMap = HashMap<usize, CubeMap>;

fn add_material(map: &mut MaterialCubeMap, material: usize, idx: usize) {
    if let Some(cmap) = map.get_mut(&material) {
        cmap.insert(idx, (material, false));
    } else {
        let mut layer_cubes = CubeMap::new();
        layer_cubes.insert(idx, (material, false));
        map.insert(material, layer_cubes);
    }
}

pub struct MaterialLayer {
    pub material: usize,
    pub vertices: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub uv: Vec<[f32; 2]>,
    pub tangents: Vec<[f32; 3]>,
}

fn populated_chunk_to_mesh(chunk: &Chunk) -> Option<Vec<(usize, Mesh)>> {
    if let Some(tiles) = &chunk.tiles {
        if let Some(revealed) = &chunk.revealed {
            if revealed.iter().filter(|r| **r == true).count() == 0 {
                // Bail out if there are no visible tiles
                return None;
            }

            let mut mat_map = Vec::<MaterialLayer>::new();

            for z in 0..CHUNK_SIZE {
                //let mut layer_cubes = CubeMap::new();
                let mut material_layer_cubes = MaterialCubeMap::new();
                for y in 0..CHUNK_SIZE {
                    for x in 0..CHUNK_SIZE {
                        let cidx = chunk_idx(x, y, z);
                        if revealed[cidx] {
                            match tiles[cidx] {
                                TileType::SemiMoltenRock => {
                                    let idx = mapidx(
                                        x + chunk.base.0,
                                        y + chunk.base.1,
                                        z + chunk.base.2,
                                    );
                                    //layer_cubes.insert(idx, (1, false));
                                    add_material(&mut material_layer_cubes, 1, idx);
                                }
                                TileType::Solid { material } => {
                                    let idx = mapidx(
                                        x + chunk.base.0,
                                        y + chunk.base.1,
                                        z + chunk.base.2,
                                    );
                                    //layer_cubes.insert(idx, (material, false));
                                    add_material(&mut material_layer_cubes, material, idx);
                                }
                                _ => {}
                            }
                        }
                    }
                }
                if !material_layer_cubes.is_empty() {
                    for (material, cubes) in material_layer_cubes.iter_mut() {
                        let mut vertices = Vec::new();
                        let mut normals = Vec::new();
                        let mut uv = Vec::new();
                        let mut tangents = Vec::new();
                        greedy_cubes(cubes, &mut vertices, &mut normals, &mut uv, &mut tangents);
                        mat_map.push(MaterialLayer {
                            material: *material,
                            vertices,
                            normals,
                            uv,
                            tangents,
                        });
                    }
                }
                //greedy_cubes(&mut layer_cubes, &mut vertices, &mut normals, &mut uv);
            }

            //println!("Vertices: {}", vertices.len());

            //if vertices.len() == 0 {
            //    return None;
            //}

            //println!("{:#?}", vertices);

            let mut meshes = Vec::new();
            for mat in mat_map.drain(..) {
                let mut mesh = Mesh::new(bevy::render::pipeline::PrimitiveTopology::TriangleList);
                mesh.set_attribute(
                    Mesh::ATTRIBUTE_POSITION,
                    VertexAttributeValues::Float3(mat.vertices),
                );
                mesh.set_attribute(
                    Mesh::ATTRIBUTE_NORMAL,
                    VertexAttributeValues::Float3(mat.normals),
                );
                mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, VertexAttributeValues::Float2(mat.uv));
                mesh.set_attribute(Mesh::ATTRIBUTE_TANGENT, VertexAttributeValues::Float3(mat.tangents));
                meshes.push((mat.material, mesh));
            }

            return Some(meshes);
        }
    }
    None
}

const GEOMETRY_SIZE: f32 = 1.0;

const CUBE_NORMALS: [[f32;3];6] = [
    [0.0, 0.0, -1.0],
    [0.0, 0.0, 1.0],
    [-1.0, 0.0, 0.0],
    [1.0, 0.0, 0.0],
    [0.0, -1.0, 0.0],
    [0.0, 1.0, 0.0],
];

const CUBE_TANGENTS: [[f32;3];6] = [
    [-1.0, 0.0, 0.0],
    [1.0, 0.0, 0.0],
    [0.0, 0.0, 1.0],
    [0.0, 0.0, -1.0],
    [1.0, 0.0, 0.0],
    [-1.0, 0.0, 0.0],
];

pub fn add_cube_geometry(
    vertices: &mut Vec<[f32; 3]>,
    normals: &mut Vec<[f32; 3]>,
    uv: &mut Vec<[f32; 2]>,
    tangents: &mut Vec<[f32; 3]>,
    x: f32,
    y: f32,
    z: f32,
    w: f32,
    h: f32,
    d: f32,
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

    #[rustfmt::skip]
    const TANGENT_GEOMETRY: [[f32; 3]; 36] = [
        CUBE_TANGENTS[0],
        CUBE_TANGENTS[0],
        CUBE_TANGENTS[0],
        CUBE_TANGENTS[0],
        CUBE_TANGENTS[0],
        CUBE_TANGENTS[0],

        CUBE_TANGENTS[1],
        CUBE_TANGENTS[1],
        CUBE_TANGENTS[1],
        CUBE_TANGENTS[1],
        CUBE_TANGENTS[1],
        CUBE_TANGENTS[1],

        CUBE_TANGENTS[2],
        CUBE_TANGENTS[2],
        CUBE_TANGENTS[2],
        CUBE_TANGENTS[2],
        CUBE_TANGENTS[2],
        CUBE_TANGENTS[2],

        CUBE_TANGENTS[3],
        CUBE_TANGENTS[3],
        CUBE_TANGENTS[3],
        CUBE_TANGENTS[3],
        CUBE_TANGENTS[3],
        CUBE_TANGENTS[3],

        CUBE_TANGENTS[4],
        CUBE_TANGENTS[4],
        CUBE_TANGENTS[4],
        CUBE_TANGENTS[4],
        CUBE_TANGENTS[4],
        CUBE_TANGENTS[4],

        CUBE_TANGENTS[5],
        CUBE_TANGENTS[5],
        CUBE_TANGENTS[5],
        CUBE_TANGENTS[5],
        CUBE_TANGENTS[5],
        CUBE_TANGENTS[5],
    ];
    tangents.extend_from_slice(&TANGENT_GEOMETRY);

    let tw = w;
    let th = h;
    #[rustfmt::skip]
    let uv_base: [[f32; 2]; 36] = [
        [0.0, 0.0],
        [tw, th],
        [tw, 0.0],
        [tw, th],
        [0.0, 0.0],
        [0.0, th],

        [0.0, 0.0],
        [tw, 0.0],
        [tw, th],
        [tw, th],
        [0.0, th],
        [0.0, 0.0],

        [tw, th],
        [tw, 0.0],
        [0.0, 0.0],
        [0.0, 0.0],
        [0.0, th],
        [tw, th],

        [tw, th],
        [0.0, 0.0],
        [tw, 0.0],
        [0.0, 0.0],
        [tw, th],
        [0.0, th],

        [tw, th],
        [tw, 0.0],
        [0.0, 0.0],
        [0.0, 0.0],
        [0.0, th],
        [tw, th],

        [tw, th],
        [tw, 0.0],
        [0.0, 0.0],
        [0.0, 0.0],
        [0.0, th],
        [tw, th],
    ];

    uv.extend_from_slice(&uv_base);
}
