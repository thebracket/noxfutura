use crate::components::prelude::*;

#[derive(TypeUuid, Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
#[uuid = "b26b41ce-91cb-4b4d-a38b-aa55d7e5508a"]
pub struct ItemStored {
    pub container: usize,
}
