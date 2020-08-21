use crate::prelude::*;

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub struct Tint {
    pub color: (f32, f32, f32),
}
