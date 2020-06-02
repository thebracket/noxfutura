use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum TileType {
    Empty,
    Solid,
    Floor,
    Wall,
    Ramp { direction: RampDirection },
    Stairs { direction: StairsType },
    SemiMoltenRock,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum RampDirection {
    NorthSouth,
    SouthNorth,
    EastWest,
    WestEast,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum StairsType {
    Up,
    Down,
    UpDown,
}
