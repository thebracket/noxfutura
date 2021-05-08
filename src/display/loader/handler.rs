use crate::render_engine::{GameMode, TickResult, pipeline2d, render_nf_background};
use egui::CtxRef;
use super::asset_loader::{ LOADER, LoaderState };

pub struct Loader {
    pipeline: Option<wgpu::RenderPipeline>,
    started_thread: bool,
}

impl Loader {
    pub fn new() -> Self {
        Self { pipeline: None, started_thread: false }
    }
}

impl GameMode for Loader {
    fn init(&mut self) {
        self.pipeline = Some(pipeline2d());
    }

    fn tick(&mut self, egui: &CtxRef, swap_chain_texture: &wgpu::SwapChainTexture) -> TickResult {
        if !self.started_thread {
            LoaderState::start_loading();
        }

        render_nf_background(&self.pipeline, swap_chain_texture);

        egui::Window::new("Nox Futura")
            .auto_sized()
            .show(egui, |ui| {
                ui.label(format!("Progress: {}%", (LOADER.read().progress * 100.0) as u32));
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
