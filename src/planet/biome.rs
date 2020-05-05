use bracket_geometry::prelude::Point;

#[derive(Clone)]
pub struct Biome {
    pub biome_type : usize,
    pub name : String,
    pub mean_temperature : i8,
    pub mean_rainfall : i8,
    pub mean_altitude : u8,
    pub mean_variance : u8,
    pub warp_mutation : u8,
    pub evil : u8,
    pub savagery : u8,
    pub center : Point
}