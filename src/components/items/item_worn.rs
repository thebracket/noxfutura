use crate::components::prelude::*;

#[derive(TypeUuid, Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
#[uuid = "43dfa102-6b8f-49f7-880c-5369623e4c25"]
pub struct ItemWorn {
    pub wearer: usize,
}
