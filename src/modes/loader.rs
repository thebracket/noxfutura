use crate::opengl::*;
use super::helpers::render_menu_background;

pub struct Loader {
    counter: i32,
}

impl Loader {
    pub fn new() -> Self {
        Self { counter: 0 }
    }

    pub fn tick(
        &mut self,
        ui: &imgui::Ui,
        gl: &super::Gl,
        resources: &super::resources::SharedResources
    ) -> super::ProgramMode {
        //super::helpers::render_menu_background(context, frame, resources);
        render_menu_background(gl, resources);

        let window = imgui::Window::new(im_str!("Nox Futura is Loading"));
        window
            .size([300.0, 100.0], Condition::FirstUseEver)
            .build(ui, || {
                ui.text(im_str!("Flipping bits at random..."));
            });

        crate::raws::load_raws();

        self.counter += 1;

        if self.counter < 1 {
            super::ProgramMode::Loader
        } else {
            super::ProgramMode::MainMenu
        }
    }
}
