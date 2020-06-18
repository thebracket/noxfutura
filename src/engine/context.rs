#![allow(dead_code)]
use super::shader::Shader;
use super::texture::Texture;

pub struct Context {
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub surface: wgpu::Surface,
    pub shaders: Vec<Shader>,
    pub textures: Vec<Texture>,
    pub swapchain_format: wgpu::TextureFormat,
}

impl<'a> Context {
    pub fn new(
        adapter: wgpu::Adapter,
        device: wgpu::Device,
        queue: wgpu::Queue,
        size: winit::dpi::PhysicalSize<u32>,
        surface: wgpu::Surface,
        swapchain_format: wgpu::TextureFormat,
    ) -> Self {
        Self {
            adapter,
            device,
            queue,
            size,
            surface,
            shaders: Vec::new(),
            textures: Vec::new(),
            swapchain_format,
        }
    }

    pub fn register_shader<S: ToString>(&mut self, vertex_src: S, frag_src: S) -> usize {
        let new_shader = Shader::from_source(&self.device, vertex_src, frag_src);
        self.shaders.push(new_shader);
        self.shaders.len() - 1
    }

    pub fn register_depth_texture(&mut self, label: &str) -> usize {
        let new_depth = Texture::create_depth_texture(&self.device, self.size, label);
        self.textures.push(new_depth);
        self.textures.len() - 1
    }

    pub fn register_texture(&mut self, bytes: &[u8], label: &str) -> usize {
        let (diffuse_texture, cmd_buffer) =
            Texture::from_bytes(&self.device, bytes, label).unwrap();
        self.queue.submit(&[cmd_buffer]);
        self.textures.push(diffuse_texture);
        self.textures.len() - 1
    }

    pub fn create_pipeline_layout(
        &self,
        bind_groups: &[&wgpu::BindGroupLayout],
    ) -> wgpu::PipelineLayout {
        self.device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                bind_group_layouts: bind_groups,
            })
    }
}
