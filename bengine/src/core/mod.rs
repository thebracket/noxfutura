mod init;
use imgui::Ui;
pub use init::Initializer;

pub struct Core<'a> {
    pub imgui: &'a Ui<'a>,
    pub frame: &'a wgpu::SwapChainFrame,
}
