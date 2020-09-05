mod loadstate;
mod play;
mod render;
mod systems;
mod uniforms;

pub use play::PlayTheGame;
pub use render::{Chunks, GBuffer, Palette, TerrainPass, Model, ModelsPass};
pub use uniforms::{Camera, CameraUniform};

pub struct GameStateResource {
    keycode: Option<bengine::VirtualKeyCode>,
    pub camera_changed: bool,
}

impl GameStateResource {
    pub fn new() -> Self {
        Self {
            keycode: None,
            camera_changed: false,
        }
    }

    pub fn frame_update(&mut self, keycode: Option<bengine::VirtualKeyCode>) {
        self.keycode = keycode;
    }
}
