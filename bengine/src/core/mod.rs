mod init;
use imgui::Ui;
use crate::{ Textures, Buffers };
pub use init::Initializer;

pub struct Core<'a> {
    pub imgui: &'a Ui::<'a>,
    pub textures: &'a mut Textures,
    pub buffers: &'a mut Buffers,
    pub frame: &'a wgpu::SwapChainFrame,
    pub device: &'a wgpu::Device,
    pub queue: &'a mut wgpu::Queue,
}