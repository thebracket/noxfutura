use crate::{GameMode, NoxMode, SharedResources};
use bengine::*;
use bengine::gui::*;

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

        gui::Window::new(im_str!("Status"))
            .position([10.0, 10.0], Condition::Always)
            .always_auto_resize(true)
            .collapsible(false)
            .build(core.imgui, || {
                core.imgui.text(ImString::new(crate::planet::get_worldgen_status()));
            });

        result
    }
}
