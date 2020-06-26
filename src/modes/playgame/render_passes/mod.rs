mod rpass_gbuffer_tester;
mod rpass_terrain_to_gbuffer;
mod rpass_vox_to_gbuffer;
mod rpass_sun_to_depth_terrain;
mod rpass_sun_to_depth_vox;
mod rpass_sunlight;
pub use rpass_gbuffer_tester::*;
pub use rpass_terrain_to_gbuffer::*;
pub use rpass_vox_to_gbuffer::*;
pub use rpass_sun_to_depth_terrain::*;
pub use rpass_sun_to_depth_vox::*;
pub use rpass_sunlight::*;
mod camera;
pub mod frustrum;
mod gbuffer;
mod texarray;
mod uniforms;

#[derive(Clone)]
pub struct ChunkModel {
    pub id: usize,
    pub x: usize,
    pub y: usize,
    pub z: usize,
}
