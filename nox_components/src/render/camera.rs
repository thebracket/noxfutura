use crate::prelude::*;

#[derive(TypeUuid, Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
#[uuid = "808fc3ff-e39d-48f2-a62f-84b83da66154"]
pub enum CameraMode {
    TopDown,
    Front,
    DiagonalNW,
    DiagonalNE,
    DiagonalSW,
    DiagonalSE,
}

#[derive(TypeUuid, Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
#[uuid = "8235310d-4285-4636-a3e6-ba13e3f3feac"]
pub struct CameraOptions {
    pub zoom_level: i32,
    pub mode: CameraMode,
}
