use glsl_to_spirv::compile;
use std::fs;
use wgpu::{Device, ShaderModule};

pub enum ShaderType {
    Vertex, Fragment
}

pub(crate) fn from_source<S: ToString>(filename: S, shader_type: ShaderType, device: &Device) -> ShaderModule {
    panic!("This doesn't currently work.");
    let filename = filename.to_string();
    let source = fs::read_to_string(&filename).expect("Unable to read shader source");

    let mut spirv_file = match shader_type {
        ShaderType::Vertex => compile(&source, glsl_to_spirv::ShaderType::Vertex),
        ShaderType::Fragment => compile(&source, glsl_to_spirv::ShaderType::Fragment),
    }.expect(&format!("Failed to compile shader {}", &filename));

    use std::io::Read;
    let mut buffer : Vec<u8> = Vec::new();
    buffer.resize(spirv_file.metadata().unwrap().len() as usize, 0);
    spirv_file.read(&mut buffer).expect("Cannot read compiled file");

    device.create_shader_module(wgpu::util::make_spirv(&buffer))
}

pub(crate) fn from_spv(shader_type: ShaderType, device: &Device, source: wgpu::ShaderModuleSource) -> ShaderModule {
    device.create_shader_module(source)
}