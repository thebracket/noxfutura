use serde::{Deserialize, Serialize};

#[derive(PartialEq, Copy, Clone, Serialize, Deserialize, Debug)]
pub enum MiningMode {
    Dig,
    Channel,
    Ramp,
    Up,
    Down,
    UpDown,
    Clear,
}
