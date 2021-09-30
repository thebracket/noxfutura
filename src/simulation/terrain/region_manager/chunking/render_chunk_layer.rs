use super::material_buffer::MaterialBuffer;
use crate::components::PlanetLocation;
use crate::simulation::{
    terrain::{ChunkLocation, RampDirection},
    REGION_HEIGHT, REGION_WIDTH,
};
use bevy::prelude::Mesh;
use std::collections::HashMap;

/// A RenderChunkLayer represents a z-layer inside a chunk, transitioning into a renderable
/// mesh. One layer per CHUNK z. Actual division of labor and geometry is handled by the
/// `MaterialBuffer`.
#[derive(Clone)]
pub struct RenderChunkLayer {
    pub location: ChunkLocation,
    pub region_id: PlanetLocation,
    materials: HashMap<usize, MaterialBuffer>,
    pub meshes: Option<Vec<(usize, Mesh)>>,
}

impl RenderChunkLayer {
    pub(crate) fn new(region_id: PlanetLocation, location: ChunkLocation) -> Self {
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

    pub(crate) fn add_cube(&mut self, material: usize, idx: usize) {
        if let Some(mb) = self.materials.get_mut(&material) {
            mb.cubes.insert(idx);
        } else {
            let mut mb = MaterialBuffer::new(material);
            mb.cubes.insert(idx);
            self.materials.insert(material, mb);
        }
    }

    pub(crate) fn add_topless_cube(&mut self, material: usize, idx: usize) {
        if let Some(mb) = self.materials.get_mut(&material) {
            mb.topless_cubes.insert(idx);
        } else {
            let mut mb = MaterialBuffer::new(material);
            mb.topless_cubes.insert(idx);
            self.materials.insert(material, mb);
        }
    }

    pub(crate) fn add_floor(&mut self, material: usize, idx: usize) {
        if let Some(mb) = self.materials.get_mut(&material) {
            mb.floors.insert(idx);
        } else {
            let mut mb = MaterialBuffer::new(material);
            mb.floors.insert(idx);
            self.materials.insert(material, mb);
        }
    }

    pub(crate) fn add_ramp(&mut self, material: usize, idx: usize, direction: RampDirection) {
        if let Some(mb) = self.materials.get_mut(&material) {
            mb.ramps.push((idx, direction));
        } else {
            let mut mb = MaterialBuffer::new(material);
            mb.ramps.push((idx, direction));
            self.materials.insert(material, mb);
        }
    }

    pub(crate) fn create_geometry(&mut self) -> Vec<(usize, Mesh)> {
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
