use crate::components::prelude::*;

#[derive(TypeUuid, Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
#[uuid = "b2723658-d31b-4759-8488-81e3b7bb35ab"]
pub struct VoxelModel {
    pub index: usize
}
