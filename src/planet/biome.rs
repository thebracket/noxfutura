use bracket_geometry::prelude::Point;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Biome {
    pub biome_type: usize,
    pub name: String,
    pub mean_temperature: i8,
    pub mean_rainfall: i8,
    pub mean_altitude: u8,
    pub mean_variance: u8,
    pub warp_mutation: u8,
    pub evil: u8,
    pub savagery: u8,
    pub center: Point,
}

impl Biome {
    pub fn new() -> Self {
        Self {
            biome_type: std::usize::MAX,
            name: String::new(),
            mean_temperature: 0,
            mean_altitude: 0,
            mean_rainfall: 0,
            mean_variance: 0,
            warp_mutation: 0,
            evil: 0,
            savagery: 0,
            center: Point::zero(),
        }
    }
}
