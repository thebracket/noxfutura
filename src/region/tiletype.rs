#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TileType {
    Empty,
    Solid,
    Floor,
    Wall,
    Ramp { direction: RampDirection },
    Stairs { direction: StairsType },
    SemiMoltenRock,
}

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
