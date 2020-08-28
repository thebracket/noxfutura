use glsl_to_spirv::compile;
use std::fs;
use wgpu::{Device, ShaderModule};
use crate::RENDER_CONTEXT;

pub(crate) fn from_spv(source: wgpu::ShaderModuleSource) -> ShaderModule {
    let rcl = RENDER_CONTEXT.read();
    let rc = rcl.as_ref().unwrap();
    rc.device.create_shader_module(source)
}