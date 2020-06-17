use crate::components::prelude::*;

#[derive(TypeUuid, Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
#[uuid = "8764dfe9-b6ff-4ece-b134-90c9992d3b88"]
pub struct Tint {
    pub color: (f32, f32, f32)
}
