mod loadstate;
mod messaging;
mod play;
mod render;
mod run_state;
mod systems;
mod ui;
mod uniforms;

pub use messaging::*;
pub use play::PlayTheGame;
pub use render::{
    Chunks, CursorPass, GBuffer, GrassPass, LightingPass, Models, ModelsPass, TerrainPass, VoxPass,
};
pub use run_state::*;
pub use uniforms::{Camera, CameraUniform};
pub use systems::RNG;

pub struct GameStateResource {
    keycode: Option<bengine::VirtualKeyCode>,
    pub camera_changed: bool,
    pub vox_moved: bool,
    pub models_moved: bool,
    pub lights_changed: bool,
    pub dirty_tiles: Vec<usize>,
}

impl GameStateResource {
    pub fn new() -> Self {
        Self {
            keycode: None,
            camera_changed: false,
            vox_moved: false,
            models_moved: false,
            lights_changed: false,
            dirty_tiles: Vec::new(),
        }
    }

    pub fn frame_update(&mut self, keycode: Option<bengine::VirtualKeyCode>) {
        self.keycode = keycode;
    }
}
