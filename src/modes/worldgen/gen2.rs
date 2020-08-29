use crate::{GameMode, NoxMode, SharedResources};
use bengine::*;

pub struct WorldGen2 {
}

impl WorldGen2 {
    pub fn new() -> Self {
        Self {}
    }
}

impl NoxMode for WorldGen2 {
    fn tick(&mut self, core: &mut Core, shared: &SharedResources) -> GameMode {
        //use gui::*;

        let result = GameMode::WorldGen2;
        shared.quad_render.render(shared.background_image, core);

        result
    }
}
