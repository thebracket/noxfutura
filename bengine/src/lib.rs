#[macro_use]
extern crate lazy_static;

mod assets;
mod core;
mod game;
pub mod helpers;
mod imgui_wgpu;
mod nf;
mod render_core;

pub use crate::assets::*;
pub use crate::core::Core;
pub use crate::render_core::*;
pub use game::BEngineGame;
pub use nf::*;
pub use winit::event::VirtualKeyCode;
pub mod random {
    pub use bracket_random::prelude::*;
}
pub mod geometry {
    pub use bracket_geometry::prelude::*;
}
pub mod vox {
    pub use dot_vox::*;
}
pub mod noise {
    pub use bracket_noise::prelude::*;
}
pub mod uv {
    pub use ultraviolet::*;
}

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
