use std::fs;

pub struct Shader {
    pub vs_module : wgpu::ShaderModule,
    pub fs_module : wgpu::ShaderModule
}

impl Shader {
    pub fn from_source<S: ToString>(device: &wgpu::Device, vertex_filename : S, fragment_filename : S) -> Self {
        let vertex_fn = vertex_filename.to_string();
        let frag_fn = fragment_filename.to_string();

        let vs_src = fs::read_to_string(vertex_fn).unwrap();
        let fs_src = fs::read_to_string(frag_fn).unwrap();

        let vs_spirv = glsl_to_spirv::compile(&vs_src, glsl_to_spirv::ShaderType::Vertex).unwrap();
        let fs_spirv = glsl_to_spirv::compile(&fs_src, glsl_to_spirv::ShaderType::Fragment).unwrap();
        let vs_data = wgpu::read_spirv(vs_spirv).unwrap();
        let fs_data = wgpu::read_spirv(fs_spirv).unwrap();
        let vs_module = device.create_shader_module(&vs_data);
        let fs_module = device.create_shader_module(&fs_data);

        Shader {
            vs_module,
            fs_module
        }
    }
}