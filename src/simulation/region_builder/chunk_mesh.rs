use crate::simulation::{chunk_idx, CHUNK_DEPTH, CHUNK_HEIGHT, CHUNK_WIDTH};
use bevy::{prelude::Mesh, render::mesh::VertexAttributeValues};

use super::chunker::{Chunk, ChunkType, TileType};

pub fn chunk_to_mesh(chunk: &Chunk) -> Option<Mesh> {
    match chunk.chunk_type {
        ChunkType::Populated => populated_chunk_to_mesh(chunk),
        _ => None,
    }
}

fn populated_chunk_to_mesh(chunk: &Chunk) -> Option<Mesh> {
    if let Some(tiles) = &chunk.tiles {
        let mut vertices = Vec::new();
        let mut normals = Vec::new();
        let mut uv = Vec::new();

        // TODO: Be greedy

        for z in 0..CHUNK_DEPTH {
            for y in 0..CHUNK_HEIGHT {
                for x in 0..CHUNK_WIDTH {
                    match tiles[chunk_idx(x, y, z)] {
                        TileType::Solid { .. } => add_cube_geometry(
                            &mut vertices,
                            &mut normals,
                            &mut uv,
                            x as f32 + chunk.base.0 as f32,
                            y as f32 + chunk.base.1 as f32,
                            z as f32 + chunk.base.2 as f32,
                            1.0,
                            1.0,
                            1.0,
                        ),
                        _ => {}
                    }
                }
            }
        }

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

        Some(mesh)
    } else {
        None
    }
}

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
    let x0 = x;
    let x1 = x0 + w;
    let y0 = z;
    let y1 = y0 + d - 0.1;
    let z0 = y;
    let z1 = z0 + h;

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
    for _ in 0..36 {
        normals.push([0.0, 0.0, 0.0]);
        uv.push([0.0, 0.0]);
    }
}
