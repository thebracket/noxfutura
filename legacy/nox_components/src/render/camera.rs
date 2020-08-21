use crate::prelude::*;

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum CameraMode {
    TopDown,
    Front,
    DiagonalNW,
    DiagonalNE,
    DiagonalSW,
    DiagonalSE,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub struct CameraOptions {
    pub zoom_level: i32,
    pub mode: CameraMode,
}
