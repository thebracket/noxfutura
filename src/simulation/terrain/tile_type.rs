#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RampDirection {
    NorthSouth,
    SouthNorth,
    EastWest,
    WestEast,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum StairsType {
    Up,
    Down,
    UpDown,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TileType {
    Empty,
    SemiMoltenRock,
    Solid,
    Floor,
    Ramp { direction: RampDirection },
    Stairs { direction: StairsType },
}
