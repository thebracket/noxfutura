use super::prelude::*;

#[derive(TypeUuid, Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
#[uuid = "5d58d328-13fd-4635-8f57-27f6ac3c8469"]
pub struct WorldPosition {
    pub planet_x : usize,
    pub planet_y : usize
}
