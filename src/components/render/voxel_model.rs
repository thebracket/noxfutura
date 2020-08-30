use crate::components::prelude::*;

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub struct VoxelModel {
    pub index: usize,
    pub rotation_radians: f32,
}
