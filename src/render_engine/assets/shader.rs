use std::fs;

use super::super::RENDER_CONTEXT;
use wgpu::ShaderModule;

pub struct Shader {
    pub fs_module: ShaderModule,
    pub vs_module: ShaderModule,
}

impl Shader {
    pub fn from_source_file<S: ToString, S1: ToString>(
        name: S1,
        vertex_filename: S,
        fragment_filename: S,
    ) -> Self {
        let vs = fs::read_to_string(vertex_filename.to_string())
            .expect("Unable to read vertex shader source file");
        let fs = fs::read_to_string(fragment_filename.to_string())
            .expect("Unable to read fragment source file");
        Self::from_source(name, vs, fs)
    }

    pub fn from_source<S: ToString, S1: ToString>(
        name: S1,
        vertex_source: S,
        frag_source: S,
    ) -> Self {
        let mut gpu_lock = RENDER_CONTEXT.write();
        let gpu = gpu_lock.as_mut().unwrap();
        let mut compiler = shaderc::Compiler::new().unwrap();
        let options = shaderc::CompileOptions::new().unwrap();
        let vertex_bin = compiler
            .compile_into_spirv(
                &vertex_source.to_string(),
                shaderc::ShaderKind::Vertex,
                "vs",
                "main",
                Some(&options),
            )
            .expect(&format!(
                "Failed to compile vertex shader [{}]",
                name.to_string()
            ));
        let fragment_bin = compiler
            .compile_into_spirv(
                &frag_source.to_string(),
                shaderc::ShaderKind::Fragment,
                "fs",
                "main",
                Some(&options),
            )
            .expect(&format!(
                "Failed to compile fragment shader [{}]",
                name.to_string()
            ));

        let vs_data = wgpu::util::make_spirv(vertex_bin.as_binary_u8());
        let fs_data = wgpu::util::make_spirv(fragment_bin.as_binary_u8());

        let vs_module = gpu
            .device
            .create_shader_module(&wgpu::ShaderModuleDescriptor {
                label: Some("Vertex Shader"),
                source: vs_data,
                flags: wgpu::ShaderFlags::default(),
            });
        let fs_module = gpu
            .device
            .create_shader_module(&wgpu::ShaderModuleDescriptor {
                label: Some("Fragment Shader"),
                source: fs_data,
                flags: wgpu::ShaderFlags::default(),
            });

        Shader {
            vs_module,
            fs_module,
        }
    }
}
