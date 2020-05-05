use super::resources::SharedResources;
use imgui::*;

pub struct PlanetGen2 {
}

impl PlanetGen2 {
    pub fn new() -> Self {
        Self {}
    }

    pub fn tick(
        &mut self,
        resources: &SharedResources,
        frame: &wgpu::SwapChainOutput,
        context: &mut crate::engine::Context,
        ui: &imgui::Ui,
    ) -> super::ProgramMode {
        super::helpers::render_menu_background(context, frame, resources);

        imgui::Window::new(im_str!("Status"))
            .position([10.0, 10.0], Condition::Always)
            .always_auto_resize(true)
            .collapsible(false)
            .build(ui, || {
                ui.text(ImString::new(crate::planet::get_worldgen_status()));
            }
        );

        super::ProgramMode::PlanetGen2
    }
}
