#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RampDirection {
    NorthSouth,
    SouthNorth,
    EastWest,
    WestEast,
}

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum StairsType {
    Up,
    Down,
    UpDown,
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TileType {
    Empty,
    SemiMoltenRock,
    Solid,
    Floor,
    Ramp { direction: RampDirection },
    Stairs { direction: StairsType },
}
