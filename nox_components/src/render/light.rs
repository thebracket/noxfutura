use crate::prelude::*;

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub struct Light {
    pub color: (f32, f32, f32),
    pub radius: usize,
    pub enabled: bool,
}
