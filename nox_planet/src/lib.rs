#[macro_use]
extern crate lazy_static;

pub use nox_raws::BlockType;
mod block;
pub use block::Block;
mod biome;
pub use biome::Biome;
mod river;
pub use river::{River, RiverStep};
mod planet;
pub use planet::*;
mod builder;
pub use builder::*;
mod savedgame;
pub use savedgame::*;
mod region;
pub use region::*;
mod indices;
pub use indices::*;
mod sphere;
pub use sphere::*;
mod groundz;
pub use groundz::*;
mod worldgen_render;
pub use worldgen_render::*;
mod planet_render;
pub use planet_render::*;
mod rex;
pub use rex::*;

pub const WORLD_HEIGHT: usize = 90;
pub const WORLD_WIDTH: usize = 180;
pub const WORLD_TILES_COUNT: usize = WORLD_HEIGHT * WORLD_WIDTH;
pub const REGION_WIDTH: usize = 256;
pub const REGION_HEIGHT: usize = 256;
pub const REGION_DEPTH: usize = 256;
pub const REGION_TILES_COUNT: usize = (REGION_WIDTH * REGION_HEIGHT * REGION_DEPTH) as usize;