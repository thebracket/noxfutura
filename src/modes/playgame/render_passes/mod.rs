mod rpass_terrain_to_gbuffer;
mod rpass_vox_to_gbuffer;
mod rpass_sunlight;
pub use rpass_terrain_to_gbuffer::*;
pub use rpass_vox_to_gbuffer::*;
pub use rpass_sunlight::*;
pub mod frustrum;
mod gbuffer;
mod texarray;
mod uniforms;
use uniforms::*;

#[derive(Clone)]
pub struct ChunkModel {
    pub id: usize,
    pub x: usize,
    pub y: usize,
    pub z: usize,
}
