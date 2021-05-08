use crate::prelude::*;

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub struct ObjModel {
    pub index: usize,
    pub rotation_radians: f32,
    pub scale: f32,
}
