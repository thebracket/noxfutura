mod main_loop;

use imgui::Ui;

pub struct Core<'a> {
    pub imgui: &'a Ui<'a>,
    pub frame: &'a wgpu::SwapChainFrame,
}

pub use main_loop::run;