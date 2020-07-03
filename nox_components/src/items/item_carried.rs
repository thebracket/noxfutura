use crate::prelude::*;

#[derive(TypeUuid, Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
#[uuid = "3b0a6bf0-3b96-4622-9848-9f10a6d50f15"]
pub struct ItemCarried {
    pub wearer: usize,
}
