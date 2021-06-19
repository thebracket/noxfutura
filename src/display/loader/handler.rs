use super::asset_loader::{LoaderState, LOADER};
use crate::render_engine::{pipeline2d, render_nf_background, GameMode, TickResult};
use egui::CtxRef;
use winit::dpi::PhysicalSize;

pub struct Loader {
    pipeline: Option<wgpu::RenderPipeline>,
    started_thread: bool,
}

impl Loader {
    pub fn new() -> Self {
        Self {
            pipeline: None,
            started_thread: false,
        }
    }
}

impl GameMode for Loader {
    fn init(&mut self) {
        self.pipeline = Some(pipeline2d());
    }

    fn tick(
        &mut self,
        _size: PhysicalSize<u32>,
        egui: &CtxRef,
        swap_chain_texture: &wgpu::SwapChainTexture,
    ) -> TickResult {
        if !self.started_thread {
            LoaderState::start_loading();
        }

        render_nf_background(&self.pipeline, swap_chain_texture);

        egui::Window::new("Nox Futura")
            .auto_sized()
            .show(egui, |ui| {
                ui.label(format!(
                    "Progress: {}%",
                    (LOADER.read().progress * 100.0) as u32
                ));
                ui.label(LOADER.read().status.clone());
            });

        let finished = LOADER.read().done;
        if finished {
            TickResult::MainMenu
        } else {
            TickResult::Continue
        }
    }
}
