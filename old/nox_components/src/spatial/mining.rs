use crate::prelude::*;

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum MiningMode {
    Dig,
    Channel,
    Ramp,
    Up,
    Down,
    UpDown,
    Clear,
}
