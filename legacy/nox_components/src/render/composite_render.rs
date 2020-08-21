use crate::prelude::*;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct VoxLayer {
    pub model: usize,
    pub tint: (f32, f32, f32),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CompositeRender {
    pub layers: Vec<VoxLayer>,
    pub rotation: f32,
}
