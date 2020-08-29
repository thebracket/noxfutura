#[macro_use]
extern crate lazy_static;

mod assets;
mod core;
mod game;
pub mod helpers;
mod imgui_wgpu;
mod render_core;

pub use crate::assets::*;
pub use crate::core::Core;
pub use crate::render_core::*;
pub use game::BEngineGame;

pub mod gui {
    pub use imgui::*;
}

pub mod gpu {
    pub use wgpu::*;
}

pub use crate::core::run;

pub fn get_window_size() -> winit::dpi::PhysicalSize<u32> {
    RENDER_CONTEXT.read().as_ref().unwrap().size
}
