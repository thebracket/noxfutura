use crate::render_engine::{pipeline2d, render_nf_background, GameMode, TickResult};
use egui::{Color32, CtxRef};
mod tagline;
use tagline::tagline;
use winit::dpi::PhysicalSize;

pub struct MainMenu {
    pipeline: Option<wgpu::RenderPipeline>,
    tagline: String,
}

impl MainMenu {
    pub fn new() -> Self {
        Self {
            pipeline: None,
            tagline: tagline(),
        }
    }
}

impl GameMode for MainMenu {
    fn pre_init(&mut self) {
        self.pipeline = Some(pipeline2d());
    }

    fn tick(
        &mut self,
        _size: PhysicalSize<u32>,
        egui: &CtxRef,
        swap_chain_texture: &wgpu::SwapChainTexture,
    ) -> TickResult {
        render_nf_background(&self.pipeline, swap_chain_texture);

        let mut result = TickResult::Continue;

        egui::Window::new("Tagline")
            .title_bar(false)
            .auto_sized()
            .show(egui, |ui| {
                ui.colored_label(Color32::from_rgb(255, 0, 0), self.tagline.clone());
            });

        egui::Window::new("Nox Futura")
            .auto_sized()
            .show(egui, |ui| {
                if ui.button("Create World").clicked() {
                    result = TickResult::WorldGen;
                }

                if ui.button("Quit").clicked() {
                    result = TickResult::Quit;
                }
            });

        egui::Window::new("Dedication")
            .title_bar(false)
            .auto_sized()
            .show(egui, |ui| {
                ui.colored_label(Color32::from_rgb(255, 255, 255), "To Kylah of the West and Jakie Monster -\nThe Bravest Little Warriors of Them All.");
            });

        egui::Window::new("Copyright")
            .title_bar(false)
            .auto_sized()
            .show(egui, |ui| {
                ui.colored_label(
                    Color32::from_rgb(255, 255, 0),
                    "(c) 2015-2020 Bracket Productions, All Rights Reserved.",
                );
            });

        result
    }
}
