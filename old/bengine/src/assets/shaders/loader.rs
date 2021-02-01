use crate::RENDER_CONTEXT;
use wgpu::ShaderModule;

pub(crate) fn from_spv(source: wgpu::ShaderModuleSource) -> ShaderModule {
    let rcl = RENDER_CONTEXT.read();
    let rc = rcl.as_ref().unwrap();
    rc.device.create_shader_module(source)
}
