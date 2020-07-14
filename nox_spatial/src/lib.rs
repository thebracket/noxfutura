mod indices;
pub use indices::*;

pub const WORLD_HEIGHT: usize = 90;
pub const WORLD_WIDTH: usize = 180;
pub const WORLD_TILES_COUNT: usize = WORLD_HEIGHT * WORLD_WIDTH;
pub const REGION_WIDTH: usize = 256;
pub const REGION_HEIGHT: usize = 256;
pub const REGION_DEPTH: usize = 256;
pub const REGION_TILES_COUNT: usize = (REGION_WIDTH * REGION_HEIGHT * REGION_DEPTH) as usize;
