use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum TileType {
    Empty,
    Solid,
    Floor{ plant: Option<usize> },
    Wall,
    Ramp { direction: RampDirection },
    Stairs { direction: StairsType },
    SemiMoltenRock,
    Window,
    TreeTrunk { tree_id: usize },
    TreeFoliage { tree_id: usize }
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
