use std::collections::HashMap;

use crate::{geometry::add_ramp_geometry, simulation::{chunk_idx, mapidx, CHUNK_SIZE}};
use bevy::{prelude::Mesh, render::mesh::VertexAttributeValues};

use super::{chunker::{Chunk, ChunkType, RampDirection, TileType}, greedy::{greedy_cubes, CubeMap}};

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
                let mut material_ramps : HashMap<usize, Vec<(usize, RampDirection)>> = HashMap::new();
                for y in 0..CHUNK_SIZE {
                    for x in 0..CHUNK_SIZE {
                        let cidx = chunk_idx(x, y, z);
                        if revealed[cidx] {
                            let idx = mapidx(
                                x + chunk.base.0,
                                y + chunk.base.1,
                                z + chunk.base.2,
                            );
                            match tiles[cidx] {
                                TileType::SemiMoltenRock => {
                                    add_material(&mut material_layer_cubes, 1, idx);
                                }
                                TileType::Solid { material } => {
                                    add_material(&mut material_layer_cubes, material, idx);
                                }
                                TileType::Ramp { direction, material } => {
                                    // Add ramp to the material list
                                    if let Some(r) = material_ramps.get_mut(&material) {
                                        r.push((idx, direction));
                                    } else {
                                        material_ramps.insert(material, vec![(idx, direction)]);
                                    }
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
                        let mut new_material_layer = MaterialLayer {
                            material: *material,
                            vertices,
                            normals,
                            uv,
                            tangents,
                        };
                        if let Some(ramps) = material_ramps.get(material) {
                            for &(ramp_idx, dir) in ramps {
                                /*add_ramp_geometry(
                                    dir,
                                    ramp_idx,
                                    &mut new_material_layer.vertices,
                                    &mut new_material_layer.normals,
                                    &mut new_material_layer.uv,
                                    &mut new_material_layer.tangents
                                );*/
                            }
                        }
                        mat_map.push(new_material_layer);
                    }
                }
            }

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

