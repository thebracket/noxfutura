use super::prelude::*;
mod orders;
pub use orders::*;

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub struct Cordex {}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub struct Building {
    pub complete: bool,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub struct Workshop {
    pub has_automatic_jobs: bool
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub struct Item {}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub struct Sentient {}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub struct Settler {
    pub miner: bool,
    pub lumberjack: bool,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub struct Vegetation {
    pub size: f32,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub struct Tree {}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub struct Terrain {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Tag(pub String);
