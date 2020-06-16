mod rpass_gbuffer_tester;
mod rpass_terrain_to_gbuffer;
mod rpass_vox_to_gbuffer;
pub use rpass_terrain_to_gbuffer::*;
pub use rpass_gbuffer_tester::*;
pub use rpass_vox_to_gbuffer::*;
mod camera;
mod gbuffer;
mod texarray;
mod uniforms;
pub mod frustrum;

#[derive(Clone)]
pub struct ChunkModel {
    pub id : usize,
    pub x : usize,
    pub y : usize,
    pub z : usize
}