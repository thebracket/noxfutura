use bevy::{prelude::Mesh, render::mesh::VertexAttributeValues};

use super::{
    greedy::greedy_cubes, ChunkIterator, ChunkLocation, PlanetLocation, RampDirection, TileType,
    REGIONS,
};
use crate::simulation::{CHUNK_SIZE, REGION_HEIGHT, REGION_WIDTH};
use std::collections::{HashMap, HashSet};

pub struct RenderChunk {
    pub region: PlanetLocation,
    pub location: ChunkLocation,
    pub layers: Option<Vec<RenderChunkLayer>>,
}

impl RenderChunk {
    fn empty(region: PlanetLocation, location: ChunkLocation) -> Self {
        Self {
            region,
            location,
            layers: None,
        }
    }
}

#[derive(Clone)]
pub struct MaterialBuffer {
    pub material: usize,
    pub cubes: HashSet<usize>,
    pub ramps: Vec<(usize, RampDirection)>,
}

impl MaterialBuffer {
    fn new(material: usize) -> Self {
        Self {
            material,
            cubes: HashSet::new(),
            ramps: Vec::new(),
        }
    }

    fn create_geometry(&mut self) -> Option<Mesh> {
        if self.cubes.is_empty() && self.ramps.is_empty() {
            return None; // Nothing to do here
        }

        let mut vertices = Vec::new();
        let mut normals = Vec::new();
        let mut uv = Vec::new();
        let mut tangents = Vec::new();
        greedy_cubes(
            &mut self.cubes,
            &mut vertices,
            &mut normals,
            &mut uv,
            &mut tangents,
        );

        if vertices.is_empty() {
            return None;
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
        mesh.set_attribute(
            Mesh::ATTRIBUTE_TANGENT,
            VertexAttributeValues::Float3(tangents),
        );
        Some(mesh)
    }
}

#[derive(Clone)]
pub struct RenderChunkLayer {
    pub location: ChunkLocation,
    pub region_id: PlanetLocation,
    materials: HashMap<usize, MaterialBuffer>,
    pub meshes: Option<Vec<(usize, Mesh)>>,
}

impl RenderChunkLayer {
    fn new(region_id: PlanetLocation, location: ChunkLocation) -> Self {
        Self {
            location,
            region_id,
            materials: HashMap::new(),
            meshes: None,
        }
    }

    pub fn to_world(&self) -> (f32, f32, f32) {
        (
            (self.region_id.x * REGION_WIDTH) as f32,
            (self.region_id.y * REGION_HEIGHT) as f32,
            0.0,
        )
    }

    fn add_cube(&mut self, material: usize, idx: usize) {
        if let Some(mb) = self.materials.get_mut(&material) {
            mb.cubes.insert(idx);
        } else {
            let mut mb = MaterialBuffer::new(material);
            mb.cubes.insert(idx);
            self.materials.insert(material, mb);
        }
    }

    fn add_ramp(&mut self, material: usize, idx: usize, direction: RampDirection) {
        if let Some(mb) = self.materials.get_mut(&material) {
            mb.ramps.push((idx, direction));
        } else {
            let mut mb = MaterialBuffer::new(material);
            mb.ramps.push((idx, direction));
            self.materials.insert(material, mb);
        }
    }

    fn create_geometry(&mut self) -> Vec<(usize, Mesh)> {
        let mut layer_meshes = Vec::new();
        for (material, buffer) in self.materials.iter_mut() {
            let maybe_mesh = buffer.create_geometry();
            if maybe_mesh.is_some() {
                layer_meshes.push((*material, maybe_mesh.unwrap()));
            }
        }
        layer_meshes
    }
}

pub fn build_render_chunk(region_id: PlanetLocation, location: ChunkLocation) -> RenderChunk {
    if is_chunk_empty(region_id, location) {
        RenderChunk::empty(region_id, location)
    } else {
        let mut chunk = RenderChunk {
            region: region_id,
            location,
            layers: Some(Vec::with_capacity(CHUNK_SIZE)),
        };

        // Build the basic geometric elements
        let region_lock = REGIONS.read();
        let region_idx = region_id.to_region_index();
        if let Some(region) = region_lock.regions.get(&region_idx) {
            for layer in 0..CHUNK_SIZE {
                let mut local_location = location;
                local_location.z += layer;
                chunk
                    .layers
                    .as_mut()
                    .unwrap()
                    .push(RenderChunkLayer::new(region_id, local_location));
            }

            ChunkIterator::new(location)
                .map(|loc| (loc, loc.to_tile_index()))
                .filter(|(_, idx)| region.revealed[*idx])
                .filter(|(_, idx)| region.tile_types[*idx] != TileType::Empty)
                .for_each(|(loc, idx)| {
                    let material = region.material[idx];
                    let layer = loc.z - location.z;
                    match region.tile_types[idx] {
                        TileType::SemiMoltenRock => {
                            chunk.layers.as_mut().unwrap()[layer].add_cube(material, idx)
                        }
                        TileType::Solid => {
                            chunk.layers.as_mut().unwrap()[layer].add_cube(material, idx)
                        }
                        TileType::Ramp { direction } => {
                            chunk.layers.as_mut().unwrap()[layer].add_ramp(material, idx, direction)
                        }
                        _ => {}
                    }
                });

            for layer in chunk.layers.as_mut().unwrap().iter_mut() {
                let mesh_list = layer.create_geometry();
                if mesh_list.is_empty() {
                    layer.meshes = None;
                } else {
                    layer.meshes = Some(mesh_list);
                }
            }

            chunk
        } else {
            println!("Returning none due to region inaccessible");
            RenderChunk::empty(region_id, location)
        }
    }
}

fn is_chunk_empty(region: PlanetLocation, location: ChunkLocation) -> bool {
    let region_lock = REGIONS.read();
    let region_id = region.to_region_index();
    if let Some(region) = region_lock.regions.get(&region_id) {
        let visible_chunks = ChunkIterator::new(location)
            .map(|loc| loc.to_tile_index())
            .filter(|idx| region.revealed[*idx])
            .filter(|idx| region.tile_types[*idx] != TileType::Empty)
            .count();
        if visible_chunks > 0 {
            return false;
        }
    }
    true
}
