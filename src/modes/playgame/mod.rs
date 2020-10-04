mod loadstate;
mod play;
mod render;
mod systems;
mod ui;
mod uniforms;
mod run_state;
mod messaging;

pub use run_state::*;
pub use play::PlayTheGame;
pub use render::{
    Chunks, GBuffer, GrassPass, LightingPass, Models, ModelsPass, Palette, TerrainPass, VoxPass,
    CursorPass
};
pub use uniforms::{Camera, CameraUniform};
pub use messaging::*;

pub struct GameStateResource {
    keycode: Option<bengine::VirtualKeyCode>,
    pub camera_changed: bool,
    pub vox_moved: bool,
    pub models_moved: bool
}

impl GameStateResource {
    pub fn new() -> Self {
        Self {
            keycode: None,
            camera_changed: false,
            vox_moved: false,
            models_moved: false
        }
    }

    pub fn frame_update(&mut self, keycode: Option<bengine::VirtualKeyCode>) {
        self.keycode = keycode;
    }
}
