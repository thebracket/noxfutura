use crate::{GameMode, NoxMode, SharedResources};
use bengine::*;

pub struct Loader {}

impl Loader {
    pub fn new() -> Self {
        Self{}
    }
}

impl NoxMode for Loader {
    fn tick(&mut self, core: &mut Core, shared: &SharedResources) -> GameMode {
        shared.quad_render.render(shared.background_image, core);

        GameMode::MainMenu
    }
}
