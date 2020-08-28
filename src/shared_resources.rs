use bengine::*;

pub struct SharedResources {
    pub background_image: usize,
    pub quad_render: helpers::BackgroundQuad,
}

impl SharedResources {
    pub fn new() -> Self {
        let background_image =
            helpers::texture_from_file("resources/images/background_image.png", "nox_bg");
        Self {
            background_image,
            quad_render: helpers::BackgroundQuad::new(),
        }
    }
}