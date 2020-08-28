use crate::{GameMode, NoxMode};
use bengine::*;

pub struct Loader {
    background_image: usize,
    quad_render: helpers::BackgroundQuad,
}

impl Loader {
    pub fn new() -> Self {
        let background_image =
            helpers::texture_from_file("resources/images/background_image.png", "nox_bg");
        Self {
            background_image,
            quad_render: helpers::BackgroundQuad::new(),
        }
    }
}

impl NoxMode for Loader {
    fn tick(&mut self, core: &mut Core) -> GameMode {
        self.quad_render.render(self.background_image, core);

        GameMode::Loader
    }
}
