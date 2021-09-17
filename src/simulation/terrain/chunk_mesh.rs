use crate::simulation::{chunk_idx, mapidx, CHUNK_SIZE};
use bevy::{prelude::Mesh, render::mesh::VertexAttributeValues};

use super::{
    chunker::{Chunk, ChunkType, TileType},
    greedy::{greedy_cubes, CubeMap},
};

pub fn chunk_to_mesh(chunk: &Chunk) -> Option<Mesh> {
    match chunk.chunk_type {
        ChunkType::Populated => populated_chunk_to_mesh(chunk),
        _ => None,
    }
}

fn populated_chunk_to_mesh(chunk: &Chunk) -> Option<Mesh> {
    if let Some(tiles) = &chunk.tiles {
        if let Some(revealed) = &chunk.revealed {
            if revealed.iter().filter(|r| **r==true).count() == 0 {
                // Bail out if there are no visible tiles
                return None;
            }

            let mut vertices = Vec::new();
            let mut normals = Vec::new();
            let mut uv = Vec::new();

            for z in 0..CHUNK_SIZE {
                let mut layer_cubes = CubeMap::new();
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
                                    layer_cubes.insert(idx, (0, false));
                                }
                                TileType::Solid { .. } => {
                                    let idx = mapidx(
                                        x + chunk.base.0,
                                        y + chunk.base.1,
                                        z + chunk.base.2,
                                    );
                                    layer_cubes.insert(idx, (0, false));
                                }
                                _ => {}
                            }
                        }
                    }
                }
                greedy_cubes(&mut layer_cubes, &mut vertices, &mut normals, &mut uv);
            }

            //println!("Vertices: {}", vertices.len());

            if vertices.len() == 0 {
                return None;
            }

            //println!("{:#?}", vertices);

            let mut mesh = Mesh::new(bevy::render::pipeline::PrimitiveTopology::TriangleList);
            mesh.set_attribute(
                Mesh::ATTRIBUTE_POSITION,
                VertexAttributeValues::Float3(vertices),
            );
            mesh.set_attribute(
                Mesh::ATTRIBUTE_NORMAL,
                VertexAttributeValues::Float3(normals),
            );
            mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, VertexAttributeValues::Float2(uv));

            return Some(mesh);
        }
    }
    None
}

const GEOMETRY_SIZE: f32 = 1.0;

pub fn add_cube_geometry(
    vertices: &mut Vec<[f32; 3]>,
    normals: &mut Vec<[f32; 3]>,
    uv: &mut Vec<[f32; 2]>,
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

    let normal_geometry = [
        [0.0, 0.0, -1.0],
        [0.0, 0.0, -1.0],
        [0.0, 0.0, -1.0],
        [0.0, 0.0, -1.0],
        [0.0, 0.0, -1.0],
        [0.0, 0.0, -1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [-1.0, 0.0, 0.0],
        [-1.0, 0.0, 0.0],
        [-1.0, 0.0, 0.0],
        [-1.0, 0.0, 0.0],
        [-1.0, 0.0, 0.0],
        [-1.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, -1.0, 0.0],
        [0.0, -1.0, 0.0],
        [0.0, -1.0, 0.0],
        [0.0, -1.0, 0.0],
        [0.0, -1.0, 0.0],
        [0.0, -1.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
    ];
    normals.extend_from_slice(&normal_geometry);

    for _ in 0..36 {
        uv.push([0.0, 0.0]);
    }
}
