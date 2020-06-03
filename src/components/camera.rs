use super::prelude::*;

#[derive(TypeUuid, Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
#[uuid = "8235310d-4285-4636-a3e6-ba13e3f3feac"]
pub struct CameraOptions {
    pub zoom_level : i32
}