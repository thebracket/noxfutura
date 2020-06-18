use crate::modes::{resources::SharedResources, helpers, ProgramMode};
use imgui::*;
use super::LOADER;

pub struct Loader {
    started_thread : bool
}

impl Loader {
    pub fn new() -> Self {
        Self { 
            started_thread : false
        }
    }

    pub fn tick(
        &mut self,
        resources: &SharedResources,
        frame: &wgpu::SwapChainOutput,
        ui: &imgui::Ui,
    ) -> ProgramMode {
        helpers::render_menu_background(frame, resources);

        if !self.started_thread {
            super::LoaderState::start_loading();
            self.started_thread = true;
        }

        let load_lock = LOADER.read();
        let load_state = load_lock.status.clone();
        let progress = load_lock.progress;
        let done = load_lock.done;
        std::mem::drop(load_lock);

        let window = imgui::Window::new(im_str!("Nox Futura is Loading"));
        window
            .size([300.0, 100.0], Condition::FirstUseEver)
            .build(ui, || {
                ProgressBar::new(progress)
                    .size([250.0, 20.0])
                    .build(ui);

                ui.text(&load_state);
            });

        if !done {
            ProgramMode::Loader
        } else {
            ProgramMode::MainMenu
        }
    }
}
