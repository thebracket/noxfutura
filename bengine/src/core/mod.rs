mod init;
use imgui::Ui;
use crate::{ Buffers };
pub use init::Initializer;

pub struct Core<'a> {
    pub imgui: &'a Ui::<'a>,
    pub buffers: &'a mut Buffers,
    pub frame: &'a wgpu::SwapChainFrame,
}