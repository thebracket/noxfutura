use super::resources::SharedResources;
use imgui::*;

pub struct Loader {
    counter: i32,
}

impl Loader {
    pub fn new() -> Self {
        Self { counter: 0 }
    }

    pub fn tick(
        &mut self,
        resources: &SharedResources,
        frame: &wgpu::SwapChainOutput,
        context: &mut crate::engine::Context,
        ui: &imgui::Ui,
    ) -> super::ProgramMode {
        super::helpers::render_menu_background(context, frame, resources);

        let window = imgui::Window::new(im_str!("Nox Futura is Loading"));
        window
            .size([300.0, 100.0], Condition::FirstUseEver)
            .build(ui, || {
                ui.text(im_str!("Flipping bits at random..."));
            });

        self.counter += 1;

        if self.counter < 1 {
            super::ProgramMode::Loader
        } else {
            super::ProgramMode::MainMenu
        }
    }
}
