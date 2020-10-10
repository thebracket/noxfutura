use crate::prelude::*;

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum WorkOrder {
    None,
    MoveRandomly,
}
