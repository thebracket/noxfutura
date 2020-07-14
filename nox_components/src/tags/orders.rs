use crate::prelude::*;

#[derive(TypeUuid, Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
#[uuid = "b5339d8c-eb62-44e6-b3d0-a87c2157dd81"]
pub enum WorkOrder {
    None,
    MoveRandomly,
}
