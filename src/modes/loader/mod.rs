use crate::{GameMode, NoxMode, SharedResources};
use bengine::*;
mod asset_loader;
pub use asset_loader::loader_progress;
use bengine::gui::*;

pub struct Loader {
    started_thread: bool,
}

impl Loader {
    pub fn new() -> Self {
        Self {
            started_thread: false,
        }
    }
}

impl NoxMode for Loader {
    fn tick(&mut self, core: &mut Core, shared: &SharedResources) -> GameMode {
        use asset_loader::*;
        shared.quad_render.render(shared.background_image, core);

        if !self.started_thread {
            LoaderState::start_loading();
            self.started_thread = true;
        }

        let load_lock = LOADER.read();
        let load_state = load_lock.status.clone();
        let progress = load_lock.progress;
        let done = load_lock.done;
        std::mem::drop(load_lock);

        let window = gui::Window::new(im_str!("Nox Futura is Loading"));
        window
            .size([300.0, 100.0], Condition::FirstUseEver)
            .build(core.imgui, || {
                ProgressBar::new(progress)
                    .size([250.0, 20.0])
                    .build(core.imgui);

                core.imgui.text(&load_state);
            });

        if !done {
            GameMode::Loader
        } else {
            GameMode::MainMenu
        }
    }
}
