#[macro_use]
extern crate lazy_static;

mod assets;
mod core;
mod game;
mod imgui_wgpu;
mod render_core;
pub mod helpers;

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
