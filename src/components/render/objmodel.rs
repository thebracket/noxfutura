use crate::components::prelude::*;

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub struct ObjModel {
    pub index: usize,
    pub rotation_radians: f32,
}
