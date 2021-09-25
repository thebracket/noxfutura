use std::collections::HashSet;
use bevy::{prelude::Mesh, render::mesh::VertexAttributeValues};
use crate::simulation::terrain::RampDirection;
use super::greedy::{greedy_cubes, greedy_floors};

/// A MaterialBuffer stores a set of materials and primitives required
/// to render a RenderChunk. They are abstract until greedy-meshing
/// kicks in (in `create_geometry`). This returns a data-set containing
/// everything you need to make a Bevy Mesh.
#[derive(Clone)]
pub struct MaterialBuffer {
    pub material: usize,
    pub cubes: HashSet<usize>,
    pub floors: HashSet<usize>,
    pub ramps: Vec<(usize, RampDirection)>,
}

impl MaterialBuffer {
    pub(crate) fn new(material: usize) -> Self {
        Self {
            material,
            cubes: HashSet::new(),
            ramps: Vec::new(),
            floors: HashSet::new(),
        }
    }

    pub(crate) fn create_geometry(&mut self) -> Option<Mesh> {
        if self.cubes.is_empty() && self.ramps.is_empty() && self.floors.is_empty() {
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
        greedy_floors(
            &mut self.floors,
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
