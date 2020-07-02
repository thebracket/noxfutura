use crate::components::prelude::*;

#[derive(TypeUuid, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[uuid = "d5922445-30a4-4a39-bd69-c336d0d9e092"]
pub struct Initiative {
    pub initiative: i32,
    pub modifier: i32,
}

impl Initiative {
    pub fn new() -> Self {
        Self{
            initiative: 0,
            modifier: 0
        }
    }
}
